//! Golden `.ll` tests for `LlvmkitBuilder`'s control-flow `IrBuilder` impl:
//! `new_block`, `position_at`, `br`, `cond_br`, `switch`, `ret`.
//!
//! All non-`br`/`unreachable` operands are function PARAMS (never bare
//! constants): llvmkit's `ConstantFolder` is on by default, so constant-only
//! arithmetic folds away — irrelevant to pure control-flow terminators
//! themselves, but we route through params anyway to mirror the spike's
//! proven `passthrough` shape exactly and to keep these tests representative
//! of how a real lifter will drive the builder.

use lift_core::IrBuilder;
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

/// Mirrors the spike's `spike_multi_block_branch`: `i64 @passthrough(i64 %0)`
/// with entry unconditionally branching to `tail`, which returns the param.
#[test]
fn br_entry_to_tail_ret_param() {
    let ll = Module::with_new("cf_br", |m| {
        let typed = m.add_typed_function::<i64, (i64,), _>("passthrough", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        let tail = b.new_block("tail");

        b.position_at(&entry);
        b.br(&tail);

        b.position_at(&tail);
        b.ret(x.as_value());

        Ok::<String, IrError>(b.finish())
    })
    .expect("br build should succeed");

    println!("=== br.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cf_br'
define i64 @passthrough(i64 %0) {
entry:
  br label %tail

tail:
  ret i64 %0
}
";
    assert_eq!(ll, expected);
}

/// `cond_br`: an `i1` param selects between two blocks, each returning a
/// distinct `i64` param.
#[test]
fn cond_br_i1_param_branches_two_ways() {
    let ll = Module::with_new("cf_condbr", |m| {
        let typed =
            m.add_typed_function::<i64, (bool, i64, i64), _>("pick", Linkage::External)?;
        let (cond, t, e) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        let then_bb = b.new_block("then");
        let else_bb = b.new_block("else");

        b.position_at(&entry);
        b.cond_br(cond.as_value(), &then_bb, &else_bb);

        b.position_at(&then_bb);
        b.ret(t.as_value());

        b.position_at(&else_bb);
        b.ret(e.as_value());

        Ok::<String, IrError>(b.finish())
    })
    .expect("cond_br build should succeed");

    println!("=== cond_br.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cf_condbr'
define i64 @pick(i1 %0, i64 %1, i64 %2) {
entry:
  br i1 %0, label %then, label %else

then:
  ret i64 %1

else:
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

/// `switch`: a 3-way switch (2 explicit cases + default) on an `i64` param,
/// each destination returning a distinct `i64` param.
#[test]
fn switch_two_cases_plus_default() {
    let ll = Module::with_new("cf_switch", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64, i64, i64), _>(
            "classify",
            Linkage::External,
        )?;
        let (scrutinee, r1, r2, rd) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        let case1_bb = b.new_block("case1");
        let case2_bb = b.new_block("case2");
        let default_bb = b.new_block("default");

        b.position_at(&entry);
        b.switch(
            scrutinee.as_value(),
            &default_bb,
            &[(1, case1_bb), (2, case2_bb)],
        );

        b.position_at(&case1_bb);
        b.ret(r1.as_value());

        b.position_at(&case2_bb);
        b.ret(r2.as_value());

        b.position_at(&default_bb);
        b.ret(rd.as_value());

        Ok::<String, IrError>(b.finish())
    })
    .expect("switch build should succeed");

    println!("=== switch.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cf_switch'
define i64 @classify(i64 %0, i64 %1, i64 %2, i64 %3) {
entry:
  switch i64 %0, label %default [
    i64 1, label %case1
    i64 2, label %case2
  ]

case1:
  ret i64 %1

case2:
  ret i64 %2

default:
  ret i64 %3
}
";
    assert_eq!(ll, expected);
}
