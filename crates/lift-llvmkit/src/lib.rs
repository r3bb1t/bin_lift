#![forbid(unsafe_code)]

//! `lift-llvmkit` — an `lift_core::IrBuilder` backend over `llvmkit`, the
//! pure-Rust LLVM IR builder/printer (<https://crates.io/crates/llvmkit>).
//!
//! # Usage contract
//!
//! `llvmkit`'s `Module` is scoped by a `for<'brand>` brand lifetime
//! (`Module::with_new`); a [`LlvmkitBuilder`] borrows `&'m Module` and cannot
//! escape that closure. Build the whole function body inside `with_new`,
//! then call [`LlvmkitBuilder::finish`] (or `Module::to_string`) and return
//! the resulting `String` (or a verified module) out of the closure.
//!
//! Constant folding is on by default, so arithmetic over two constants folds
//! away before becoming an instruction; golden `.ll` tests must use
//! non-constant operands (function parameters, loaded values) to see a real
//! instruction emitted.

mod builder;

pub use builder::LlvmkitBuilder;
