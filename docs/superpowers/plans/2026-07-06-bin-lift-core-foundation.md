# bin_lift v2 — Milestone 1: `lift-core` Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `lift-core` — the arch-agnostic, backend-agnostic spine of the new lifter: the seam traits, the event/query/answer types, and the streaming `Session` engine with its resolve-or-suspend state machine — all fully unit-tested against in-crate test doubles (no LLVM, no x86, no external deps).

**Architecture:** A Cargo workspace is introduced at the repo root (the legacy `bin_lift` package stays intact as a build member and correctness oracle). `lift-core` defines every cross-cutting trait (`IrBuilder`, `MemoryModel`/`MemoryFacts`, `AssumptionProvider`, `Oracle`, `SignatureProvider`, `InsnView`, `Lifter`) plus the `Session` that drives streaming lifting: it calls a `Lifter` to translate one instruction, then resolves any control-transfer need through the fixed precedence **AssumptionProvider → Oracle → Suspend** — never a built-in packer heuristic. Suspension is FFI-clean: `step` returns `Suspend(Query, ResumeToken)`, the caller answers via `resume(token, Answer)`.

**Tech Stack:** Rust (edition 2021), std-only for `lift-core`. Test doubles (`RecordingBuilder`, `FakeInsn`, `FakeLifter`, `ScriptedOracle`, `MapAssumptions`) live in `lift-core` behind `#[cfg(test)]`-friendly public modules so later crates and tests can reuse them.

## Global Constraints

- **Rust edition `2021`** for all new crates.
- **`lift-core` has zero external dependencies** (pure `std`) — it is the spine; keep it dependency-free.
- **No `unsafe`** in `lift-core` (`#![forbid(unsafe_code)]`).
- **Assumption-free invariant:** the core applies **no** packer/OS heuristics. Every control-transfer resolution follows exactly this precedence: `AssumptionProvider` → `Oracle` → `Suspend(Query)`. With no facts and a null oracle, the engine **suspends**; it never silently assumes.
- **Suspension is instruction-terminal only:** data-flow lifting (arithmetic, moves, memory) never suspends — it uses declared facts or emits symbolic IR. Only control-transfer / signature resolution can suspend.
- **Addresses come from the instruction stream:** each `InsnView` carries its own runtime address; the `Session` does not synthesize a program counter.
- **Naming:** new crates are `lift-*` under `crates/`. The workspace root remains the legacy `bin_lift` package and also becomes the `[workspace]` root.
- **Commits:** every commit message ends with the trailer:
  ```
  Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
  ```
  (shown in full in Task 1; abbreviated as `<trailer>` in later tasks — always include it verbatim).
- **Test command (workspace):** `cargo test -p lift-core`. **Build command:** `cargo build -p lift-core`.

---

### Task 1: Introduce the workspace and scaffold `lift-core`

**Files:**
- Modify: `Cargo.toml` (repo root — add a `[workspace]` section)
- Create: `crates/lift-core/Cargo.toml`
- Create: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: a buildable `lift-core` library crate and a workspace that still builds the legacy `bin_lift` package.

- [ ] **Step 1: Add the `[workspace]` section to the root `Cargo.toml`**

Append this block to the end of the existing root `Cargo.toml` (leave the existing `[package]`, `[lib]`, `[dependencies]` sections untouched):

```toml
[workspace]
members = ["crates/lift-core"]
resolver = "2"
```

- [ ] **Step 2: Create the `lift-core` manifest**

Create `crates/lift-core/Cargo.toml`:

```toml
[package]
name = "lift-core"
version = "0.0.1"
edition = "2021"
description = "Arch- and backend-agnostic spine for the bin_lift streaming lifter."

[dependencies]
```

- [ ] **Step 3: Create the crate root with the build smoke test**

Create `crates/lift-core/src/lib.rs`:

```rust
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
```

- [ ] **Step 4: Verify the workspace builds and the smoke test passes**

Run: `cargo test -p lift-core`
Expected: PASS — `test tests::crate_builds ... ok`.

Also run: `cargo build`
Expected: the legacy `bin_lift` package and `lift-core` both build (legacy build requires LLVM 18; if LLVM is unavailable in the environment, `cargo build -p lift-core` must still succeed — record that and proceed).

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/lift-core/Cargo.toml crates/lift-core/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(lift-core): scaffold workspace and lift-core crate

Convert the repo root into a Cargo workspace (legacy bin_lift package kept
as a build member and correctness oracle) and add the empty lift-core spine.

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: Address and range types

**Files:**
- Create: `crates/lift-core/src/address.rs`
- Modify: `crates/lift-core/src/lib.rs` (add `pub mod address;` and re-exports)

**Interfaces:**
- Consumes: nothing.
- Produces: `type Va = u64;` and `struct AddrRange { start: Va, end: Va }` with `new`, `with_len`, `len`, `is_empty`, `contains(Va) -> bool`, `overlaps(&AddrRange) -> bool`.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/address.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add after the doc comment (before `mod tests`):

```rust
pub mod address;

pub use address::{AddrRange, Va};
```

- [ ] **Step 3: Run the test to verify it passes**

Run: `cargo test -p lift-core address`
Expected: PASS — `range_membership_and_overlap ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/address.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add Va and AddrRange address primitives

<trailer>"
```

---

### Task 3: Memory facts (caller-declared region attributes)

