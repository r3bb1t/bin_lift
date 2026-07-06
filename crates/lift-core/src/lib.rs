#![forbid(unsafe_code)]

//! `lift-core` — the arch- and backend-agnostic spine of the bin_lift lifter.
//!
//! # Assumption-free by construction
//! Control-transfer resolution always follows `AssumptionProvider` -> `Oracle` ->
//! suspend. The core never applies a built-in packer/OS heuristic; all such
//! knowledge is caller-declared via [`AssumptionProvider`] and [`MemoryFacts`].
//!
//! # Streaming
//! Drive lifting with [`Session::step`]; answer any [`StepOutcome::Suspend`] with
//! [`Session::resume`].

pub mod address;
pub mod assume;
pub mod event;
pub mod insn;
pub mod ir;
pub mod lifter;
pub mod memory;
pub mod oracle;
pub mod protocol;
pub mod session;
pub mod signature;

pub use address::{AddrRange, Va};
pub use assume::{AssumptionProvider, MapAssumptions, NoAssumptions, PredicateVerdict};
pub use event::{BranchKind, Event, EventKind, Events, MemRw, Seg};
pub use insn::{FakeInsn, InsnView, OperandView};
pub use ir::{IcmpPred, IrBuilder, RecordingBuilder};
pub use lifter::{FakeLifter, Lifter, Transfer};
pub use memory::{MemoryAttr, MemoryFacts};
pub use oracle::{NullOracle, Oracle, ScriptedOracle, SnapshotId};
pub use protocol::{Answer, Query, ResumeToken, StepOutcome};
pub use session::Session;
pub use signature::{
    AbiKind, CallEffects, FnSig, MapSignatures, MemEffect, NoSignatures, RegId, RegSet,
    SignatureProvider,
};

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(2 + 2, 4);
    }
}
