//! `LlvmkitBuilder`: an `lift_core::IrBuilder` impl over `llvmkit` v0.0.3.
//!
//! # Usage contract
//!
//! The whole build must happen inside one [`Module::with_new`] brand-scoped
//! closure: `LlvmkitBuilder` borrows `&'m Module<'ctx, B, Unverified>`, and
//! the `for<'brand>` brand lifetime it carries cannot escape that closure.
//! Callers build the module, drive a `LlvmkitBuilder` to emit a function
//! body, then call [`LlvmkitBuilder::finish`] (or `m.to_string()`) *inside*
//! the closure and return the resulting `String` (or a verified module) out.
//!
//! Constant folding is on by default (llvmkit's `ConstantFolder`): arithmetic
//! over two constants folds away before it ever becomes an instruction. Tests
//! that need to assert a real instruction must feed non-constant operands
//! (function parameters, loaded values, etc.).

use lift_core::{IrBuilder, Va};
use llvmkit::ir::{
    BasicBlock, BasicBlockLabel, ConstantFolder, Dyn, FunctionValue, IRBuilder, Module,
    Positioned, Unverified, Value,
};

/// A control-flow/data-flow IR backend that emits `llvmkit` IR for a single
/// function.
///
/// - `Value = llvmkit::ir::Value<'ctx, B>` (width-erased; carries its own
///   type via `TypeId`).
/// - `Block = usize`, an index into `labels`/`blocks`.
///
/// The builder holds at most one live `IRBuilder` at a time (`self.builder`),
/// since llvmkit's terminator-emitting `build_*` methods consume the builder
/// by value. `position_at` replaces it with a fresh, freshly-positioned
/// builder; terminators `take()` it out, emit, and leave `None` until the
/// next `position_at`.
pub struct LlvmkitBuilder<'m, 'ctx, B: llvmkit::ir::ModuleBrand + 'ctx> {
    module: &'m Module<'ctx, B, Unverified>,
    func: FunctionValue<'ctx, Dyn, B>,
    /// The currently positioned builder, if any. `None` right after a
    /// terminator has been emitted and before the next `position_at`.
    builder: Option<IRBuilder<'m, 'ctx, B, ConstantFolder, Positioned, Dyn>>,
    /// Copyable block labels, indexed by `Block` (our `usize` handle). These
    /// are what branch/switch instructions target.
    labels: Vec<BasicBlockLabel<'ctx, Dyn, B>>,
    /// The insertion-capable `BasicBlock` handles, indexed by `Block`.
    /// `position_at` takes ownership of the entry (`BasicBlock` is `!Copy`
    /// and consumed exactly once by `position_at_end`).
    blocks: Vec<Option<BasicBlock<'ctx, Dyn, llvmkit::ir::Unsealed, B>>>,
}

impl<'m, 'ctx, B: llvmkit::ir::ModuleBrand + 'ctx> LlvmkitBuilder<'m, 'ctx, B> {
    /// Create a builder for `func` inside `module`. Does not create any
    /// blocks; call `new_block` + `position_at` before emitting anything.
    pub fn new(module: &'m Module<'ctx, B, Unverified>, func: FunctionValue<'ctx, Dyn, B>) -> Self {
        Self { module, func, builder: None, labels: Vec::new(), blocks: Vec::new() }
    }

    /// Render the module to textual `.ll`.
    pub fn finish(&self) -> String {
        self.module.to_string()
    }

    /// Take the positioned builder, expecting it to be present. Used right
    /// before emitting a terminator, which consumes the builder by value.
    fn take_builder(&mut self) -> IRBuilder<'m, 'ctx, B, ConstantFolder, Positioned, Dyn> {
        self.builder
            .take()
            .expect("LlvmkitBuilder: no positioned builder (call position_at first)")
    }
}

impl<'m, 'ctx, B: llvmkit::ir::ModuleBrand + 'ctx> IrBuilder for LlvmkitBuilder<'m, 'ctx, B> {
    type Value = Value<'ctx, B>;
    type Block = usize;

    fn const_addr(&mut self, value: Va) -> Self::Value {
        self.module
            .custom_width_int_type(64)
            .expect("const_addr: 64-bit int type")
            .const_int_checked(value as i64)
            .expect("const_addr: value fits in 64 bits")
            .as_value()
    }

    fn new_block(&mut self, label: &str) -> Self::Block {
        let bb = self.func.append_basic_block(self.module, label);
        let idx = self.labels.len();
        self.labels.push(bb.label());
        self.blocks.push(Some(bb));
        idx
    }

    fn position_at(&mut self, block: &Self::Block) {
        let bb = self.blocks[*block]
            .take()
            .expect("position_at: block already positioned (or invalid index)");
        self.builder = Some(IRBuilder::new(self.module).position_at_end(bb));
    }

    fn br(&mut self, target: &Self::Block) {
        let b = self.take_builder();
        b.build_br(self.labels[*target]).expect("br: build_br");
    }

    fn cond_br(&mut self, cond: Self::Value, if_true: &Self::Block, if_false: &Self::Block) {
        let b = self.take_builder();
        b.build_cond_br(cond, self.labels[*if_true], self.labels[*if_false])
            .expect("cond_br: build_cond_br");
    }

    fn switch(&mut self, scrutinee: Self::Value, default: &Self::Block, cases: &[(Va, Self::Block)]) {
        let b = self.take_builder();
        let (_bb, mut sw) = b
            .build_switch(scrutinee, self.labels[*default], "")
            .expect("switch: build_switch");
        for (case_value, target) in cases {
            let case_const = self
                .module
                .custom_width_int_type(64)
                .expect("switch: 64-bit int type")
                .const_int_checked(*case_value as i64)
                .expect("switch: case value fits in 64 bits");
            sw = sw
                .add_case(case_const, self.labels[*target])
                .expect("switch: add_case");
        }
        sw.finish();
    }

    fn ret(&mut self, value: Self::Value) {
        let b = self.take_builder();
        b.build_ret(value).expect("ret: build_ret");
    }

    fn unreachable(&mut self) {
        let b = self.take_builder();
        b.build_unreachable();
    }
}
