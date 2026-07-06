//! Arch-neutral instruction view. A decoder frontend (zydis, iced, an emulator's
//! decoder) implements `InsnView` so `lift-core` stays decoder-agnostic without
//! re-modeling every architecture's operands.

use crate::address::Va;

/// A read-only view of one decoded operand. Extended by arch crates in later
/// milestones; declared here so the seam exists.
pub trait OperandView {
    /// Operand width in bits (8/16/32/64/...).
    fn bit_width(&self) -> u16;
}

/// A read-only view of one decoded instruction.
#[allow(clippy::len_without_is_empty)]
pub trait InsnView {
    /// Runtime virtual address of this instruction.
    fn address(&self) -> Va;
    /// Encoded length in bytes.
    fn len(&self) -> u8;
    /// Canonical mnemonic (lowercase), e.g. "mov", "jmp", "ret".
    fn mnemonic(&self) -> &str;
}

/// A synthetic instruction for tests.
#[derive(Clone, Debug)]
pub struct FakeInsn {
    pub address: Va,
    pub len: u8,
    pub mnemonic: String,
}

impl FakeInsn {
    pub fn with(address: Va, len: u8, mnemonic: &str) -> Self {
        Self { address, len, mnemonic: mnemonic.to_string() }
    }

    pub fn nop(address: Va) -> Self {
        Self::with(address, 1, "nop")
    }
}

impl InsnView for FakeInsn {
    fn address(&self) -> Va {
        self.address
    }

    fn len(&self) -> u8 {
        self.len
    }

    fn mnemonic(&self) -> &str {
        &self.mnemonic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_insn_exposes_view() {
        let i = FakeInsn::with(0x1000, 5, "jmp");
        let v: &dyn InsnView = &i;
        assert_eq!(v.address(), 0x1000);
        assert_eq!(v.len(), 5);
        assert_eq!(v.mnemonic(), "jmp");
    }
}
