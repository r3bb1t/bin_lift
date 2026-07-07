//! The streaming lifting engine. It calls a `Lifter` for one instruction's body,
//! then resolves any control-transfer need via the fixed precedence
//! AssumptionProvider -> Oracle -> Suspend. It never applies a built-in heuristic.

use crate::address::Va;
use crate::assume::{AssumptionProvider, PredicateVerdict};
use crate::event::{BranchKind, Event, EventKind};
use crate::insn::InsnView;
use crate::ir::IrBuilder;
use crate::lifter::{Lifter, Transfer};
use crate::memory::MemoryFacts;
use crate::oracle::Oracle;
use crate::protocol::{Answer, Query, ResumeToken, StepOutcome};

/// A resolution the engine is waiting on when suspended.
#[derive(Clone, Copy, Debug)]
enum Pending {
    Branch { from: Va, taken_target: Va, fallthrough: Va },
    Indirect { site: Va },
}

pub struct Session<L, A: AssumptionProvider, O: Oracle> {
    lifter: L,
    assume: A,
    oracle: O,
    facts: MemoryFacts,
    entry: Va,
    pending: Option<(ResumeToken, Pending)>,
    next_token: u64,
}

impl<L, A: AssumptionProvider, O: Oracle> Session<L, A, O> {
    /// Build a session. The IR backend is NOT owned here — it is injected
    /// per call into [`Session::step`]/[`Session::resume`]. This keeps the
    /// `Session` free of the backend's brand lifetime, so a real
    /// `LlvmkitBuilder` (whose brand cannot escape `Module::with_new`) can
    /// live inside that closure while the `Session` is created outside it.
    pub fn new(lifter: L, assume: A, oracle: O, facts: MemoryFacts, entry: Va) -> Self {
        Self {
            lifter,
            assume,
            oracle,
            facts,
            entry,
            pending: None,
            next_token: 0,
        }
    }

    pub fn assumptions(&self) -> &A {
        &self.assume
    }

    pub fn facts(&self) -> &MemoryFacts {
        &self.facts
    }

    pub fn entry(&self) -> Va {
        self.entry
    }

    fn issue_token(&mut self) -> ResumeToken {
        let t = ResumeToken(self.next_token);
        self.next_token += 1;
        t
    }

    /// Lift one instruction, emitting IR into the injected backend `builder`.
    pub fn step<B: IrBuilder>(&mut self, insn: &dyn InsnView, builder: &mut B) -> StepOutcome
    where
        L: Lifter<B>,
    {
        assert!(self.pending.is_none(), "call resume() before step() while suspended");
        let from = insn.address();
        let transfer = self.lifter.lift_body(insn, builder);
        match transfer {
            Transfer::Fallthrough => StepOutcome::Continue(Vec::new()),
            Transfer::Static { kind, to, taken } => self.finish_static(builder, from, kind, to, taken),
            Transfer::ResolveBranch { taken_target, fallthrough } => {
                self.resolve_branch(builder, from, taken_target, fallthrough)
            }
            Transfer::ResolveIndirect { site } => self.resolve_indirect(builder, site),
            Transfer::Call { target } => {
                let ev = Event::new(from, EventKind::Call { from, target });
                StepOutcome::Continue(vec![ev])
            }
            Transfer::ReturnTo { to } => self.finish_return(builder, from, to),
            Transfer::End => StepOutcome::BlockEnd(Vec::new()),
        }
    }

    /// Answer a suspension and continue, emitting any terminator IR into the
    /// injected `builder`. Pass the SAME backend instance used for the
    /// `step` that suspended.
    pub fn resume<B: IrBuilder>(
        &mut self,
        builder: &mut B,
        token: ResumeToken,
        answer: Answer,
    ) -> StepOutcome {
        let (expected, pending) = self.pending.take().expect("resume() with no pending suspension");
        assert_eq!(token, expected, "resume token mismatch");
        match (pending, answer) {
            (Pending::Branch { from, taken_target, fallthrough }, Answer::BranchVerdict { taken }) => {
                let to = if taken { taken_target } else { fallthrough };
                self.finish_conditional(builder, from, to, taken)
            }
            (Pending::Indirect { site }, Answer::Targets(targets)) => {
                self.finish_indirect(builder, site, targets)
            }
            _ => panic!("resume answer does not match pending query"),
        }
    }