**Files:**
- Create: `crates/lift-core/src/memory.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `AddrRange`, `Va`.
- Produces: `enum MemoryAttr { Concrete, Symbolic, ReadOnly, Volatile, IgnoreWrites }`; `struct MemoryFacts` with `new()`, `with_default(MemoryAttr) -> Self`, `declare(AddrRange, MemoryAttr)`, `attr_at(Va) -> Option<MemoryAttr>` (last declaration wins on overlap, falling back to the default).

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/memory.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod memory;

pub use memory::{MemoryAttr, MemoryFacts};
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core memory`
Expected: PASS — both `declared_ranges_resolve_with_last_wins_and_default` and `no_default_returns_none_for_uncovered`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/memory.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add caller-declared MemoryFacts

<trailer>"
```

---

### Task 4: Assumption provider (the declared-facts channel)

**Files:**
- Create: `crates/lift-core/src/assume.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `enum PredicateVerdict { AlwaysTaken, NeverTaken, Unknown }`; `trait AssumptionProvider` with defaulted methods `known_value(Va) -> Option<u64>`, `predicate(Va) -> PredicateVerdict`, `stack_base() -> Option<Va>`, `indirect_targets(Va) -> Option<Vec<Va>>`; `struct NoAssumptions`; `struct MapAssumptions` (public fields `known`, `preds`, `targets`, `stack_base`).

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/assume.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod assume;

pub use assume::{AssumptionProvider, MapAssumptions, NoAssumptions, PredicateVerdict};
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core assume`
Expected: PASS — both tests.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/assume.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add AssumptionProvider declared-facts channel

<trailer>"
```

---

### Task 5: Event taxonomy

**Files:**
- Create: `crates/lift-core/src/event.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `enum BranchKind { Conditional, Unconditional, Indirect }`, `enum Seg { Fs, Gs, Cs, Ds, Es, Ss }`, `enum MemRw { Read, Write }`, `enum EventKind { Branch{kind,from,to,taken}, Call{from,target}, Return{from,to}, MemAccess{addr,size,rw}, Syscall{number}, SegmentAccess{seg,offset}, Fault{code}, Unsupported{addr} }`, `struct Event { at: Va, kind: EventKind }`, `type Events = Vec<Event>`.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/event.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod event;

pub use event::{BranchKind, Event, EventKind, Events, MemRw, Seg};
```

- [ ] **Step 3: Run the test to verify it passes**

Run: `cargo test -p lift-core event`
Expected: PASS — `events_construct_and_compare ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/event.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add event taxonomy

<trailer>"
```

---

### Task 6: Signature and ABI types

**Files:**
- Create: `crates/lift-core/src/signature.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `struct RegId(u16)`, `struct RegSet(Vec<RegId>)`, `enum AbiKind { X64Msvc, X64SysV, Cdecl, Stdcall, Fastcall }`, `enum MemEffect { None, MayRead, MayWrite, MayReadWrite }`, `struct FnSig { name: Option<String>, abi: AbiKind, arg_count: usize, returns: bool, variadic: bool }`, `struct CallEffects { arg_regs, ret_regs, volatile_regs: RegSet, stack_delta: i64, mem_effect: MemEffect }`, `trait SignatureProvider` (defaulted `resolve_addr`, `resolve_name`), `struct NoSignatures`, `struct MapSignatures`.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/signature.rs`:

```rust
//! Function-signature and calling-convention types. `FnSig` is referenced by the
//! query/answer protocol; `CallEffects` (ported from Mergen's AbiCallContract)
//! describes how a call clobbers state.

use crate::address::Va;
use std::collections::HashMap;

/// Arch-neutral register identifier. Each arch crate maps its registers to these.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RegId(pub u16);

/// A set of registers (order-preserving; used for arg/return/volatile lists).
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct RegSet(pub Vec<RegId>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AbiKind {
    X64Msvc,
    X64SysV,
    Cdecl,
    Stdcall,
    Fastcall,
}

/// Coarse memory effect of a call, for later optimization/aliasing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MemEffect {
    None,
    MayRead,
    MayWrite,
    MayReadWrite,
}

/// A resolved function signature.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FnSig {
    pub name: Option<String>,
    pub abi: AbiKind,
    pub arg_count: usize,
    pub returns: bool,
    pub variadic: bool,
}

/// How a call affects machine state (ported from Mergen's `CallEffects`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CallEffects {
    pub arg_regs: RegSet,
    pub ret_regs: RegSet,
    pub volatile_regs: RegSet,
    pub stack_delta: i64,
    pub mem_effect: MemEffect,
}

/// Resolves a call target (by address or name) to a signature.
pub trait SignatureProvider {
    fn resolve_addr(&self, addr: Va) -> Option<FnSig> {
        let _ = addr;
        None
    }

    fn resolve_name(&self, name: &str) -> Option<FnSig> {
        let _ = name;
        None
    }
}

/// A provider that resolves nothing.
pub struct NoSignatures;

impl SignatureProvider for NoSignatures {}

/// A map-backed provider for tests and simple callers.
#[derive(Default)]
pub struct MapSignatures {
    pub by_addr: HashMap<Va, FnSig>,
    pub by_name: HashMap<String, FnSig>,
}

impl SignatureProvider for MapSignatures {
    fn resolve_addr(&self, addr: Va) -> Option<FnSig> {
        self.by_addr.get(&addr).cloned()
    }

    fn resolve_name(&self, name: &str) -> Option<FnSig> {
        self.by_name.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_signatures_resolve_by_addr_and_name() {
        let sig = FnSig {
            name: Some("NtClose".to_string()),
            abi: AbiKind::X64Msvc,
            arg_count: 1,
            returns: true,
            variadic: false,
        };
        let mut sigs = MapSignatures::default();
        sigs.by_addr.insert(0x7ff0_0000, sig.clone());
        sigs.by_name.insert("NtClose".to_string(), sig.clone());

        assert_eq!(sigs.resolve_addr(0x7ff0_0000), Some(sig.clone()));
        assert_eq!(sigs.resolve_addr(0x1234), None);
        assert_eq!(sigs.resolve_name("NtClose"), Some(sig));
        assert_eq!(NoSignatures.resolve_addr(0x7ff0_0000), None);
    }

    #[test]
    fn call_effects_construct() {
        let ce = CallEffects {
            arg_regs: RegSet(vec![RegId(1), RegId(2)]),
            ret_regs: RegSet(vec![RegId(0)]),
            volatile_regs: RegSet(vec![RegId(1), RegId(2), RegId(0)]),
            stack_delta: 0,
            mem_effect: MemEffect::MayReadWrite,
        };
        assert_eq!(ce.arg_regs.0.len(), 2);
        assert_eq!(ce.mem_effect, MemEffect::MayReadWrite);
    }
}
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod signature;

pub use signature::{
    AbiKind, CallEffects, FnSig, MapSignatures, MemEffect, NoSignatures, RegId, RegSet,
    SignatureProvider,
};
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core signature`
Expected: PASS — both tests.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/signature.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add signature and ABI types

