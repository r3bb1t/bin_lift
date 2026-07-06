//! Virtual-address primitives.

/// A guest virtual address.
pub type Va = u64;

/// A half-open virtual-address range `[start, end)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AddrRange {
    pub start: Va,
    pub end: Va,
}

impl AddrRange {
    /// Create `[start, end)`. Panics if `end < start`.
    pub fn new(start: Va, end: Va) -> Self {
        assert!(end >= start, "AddrRange end < start");
        Self { start, end }
    }

    /// Create `[start, start + len)`.
    pub fn with_len(start: Va, len: u64) -> Self {
        Self { start, end: start.checked_add(len).expect("AddrRange overflow") }
    }

    pub fn len(&self) -> u64 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    /// True if `addr` is within `[start, end)`.
    pub fn contains(&self, addr: Va) -> bool {
        addr >= self.start && addr < self.end
    }

    /// True if the two ranges share at least one address.
    pub fn overlaps(&self, other: &AddrRange) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_membership_and_overlap() {
        let r = AddrRange::with_len(0x1000, 0x1000); // [0x1000, 0x2000)
        assert_eq!(r.len(), 0x1000);
        assert!(!r.is_empty());
        assert!(r.contains(0x1000));
        assert!(r.contains(0x1fff));
        assert!(!r.contains(0x2000));
        assert!(!r.contains(0x0fff));

        assert!(r.overlaps(&AddrRange::new(0x1800, 0x2800)));
        assert!(!r.overlaps(&AddrRange::new(0x2000, 0x3000)));
        assert!(AddrRange::new(0x100, 0x100).is_empty());
    }
}
