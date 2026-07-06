//! Feedback events emitted to the caller as instructions are lifted. These mirror
//! into a `#[repr(C)]` form in the FFI crate; here they are ergonomic Rust enums.

use crate::address::Va;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BranchKind {
    Conditional,
    Unconditional,
    Indirect,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seg {
    Fs,
    Gs,
    Cs,
    Ds,
    Es,
    Ss,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemRw {
    Read,
    Write,
}

/// What happened while lifting an instruction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EventKind {
    Branch { kind: BranchKind, from: Va, to: Va, taken: bool },
    Call { from: Va, target: Va },
    Return { from: Va, to: Va },
    MemAccess { addr: Va, size: u32, rw: MemRw },
    Syscall { number: u32 },
    SegmentAccess { seg: Seg, offset: u64 },
    Fault { code: u32 },
    Unsupported { addr: Va },
}

/// An event tagged with the address of the instruction that produced it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    pub at: Va,
    pub kind: EventKind,
}

impl Event {
    pub fn new(at: Va, kind: EventKind) -> Self {
        Self { at, kind }
    }
}

/// A batch of events produced by a single `step`/`resume`.
pub type Events = Vec<Event>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_construct_and_compare() {
        let e = Event::new(
            0x1000,
            EventKind::Branch {
                kind: BranchKind::Unconditional,
                from: 0x1000,
                to: 0x2000,
                taken: true,
            },
        );
        assert_eq!(e.at, 0x1000);
        assert_eq!(
            e.kind,
            EventKind::Branch {
                kind: BranchKind::Unconditional,
                from: 0x1000,
                to: 0x2000,
                taken: true,
            }
        );
    }
}