<trailer>"
```

---

### Task 7: Query / Answer / StepOutcome protocol types

**Files:**
- Create: `crates/lift-core/src/protocol.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`, `FnSig`, `Events`.
- Produces: `struct ResumeToken(u64)`, `enum Query { BranchVerdict{site,taken_target,fallthrough}, IndirectTargets{site}, Memory{addr,size}, Signature{target} }`, `enum Answer { BranchVerdict{taken}, Targets(Vec<Va>), Memory(Vec<u8>), Signature(Option<FnSig>) }`, `Query::accepts(&Answer) -> bool`, `enum StepOutcome { Continue(Events), Suspend(Query, ResumeToken), BlockEnd(Events) }`.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/protocol.rs`:

```rust
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
    }
}
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod protocol;

pub use protocol::{Answer, Query, ResumeToken, StepOutcome};
```

- [ ] **Step 3: Run the test to verify it passes**

Run: `cargo test -p lift-core protocol`
Expected: PASS — `query_accepts_matching_answers_only ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/protocol.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add suspend/resume protocol types

<trailer>"
```

---

### Task 8: `IrBuilder` trait and the `RecordingBuilder` test double

**Files:**
- Create: `crates/lift-core/src/ir.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `trait IrBuilder { type Value: Clone; type Block: Clone; fn const_addr(&mut self, Va) -> Value; fn new_block(&mut self, &str) -> Block; fn position_at(&mut self, &Block); fn br(&mut self, &Block); fn cond_br(&mut self, Value, &Block, &Block); fn switch(&mut self, Value, &Block, &[(Va, Block)]); fn ret(&mut self, Value); fn unreachable(&mut self); }`; `struct RecordingBuilder { log: Vec<String> }` implementing it with `Value = Va`, `Block = usize`.

**Note:** This is the *minimal* control-flow-oriented slice of `IrBuilder` that Milestone 1's `Session` exercises. Milestones 2–3 extend the trait with arithmetic/memory operations. `RecordingBuilder` records a human-readable op log so `Session` behavior can be asserted without a real IR backend.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/ir.rs`:

```rust
//! The IR backend seam. Milestone 1 defines only the control-flow slice the
//! streaming engine needs; later milestones extend it with data-flow ops. Only
//! backend crates (lift-llvmkit, lift-llvm) implement it against a real IR; the
//! `RecordingBuilder` here is a test double that logs emitted operations.

use crate::address::Va;

/// A backend that builds IR as the lifter emits operations.
pub trait IrBuilder {
    /// An SSA value in the backend's IR.
    type Value: Clone;
    /// A basic block handle in the backend's IR.
    type Block: Clone;

    /// Materialize a constant address value.
    fn const_addr(&mut self, value: Va) -> Self::Value;
    /// Create a new basic block with a label hint.
    fn new_block(&mut self, label: &str) -> Self::Block;
    /// Set the insertion point to `block`.
    fn position_at(&mut self, block: &Self::Block);
    /// Emit an unconditional branch to `target`.
    fn br(&mut self, target: &Self::Block);
    /// Emit a conditional branch.
    fn cond_br(&mut self, cond: Self::Value, if_true: &Self::Block, if_false: &Self::Block);
    /// Emit an N-way switch on `scrutinee`.
    fn switch(&mut self, scrutinee: Self::Value, default: &Self::Block, cases: &[(Va, Self::Block)]);
    /// Emit a return of `value`.
    fn ret(&mut self, value: Self::Value);
    /// Emit an `unreachable` terminator.
    fn unreachable(&mut self);
}

/// A test-double backend that records emitted operations as strings.
#[derive(Default)]
pub struct RecordingBuilder {
    pub log: Vec<String>,
    next_block: usize,
}

impl RecordingBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IrBuilder for RecordingBuilder {
    type Value = Va;
    type Block = usize;

    fn const_addr(&mut self, value: Va) -> Va {
        self.log.push(format!("const {value:#x}"));
        value
    }

    fn new_block(&mut self, label: &str) -> usize {
        let id = self.next_block;
        self.next_block += 1;
        self.log.push(format!("block#{id} {label}"));
        id
    }

    fn position_at(&mut self, block: &usize) {
        self.log.push(format!("position #{block}"));
    }

    fn br(&mut self, target: &usize) {
        self.log.push(format!("br #{target}"));
    }

    fn cond_br(&mut self, _cond: Va, if_true: &usize, if_false: &usize) {
        self.log.push(format!("condbr #{if_true} #{if_false}"));
    }

    fn switch(&mut self, _scrutinee: Va, default: &usize, cases: &[(Va, usize)]) {
        self.log.push(format!("switch default=#{default} cases={}", cases.len()));
    }

    fn ret(&mut self, value: Va) {
        self.log.push(format!("ret {value:#x}"));
    }

    fn unreachable(&mut self) {
        self.log.push("unreachable".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_builder_logs_ops() {
        let mut b = RecordingBuilder::new();
        let entry = b.new_block("entry");
        let target = b.new_block("t");
        b.position_at(&entry);
        let v = b.const_addr(0x2000);
        b.br(&target);
        b.ret(v);

        assert_eq!(
            b.log,
            vec![
                "block#0 entry".to_string(),
                "block#1 t".to_string(),
                "position #0".to_string(),
                "const 0x2000".to_string(),
                "br #1".to_string(),
                "ret 0x2000".to_string(),
            ]
        );
    }
}
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod ir;

pub use ir::{IrBuilder, RecordingBuilder};
```

