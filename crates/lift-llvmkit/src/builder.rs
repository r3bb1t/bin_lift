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

use lift_core::{IcmpPred, IrBuilder, Va};
use llvmkit::ir::{
    BasicBlock, BasicBlockLabel, ConstantFolder, Dyn, FunctionValue, IRBuilder, IntDyn,
    IntPredicate, IntValue, Module, Positioned, Unverified, Value,
};

/// Map `lift_core::IcmpPred` to `llvmkit::ir::IntPredicate`. The two enums
/// mirror each other one-for-one (see `IcmpPred`'s doc comment).
fn to_llvmkit_pred(pred: IcmpPred) -> IntPredicate {
    match pred {
        IcmpPred::Eq => IntPredicate::Eq,
        IcmpPred::Ne => IntPredicate::Ne,
        IcmpPred::Ult => IntPredicate::Ult,
        IcmpPred::Ule => IntPredicate::Ule,
        IcmpPred::Ugt => IntPredicate::Ugt,
        IcmpPred::Uge => IntPredicate::Uge,
        IcmpPred::Slt => IntPredicate::Slt,
        IcmpPred::Sle => IntPredicate::Sle,
        IcmpPred::Sgt => IntPredicate::Sgt,
        IcmpPred::Sge => IntPredicate::Sge,
    }
}

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
///
/// `load`/`store` are *flat* raw memory ops: an address `Value` is
/// `inttoptr`'d to the module's opaque pointer type and read/written
/// directly, with no aliasing model. A `MemoryModel`/`solve_load`-style
/// aliasing layer on top of this is deferred to Milestone 4.
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

    /// Borrow the positioned builder, expecting it to be present. Used for
    /// non-terminator data-flow ops (`build_int_*_dyn` etc.), which take
    /// `&self` and leave the builder positioned for the next instruction.
    fn builder(&self) -> &IRBuilder<'m, 'ctx, B, ConstantFolder, Positioned, Dyn> {
        self.builder
            .as_ref()
            .expect("LlvmkitBuilder: no positioned builder (call position_at first)")
    }

    /// Shared width-branch for `zext`/`sext`/`trunc`/`zext_or_trunc`:
    /// strictly narrower than the operand's runtime width truncates,
    /// strictly wider extends (via `widen`), and equal width passes the
    /// input through unchanged (llvmkit's typed cast builders reject
    /// equal-width trunc/zext/sext, so this branch is on us).
    fn cast_to_width(
        &self,
        value: Value<'ctx, B>,
        width: u32,
        widen: impl FnOnce(
            &IRBuilder<'m, 'ctx, B, ConstantFolder, Positioned, Dyn>,
            IntValue<'ctx, IntDyn, B>,
            llvmkit::ir::IntType<'ctx, IntDyn, B>,
        ) -> llvmkit::ir::IrResult<IntValue<'ctx, IntDyn, B>>,
    ) -> Value<'ctx, B> {
        let iv = IntValue::<IntDyn, B>::try_from(value).expect("cast: operand must be an integer");
        let src_width = iv.ty().bit_width();
        match width.cmp(&src_width) {
            std::cmp::Ordering::Less => {
                let dst_ty = self
                    .module
                    .custom_width_int_type(width)
                    .expect("cast: valid destination int width");
                self.builder()
                    .build_trunc_dyn(iv, dst_ty, "")
                    .expect("cast: build_trunc_dyn")
                    .as_value()
            }
            std::cmp::Ordering::Greater => {
                let dst_ty = self
                    .module
                    .custom_width_int_type(width)
                    .expect("cast: valid destination int width");
                widen(self.builder(), iv, dst_ty)
                    .expect("cast: widen")
                    .as_value()
            }
            std::cmp::Ordering::Equal => value,
        }
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

    fn const_int(&mut self, width: u32, value: u64) -> Self::Value {
        self.module
            .custom_width_int_type(width)
            .expect("const_int: valid int width")
            .const_int_checked(value as i64)
            .expect("const_int: value fits in width")
            .as_value()
    }

    fn const_zero(&mut self, width: u32) -> Self::Value {
        self.module
            .custom_width_int_type(width)
            .expect("const_zero: valid int width")
            .const_zero()
            .as_value()
    }

    fn const_ones(&mut self, width: u32) -> Self::Value {
        self.module
            .custom_width_int_type(width)
            .expect("const_ones: valid int width")
            .const_all_ones()
            .as_value()
    }

    fn iadd(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_add_dyn(a, b, "")
            .expect("iadd: build_int_add_dyn")
    }

    fn isub(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_sub_dyn(a, b, "")
            .expect("isub: build_int_sub_dyn")
    }

    fn imul(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_mul_dyn(a, b, "")
            .expect("imul: build_int_mul_dyn")
    }

    fn and(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_and_dyn(a, b, "")
            .expect("and: build_int_and_dyn")
    }

    fn or(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_or_dyn(a, b, "")
            .expect("or: build_int_or_dyn")
    }

    fn xor(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_xor_dyn(a, b, "")
            .expect("xor: build_int_xor_dyn")
    }

    fn not(&mut self, a: Self::Value) -> Self::Value {
        // No dedicated `not` opcode in LLVM IR: synthesize `xor a, -1`.
        let width = IntValue::<IntDyn, B>::try_from(a)
            .expect("not: operand must be an integer")
            .ty()
            .bit_width();
        let ones = self.const_ones(width);
        self.builder()
            .build_int_xor_dyn(a, ones, "")
            .expect("not: build_int_xor_dyn")
    }

    fn shl(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_shl_dyn(a, b, "")
            .expect("shl: build_int_shl_dyn")
    }

    fn lshr(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_lshr_dyn(a, b, "")
            .expect("lshr: build_int_lshr_dyn")
    }

    fn ashr(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        self.builder()
            .build_int_ashr_dyn(a, b, "")
            .expect("ashr: build_int_ashr_dyn")
    }

    fn urem(&mut self, a: Self::Value, b: Self::Value) -> Self::Value {
        // No dyn form for urem: round-trip operands to `IntValue<IntDyn>`
        // and call the typed builder with `W = IntDyn`.
        let a = IntValue::<IntDyn, B>::try_from(a).expect("urem: lhs must be an integer");
        let b = IntValue::<IntDyn, B>::try_from(b).expect("urem: rhs must be an integer");
        self.builder()
            .build_int_urem::<IntDyn, _, _, _>(a, b, "")
            .expect("urem: build_int_urem")
            .as_value()
    }

    fn icmp(&mut self, pred: IcmpPred, a: Self::Value, b: Self::Value) -> Self::Value {
        // No dyn form for icmp: round-trip operands to `IntValue<IntDyn>` and
        // call the typed builder with `W = IntDyn`. Result is `IntValue<bool>`
        // (i1); erase it back to `Value`.
        let a = IntValue::<IntDyn, B>::try_from(a).expect("icmp: lhs must be an integer");
        let b = IntValue::<IntDyn, B>::try_from(b).expect("icmp: rhs must be an integer");
        self.builder()
            .build_int_cmp::<IntDyn, _, _, _>(to_llvmkit_pred(pred), a, b, "")
            .expect("icmp: build_int_cmp")
            .as_value()
    }

    fn zext(&mut self, value: Self::Value, width: u32) -> Self::Value {
        self.cast_to_width(value, width, |b, iv, dst_ty| b.build_zext_dyn(iv, dst_ty, ""))
    }

    fn sext(&mut self, value: Self::Value, width: u32) -> Self::Value {
        self.cast_to_width(value, width, |b, iv, dst_ty| b.build_sext_dyn(iv, dst_ty, ""))
    }

    fn trunc(&mut self, value: Self::Value, width: u32) -> Self::Value {
        self.cast_to_width(value, width, |_, _, _| {
            panic!("trunc: width is wider than operand (caller bug: use zext/sext instead)")
        })
    }

    fn zext_or_trunc(&mut self, value: Self::Value, width: u32) -> Self::Value {
        self.cast_to_width(value, width, |b, iv, dst_ty| b.build_zext_dyn(iv, dst_ty, ""))
    }

    fn select(&mut self, cond: Self::Value, a: Self::Value, b: Self::Value) -> Self::Value {
        // `cond` (an erased `Value`) round-trips into `IntValue<bool>` via
        // the `IntoIntValue<'ctx, bool, B>` blanket impl for `Value`, so it
        // can be passed straight through as `C` without a manual conversion.
        let a = IntValue::<IntDyn, B>::try_from(a).expect("select: true arm must be an integer");
        let b = IntValue::<IntDyn, B>::try_from(b).expect("select: false arm must be an integer");
        self.builder()
            .build_select::<_, IntValue<'ctx, IntDyn, B>, _>(cond, a, b, "")
            .expect("select: build_select")
            .as_value()
    }

    fn load(&mut self, addr: Self::Value, width: u32) -> Self::Value {
        // Flat raw load: `inttoptr` the address to the module's opaque
        // pointer type, then `load <width>, ptr <p>`. No aliasing model here
        // (see the module-level doc comment); that's `MemoryModel`/
        // `solve_load`, deferred to M4.
        let addr = IntValue::<IntDyn, B>::try_from(addr).expect("load: address must be an integer");
        let ptr_ty = self.module.ptr_type(0);
        let p = self
            .builder()
            .build_int_to_ptr::<IntDyn, _>(addr, ptr_ty, "p")
            .expect("load: build_int_to_ptr");
        let ty = self
            .module
            .custom_width_int_type(width)
            .expect("load: valid int width");
        self.builder()
            .build_load(ty, p, "r")
            .expect("load: build_load")
    }

    fn store(&mut self, addr: Self::Value, value: Self::Value) {
        // Flat raw store: `inttoptr` the address, then `store <value>, ptr
        // <p>`. Same caveat as `load` above — no aliasing model yet (M4).
        let addr = IntValue::<IntDyn, B>::try_from(addr).expect("store: address must be an integer");
        let ptr_ty = self.module.ptr_type(0);
        let p = self
            .builder()
            .build_int_to_ptr::<IntDyn, _>(addr, ptr_ty, "p")
            .expect("store: build_int_to_ptr");
        self.builder()
            .build_store(value, p)
            .expect("store: build_store");
    }
}
