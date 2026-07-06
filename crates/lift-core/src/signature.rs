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