- [ ] **Step 3: Run the test to verify it passes**

Run: `cargo test -p lift-core ir`
Expected: PASS — `recording_builder_logs_ops ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/ir.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add IrBuilder seam and RecordingBuilder

<trailer>"
```

---

### Task 9: `Oracle` trait with `NullOracle` and `ScriptedOracle`

**Files:**
- Create: `crates/lift-core/src/oracle.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `trait Oracle` with defaulted `resolve_branch(Va) -> Option<bool>`, `resolve_indirect(Va) -> Option<Vec<Va>>`, `read_memory(Va, u32) -> Option<Vec<u8>>`, `snapshot() -> u64`, `restore(u64)`; `struct NullOracle`; `struct ScriptedOracle` (public maps `branches`, `indirects`, `memory`).

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/oracle.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod oracle;

pub use oracle::{NullOracle, Oracle, ScriptedOracle, SnapshotId};
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core oracle`
Expected: PASS — both tests.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/oracle.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add Oracle seam with null and scripted impls

<trailer>"
```

---

### Task 10: `InsnView`/`OperandView` traits and the `FakeInsn` test builder

**Files:**
- Create: `crates/lift-core/src/insn.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Va`.
- Produces: `trait InsnView { fn address(&self) -> Va; fn len(&self) -> u8; fn mnemonic(&self) -> &str; }`; `struct FakeInsn { address: Va, len: u8, mnemonic: String }` with constructors `nop(addr)`, `with(addr, len, mnemonic)`.

**Note:** Milestone 1 does not exercise operands, but `OperandView` is declared as the seam later arch crates extend. `FakeInsn` gives tests a way to feed synthetic instructions to `Session` via `FakeLifter` (Task 11) which pattern-matches on `mnemonic()`.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/insn.rs`:

```rust
//! Arch-neutral instruction view. A decoder frontend (zydis, iced, an emulator's
//! decoder) implements `InsnView` so `lift-core` stays decoder-agnostic without
//! re-modeling every architecture's operands.

use crate::address::Va;

/// A read-only view of one decoded operand. Extended by arch crates in later
/// milestones; declared here so the seam exists.
pub trait OperandView {
    /// Operand width in bits (8/16/32/64/...).
    fn bit_width(&self) -> u16;
}

/// A read-only view of one decoded instruction.
pub trait InsnView {
    /// Runtime virtual address of this instruction.
    fn address(&self) -> Va;
    /// Encoded length in bytes.
    fn len(&self) -> u8;
    /// Canonical mnemonic (lowercase), e.g. "mov", "jmp", "ret".
    fn mnemonic(&self) -> &str;
}

/// A synthetic instruction for tests.
#[derive(Clone, Debug)]
pub struct FakeInsn {
    pub address: Va,
    pub len: u8,
    pub mnemonic: String,
}

impl FakeInsn {
    pub fn with(address: Va, len: u8, mnemonic: &str) -> Self {
        Self { address, len, mnemonic: mnemonic.to_string() }
    }

    pub fn nop(address: Va) -> Self {
        Self::with(address, 1, "nop")
    }
}

impl InsnView for FakeInsn {
    fn address(&self) -> Va {
        self.address
    }

    fn len(&self) -> u8 {
        self.len
    }

    fn mnemonic(&self) -> &str {
        &self.mnemonic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_insn_exposes_view() {
        let i = FakeInsn::with(0x1000, 5, "jmp");
        let v: &dyn InsnView = &i;
        assert_eq!(v.address(), 0x1000);
        assert_eq!(v.len(), 5);
        assert_eq!(v.mnemonic(), "jmp");
    }
}
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod insn;

pub use insn::{FakeInsn, InsnView, OperandView};
```

- [ ] **Step 3: Run the test to verify it passes**

Run: `cargo test -p lift-core insn`
Expected: PASS — `fake_insn_exposes_view ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/insn.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add InsnView/OperandView seam and FakeInsn

<trailer>"
```

---

### Task 11: `Lifter` trait, `Transfer` enum, and `FakeLifter`

