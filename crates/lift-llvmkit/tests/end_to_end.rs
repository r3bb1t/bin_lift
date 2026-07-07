//! End-to-end golden test: a realistic small function resembling a lifted
//! x86 `ADD` handler, driven entirely through the `lift_core::IrBuilder`
//! surface (control flow, arithmetic, compares, casts, memory), exercising
//! the whole op surface together and then `verify()`ing the resulting
//! module.
//!
//! Shape (mirrors what an M3 x86 `ADD` semantics handler will emit):
//!   - `sum = iadd(%0, %1)`               (the ADD result)
//!   - `zf = icmp eq sum, 0`              (zero flag)
//!   - `cf = icmp ult sum, %0`            (carry flag: unsigned overflow)
//!   - `flags = (zext(cf) << 1) | zext(zf)`  (packed into a byte-ish value)
//!   - `store flags` to a "flags register" address (param `%2`)
//!   - `ret sum`
//!
//! All arithmetic/compare operands are function PARAMS (never two bare
//! constants): llvmkit's `ConstantFolder` is on by default, so constant-only
//! ops fold away before becoming a real instruction, which would make the
//! golden assertion vacuous. `const_zero` is fine here because it is paired
//! with a non-constant operand (`sum`).

use lift_core::{IcmpPred, IrBuilder};
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

#[test]
fn lifted_add_end_to_end_golden_and_verify() {
    let ll = Module::with_new("e2e_add", |m| {
        // i64 lifted_add(i64 %0, i64 %1, i64 %2)
        //   %0, %1: the two operands to add.
        //   %2: address of a "flags register" to store the packed ZF/CF into.
        let typed = m.add_typed_function::<i64, (i64, i64, i64), _>(
            "lifted_add",
            Linkage::External,
        )?;
        let (lhs, rhs, flags_addr) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);

        let lhs = lhs.as_value();
        let rhs = rhs.as_value();

        // sum = lhs + rhs
        let sum = b.iadd(lhs, rhs);

        // zf = (sum == 0)
        let zero = b.const_zero(64);
        let zf = b.icmp(IcmpPred::Eq, sum, zero);

        // cf = (sum < lhs) [unsigned] -- classic overflow-detection idiom
        let cf = b.icmp(IcmpPred::Ult, sum, lhs);

        // Pack into a single flags value: (zext(cf) << 1) | zext(zf)
        let zf64 = b.zext(zf, 64);
        let cf64 = b.zext(cf, 64);
        let one = b.const_int(64, 1);
        let cf_shifted = b.shl(cf64, one);
        let flags = b.or(cf_shifted, zf64);

        // Store the packed flags to the caller-supplied address, then
        // return the arithmetic result.
        b.store(flags_addr.as_value(), flags);
        b.ret(sum);

        Ok::<String, IrError>(b.finish())
    })
    .expect("lifted_add build should succeed");

    println!("=== lifted_add.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'e2e_add'
define i64 @lifted_add(i64 %0, i64 %1, i64 %2) {
entry:
  %3 = add i64 %0, %1
  %4 = icmp eq i64 %3, 0
  %5 = icmp ult i64 %3, %0
  %6 = zext i1 %4 to i64
  %7 = zext i1 %5 to i64
  %8 = shl i64 %7, 1
  %9 = or i64 %8, %6
  %p = inttoptr i64 %2 to ptr
  store i64 %9, ptr %p
  ret i64 %3
}
";
    assert_eq!(ll, expected);
}

/// Same build, but additionally runs `Module::verify_borrowed()` on the
/// finished module and asserts it reports the module as well-formed. This is
/// the integration point that proves the whole op surface (control flow +
/// arithmetic + compares + casts + memory) produces IR LLVM itself would
/// accept, not just IR that happens to print correctly.
#[test]
fn lifted_add_end_to_end_verifies() {
    let verify_result = Module::with_new("e2e_add_verify", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64, i64), _>(
            "lifted_add",
            Linkage::External,
        )?;
        let (lhs, rhs, flags_addr) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);

        let lhs = lhs.as_value();
        let rhs = rhs.as_value();

        let sum = b.iadd(lhs, rhs);

        let zero = b.const_zero(64);
        let zf = b.icmp(IcmpPred::Eq, sum, zero);
        let cf = b.icmp(IcmpPred::Ult, sum, lhs);

        let zf64 = b.zext(zf, 64);
        let cf64 = b.zext(cf, 64);
        let one = b.const_int(64, 1);
        let cf_shifted = b.shl(cf64, one);
        let flags = b.or(cf_shifted, zf64);

        b.store(flags_addr.as_value(), flags);
        b.ret(sum);

        Ok::<Result<(), IrError>, IrError>(m.verify_borrowed())
    })
    .expect("lifted_add (verify variant) build should succeed");

    assert!(
        verify_result.is_ok(),
        "expected the lifted_add module to verify cleanly, got: {verify_result:?}"
    );
}
