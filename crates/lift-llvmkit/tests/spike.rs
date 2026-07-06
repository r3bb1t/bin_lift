//! Milestone 2 de-risking spike for `llvmkit` (r3bb1t's pure-Rust LLVM IR).
//!
//! Proves the crate builds in this environment and captures the canonical
//! working patterns the real backend will be built on:
//!   1. `add(i64, i64) -> i64` typed build + textual `.ll` print (the gate).
//!   2. Multi-block build with an unconditional branch via a `Copy` label.
//!   3. Homogeneous dynamic-width (`IntDyn`) values + erased add.
//!
//! The `llvmkit` umbrella crate re-exports `llvmkit-ir` under `llvmkit::ir`.

use llvmkit::ir::{IRBuilder, IrError, Linkage, Module};

/// (1) THE SPIKE GATE: build `i64 @add(i64, i64)` returning `a + b`,
/// print it to textual `.ll`, and assert the shape.
#[test]
fn spike_add_i64_prints_ll() {
    let ll = Module::with_new("spike", |m| {
        // Function + entry block, all inside the brand-scoped closure.
        let f = m.add_typed_function::<i64, (i64, i64), _>("add", Linkage::External)?;
        let entry = f.append_basic_block(&m, "entry");

        // Builder: Unpositioned -> Positioned CONSUMES the builder.
        // `new_for::<i64>` gives a statically-typed return marker.
        let b = IRBuilder::new_for::<i64>(&m).position_at_end(entry);

        // Params come off the typed function value as a tuple.
        let (lhs, rhs) = f.params();

        // Data-flow op: takes &self, width via turbofish, returns IrResult.
        let sum = b.build_int_add::<i64, _, _, _>(lhs, rhs, "sum")?;

        // Terminator CONSUMES the builder (self by value).
        b.build_ret(sum)?;

        // `Module: Display` renders textual `.ll` (works Unverified).
        Ok::<String, IrError>(m.to_string())
    })
    .expect("spike build should succeed");

    // Emit the exact formatting so it can be eyeballed / used as a golden.
    println!("=== add.ll ===\n{ll}");

    assert!(ll.contains("define"), "missing `define`:\n{ll}");
    assert!(ll.contains("@add"), "missing `@add`:\n{ll}");
    assert!(ll.contains("add i64"), "missing `add i64`:\n{ll}");
    assert!(ll.contains("ret i64"), "missing `ret i64`:\n{ll}");
}

/// (2) MULTI-BLOCK + BRANCH probe. Confirms how block handles flow to
/// `build_br`: a `BasicBlockLabel` is `Copy`, produced by `block.label()`,
/// and can be captured up-front / stored in a `Vec` and branched to.
#[test]
fn spike_multi_block_branch() {
    let ll = Module::with_new("spike_br", |m| {
        let f = m.add_typed_function::<i64, (i64,), _>("passthrough", Linkage::External)?;

        // Append N blocks; grab their Copy labels up front (like a backend
        // that resolves all block handles before emitting bodies).
        let entry = f.append_basic_block(&m, "entry");
        let tail = f.append_basic_block(&m, "tail");
        let labels: Vec<_> = vec![entry.label(), tail.label()];

        let (x,) = f.params();

        // entry: unconditional branch to `tail`. `build_br` CONSUMES self and
        // takes anything `IntoBasicBlockLabel` — a bare `Copy` label works.
        let b = IRBuilder::new_for::<i64>(&m).position_at_end(entry);
        b.build_br(labels[1])?;

        // tail: ret %x
        let b = IRBuilder::new_for::<i64>(&m).position_at_end(tail);
        b.build_ret(x)?;

        // labels[0] (entry) is still usable here — labels are Copy, not moved.
        let _ = labels[0];

        Ok::<String, IrError>(m.to_string())
    })
    .expect("multi-block build should succeed");

    println!("=== passthrough.ll ===\n{ll}");
    assert!(ll.contains("br label"), "missing `br label`:\n{ll}");
    assert!(ll.contains("ret i64"), "missing `ret i64`:\n{ll}");
}

/// (3) DYNAMIC-WIDTH (`IntDyn`) probe. This is the homogeneous value type the
/// backend will store: `custom_width_int_type(bits)` -> `IntType<IntDyn>`,
/// `.const_int(..)` -> constant, `.as_value()` erases to `Value`, and
/// `build_int_add_dyn` adds two erased `Value`s.
#[test]
fn spike_int_dyn_homogeneous_add() {
    let ll = Module::with_new("spike_dyn", |m| {
        // A function that just returns a constant, so the dyn add has a home.
        let f = m.add_typed_function::<i64, (), _>("dyn_add", Linkage::External)?;
        let entry = f.append_basic_block(&m, "entry");
        let b = IRBuilder::new_for::<i64>(&m).position_at_end(entry);

        // Dynamic-width i64 type + two constants, erased to `Value`.
        // GOTCHA: on an `IntDyn` type, `const_int` is unavailable (its bound
        // requires `Error = Infallible`); the dynamic-width path is fallible,
        // so use `const_int_checked(..)?`.
        let i64_dyn = m.custom_width_int_type(64)?;
        let a = i64_dyn.const_int_checked(40_i64)?.as_value();
        let c = i64_dyn.const_int_checked(2_i64)?.as_value();

        // Erased homogeneous add over `Value` operands -> `Value`.
        // This is the backend's homogeneous store type.
        let _sum: llvmkit::ir::Value<_> = b.build_int_add_dyn(a, c, "sum")?;

        // GOTCHA: a `new_for::<i64>` builder's `build_ret` wants a *typed* i64
        // return value, NOT an `IntDyn` one. Build the return const from the
        // statically-typed `i64_type()` (whose `const_int` IS infallible).
        b.build_ret(m.i64_type().const_int(42_i64))?;

        Ok::<String, IrError>(m.to_string())
    })
    .expect("int-dyn build should succeed");

    println!("=== dyn_add.ll ===\n{ll}");
    assert!(ll.contains("@dyn_add"), "missing `@dyn_add`:\n{ll}");
    assert!(ll.contains("ret i64"), "missing `ret i64`:\n{ll}");
}