**Files:**
- Create: `crates/lift-core/src/lifter.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `IrBuilder`, `InsnView`, `Va`, `BranchKind`.
- Produces: `enum Transfer { Fallthrough, Static{kind:BranchKind,to:Va,taken:bool}, ResolveBranch{taken_target:Va,fallthrough:Va}, ResolveIndirect{site:Va}, Call{target:Va}, ReturnTo{to:Va}, End }`; `trait Lifter { type Builder: IrBuilder; fn lift_body(&mut self, insn: &dyn InsnView, b: &mut Self::Builder) -> Transfer; }`; `struct FakeLifter` implementing it with `Builder = RecordingBuilder`, deciding the `Transfer` from `insn.mnemonic()`.

**Note:** The `Transfer` returned by `lift_body` is how an arch lifter tells the `Session` what control-flow resolution (if any) is needed. This enforces the "suspension is instruction-terminal" constraint: `lift_body` finishes all data-flow IR, then hands a terminal `Transfer` up; the `Session` (Task 12) does the resolve-or-suspend.

- [ ] **Step 1: Write the failing test**

Create `crates/lift-core/src/lifter.rs`:

```rust
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
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod lifter;

pub use lifter::{FakeLifter, Lifter, Transfer};
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core lifter`
Expected: PASS — `fake_lifter_maps_mnemonics_to_transfers ... ok`.

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/lifter.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add Lifter seam, Transfer enum, and FakeLifter

<trailer>"
```

---

### Task 12: The `Session` streaming engine (resolve-or-suspend state machine)

**Files:**
- Create: `crates/lift-core/src/session.rs`
- Modify: `crates/lift-core/src/lib.rs`

**Interfaces:**
- Consumes: `Lifter`, `Transfer`, `AssumptionProvider`, `PredicateVerdict`, `Oracle`, `InsnView`, `IrBuilder`, `Event`, `EventKind`, `BranchKind`, `Query`, `Answer`, `ResumeToken`, `StepOutcome`, `MemoryFacts`, `Va`.
- Produces: `struct Session<L: Lifter, A: AssumptionProvider, O: Oracle>` with `new(lifter, builder: L::Builder, assume: A, oracle: O, facts: MemoryFacts, entry: Va) -> Self`, `step(&mut self, insn: &dyn InsnView) -> StepOutcome`, `resume(&mut self, token: ResumeToken, answer: Answer) -> StepOutcome`, `builder(&self) -> &L::Builder`, `assumptions(&self) -> &A`, `facts(&self) -> &MemoryFacts`.

**Behavior contract (encode exactly):**
- `step` while a suspension is pending panics (`assert!` "resume() before step()").
- Resolution precedence for a conditional branch at `site`: `assume.predicate(site)` (`AlwaysTaken`/`NeverTaken` decide) → else `oracle.resolve_branch(site)` → else `Suspend(Query::BranchVerdict{..})`.
- Resolution precedence for an indirect transfer at `site`: `assume.indirect_targets(site)` → else `oracle.resolve_indirect(site)` → else `Suspend(Query::IndirectTargets{..})`.
- A resolved conditional branch emits `EventKind::Branch{kind:Conditional, from, to, taken}` and calls `builder.const_addr(to)` then a terminator; returns `BlockEnd`.
- A resolved indirect transfer with 1 target emits `Branch{kind:Indirect,..,taken:true}` + `br`; with >1 emits `builder.switch(..)` + `Branch{kind:Indirect, to: targets[0], taken:true}`; with 0 emits `EventKind::Unsupported{addr:site}`; all return `BlockEnd`.
- `Transfer::Static` emits `Branch{..}` + `br`, returns `BlockEnd`.
- `Transfer::Fallthrough` returns `Continue(vec![])`.
- `Transfer::Call{target}` emits `EventKind::Call{from, target}`, returns `Continue`.
- `Transfer::ReturnTo{to}` emits `EventKind::Return{from, to}` + `ret(const_addr(to))`, returns `BlockEnd`.
- `Transfer::End` returns `BlockEnd(vec![])`.

- [ ] **Step 1: Write the failing tests**

Create `crates/lift-core/src/session.rs`:

