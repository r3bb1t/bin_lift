//! Golden `.ll` tests for `LlvmkitBuilder`'s compares / casts / select
//! `IrBuilder` impl: `icmp`, `zext/sext/trunc/zext_or_trunc`, `select`.
//!
//! All operands are function PARAMS (never bare constants): llvmkit's
//! `ConstantFolder` is on by default, so constant-only arithmetic/casts fold
//! away before they ever become an instruction.

use lift_core::{IcmpPred, IrBuilder};
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

#[test]
fn icmp_ult_two_params() {
    let ll = Module::with_new("cmp_ult", |m| {
        let typed = m.add_typed_function::<bool, (i64, i64), _>("ult_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.icmp(IcmpPred::Ult, x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("icmp ult build should succeed");

    println!("=== icmp_ult.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cmp_ult'
define i1 @ult_fn(i64 %0, i64 %1) {
entry:
  %2 = icmp ult i64 %0, %1
  ret i1 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn icmp_eq_two_params() {
    let ll = Module::with_new("cmp_eq", |m| {
        let typed = m.add_typed_function::<bool, (i64, i64), _>("eq_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.icmp(IcmpPred::Eq, x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("icmp eq build should succeed");

    println!("=== icmp_eq.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cmp_eq'
define i1 @eq_fn(i64 %0, i64 %1) {
entry:
  %2 = icmp eq i64 %0, %1
  ret i1 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn icmp_slt_two_params() {
    let ll = Module::with_new("cmp_slt", |m| {
        let typed = m.add_typed_function::<bool, (i64, i64), _>("slt_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.icmp(IcmpPred::Slt, x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("icmp slt build should succeed");

    println!("=== icmp_slt.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cmp_slt'
define i1 @slt_fn(i64 %0, i64 %1) {
entry:
  %2 = icmp slt i64 %0, %1
  ret i1 %2
}
";
    assert_eq!(ll, expected);
}

/// `trunc i64 -> i32`: narrowing cast, one real instruction.
#[test]
fn trunc_i64_param_to_i32() {
    let ll = Module::with_new("cast_trunc", |m| {
        let typed = m.add_typed_function::<i32, (i64,), _>("trunc_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.trunc(x.as_value(), 32);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("trunc build should succeed");

    println!("=== trunc.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cast_trunc'
define i32 @trunc_fn(i64 %0) {
entry:
  %1 = trunc i64 %0 to i32
  ret i32 %1
}
";
    assert_eq!(ll, expected);
}

/// `zext i32 -> i64`: widening cast, one real instruction.
#[test]
fn zext_i32_param_to_i64() {
    let ll = Module::with_new("cast_zext", |m| {
        let typed = m.add_typed_function::<i64, (i32,), _>("zext_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.zext(x.as_value(), 64);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("zext build should succeed");

    println!("=== zext.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cast_zext'
define i64 @zext_fn(i32 %0) {
entry:
  %1 = zext i32 %0 to i64
  ret i64 %1
}
";
    assert_eq!(ll, expected);
}

/// `sext i8 -> i32`: widening signed cast, one real instruction.
#[test]
fn sext_i8_param_to_i32() {
    let ll = Module::with_new("cast_sext", |m| {
        let typed = m.add_typed_function::<i32, (i8,), _>("sext_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.sext(x.as_value(), 32);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("sext build should succeed");

    println!("=== sext.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cast_sext'
define i32 @sext_fn(i8 %0) {
entry:
  %1 = sext i8 %0 to i32
  ret i32 %1
}
";
    assert_eq!(ll, expected);
}

/// `zext_or_trunc` widening (i32 -> i64): behaves like `zext`, one real
/// instruction.
#[test]
fn zext_or_trunc_widens_like_zext() {
    let ll = Module::with_new("cast_zot_widen", |m| {
        let typed = m.add_typed_function::<i64, (i32,), _>("zot_widen_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.zext_or_trunc(x.as_value(), 64);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("zext_or_trunc widen build should succeed");

    println!("=== zext_or_trunc_widen.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'cast_zot_widen'
define i64 @zot_widen_fn(i32 %0) {
entry:
  %1 = zext i32 %0 to i64
  ret i64 %1
}
";
    assert_eq!(ll, expected);
}

/// `zext_or_trunc` at equal width (i64 -> i64) must emit NO cast
/// instruction: the value passes through unchanged.
#[test]
fn zext_or_trunc_equal_width_is_passthrough() {
    let ll = Module::with_new("cast_zot_equal", |m| {
        let typed = m.add_typed_function::<i64, (i64,), _>("zot_equal_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.zext_or_trunc(x.as_value(), 64);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("zext_or_trunc equal-width build should succeed");

    println!("=== zext_or_trunc_equal.ll ===\n{ll}");

    // No cast instruction: entry just returns the param directly.
    let expected = "\
; ModuleID = 'cast_zot_equal'
define i64 @zot_equal_fn(i64 %0) {
entry:
  ret i64 %0
}
";
    assert_eq!(ll, expected);
}

/// `select i1 %c, i64 %0, i64 %1`.
#[test]
fn select_i1_cond_two_i64_arms() {
    let ll = Module::with_new("select_basic", |m| {
        let typed =
            m.add_typed_function::<i64, (bool, i64, i64), _>("select_fn", Linkage::External)?;
        let (cond, t, e) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.select(cond.as_value(), t.as_value(), e.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("select build should succeed");

    println!("=== select.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'select_basic'
define i64 @select_fn(i1 %0, i64 %1, i64 %2) {
entry:
  %3 = select i1 %0, i64 %1, i64 %2
  ret i64 %3
}
";
    assert_eq!(ll, expected);
}

/// Closes a Task 4 coverage gap: `not` on a non-64-bit (i32) operand proves
/// the width derivation is per-operand (`v.ty().bit_width()`), not a
/// hardcoded 64.
#[test]
fn not_i32_param_is_xor_all_ones_i32() {
    let ll = Module::with_new("arith_not32", |m| {
        let typed = m.add_typed_function::<i32, (i32,), _>("not32_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.not(x.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("not (i32) build should succeed");

    println!("=== not32.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_not32'
define i32 @not32_fn(i32 %0) {
entry:
  %1 = xor i32 %0, -1
  ret i32 %1
}
";
    assert_eq!(ll, expected);
}
