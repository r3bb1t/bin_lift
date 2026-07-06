#![forbid(unsafe_code)]

//! `lift-core` — the arch- and backend-agnostic spine of the bin_lift lifter.
//!
//! It defines the seam traits (`IrBuilder`, `MemoryModel`, `AssumptionProvider`,
//! `Oracle`, `SignatureProvider`, `InsnView`, `Lifter`) and the streaming
//! [`Session`] engine. The core is assumption-free: control-transfer resolution
//! always follows `AssumptionProvider` -> `Oracle` -> suspend, never a built-in
//! heuristic.

#[cfg(test)]
mod tests {
    #[test]
    fn crate_builds() {
        assert_eq!(2 + 2, 4);
    }
}