    fn resolve_branch<B: IrBuilder>(
        &mut self,
        builder: &mut B,
        from: Va,
        taken_target: Va,
        fallthrough: Va,
    ) -> StepOutcome {
        let taken = match self.assume.predicate(from) {
            PredicateVerdict::AlwaysTaken => Some(true),
            PredicateVerdict::NeverTaken => Some(false),
            PredicateVerdict::Unknown => self.oracle.resolve_branch(from),
        };
        match taken {
            Some(t) => {
                let to = if t { taken_target } else { fallthrough };
                self.finish_conditional(builder, from, to, t)
            }
            None => {
                let token = self.issue_token();
                self.pending = Some((token, Pending::Branch { from, taken_target, fallthrough }));
                StepOutcome::Suspend(Query::BranchVerdict { site: from, taken_target, fallthrough }, token)
            }
        }
    }

    fn resolve_indirect<B: IrBuilder>(&mut self, builder: &mut B, site: Va) -> StepOutcome {
        let targets = self
            .assume
            .indirect_targets(site)
            .or_else(|| self.oracle.resolve_indirect(site));
        match targets {
            Some(ts) => self.finish_indirect(builder, site, ts),
            None => {
                let token = self.issue_token();
                self.pending = Some((token, Pending::Indirect { site }));
                StepOutcome::Suspend(Query::IndirectTargets { site }, token)
            }
        }
    }

    fn finish_static<B: IrBuilder>(
        &mut self,
        builder: &mut B,
        from: Va,
        kind: BranchKind,
        to: Va,
        taken: bool,
    ) -> StepOutcome {
        // M1 records the address op for the builder-log tests; M2 wires this value into the real terminator.
        let v = builder.const_addr(to);
        let block = builder.new_block("target");
        builder.br(&block);
        let _ = v;
        StepOutcome::BlockEnd(vec![Event::new(from, EventKind::Branch { kind, from, to, taken })])
    }

    fn finish_conditional<B: IrBuilder>(
        &mut self,
        builder: &mut B,
        from: Va,
        to: Va,
        taken: bool,
    ) -> StepOutcome {
        let v = builder.const_addr(to);
        let block = builder.new_block("target");
        builder.br(&block);
        let _ = v;
        StepOutcome::BlockEnd(vec![Event::new(
            from,
            EventKind::Branch { kind: BranchKind::Conditional, from, to, taken },
        )])
    }

    fn finish_indirect<B: IrBuilder>(
        &mut self,
        builder: &mut B,
        site: Va,
        targets: Vec<Va>,
    ) -> StepOutcome {
        match targets.len() {
            0 => StepOutcome::BlockEnd(vec![Event::new(site, EventKind::Unsupported { addr: site })]),
            1 => {
                let to = targets[0];
                let v = builder.const_addr(to);
                let block = builder.new_block("indirect_target");
                builder.br(&block);
                let _ = v;
                StepOutcome::BlockEnd(vec![Event::new(
                    site,
                    EventKind::Branch { kind: BranchKind::Indirect, from: site, to, taken: true },
                )])
            }
            _ => {
                let scrut = builder.const_addr(site);
                let default = builder.new_block("switch_default");
                let cases: Vec<(Va, B::Block)> = targets
                    .iter()
                    .map(|&t| (t, builder.new_block("case")))
                    .collect();
                builder.switch(scrut, &default, &cases);
                StepOutcome::BlockEnd(vec![Event::new(
                    site,
                    EventKind::Branch { kind: BranchKind::Indirect, from: site, to: targets[0], taken: true },
                )])
            }
        }
    }

