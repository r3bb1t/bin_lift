//! M3 de-risking spike proof: a `lift_core::Session` — created OUTSIDE the
//! brand closure — drives a real `LlvmkitBuilder` created INSIDE
//! `Module::with_new`, and real `.ll` comes out.
//!
//! This is the key composition artifact the M3 x86 handlers and the eventual
//! facade reuse: because the refactored `Session` no longer owns the builder
//! (it takes `&mut B` per `step`/`resume`), it carries none of llvmkit's
//! `for<'brand>` brand lifetime. So the `Session` lives outside the closure
//! and the brand-scoped `LlvmkitBuilder` lives inside it — they compose by
//! passing the builder in per call.
//!
//! The lifter here is `lift_core::FakeLifter`, which is generic over any
//! `B: IrBuilder` and so drives `LlvmkitBuilder` unchanged (the same handlers
//! that drive `RecordingBuilder` in lift-core's unit tests). All operands are
//! constants/addresses because `FakeLifter` only emits control-flow
//! terminators (`br`/`ret`) plus a `const_addr`; that's exactly the streaming
//! control-flow slice this spike needs to prove.

use lift_core::{
    Answer, FakeInsn, FakeLifter, IrBuilder, MemoryFacts, NoAssumptions, NullOracle, Query,
    Session, StepOutcome,
};
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

/// A `Session` created OUTSIDE `with_new` drives an `LlvmkitBuilder` created
/// INSIDE it. We step a `ret 0x2000` instruction: `FakeLifter` maps it to
/// `Transfer::ReturnTo`, and `Session::finish_return` emits `const_addr` +
/// `ret` into the injected builder. The resulting `.ll` must contain a real
/// `ret i64 8192` (0x2000).
#[test]
fn session_drives_llvmkit_builder_inside_with_new() {
    // The Session is constructed here, entirely outside the brand closure —
    // proof that it carries no brand lifetime.
    let mut session: Session<FakeLifter, NoAssumptions, NullOracle> =
        Session::new(FakeLifter, NoAssumptions, NullOracle, MemoryFacts::new(), 0x1000);

    let ll = Module::with_new("session_ret", |m| {
        let typed = m.add_typed_function::<i64, (), _>("lifted", Linkage::External)?;
        let f = typed.as_function().as_dyn();

        // The brand-scoped backend is created INSIDE the closure...
        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);

        // ...and the Session (from outside) drives it by &mut per step.
        let out = session.step(&FakeInsn::with(0x1234, 3, "ret 0x2000"), &mut b);
        assert!(
            matches!(out, StepOutcome::BlockEnd(_)),
            "ret should end the block, got {out:?}"
        );

        Ok::<String, IrError>(b.finish())
    })
    .expect("session-driven build should succeed");

    println!("=== session_ret.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'session_ret'
define i64 @lifted() {
entry:
  ret i64 8192
}
";
    assert_eq!(ll, expected);
}

/// The full streaming shape: a `step` that SUSPENDS (conditional branch with
/// no facts) and a `resume` that emits the terminator — all with the same
/// brand-scoped `LlvmkitBuilder` inside one `with_new`. Proves suspend/resume
/// composes with the closure-scoped backend, not just a single `step`.
#[test]
fn suspend_then_resume_drives_llvmkit_builder() {
    let mut session: Session<FakeLifter, NoAssumptions, NullOracle> =
        Session::new(FakeLifter, NoAssumptions, NullOracle, MemoryFacts::new(), 0x1000);

    let ll = Module::with_new("session_condbr", |m| {
        let typed = m.add_typed_function::<i64, (), _>("lifted", Linkage::External)?;
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);

        // jcc taken=0x40 fallthrough=0x32 at 0x30: no facts => SUSPEND.
        let out = session.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"), &mut b);
        let token = match out {
            StepOutcome::Suspend(Query::BranchVerdict { site, .. }, tok) => {
                assert_eq!(site, 0x30);
                tok
            }
            other => panic!("expected suspend with no facts, got {other:?}"),
        };

        // Answer taken=true => finish_conditional emits const_addr(0x40) + br
        // into a fresh block, ending the block.
        let out = session.resume(&mut b, token, Answer::BranchVerdict { taken: true });
        assert!(
            matches!(out, StepOutcome::BlockEnd(_)),
            "resume should end the block, got {out:?}"
        );

        Ok::<String, IrError>(b.finish())
    })
    .expect("suspend/resume-driven build should succeed");

    println!("=== session_condbr.ll ===\n{ll}");

    // The engine's finish_conditional emits `const_addr(0x40)` (folded away —
    // it is an unused constant) then `br label %target`. The entry block ends
    // with that unconditional branch; the `target` block is left un-terminated
    // (the streaming engine hasn't lifted its body yet), which is exactly the
    // in-progress control-flow graph a real function walk produces.
    assert!(
        ll.contains("br label %target"),
        "expected a real branch terminator in:\n{ll}"
    );
}
