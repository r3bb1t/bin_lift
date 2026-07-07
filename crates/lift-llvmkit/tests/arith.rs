//! Golden `.ll` tests for `LlvmkitBuilder`'s integer arithmetic / bitwise /
//! shift `IrBuilder` impl: `iadd/isub/imul`, `and/or/xor/not`,
//! `shl/lshr/ashr`, `urem`.
//!
//! All operands are function PARAMS (never bare constants): llvmkit's
//! `ConstantFolder` is on by default, so constant-only arithmetic folds away
//! before it ever becomes an instruction — asserting an op over two
//! constants would be a vacuous test.
//!
//! Each test builds `i64 @<op>(i64 %0, i64 %1)` computing `%0 <op> %1` (or,
//! for `not`, `i64 @not(i64 %0)` computing `not %0`), returns the result, and
//! asserts the exact rendered `.ll`.

use lift_core::IrBuilder;
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

#[test]
fn iadd_two_params() {
    let ll = Module::with_new("arith_add", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("add", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.iadd(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("iadd build should succeed");

    println!("=== iadd.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_add'
define i64 @add(i64 %0, i64 %1) {
entry:
  %2 = add i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn isub_two_params() {
    let ll = Module::with_new("arith_sub", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("sub", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.isub(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("isub build should succeed");

    println!("=== isub.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_sub'
define i64 @sub(i64 %0, i64 %1) {
entry:
  %2 = sub i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn imul_two_params() {
    let ll = Module::with_new("arith_mul", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("mul", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.imul(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("imul build should succeed");

    println!("=== imul.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_mul'
define i64 @mul(i64 %0, i64 %1) {
entry:
  %2 = mul i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn and_two_params() {
    let ll = Module::with_new("arith_and", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("and_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.and(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("and build should succeed");

    println!("=== and.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_and'
define i64 @and_fn(i64 %0, i64 %1) {
entry:
  %2 = and i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn or_two_params() {
    let ll = Module::with_new("arith_or", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("or_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.or(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("or build should succeed");

    println!("=== or.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_or'
define i64 @or_fn(i64 %0, i64 %1) {
entry:
  %2 = or i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn xor_two_params() {
    let ll = Module::with_new("arith_xor", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("xor_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.xor(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("xor build should succeed");

    println!("=== xor.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_xor'
define i64 @xor_fn(i64 %0, i64 %1) {
entry:
  %2 = xor i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

/// `not %0` has no dedicated LLVM opcode: it's sugar for `xor %0, -1`.
#[test]
fn not_one_param_is_xor_all_ones() {
    let ll = Module::with_new("arith_not", |m| {
        let typed = m.add_typed_function::<i64, (i64,), _>("not_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.not(x.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("not build should succeed");

    println!("=== not.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_not'
define i64 @not_fn(i64 %0) {
entry:
  %1 = xor i64 %0, -1
  ret i64 %1
}
";
    assert_eq!(ll, expected);
}

#[test]
fn shl_two_params() {
    let ll = Module::with_new("arith_shl", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("shl_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.shl(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("shl build should succeed");

    println!("=== shl.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_shl'
define i64 @shl_fn(i64 %0, i64 %1) {
entry:
  %2 = shl i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn lshr_two_params() {
    let ll = Module::with_new("arith_lshr", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("lshr_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.lshr(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("lshr build should succeed");

    println!("=== lshr.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_lshr'
define i64 @lshr_fn(i64 %0, i64 %1) {
entry:
  %2 = lshr i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn ashr_two_params() {
    let ll = Module::with_new("arith_ashr", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("ashr_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.ashr(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("ashr build should succeed");

    println!("=== ashr.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_ashr'
define i64 @ashr_fn(i64 %0, i64 %1) {
entry:
  %2 = ashr i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

#[test]
fn urem_two_params() {
    let ll = Module::with_new("arith_urem", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("urem_fn", Linkage::External)?;
        let (x, y) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let r = b.urem(x.as_value(), y.as_value());
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("urem build should succeed");

    println!("=== urem.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_urem'
define i64 @urem_fn(i64 %0, i64 %1) {
entry:
  %2 = urem i64 %0, %1
  ret i64 %2
}
";
    assert_eq!(ll, expected);
}

/// `const_int`/`const_zero`/`const_ones`: constant-folded against a param
/// (so the *result* is a real instruction, not a folded constant), letting
/// us assert the constant's own printed form (`42`, `0`, `-1`) as an operand.
#[test]
fn const_int_zero_ones_render_as_operands() {
    let ll = Module::with_new("arith_consts", |m| {
        let typed = m.add_typed_function::<i64, (i64,), _>("consts_fn", Linkage::External)?;
        let (x,) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        let c42 = b.const_int(64, 42);
        let zero = b.const_zero(64);
        let ones = b.const_ones(64);
        let r1 = b.iadd(x.as_value(), c42);
        let r2 = b.iadd(r1, zero);
        let r3 = b.iadd(r2, ones);
        b.ret(r3);

        Ok::<String, IrError>(b.finish())
    })
    .expect("const build should succeed");

    println!("=== consts.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'arith_consts'
define i64 @consts_fn(i64 %0) {
entry:
  %1 = add i64 %0, 42
  %2 = add i64 %1, 0
  %3 = add i64 %2, -1
  ret i64 %3
}
";
    assert_eq!(ll, expected);
}
