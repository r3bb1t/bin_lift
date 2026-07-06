//! The caller-declared assumption channel. Keeps the core assumption-free: the
//! lifter consults declared facts here before ever suspending to ask the caller,
//! and never applies a built-in heuristic (no hardcoded stack sentinels, no
//! "this is a packer section" rules).

use crate::address::Va;
use std::collections::HashMap;

/// A caller's verdict on a conditional predicate at a site.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PredicateVerdict {
    AlwaysTaken,
    NeverTaken,
    Unknown,
}

/// Facts the caller declares to the lifter. All methods default to "I don't know"
/// so an empty provider forces the engine to stay fully general.
pub trait AssumptionProvider {
    /// A value known to be constant at `site`, if the caller has declared it.
    fn known_value(&self, site: Va) -> Option<u64> {
        let _ = site;
        None
    }

    /// A verdict on the conditional branch at `site`.
    fn predicate(&self, site: Va) -> PredicateVerdict {
        let _ = site;
        PredicateVerdict::Unknown
    }

    /// The canonical entry stack pointer, if declared (replaces Mergen's
    /// hardcoded `STACKP_VALUE`). Used to tell a real return from a ROP dispatch.
    fn stack_base(&self) -> Option<Va> {
        None
    }

    /// Declared targets for the indirect transfer at `site`.
    fn indirect_targets(&self, site: Va) -> Option<Vec<Va>> {
        let _ = site;
        None
    }
}

/// An assumption provider that declares nothing. The engine stays fully general.
pub struct NoAssumptions;

impl AssumptionProvider for NoAssumptions {}

/// A map-backed provider for tests and simple callers.
#[derive(Default)]
pub struct MapAssumptions {
    pub known: HashMap<Va, u64>,
    pub preds: HashMap<Va, PredicateVerdict>,
    pub targets: HashMap<Va, Vec<Va>>,
    pub stack_base: Option<Va>,
}

impl AssumptionProvider for MapAssumptions {
    fn known_value(&self, site: Va) -> Option<u64> {
        self.known.get(&site).copied()
    }

    fn predicate(&self, site: Va) -> PredicateVerdict {
        self.preds.get(&site).copied().unwrap_or(PredicateVerdict::Unknown)
    }

    fn stack_base(&self) -> Option<Va> {
        self.stack_base
    }

    fn indirect_targets(&self, site: Va) -> Option<Vec<Va>> {
        self.targets.get(&site).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_assumptions_knows_nothing() {
        let a = NoAssumptions;
        assert_eq!(a.known_value(0x10), None);
        assert_eq!(a.predicate(0x10), PredicateVerdict::Unknown);
        assert_eq!(a.stack_base(), None);
        assert_eq!(a.indirect_targets(0x10), None);
    }

    #[test]
    fn map_assumptions_returns_declared_facts() {
        let mut a = MapAssumptions::default();
        a.preds.insert(0x40, PredicateVerdict::AlwaysTaken);
        a.targets.insert(0x50, vec![0x100, 0x200]);
        a.stack_base = Some(0x14fea0);

        assert_eq!(a.predicate(0x40), PredicateVerdict::AlwaysTaken);
        assert_eq!(a.predicate(0x41), PredicateVerdict::Unknown);
        assert_eq!(a.indirect_targets(0x50), Some(vec![0x100, 0x200]));
        assert_eq!(a.stack_base(), Some(0x14fea0));
    }
}
