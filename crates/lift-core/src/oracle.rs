//! The live-oracle seam: an emulator/instrumentation (Frida, Unicorn, Sogen) the
//! engine can consult to concretize a value, resolve a branch, or snapshot/probe/
//! roll back. All methods default to "I can't help" so a null oracle forces the
//! engine to suspend and ask the caller directly.

use crate::address::Va;
use std::collections::HashMap;

/// A snapshot handle the oracle understands.
pub type SnapshotId = u64;

pub trait Oracle {
    /// Concrete verdict for the conditional branch at `site`.
    fn resolve_branch(&mut self, site: Va) -> Option<bool> {
        let _ = site;
        None
    }

    /// Concrete target set for the indirect transfer at `site`.
    fn resolve_indirect(&mut self, site: Va) -> Option<Vec<Va>> {
        let _ = site;
        None
    }

    /// Concrete bytes at `addr` for `size` bytes.
    fn read_memory(&mut self, addr: Va, size: u32) -> Option<Vec<u8>> {
        let _ = (addr, size);
        None
    }

    /// Capture current state; returns a handle usable with `restore`.
    fn snapshot(&mut self) -> SnapshotId {
        0
    }

    /// Roll back to a previously captured state.
    fn restore(&mut self, id: SnapshotId) {
        let _ = id;
    }
}

/// An oracle that never helps.
pub struct NullOracle;

impl Oracle for NullOracle {}

/// A scripted oracle for tests: answers from fixed maps.
#[derive(Default)]
pub struct ScriptedOracle {
    pub branches: HashMap<Va, bool>,
    pub indirects: HashMap<Va, Vec<Va>>,
    pub memory: HashMap<Va, Vec<u8>>,
    pub snapshots: u64,
}

impl Oracle for ScriptedOracle {
    fn resolve_branch(&mut self, site: Va) -> Option<bool> {
        self.branches.get(&site).copied()
    }

    fn resolve_indirect(&mut self, site: Va) -> Option<Vec<Va>> {
        self.indirects.get(&site).cloned()
    }

    fn read_memory(&mut self, addr: Va, _size: u32) -> Option<Vec<u8>> {
        self.memory.get(&addr).cloned()
    }

    fn snapshot(&mut self) -> SnapshotId {
        self.snapshots += 1;
        self.snapshots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_oracle_helps_with_nothing() {
        let mut o = NullOracle;
        assert_eq!(o.resolve_branch(0x10), None);
        assert_eq!(o.resolve_indirect(0x10), None);
        assert_eq!(o.read_memory(0x10, 4), None);
    }

    #[test]
    fn scripted_oracle_answers_from_maps() {
        let mut o = ScriptedOracle::default();
        o.branches.insert(0x40, true);
        o.indirects.insert(0x50, vec![0x100]);
        assert_eq!(o.resolve_branch(0x40), Some(true));
        assert_eq!(o.resolve_indirect(0x50), Some(vec![0x100]));
        let s = o.snapshot();
        o.restore(s);
        assert_eq!(s, 1);
    }
}
