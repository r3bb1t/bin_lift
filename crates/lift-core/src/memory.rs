//! Caller-declared memory facts. This is where knowledge like "ignore writes to
//! this section" or "this region is a read-only constant image" lives — supplied
//! by the caller, never inferred by the core.

use crate::address::{AddrRange, Va};

/// How the caller has declared a memory region should be treated.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemoryAttr {
    /// Reads may be folded from caller-provided bytes.
    Concrete,
    /// Reads must emit symbolic load IR.
    Symbolic,
    /// Constant image: reads folded, writes are an error/ignored per policy.
    ReadOnly,
    /// Contents may change unpredictably; do not cache values across accesses.
    Volatile,
    /// Writes to this region are discarded (e.g. a scratch region the caller
    /// has declared irrelevant). Never inferred — always declared.
    IgnoreWrites,
}

/// An ordered set of caller-declared region attributes plus an optional default.
#[derive(Clone, Default)]
pub struct MemoryFacts {
    ranges: Vec<(AddrRange, MemoryAttr)>,
    default: Option<MemoryAttr>,
}

impl MemoryFacts {
    pub fn new() -> Self {
        Self { ranges: Vec::new(), default: None }
    }

    /// Set the attribute returned for addresses not covered by any declared range.
    pub fn with_default(mut self, attr: MemoryAttr) -> Self {
        self.default = Some(attr);
        self
    }

    /// Declare `attr` over `range`. Later declarations win where ranges overlap.
    pub fn declare(&mut self, range: AddrRange, attr: MemoryAttr) {
        self.ranges.push((range, attr));
    }

    /// Resolve the attribute for `addr`: the most-recently declared covering
    /// range wins; otherwise the default; otherwise `None`.
    pub fn attr_at(&self, addr: Va) -> Option<MemoryAttr> {
        self.ranges
            .iter()
            .rev()
            .find(|(range, _)| range.contains(addr))
            .map(|(_, attr)| *attr)
            .or(self.default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_ranges_resolve_with_last_wins_and_default() {
        let mut facts = MemoryFacts::new().with_default(MemoryAttr::Symbolic);
        facts.declare(AddrRange::with_len(0x1000, 0x1000), MemoryAttr::Concrete);
        // Overlapping later declaration wins.
        facts.declare(AddrRange::with_len(0x1800, 0x0800), MemoryAttr::IgnoreWrites);

        assert_eq!(facts.attr_at(0x1400), Some(MemoryAttr::Concrete));
        assert_eq!(facts.attr_at(0x1900), Some(MemoryAttr::IgnoreWrites));
        // Uncovered address falls back to the declared default.
        assert_eq!(facts.attr_at(0x9000), Some(MemoryAttr::Symbolic));
    }

    #[test]
    fn no_default_returns_none_for_uncovered() {
        let facts = MemoryFacts::new();
        assert_eq!(facts.attr_at(0x1000), None);
    }
}
