//! The IR backend seam. Milestone 1 defines only the control-flow slice the
//! streaming engine needs; later milestones extend it with data-flow ops. Only
//! backend crates (lift-llvmkit, lift-llvm) implement it against a real IR; the
//! `RecordingBuilder` here is a test double that logs emitted operations.

use crate::address::Va;

/// An integer comparison predicate for [`IrBuilder::icmp`].
///
/// Mirrors `llvmkit::ir::IntPredicate` one-for-one so backends can map this
/// enum to their native predicate type with a trivial `match`/`From` impl.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IcmpPred {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Unsigned less-than.
    Ult,
    /// Unsigned less-than-or-equal.
    Ule,
    /// Unsigned greater-than.
    Ugt,
    /// Unsigned greater-than-or-equal.
    Uge,
    /// Signed less-than.
    Slt,
    /// Signed less-than-or-equal.
    Sle,
    /// Signed greater-than.
    Sgt,
    /// Signed greater-than-or-equal.
    Sge,
}

/// A backend that builds IR as the lifter emits operations.
pub trait IrBuilder {
    /// An SSA value in the backend's IR.
    type Value: Clone;
    /// A basic block handle in the backend's IR.
    type Block: Clone;

    /// Materialize a constant address value.
    fn const_addr(&mut self, value: Va) -> Self::Value;
    /// Create a new basic block with a label hint.
    fn new_block(&mut self, label: &str) -> Self::Block;
    /// Set the insertion point to `block`.
    fn position_at(&mut self, block: &Self::Block);
    /// Emit an unconditional branch to `target`.
    fn br(&mut self, target: &Self::Block);
    /// Emit a conditional branch.
    fn cond_br(&mut self, cond: Self::Value, if_true: &Self::Block, if_false: &Self::Block);
    /// Emit an N-way switch on `scrutinee`.
    fn switch(&mut self, scrutinee: Self::Value, default: &Self::Block, cases: &[(Va, Self::Block)]);
    /// Emit a return of `value`.
    fn ret(&mut self, value: Self::Value);
    /// Emit an `unreachable` terminator.
    fn unreachable(&mut self);

    /// Materialize an integer constant of `width` bits.
    fn const_int(&mut self, width: u32, value: u64) -> Self::Value;
    /// Materialize the zero constant of `width` bits.
    fn const_zero(&mut self, width: u32) -> Self::Value;
    /// Materialize the all-ones constant of `width` bits.
    fn const_ones(&mut self, width: u32) -> Self::Value;

