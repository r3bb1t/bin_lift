//! Pins the core invariant: with no declared facts and a null oracle, the engine
//! must SUSPEND at an unresolved branch rather than silently assuming; once a fact
//! is declared, the same input resolves without suspending.

use lift_core::{
    Answer, BranchKind, EventKind, FakeInsn, FakeLifter, MapAssumptions, MemoryFacts,
    NoAssumptions, NullOracle, PredicateVerdict, Query, Session, StepOutcome,
};

fn make_session_no_facts() -> Session<FakeLifter, NoAssumptions, NullOracle> {
    Session::new(
        FakeLifter,
        lift_core::RecordingBuilder::new(),
        NoAssumptions,
        NullOracle,
        MemoryFacts::new(),
        0x1000,
    )
}

#[test]
fn no_facts_forces_suspension_not_assumption() {
    let mut s = make_session_no_facts();
    let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
    match out {
        StepOutcome::Suspend(Query::BranchVerdict { site, .. }, _) => assert_eq!(site, 0x30),
        other => panic!("expected suspension with no facts, got {other:?}"),
    }
}

#[test]
fn declaring_a_predicate_fact_resolves_without_suspension() {
    let mut a = MapAssumptions::default();
    a.preds.insert(0x30, PredicateVerdict::AlwaysTaken);
    let mut s = Session::new(
        FakeLifter,
        lift_core::RecordingBuilder::new(),
        a,
        NullOracle,
        MemoryFacts::new(),
        0x1000,
    );
    let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
    match out {
        StepOutcome::BlockEnd(events) => {
            assert_eq!(events.len(), 1);
            assert_eq!(
                events[0].kind,
                EventKind::Branch {
                    kind: BranchKind::Conditional,
                    from: 0x30,
                    to: 0x40,
                    taken: true,
                }
            );
        }
        StepOutcome::Suspend(..) => panic!("declared fact should have prevented suspension"),
        other => panic!("unexpected outcome {other:?}"),
    }
}

#[test]
fn indirect_with_no_facts_suspends_then_resume_targets() {
    let mut s = make_session_no_facts();
    let out = s.step(&FakeInsn::with(0x50, 2, "jmp_ind"));
    let token = match out {
        StepOutcome::Suspend(Query::IndirectTargets { site }, tok) => {
            assert_eq!(site, 0x50);
            tok
        }
        other => panic!("expected indirect suspension, got {other:?}"),
    };
    let out = s.resume(token, Answer::Targets(vec![0x900, 0xa00]));
    assert!(matches!(out, StepOutcome::BlockEnd(_)));
}
