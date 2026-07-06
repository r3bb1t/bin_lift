//! The IR backend seam. Milestone 1 defines only the control-flow slice the
//! streaming engine needs; later milestones extend it with data-flow ops. Only
//! backend crates (lift-llvmkit, lift-llvm) implement it against a real IR; the
//! `RecordingBuilder` here is a test double that logs emitted operations.

use crate::address::Va;

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
}

/// A test-double backend that records emitted operations as strings.
#[derive(Default)]
pub struct RecordingBuilder {
    pub log: Vec<String>,
    next_block: usize,
}

impl RecordingBuilder {
    pub fn new() -> Self {
        Self::default()
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
}