    /// Integer addition.
    fn iadd(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Integer subtraction.
    fn isub(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Integer multiplication.
    fn imul(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;

    /// Bitwise AND.
    fn and(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Bitwise OR.
    fn or(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Bitwise XOR.
    fn xor(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Bitwise NOT.
    fn not(&mut self, a: Self::Value) -> Self::Value;

    /// Logical shift left.
    fn shl(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Logical shift right.
    fn lshr(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;
    /// Arithmetic shift right.
    fn ashr(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;

    /// Unsigned remainder.
    fn urem(&mut self, a: Self::Value, b: Self::Value) -> Self::Value;

    /// Integer comparison, producing an `i1`-equivalent boolean value.
    fn icmp(&mut self, pred: IcmpPred, a: Self::Value, b: Self::Value) -> Self::Value;

    /// Zero-extend `value` to `width` bits (`width` must be `>=` the input width).
    fn zext(&mut self, value: Self::Value, width: u32) -> Self::Value;
    /// Sign-extend `value` to `width` bits (`width` must be `>=` the input width).
    fn sext(&mut self, value: Self::Value, width: u32) -> Self::Value;
    /// Truncate `value` to `width` bits (`width` must be `<=` the input width).
    fn trunc(&mut self, value: Self::Value, width: u32) -> Self::Value;
    /// Zero-extend or truncate `value` to `width` bits, whichever applies; a
    /// no-op if `value` is already `width` bits wide.
    fn zext_or_trunc(&mut self, value: Self::Value, width: u32) -> Self::Value;

    /// Select `a` if `cond` is true, else `b`.
    fn select(&mut self, cond: Self::Value, a: Self::Value, b: Self::Value) -> Self::Value;

    /// Load a `width`-bit value from `addr`.
    fn load(&mut self, addr: Self::Value, width: u32) -> Self::Value;
    /// Store `value` to `addr`.
    fn store(&mut self, addr: Self::Value, value: Self::Value);
}

/// A test-double backend that records emitted operations as strings.
#[derive(Default)]
pub struct RecordingBuilder {
    pub log: Vec<String>,
    next_block: usize,
    next_value: u64,
}

impl RecordingBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mint a fresh synthetic SSA value and log `text` against it.
    fn record(&mut self, text: String) -> Va {
        self.log.push(text);
        let id = self.next_value;
        self.next_value += 1;
        id
    }
}

impl IcmpPred {
    /// Lowercase mnemonic used by `RecordingBuilder`'s op log.
    fn mnemonic(self) -> &'static str {
        match self {
            IcmpPred::Eq => "eq",
            IcmpPred::Ne => "ne",
            IcmpPred::Ult => "ult",
            IcmpPred::Ule => "ule",
            IcmpPred::Ugt => "ugt",
            IcmpPred::Uge => "uge",
            IcmpPred::Slt => "slt",
            IcmpPred::Sle => "sle",
            IcmpPred::Sgt => "sgt",
            IcmpPred::Sge => "sge",
        }
    }
}

impl IrBuilder for RecordingBuilder {
    type Value = Va;
    type Block = usize;

    fn const_addr(&mut self, value: Va) -> Va {
        self.log.push(format!("const {value:#x}"));
        value
    }

    fn new_block(&mut self, label: &str) -> usize {
        let id = self.next_block;
        self.next_block += 1;
        self.log.push(format!("block#{id} {label}"));
        id
    }

    fn position_at(&mut self, block: &usize) {
        self.log.push(format!("position #{block}"));
    }

    fn br(&mut self, target: &usize) {
        self.log.push(format!("br #{target}"));
    }

    fn cond_br(&mut self, _cond: Va, if_true: &usize, if_false: &usize) {
        self.log.push(format!("condbr #{if_true} #{if_false}"));
    }

    fn switch(&mut self, _scrutinee: Va, default: &usize, cases: &[(Va, usize)]) {
        self.log.push(format!("switch default=#{default} cases={}", cases.len()));
    }

    fn ret(&mut self, value: Va) {
        self.log.push(format!("ret {value:#x}"));
    }

    fn unreachable(&mut self) {
        self.log.push("unreachable".to_string());
    }

    fn const_int(&mut self, width: u32, value: u64) -> Va {
        self.record(format!("const {width} {value:#x}"))
    }

    fn const_zero(&mut self, width: u32) -> Va {
        self.record(format!("const_zero {width}"))
    }

    fn const_ones(&mut self, width: u32) -> Va {
        self.record(format!("const_ones {width}"))
    }

    fn iadd(&mut self, _a: Va, _b: Va) -> Va {
        self.record("iadd".to_string())
    }

    fn isub(&mut self, _a: Va, _b: Va) -> Va {
        self.record("isub".to_string())
    }

    fn imul(&mut self, _a: Va, _b: Va) -> Va {
        self.record("imul".to_string())
    }

    fn and(&mut self, _a: Va, _b: Va) -> Va {
        self.record("and".to_string())
    }

    fn or(&mut self, _a: Va, _b: Va) -> Va {
        self.record("or".to_string())
    }

    fn xor(&mut self, _a: Va, _b: Va) -> Va {
        self.record("xor".to_string())
    }

    fn not(&mut self, _a: Va) -> Va {
        self.record("not".to_string())
    }

    fn shl(&mut self, _a: Va, _b: Va) -> Va {
        self.record("shl".to_string())
    }

    fn lshr(&mut self, _a: Va, _b: Va) -> Va {
        self.record("lshr".to_string())
    }

    fn ashr(&mut self, _a: Va, _b: Va) -> Va {
        self.record("ashr".to_string())
    }

    fn urem(&mut self, _a: Va, _b: Va) -> Va {
        self.record("urem".to_string())
    }

    fn icmp(&mut self, pred: IcmpPred, _a: Va, _b: Va) -> Va {
        self.record(format!("icmp {}", pred.mnemonic()))
    }

    fn zext(&mut self, _value: Va, width: u32) -> Va {
        self.record(format!("zext {width}"))
    }

    fn sext(&mut self, _value: Va, width: u32) -> Va {
        self.record(format!("sext {width}"))
    }

    fn trunc(&mut self, _value: Va, width: u32) -> Va {
        self.record(format!("trunc {width}"))
    }

    fn zext_or_trunc(&mut self, _value: Va, width: u32) -> Va {
        self.record(format!("zext_or_trunc {width}"))
    }

    fn select(&mut self, _cond: Va, _a: Va, _b: Va) -> Va {
        self.record("select".to_string())
    }

    fn load(&mut self, _addr: Va, width: u32) -> Va {
        self.record(format!("load {width}"))
    }

    fn store(&mut self, _addr: Va, _value: Va) {
        self.log.push("store".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_builder_logs_ops() {
        let mut b = RecordingBuilder::new();
        let entry = b.new_block("entry");
        let target = b.new_block("t");
        b.position_at(&entry);
        let v = b.const_addr(0x2000);
        b.br(&target);
        b.ret(v);

        assert_eq!(
            b.log,
            vec![
                "block#0 entry".to_string(),
                "block#1 t".to_string(),
                "position #0".to_string(),
                "const 0x2000".to_string(),
                "br #1".to_string(),
                "ret 0x2000".to_string(),
            ]
        );
    }

    #[test]
    fn recording_builder_logs_data_flow_ops() {
        let mut b = RecordingBuilder::new();

        let a = b.const_int(64, 0x10);
        let z = b.const_zero(32);
        let ones = b.const_ones(8);

        let sum = b.iadd(a, z);
        let diff = b.isub(sum, a);
        let prod = b.imul(diff, a);

        let anded = b.and(prod, ones);
        let ored = b.or(anded, ones);
        let xored = b.xor(ored, ones);
        let inv = b.not(xored);

        let shl = b.shl(inv, a);
        let lshr = b.lshr(shl, a);
        let ashr = b.ashr(lshr, a);

        let rem = b.urem(ashr, a);

        let cmp = b.icmp(IcmpPred::Ult, rem, a);

        let z32 = b.zext(cmp, 32);
        let s64 = b.sext(z32, 64);
        let t8 = b.trunc(s64, 8);
        let zt16 = b.zext_or_trunc(t8, 16);

        let sel = b.select(cmp, zt16, a);

        let loaded = b.load(sel, 32);
        b.store(sel, loaded);

        assert_eq!(
            b.log,
            vec![
                "const 64 0x10".to_string(),
                "const_zero 32".to_string(),
                "const_ones 8".to_string(),
                "iadd".to_string(),
                "isub".to_string(),
                "imul".to_string(),
                "and".to_string(),
                "or".to_string(),
                "xor".to_string(),
                "not".to_string(),
                "shl".to_string(),
                "lshr".to_string(),
                "ashr".to_string(),
                "urem".to_string(),
                "icmp ult".to_string(),
                "zext 32".to_string(),
                "sext 64".to_string(),
                "trunc 8".to_string(),
                "zext_or_trunc 16".to_string(),
                "select".to_string(),
                "load 32".to_string(),
                "store".to_string(),
            ]
        );
    }
}
