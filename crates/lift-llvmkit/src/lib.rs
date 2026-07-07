#![forbid(unsafe_code)]

//! `lift-llvmkit` — a golden-`.ll`-tested [`lift_core::IrBuilder`] backend
//! over `llvmkit`, the pure-Rust LLVM IR builder/printer
//! (<https://crates.io/crates/llvmkit>, pinned to `=0.0.3`).
//!
//! [`LlvmkitBuilder`] implements the full `IrBuilder` surface: control flow
//! (`new_block`/`position_at`/`br`/`cond_br`/`switch`/`ret`/`unreachable`),
//! integer arithmetic/bitwise/shifts (`iadd`/`isub`/`imul`, `and`/`or`/`xor`/
//! `not`, `shl`/`lshr`/`ashr`, `urem`), compares/casts/select (`icmp`,
//! `zext`/`sext`/`trunc`/`zext_or_trunc`, `select`), and minimal flat memory
//! (`load`/`store`). `Value = llvmkit::ir::Value<'ctx, B>` is fully
//! width-erased; `Block = usize` indexes an internal label table.
//!
//! # Usage contract: the brand closure
//!
//! `llvmkit`'s `Module` is scoped by a `for<'brand> FnOnce` brand lifetime
//! (`Module::with_new`): the closure receives `Module<'brand, Brand<'brand>,
//! Unverified>` by value, and nothing tagged with that `'brand` — including a
//! [`LlvmkitBuilder`], which borrows `&'m Module` — can escape the closure.
//! In practice this means: **build the whole function body, print it (or
//! verify it), and produce your final owned result (a `String`, a bool, an
//! `IrResult<()>`, ...) all inside `with_new`**, then return that value out.
//!
//! ```
//! use lift_core::IrBuilder;
//! use llvmkit::ir::{IrError, Linkage, Module};
//! use lift_llvmkit::LlvmkitBuilder;
//!
//! let ll = Module::with_new("m", |m| {
//!     let typed = m.add_typed_function::<i64, (i64, i64), _>("add", Linkage::External)?;
//!     let (x, y) = typed.params();
//!     let f = typed.as_function().as_dyn();
//!
//!     let mut b = LlvmkitBuilder::new(&m, f);
//!     let entry = b.new_block("entry");
//!     b.position_at(&entry);
//!     let r = b.iadd(x.as_value(), y.as_value());
//!     b.ret(r);
//!
//!     Ok::<String, IrError>(b.finish())
//! })
//! .expect("build should succeed");
//! assert!(ll.contains("add i64"));
//! ```
//!
//! For the common "one function per module" shape above, [`build_module`] is
//! a thin ergonomic wrapper around `with_new` + `add_typed_function` +
//! `LlvmkitBuilder::new` that hands you `(&mut LlvmkitBuilder, params)`
//! directly:
//!
//! ```
//! use lift_core::IrBuilder;
//! use lift_llvmkit::build_module;
//!
//! let ll = build_module::<i64, (i64, i64), _>("m", "add", |b, (x, y)| {
//!     let entry = b.new_block("entry");
//!     b.position_at(&entry);
//!     let r = b.iadd(x.as_value(), y.as_value());
//!     b.ret(r);
//!     b.finish()
//! });
//! assert!(ll.contains("add i64"));
//! ```
//!
//! Reach for the manual `with_new` pattern instead of `build_module` when a
//! build needs more than one function in the module, needs the `Module`
//! itself in scope (e.g. to call `verify`/`verify_borrowed`), or otherwise
//! doesn't fit the "single typed function" shape.
//!
//! # Constant-folding caveat (golden tests)
//!
//! `llvmkit`'s `ConstantFolder` is on by default: arithmetic/casts/compares
//! over two *constant* operands fold away before ever becoming a real
//! instruction. Golden `.ll` tests (and any caller who wants to observe a
//! specific instruction) must feed at least one non-constant operand —
//! typically a function parameter or a loaded value. An arithmetic golden
//! test built from two bare constants would silently assert a folded
//! constant instead of the intended instruction.
//!
//! # Memory model
//!
//! `load`/`store` are *flat* raw memory primitives: an address `Value` is
//! `inttoptr`'d to the module's opaque pointer type and read/written
//! directly, with no aliasing model. A `MemoryModel`/`solve_load`-style
//! aliasing layer on top of this flat load/store is deferred to Milestone 4.
//!
//! # Forward pointers
//!
//! - **Milestone 3** composes a [`LlvmkitBuilder`] (or a from-scratch
//!   `Session`) with the brand closure across a full function/basic-block
//!   walk; that composition — how a streaming `Session::step` loop interacts
//!   with the `for<'brand>` scoping — is out of scope here and left to M3.
//!   M3 may also add signed/other division/remainder ops (`sdiv`/`srem`/
//!   `div`) if a semantics handler needs them; only `urem` is in the M1
//!   census.
//! - **Milestone 4** adds the `MemoryModel`/`solve_load` aliasing layer on
//!   top of the flat `load`/`store` here.
//! - Calls, GEP-for-structs (M5) and optimization passes (M6, via inkwell —
//!   llvmkit has none yet) are further out.

mod builder;

pub use builder::{build_module, LlvmkitBuilder};
