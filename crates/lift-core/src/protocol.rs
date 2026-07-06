//! The streaming suspend/resume protocol. Deliberately FFI-clean: a caller drains
//! a `Query`, probes its emulator, and answers with `resume(token, Answer)` — no
//! callback pointers cross a boundary.

use crate::address::Va;
use crate::event::Events;
use crate::signature::FnSig;

/// Opaque handle correlating a `Suspend` with its `resume`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResumeToken(pub u64);

/// A resolution the engine needs before it can continue.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Query {
    /// Is the conditional branch at `site` taken? (real vs. fake / opaque)
    BranchVerdict { site: Va, taken_target: Va, fallthrough: Va },
    /// What are the possible targets of the indirect transfer at `site`?
    IndirectTargets { site: Va },
    /// Concrete bytes at `addr` for `size` bytes.
    Memory { addr: Va, size: u32 },
    /// The signature of the call `target`.
    Signature { target: Va },
}

/// The caller's answer to a `Query`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Answer {
    BranchVerdict { taken: bool },
    Targets(Vec<Va>),
    Memory(Vec<u8>),
    Signature(Option<FnSig>),
}

impl Query {
    /// True if `answer` is the right variant for this query.
    pub fn accepts(&self, answer: &Answer) -> bool {
        matches!(
            (self, answer),
            (Query::BranchVerdict { .. }, Answer::BranchVerdict { .. })
                | (Query::IndirectTargets { .. }, Answer::Targets(_))
                | (Query::Memory { .. }, Answer::Memory(_))
                | (Query::Signature { .. }, Answer::Signature(_))
        )
    }
}

/// The result of a single `step` or `resume`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StepOutcome {
    /// Instruction lifted; execution continues to the next instruction.
    Continue(Events),
    /// The engine needs a resolution; answer via `resume(token, ..)`.
    Suspend(Query, ResumeToken),
    /// A block terminator was lifted; the block is complete.
    BlockEnd(Events),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_accepts_matching_answers_only() {
        let q = Query::IndirectTargets { site: 0x10 };
        assert!(q.accepts(&Answer::Targets(vec![0x20])));
        assert!(!q.accepts(&Answer::BranchVerdict { taken: true }));

        let q = Query::BranchVerdict { site: 0x10, taken_target: 0x20, fallthrough: 0x14 };
        assert!(q.accepts(&Answer::BranchVerdict { taken: false }));
        assert!(!q.accepts(&Answer::Memory(vec![])));

        let q = Query::Memory { addr: 0x10, size: 4 };
        assert!(q.accepts(&Answer::Memory(vec![0u8; 4])));
        assert!(!q.accepts(&Answer::Targets(vec![])));

        let q = Query::Signature { target: 0x10 };
        assert!(q.accepts(&Answer::Signature(None)));
        assert!(!q.accepts(&Answer::BranchVerdict { taken: true }));
    }
}