    fn finish_return<B: IrBuilder>(&mut self, builder: &mut B, from: Va, to: Va) -> StepOutcome {
        let v = builder.const_addr(to);
        builder.ret(v);
        StepOutcome::BlockEnd(vec![Event::new(from, EventKind::Return { from, to })])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assume::{MapAssumptions, NoAssumptions, PredicateVerdict};
    use crate::insn::FakeInsn;
    use crate::ir::RecordingBuilder;
    use crate::lifter::FakeLifter;
    use crate::oracle::{NullOracle, ScriptedOracle};

    fn bare_session() -> Session<FakeLifter, NoAssumptions, NullOracle> {
        Session::new(FakeLifter, NoAssumptions, NullOracle, MemoryFacts::new(), 0x1000)
    }

    #[test]
    fn nop_continues_without_events() {
        let mut s = bare_session();
        let mut b = RecordingBuilder::new();
        let out = s.step(&FakeInsn::nop(0x1000), &mut b);
        assert_eq!(out, StepOutcome::Continue(Vec::new()));
    }

    #[test]
    fn direct_jump_emits_branch_and_block_end() {
        let mut s = bare_session();
        let mut b = RecordingBuilder::new();
        let out = s.step(&FakeInsn::with(0x1000, 5, "jmp 0x2000"), &mut b);
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x1000,
                EventKind::Branch {
                    kind: BranchKind::Unconditional,
                    from: 0x1000,
                    to: 0x2000,
                    taken: true,
                },
            )])
        );
        assert!(b.log.iter().any(|l| l == "const 0x2000"));
    }

    #[test]
    fn conditional_branch_with_no_facts_suspends_then_resumes() {
        let mut s = bare_session();
        let mut b = RecordingBuilder::new();
        // jcc taken=0x40 fallthrough=0x32 at 0x30
        let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"), &mut b);
        let token = match out {
            StepOutcome::Suspend(Query::BranchVerdict { site, taken_target, fallthrough }, tok) => {
                assert_eq!((site, taken_target, fallthrough), (0x30, 0x40, 0x32));
                tok
            }
            other => panic!("expected suspend, got {other:?}"),
        };
        let out = s.resume(&mut b, token, Answer::BranchVerdict { taken: true });
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x30,
                EventKind::Branch { kind: BranchKind::Conditional, from: 0x30, to: 0x40, taken: true },
            )])
        );
    }

    #[test]
    fn conditional_branch_resolved_by_declared_predicate_does_not_suspend() {
        let mut a = MapAssumptions::default();
        a.preds.insert(0x30, PredicateVerdict::NeverTaken);
        let mut s = Session::new(FakeLifter, a, NullOracle, MemoryFacts::new(), 0x1000);
        let mut b = RecordingBuilder::new();
        let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"), &mut b);
        // NeverTaken -> falls through to 0x32, no suspend.
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x30,
                EventKind::Branch { kind: BranchKind::Conditional, from: 0x30, to: 0x32, taken: false },
            )])
        );
    }

    #[test]
    fn indirect_branch_resolved_by_oracle_without_suspend() {
        let mut o = ScriptedOracle::default();
        o.indirects.insert(0x50, vec![0x900]);
        let mut s = Session::new(FakeLifter, NoAssumptions, o, MemoryFacts::new(), 0x1000);
        let mut b = RecordingBuilder::new();
        let out = s.step(&FakeInsn::with(0x50, 2, "jmp_ind"), &mut b);
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x50,
                EventKind::Branch { kind: BranchKind::Indirect, from: 0x50, to: 0x900, taken: true },
            )])
        );
    }

    #[test]
    #[should_panic(expected = "resume() before step()")]
    fn stepping_while_suspended_panics() {
        let mut s = bare_session();
        let mut b = RecordingBuilder::new();
        let _ = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"), &mut b); // suspends
        let _ = s.step(&FakeInsn::nop(0x40), &mut b); // misuse
    }
}