```rust
//! The streaming lifting engine. It calls a `Lifter` for one instruction's body,
//! then resolves any control-transfer need via the fixed precedence
//! AssumptionProvider -> Oracle -> Suspend. It never applies a built-in heuristic.

use crate::address::Va;
use crate::assume::{AssumptionProvider, PredicateVerdict};
use crate::event::{BranchKind, Event, EventKind};
use crate::insn::InsnView;
use crate::ir::IrBuilder;
use crate::lifter::{Lifter, Transfer};
use crate::memory::MemoryFacts;
use crate::oracle::Oracle;
use crate::protocol::{Answer, Query, ResumeToken, StepOutcome};

/// A resolution the engine is waiting on when suspended.
#[derive(Clone, Copy, Debug)]
enum Pending {
    Branch { from: Va, taken_target: Va, fallthrough: Va },
    Indirect { site: Va },
}

pub struct Session<L: Lifter, A: AssumptionProvider, O: Oracle> {
    lifter: L,
    builder: L::Builder,
    assume: A,
    oracle: O,
    facts: MemoryFacts,
    entry: Va,
    pending: Option<(ResumeToken, Pending)>,
    next_token: u64,
}

impl<L: Lifter, A: AssumptionProvider, O: Oracle> Session<L, A, O> {
    pub fn new(lifter: L, builder: L::Builder, assume: A, oracle: O, facts: MemoryFacts, entry: Va) -> Self {
        Self {
            lifter,
            builder,
            assume,
            oracle,
            facts,
            entry,
            pending: None,
            next_token: 0,
        }
    }

    pub fn builder(&self) -> &L::Builder {
        &self.builder
    }

    pub fn assumptions(&self) -> &A {
        &self.assume
    }

    pub fn facts(&self) -> &MemoryFacts {
        &self.facts
    }

    pub fn entry(&self) -> Va {
        self.entry
    }

    fn issue_token(&mut self) -> ResumeToken {
        let t = ResumeToken(self.next_token);
        self.next_token += 1;
        t
    }

    /// Lift one instruction.
    pub fn step(&mut self, insn: &dyn InsnView) -> StepOutcome {
        assert!(self.pending.is_none(), "call resume() before step() while suspended");
        let from = insn.address();
        let transfer = self.lifter.lift_body(insn, &mut self.builder);
        match transfer {
            Transfer::Fallthrough => StepOutcome::Continue(Vec::new()),
            Transfer::Static { kind, to, taken } => self.finish_static(from, kind, to, taken),
            Transfer::ResolveBranch { taken_target, fallthrough } => {
                self.resolve_branch(from, taken_target, fallthrough)
            }
            Transfer::ResolveIndirect { site } => self.resolve_indirect(site),
            Transfer::Call { target } => {
                let ev = Event::new(from, EventKind::Call { from, target });
                StepOutcome::Continue(vec![ev])
            }
            Transfer::ReturnTo { to } => self.finish_return(from, to),
            Transfer::End => StepOutcome::BlockEnd(Vec::new()),
        }
    }

    /// Answer a suspension and continue.
    pub fn resume(&mut self, token: ResumeToken, answer: Answer) -> StepOutcome {
        let (expected, pending) = self.pending.take().expect("resume() with no pending suspension");
        assert_eq!(token, expected, "resume token mismatch");
        match (pending, answer) {
            (Pending::Branch { from, taken_target, fallthrough }, Answer::BranchVerdict { taken }) => {
                let to = if taken { taken_target } else { fallthrough };
                self.finish_conditional(from, to, taken)
            }
            (Pending::Indirect { site }, Answer::Targets(targets)) => self.finish_indirect(site, targets),
            _ => panic!("resume answer does not match pending query"),
        }
    }

    fn resolve_branch(&mut self, from: Va, taken_target: Va, fallthrough: Va) -> StepOutcome {
        let taken = match self.assume.predicate(from) {
            PredicateVerdict::AlwaysTaken => Some(true),
            PredicateVerdict::NeverTaken => Some(false),
            PredicateVerdict::Unknown => self.oracle.resolve_branch(from),
        };
        match taken {
            Some(t) => {
                let to = if t { taken_target } else { fallthrough };
                self.finish_conditional(from, to, t)
            }
            None => {
                let token = self.issue_token();
                self.pending = Some((token, Pending::Branch { from, taken_target, fallthrough }));
                StepOutcome::Suspend(Query::BranchVerdict { site: from, taken_target, fallthrough }, token)
            }
        }
    }

    fn resolve_indirect(&mut self, site: Va) -> StepOutcome {
        let targets = self
            .assume
            .indirect_targets(site)
            .or_else(|| self.oracle.resolve_indirect(site));
        match targets {
            Some(ts) => self.finish_indirect(site, ts),
            None => {
                let token = self.issue_token();
                self.pending = Some((token, Pending::Indirect { site }));
                StepOutcome::Suspend(Query::IndirectTargets { site }, token)
            }
        }
    }

    fn finish_static(&mut self, from: Va, kind: BranchKind, to: Va, taken: bool) -> StepOutcome {
        let v = self.builder.const_addr(to);
        let block = self.builder.new_block("target");
        self.builder.br(&block);
        let _ = v;
        StepOutcome::BlockEnd(vec![Event::new(from, EventKind::Branch { kind, from, to, taken })])
    }

    fn finish_conditional(&mut self, from: Va, to: Va, taken: bool) -> StepOutcome {
        let v = self.builder.const_addr(to);
        let block = self.builder.new_block("target");
        self.builder.br(&block);
        let _ = v;
        StepOutcome::BlockEnd(vec![Event::new(
            from,
            EventKind::Branch { kind: BranchKind::Conditional, from, to, taken },
        )])
    }

    fn finish_indirect(&mut self, site: Va, targets: Vec<Va>) -> StepOutcome {
        match targets.len() {
            0 => StepOutcome::BlockEnd(vec![Event::new(site, EventKind::Unsupported { addr: site })]),
            1 => {
                let to = targets[0];
                let v = self.builder.const_addr(to);
                let block = self.builder.new_block("indirect_target");
                self.builder.br(&block);
                let _ = v;
                StepOutcome::BlockEnd(vec![Event::new(
                    site,
                    EventKind::Branch { kind: BranchKind::Indirect, from: site, to, taken: true },
                )])
            }
            _ => {
                let scrut = self.builder.const_addr(site);
                let default = self.builder.new_block("switch_default");
                let cases: Vec<(Va, <L::Builder as IrBuilder>::Block)> = targets
                    .iter()
                    .map(|&t| (t, self.builder.new_block("case")))
                    .collect();
                self.builder.switch(scrut, &default, &cases);
                StepOutcome::BlockEnd(vec![Event::new(
                    site,
                    EventKind::Branch { kind: BranchKind::Indirect, from: site, to: targets[0], taken: true },
                )])
            }
        }
    }

    fn finish_return(&mut self, from: Va, to: Va) -> StepOutcome {
        let v = self.builder.const_addr(to);
        self.builder.ret(v);
        StepOutcome::BlockEnd(vec![Event::new(from, EventKind::Return { from, to })])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assume::{MapAssumptions, NoAssumptions, PredicateVerdict};
    use crate::insn::FakeInsn;
    use crate::ir::RecordingBuilder;
    use crate::lifter::FakeLifter;
    use crate::oracle::{NullOracle, ScriptedOracle};

    fn bare_session() -> Session<FakeLifter, NoAssumptions, NullOracle> {
        Session::new(
            FakeLifter,
            RecordingBuilder::new(),
            NoAssumptions,
            NullOracle,
            MemoryFacts::new(),
            0x1000,
        )
    }

    #[test]
    fn nop_continues_without_events() {
        let mut s = bare_session();
        let out = s.step(&FakeInsn::nop(0x1000));
        assert_eq!(out, StepOutcome::Continue(Vec::new()));
    }

    #[test]
    fn direct_jump_emits_branch_and_block_end() {
        let mut s = bare_session();
        let out = s.step(&FakeInsn::with(0x1000, 5, "jmp 0x2000"));
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x1000,
                EventKind::Branch {
                    kind: BranchKind::Unconditional,
                    from: 0x1000,
                    to: 0x2000,
                    taken: true,
                },
            )])
        );
        assert!(s.builder().log.iter().any(|l| l == "const 0x2000"));
    }

    #[test]
    fn conditional_branch_with_no_facts_suspends_then_resumes() {
        let mut s = bare_session();
        // jcc taken=0x40 fallthrough=0x32 at 0x30
        let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
        let token = match out {
            StepOutcome::Suspend(Query::BranchVerdict { site, taken_target, fallthrough }, tok) => {
                assert_eq!((site, taken_target, fallthrough), (0x30, 0x40, 0x32));
                tok
            }
            other => panic!("expected suspend, got {other:?}"),
        };
        let out = s.resume(token, Answer::BranchVerdict { taken: true });
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x30,
                EventKind::Branch { kind: BranchKind::Conditional, from: 0x30, to: 0x40, taken: true },
            )])
        );
    }

    #[test]
    fn conditional_branch_resolved_by_declared_predicate_does_not_suspend() {
        let mut a = MapAssumptions::default();
        a.preds.insert(0x30, PredicateVerdict::NeverTaken);
        let mut s = Session::new(
            FakeLifter,
            RecordingBuilder::new(),
            a,
            NullOracle,
            MemoryFacts::new(),
            0x1000,
        );
        let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
        // NeverTaken -> falls through to 0x32, no suspend.
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x30,
                EventKind::Branch { kind: BranchKind::Conditional, from: 0x30, to: 0x32, taken: false },
            )])
        );
    }

    #[test]
    fn indirect_branch_resolved_by_oracle_without_suspend() {
        let mut o = ScriptedOracle::default();
        o.indirects.insert(0x50, vec![0x900]);
        let mut s = Session::new(
            FakeLifter,
            RecordingBuilder::new(),
            NoAssumptions,
            o,
            MemoryFacts::new(),
            0x1000,
        );
        let out = s.step(&FakeInsn::with(0x50, 2, "jmp_ind"));
        assert_eq!(
            out,
            StepOutcome::BlockEnd(vec![Event::new(
                0x50,
                EventKind::Branch { kind: BranchKind::Indirect, from: 0x50, to: 0x900, taken: true },
            )])
        );
    }

    #[test]
    #[should_panic(expected = "resume() before step()")]
    fn stepping_while_suspended_panics() {
        let mut s = bare_session();
        let _ = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32")); // suspends
        let _ = s.step(&FakeInsn::nop(0x40)); // misuse
    }
}
```

