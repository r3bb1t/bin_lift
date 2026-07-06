#![forbid(unsafe_code)]

//! `lift-core` — the arch- and backend-agnostic spine of the bin_lift lifter.
//!
//! It defines the seam traits (`IrBuilder`, `MemoryModel`, `AssumptionProvider`,
//! `Oracle`, `SignatureProvider`, `InsnView`, `Lifter`) and the streaming
//! [`Session`] engine. The core is assumption-free: control-transfer resolution
//! always follows `AssumptionProvider` -> `Oracle` -> suspend, never a built-in
//! heuristic.

pub mod address;
pub mod assume;
pub mod event;
pub mod memory;
pub mod signature;

pub use address::{AddrRange, Va};
pub use assume::{AssumptionProvider, MapAssumptions, NoAssumptions, PredicateVerdict};
pub use event::{BranchKind, Event, EventKind, Events, MemRw, Seg};
pub use memory::{MemoryAttr, MemoryFacts};
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
