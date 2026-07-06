//! The per-architecture semantics seam. An arch crate implements `Lifter` to
//! translate one instruction's data-flow into IR (via the builder) and return a
//! terminal `Transfer` describing any control-flow resolution the engine must do.

use crate::event::BranchKind;
use crate::insn::InsnView;
use crate::ir::{IrBuilder, RecordingBuilder};
use crate::address::Va;

/// What an instruction does to control flow, reported after its body is lifted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Transfer {
    /// Continue to the next instruction.
    Fallthrough,
    /// A statically-resolved transfer (direct jump / already-known target).
    Static { kind: BranchKind, to: Va, taken: bool },
    /// A conditional branch needing a taken/not-taken verdict.
    ResolveBranch { taken_target: Va, fallthrough: Va },
    /// An indirect transfer needing a target set.
    ResolveIndirect { site: Va },
    /// A call to `target` (signature resolution handled in a later milestone).
    Call { target: Va },
    /// A return to `to`.
    ReturnTo { to: Va },
    /// End of the lifted function.
    End,
}

/// Translates one instruction into IR + a terminal `Transfer`.
pub trait Lifter {
    type Builder: IrBuilder;

    fn lift_body(&mut self, insn: &dyn InsnView, builder: &mut Self::Builder) -> Transfer;
}

/// A test lifter that decides the `Transfer` from the mnemonic. Recognized forms:
/// - "nop"              -> Fallthrough (no IR)
/// - "jmp <hex>"        -> Static unconditional to <hex>
/// - "jcc <hex> <hex>"  -> ResolveBranch { taken_target=<1st>, fallthrough=<2nd> }
/// - "jmp_ind"          -> ResolveIndirect { site = insn.address() }
/// - "call <hex>"       -> Call { target=<hex> }
/// - "ret <hex>"        -> ReturnTo { to=<hex> }
/// - "end"              -> End
#[derive(Default)]
pub struct FakeLifter;

fn parse_hex(tok: &str) -> Va {
    let t = tok.strip_prefix("0x").unwrap_or(tok);
    Va::from_str_radix(t, 16).expect("FakeLifter: bad hex token")
}

impl Lifter for FakeLifter {
    type Builder = RecordingBuilder;

    fn lift_body(&mut self, insn: &dyn InsnView, builder: &mut RecordingBuilder) -> Transfer {
        let m = insn.mnemonic();
        let parts: Vec<&str> = m.split_whitespace().collect();
        match parts.as_slice() {
            ["nop"] => Transfer::Fallthrough,
            ["jmp", tgt] => Transfer::Static {
                kind: BranchKind::Unconditional,
                to: parse_hex(tgt),
                taken: true,
            },
            ["jcc", taken, fallthrough] => Transfer::ResolveBranch {
                taken_target: parse_hex(taken),
                fallthrough: parse_hex(fallthrough),
            },
            ["jmp_ind"] => Transfer::ResolveIndirect { site: insn.address() },
            ["call", tgt] => Transfer::Call { target: parse_hex(tgt) },
            ["ret", to] => Transfer::ReturnTo { to: parse_hex(to) },
            ["end"] => Transfer::End,
            other => {
                // Represent an unknown instruction as a no-op body; the engine
                // treats an unrecognized mnemonic conservatively.
                let _ = other;
                builder.log.push(format!("body {m}"));
                Transfer::Fallthrough
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insn::FakeInsn;

    #[test]
    fn fake_lifter_maps_mnemonics_to_transfers() {
        let mut l = FakeLifter;
        let mut b = RecordingBuilder::new();

        assert_eq!(l.lift_body(&FakeInsn::nop(0x10), &mut b), Transfer::Fallthrough);
        assert_eq!(
            l.lift_body(&FakeInsn::with(0x20, 5, "jmp 0x2000"), &mut b),
            Transfer::Static { kind: BranchKind::Unconditional, to: 0x2000, taken: true }
        );
        assert_eq!(
            l.lift_body(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"), &mut b),
            Transfer::ResolveBranch { taken_target: 0x40, fallthrough: 0x32 }
        );
        assert_eq!(
            l.lift_body(&FakeInsn::with(0x50, 2, "jmp_ind"), &mut b),
            Transfer::ResolveIndirect { site: 0x50 }
        );
        assert_eq!(
            l.lift_body(&FakeInsn::with(0x60, 5, "call 0x7000"), &mut b),
            Transfer::Call { target: 0x7000 }
        );
    }
}