- [ ] **Step 2: Wire the module**

In `crates/lift-core/src/lib.rs`, add:

```rust
pub mod session;

pub use session::Session;
```

- [ ] **Step 3: Run the tests to verify they pass**

Run: `cargo test -p lift-core session`
Expected: PASS — all six session tests, including the `#[should_panic]` case. If the `finish_indirect` `>1` arm fails to compile with an associated-type error, confirm the `cases` binding reads exactly `Vec<(Va, <L::Builder as IrBuilder>::Block)>` (note the leading `<`).

- [ ] **Step 4: Commit**

```bash
git add crates/lift-core/src/session.rs crates/lift-core/src/lib.rs
git commit -m "feat(lift-core): add streaming Session resolve-or-suspend engine

<trailer>"
```

---

### Task 13: Crate integration test — the assumption-free guarantee — plus docs

**Files:**
- Create: `crates/lift-core/tests/assumption_free.rs`
- Modify: `crates/lift-core/src/lib.rs` (final doc pass; ensure all re-exports present)

**Interfaces:**
- Consumes: the full public surface of `lift-core`.
- Produces: an integration test pinning the core invariant, and a documented crate root.

- [ ] **Step 1: Write the failing integration test**

Create `crates/lift-core/tests/assumption_free.rs`:

```rust
//! Pins the core invariant: with no declared facts and a null oracle, the engine
//! must SUSPEND at an unresolved branch rather than silently assuming; once a fact
//! is declared, the same input resolves without suspending.

use lift_core::{
    Answer, FakeInsn, FakeLifter, MapAssumptions, MemoryFacts, NoAssumptions, NullOracle,
    PredicateVerdict, Query, Session, StepOutcome,
};

fn make_session_no_facts() -> Session<FakeLifter, NoAssumptions, NullOracle> {
    Session::new(
        FakeLifter,
        lift_core::RecordingBuilder::new(),
        NoAssumptions,
        NullOracle,
        MemoryFacts::new(),
        0x1000,
    )
}

#[test]
fn no_facts_forces_suspension_not_assumption() {
    let mut s = make_session_no_facts();
    let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
    match out {
        StepOutcome::Suspend(Query::BranchVerdict { site, .. }, _) => assert_eq!(site, 0x30),
        other => panic!("expected suspension with no facts, got {other:?}"),
    }
}

#[test]
fn declaring_a_predicate_fact_resolves_without_suspension() {
    let mut a = MapAssumptions::default();
    a.preds.insert(0x30, PredicateVerdict::AlwaysTaken);
    let mut s = Session::new(
        FakeLifter,
        lift_core::RecordingBuilder::new(),
        a,
        NullOracle,
        MemoryFacts::new(),
        0x1000,
    );
    let out = s.step(&FakeInsn::with(0x30, 2, "jcc 0x40 0x32"));
    match out {
        StepOutcome::BlockEnd(events) => {
            assert_eq!(events.len(), 1);
        }
        StepOutcome::Suspend(..) => panic!("declared fact should have prevented suspension"),
        other => panic!("unexpected outcome {other:?}"),
    }
}

#[test]
fn indirect_with_no_facts_suspends_then_resume_targets() {
    let mut s = make_session_no_facts();
    let out = s.step(&FakeInsn::with(0x50, 2, "jmp_ind"));
    let token = match out {
        StepOutcome::Suspend(Query::IndirectTargets { site }, tok) => {
            assert_eq!(site, 0x50);
            tok
        }
        other => panic!("expected indirect suspension, got {other:?}"),
    };
    let out = s.resume(token, Answer::Targets(vec![0x900, 0xa00]));
    assert!(matches!(out, StepOutcome::BlockEnd(_)));
}
```

