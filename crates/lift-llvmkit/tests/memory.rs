//! Golden `.ll` tests for `LlvmkitBuilder`'s minimal flat memory primitives:
//! `load`/`store`. Both round-trip an integer address `Value` through
//! `inttoptr` to the module's opaque pointer type, then call
//! `build_load`/`build_store`.
//!
//! These are *flat* raw load/store: no aliasing model, no `MemoryModel`. That
//! layer (`solve_load` and friends) is M4.
//!
//! All operands are function PARAMS (never bare constants): llvmkit's
//! `ConstantFolder` is on by default, so constant-only operations fold away
//! before they ever become an instruction.

use lift_core::IrBuilder;
use lift_llvmkit::LlvmkitBuilder;
use llvmkit::ir::{IrError, Linkage, Module};

/// `i64 f(i64 %0, i64 %1)`: store `%1` to address `%0`, then load a 64-bit
/// value back from address `%0` and return it.
#[test]
fn store_then_load_roundtrip() {
    let ll = Module::with_new("mem_roundtrip", |m| {
        let typed = m.add_typed_function::<i64, (i64, i64), _>("f", Linkage::External)?;
        let (addr, value) = typed.params();
        let f = typed.as_function().as_dyn();

        let mut b = LlvmkitBuilder::new(&m, f);
        let entry = b.new_block("entry");
        b.position_at(&entry);
        b.store(addr.as_value(), value.as_value());
        let r = b.load(addr.as_value(), 64);
        b.ret(r);

        Ok::<String, IrError>(b.finish())
    })
    .expect("store/load build should succeed");

    println!("=== mem_roundtrip.ll ===\n{ll}");

    let expected = "\
; ModuleID = 'mem_roundtrip'
define i64 @f(i64 %0, i64 %1) {
entry:
  %p = inttoptr i64 %0 to ptr
  store i64 %1, ptr %p
  %p1 = inttoptr i64 %0 to ptr
  %r = load i64, ptr %p1
  ret i64 %r
}
";
    assert_eq!(ll, expected);
}