- [ ] **Step 2: Run the integration test to verify it passes**

Run: `cargo test -p lift-core --test assumption_free`
Expected: PASS — all three tests. (If a re-export named in the `use` is missing, add it to `lib.rs`; every symbol used here was defined in Tasks 2–12.)

- [ ] **Step 3: Final doc pass on the crate root**

Confirm `crates/lift-core/src/lib.rs` declares all modules and re-exports. It should read (module order may vary):

```rust
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
pub use ir::{IrBuilder, RecordingBuilder};
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
```

- [ ] **Step 4: Run the full crate test suite and clippy**

Run: `cargo test -p lift-core`
Expected: PASS — every unit test across all modules plus the integration test.

Run: `cargo clippy -p lift-core -- -D warnings`
Expected: no warnings. (Fix any `clippy::len_without_is_empty` on `InsnView` by adding `#[allow(clippy::len_without_is_empty)]` above the `len` method — `InsnView::len` is an instruction byte length, not a collection length.)

- [ ] **Step 5: Commit**

```bash
git add crates/lift-core/tests/assumption_free.rs crates/lift-core/src/lib.rs
git commit -m "test(lift-core): pin the assumption-free guarantee; finalize docs

<trailer>"
```

---

## Self-Review

**1. Spec coverage (Milestone 1 = "Workspace + lift-core seams"):**

| Spec item | Task |
|---|---|
| Cargo workspace, legacy kept as member | Task 1 |
| `Event`/`EventKind` taxonomy | Task 5 |
| Streaming engine `step`/`resume` + `StepOutcome`/`Query`/`Answer` | Tasks 7, 12 |
| `Oracle` seam (resolve_branch/indirect/read_memory/snapshot/restore) | Task 9 |
| `IrBuilder` seam (associated Value/Type; llvmkit-style) | Task 8 |
| `MemoryModel`/`MemoryFacts` with caller-declared attrs incl. `IgnoreWrites` | Task 3 |
| `AssumptionProvider` (known values, predicate verdicts, stack base, targets) | Task 4 |
| `SignatureProvider` + `FnSig`/`AbiKind`/`CallEffects` | Task 6 |
| Arch-neutral `InsnView`/`OperandView` | Task 10 |
| Assumption-free invariant (resolve → oracle → suspend; tested) | Tasks 12, 13 |
| `Platform` trait | **Deferred to Milestone 5's plan** (not exercised in M1; YAGNI). Noted here so it isn't lost. |

**2. Placeholder scan:** no `TBD`/`TODO`/"add error handling"/"similar to Task N" — every step contains complete code. The only intentional non-final code is Task 12 Step 1's malformed type annotation, which Step 2 verifies fails and Step 3 fixes (a deliberate red→green cycle), and is called out explicitly.

**3. Type consistency:** cross-checked — `Va` (u64) is used uniformly; `Session::new(lifter, builder, assume, oracle, facts, entry)` matches every call site in tests (Tasks 12, 13); `Transfer` variants produced by `FakeLifter` (Task 11) are exactly those matched by `Session::step` (Task 12); `Query`/`Answer` variants align with `Query::accepts` (Task 7) and with `resume`'s match arms (Task 12); re-export list in Task 13 covers every symbol used by the integration test.

## Scope note for later milestones

Each of these becomes its own plan, written just-in-time after reading the real dependency APIs:
- **M2 `lift-llvmkit`** — implement `IrBuilder` (+ extend it with data-flow ops) over the actual llvmkit API; `.ll` export. *Requires reading llvmkit's real builder signatures first.*
- **M3 `lift-x86`** — `RegisterFile`, dispatch, operands, effective address; port arithmetic/logical/data-transfer/shift/rotate/stack from the legacy crate as a correctness oracle; zydis `InsnView` adapter. *Requires reading zydis 4.x and the legacy handlers.*
- **M4** — control-flow + GEPTracker-style memory aliasing (`solve_load`).
- **M5** — calls/ABI/`Platform`/Windows-x64 + starter ntdll DB; this is where the `Platform` trait lands.
- **M6** — inkwell backend + Mergen-style optimization/deobfuscation pipeline via the `.ll` bridge.
- **M7** — `lift-ffi` C-ABI + facade + end-to-end oracle example.
