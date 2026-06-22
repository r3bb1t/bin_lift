# Repository Guidelines

## Project Overview

`bin_lift` is a Rust library for lifting raw binary data / Zydis-decoded x86 instruction streams into verified textual LLVM IR. It is inspired by RetDec/capstone2llvmir, remill, and Mergen. The public crate name is `zydis2llvmir`; core IR generation uses Rust, Zydis, and the sibling `llvmkit` path dependency.

LLVM command-line tools are optional for post-processing emitted `.ll`; they are not required for normal lifting or module verification.

## Architecture & Data Flow

Typical flow:

1. Decode bytes with `zydis::Decoder` into `FullInstruction`s.
2. Create an llvmkit module with `Module::with_new("protected", |module| ...)`.
3. Call `zydis2llvmir::compiler::lift_to_ir_text(module, &instructions, mode, runtime_address)`.
4. `compiler::create_func` builds the external `@protected` function signature.
5. `LifterX86::new` seeds register/flag state from function params, creates one `entry` block, and allocates local `stackmemory`.
6. `src/lifter/semantics/x86/mod.rs` dispatches each mnemonic to an instruction-family lifter.
7. `RET` terminates the function when `runtime_address` is `None`; otherwise it updates SP and continues.

Architecture notes:

- `src/compiler/mod.rs` owns orchestration, `@protected` ABI construction, module verification, and final IR formatting.
- `src/lifter/mod.rs` owns lifter state: module, positioned builder, mode, register/flag map, stack memory, and optional runtime address.
- `src/lifter/mergen_getters_and_setters.rs` is the Mergen-reference-style memory path: effective address is `base + index * scale + displacement`, then a GEP into local `stackmemory`.
- Memory is modeled as local stack-like storage, not host process memory. `GS` segment addressing is explicitly unsupported.
- `Register::IP` becomes a concrete runtime address only when `runtime_address` is set; otherwise it can be LLVM `undef`.
- Keep `zydis::Decoder` and `zydis::MachineMode` matched. Do not decode x86-64 bytes with `Decoder::new32()` and then interpret the IR as the original program.

## Key Directories

- `src/compiler/` — public compiler facade, `lift_to_ir_text`, `Compiler`, function ABI construction, compiler errors, context helpers.
- `src/lifter/` — x86 lifter state, operand getters/setters, Mergen-style memory helpers, flag formulas, and errors.
- `src/lifter/semantics/x86/` — instruction-family implementations (`binary`, `logical`, `dataxfer`, `push`, `pop`, `call`, `ret`, `shift`, `rotate`, `cmov`, `setcc`, etc.).
- `tests/` — integration tests for llvmkit IR generation and migration guards.
- `examples/` — manual IR-printing examples and trace-lifting inputs.
- `comparisons/` — old/original vs llvmkit IR comparison artifacts; use for inspection, not as live test fixtures.
- `docs/superpowers/plans/` — historical implementation plans; useful context, not guaranteed current truth.

## Development Commands

Run from repository root:

```bash
cargo build
cargo test
cargo test --test llvmkit_ir_generation
cargo test --test no_inkwell_dependency
cargo run --example simple_add
cargo run --example new_lift_add
cargo run --example lift_vmp_trace
cargo fmt
```

Caveats:

- `cargo run --example lift_vmp_trace` writes `lifted.ll` in the repo root.
- Examples print protected-function IR; they do not produce a complete executable `main` wrapper.
- To compile emitted IR as a program, add a matching caller/entry point or run an explicit external backend step.

## Code Conventions & Common Patterns

- Use local `Result<T>` aliases and propagate errors with `?`.
- Use `Error::UnsupportedInstr(&'static str)` for unsupported forms instead of silently accepting bad semantics.
- Route operand reads through `load_single_op` / `load_single_int_op`; route writes through `store_op`.
- Use `create_z_ext_or_trunc` for normal integer resizing; use explicit sign extension only for instructions that require it (`MOVSX`, `MOVSXD`, conversion helpers).
- Use `self.builder()?` for IR emission. After `take_builder()`, further builder access returns `FunctionAlreadyTerminated`.
- Keep function-signature changes in sync between `src/compiler/mod.rs::create_func` and `src/lifter/mod.rs::prep_regs_hashmap_experimental`.
- Register model: BP/SP families are canonicalized by mode; high-byte registers (`AH`, `CH`, `DH`, `BH`) are handled by shift/trunc/merge logic.
- Flags are bool-like integer values in the register map. `FLAGS`/`RFLAGS` packing and unpacking is centralized in getters/setters.
- IR names are semantic hints (`add_result`, `computed_pf`, `push_sp`, `effective_address_`); llvmkit handles duplicate suffixes.
- `src/lifter/definintions.rs` is misspelled but is the actual module name; do not rename casually.

## Important Files

- `Cargo.toml` — package `bin_lift`, lib crate `zydis2llvmir`, Rust 2021, dependencies.
- `.cargo/config.toml` — sets `CMAKE_GENERATOR = "Ninja Multi-Config"` for Cargo builds.
- `src/lib.rs` — public module exports.
- `src/compiler/mod.rs` — main API and protected-function ABI.
- `src/lifter/mod.rs` — lifter construction and register/flag param mapping.
- `src/lifter/mergen_getters_and_setters.rs` — central memory addressing model.
- `src/lifter/flagops.rs` — shared flag computations.
- `src/lifter/semantics/x86/mod.rs` — mnemonic dispatcher.
- `tests/llvmkit_ir_generation.rs` — primary IR shape regression suite.
- `tests/no_inkwell_dependency.rs` — guard against reintroducing Inkwell in `Cargo.toml`.
- `examples/simple_add.rs`, `examples/new_lift_add.rs`, `examples/lift_vmp_trace.rs` — manual usage examples.
- `comparisons/*.ll` — comparison artifacts; pair original/llvmkit and optimized/unoptimized like-for-like.

## Runtime/Tooling Preferences

- Required runtime: Rust/Cargo, edition 2021.
- Native dependency builds may require CMake/Ninja-compatible tooling because Zydis uses native build dependencies; `.cargo/config.toml` selects Ninja Multi-Config.
- The sibling path dependency `../rllvm/llvmkit` must exist.
- No `Makefile`, `Justfile`, `rust-toolchain`, `rustfmt.toml`, or `clippy.toml` is present; use standard Cargo tooling.
- `Cargo.lock` is tracked. Keep it updated when dependency resolution changes.
- `.gitignore` is minimal (`/target`, `/ignored`); avoid leaving generated files such as `lifted.ll` unless intentionally tracked.

## Testing & QA

Test framework: built-in Rust `#[test]` only. No custom test harness or snapshot framework is configured.

Primary checks:

```bash
cargo test
cargo test --test llvmkit_ir_generation
cargo test --test no_inkwell_dependency
cargo test <test_name>
```

Testing patterns:

- Tests decode inline byte fixtures with Zydis, lift via `compiler::lift_to_ir_text`, and assert IR substrings.
- Existing tests return `Result<(), Box<dyn std::error::Error>>` so decoder/compiler failures propagate cleanly.
- Assertions print `{ir}` on failure for debugging.
- Current tests verify text shape, not execution, external assembly, optimization, or native backend behavior.
- Add focused regression tests in `tests/llvmkit_ir_generation.rs` for lifter/IR shape changes.

Comparison-file caveats:

- `comparisons/original_*.ll` are original/Inkwell-style artifacts; `comparisons/llvmkit_*.ll` are llvmkit artifacts.
- Appended `@main` wrappers in comparison files are stale/mismatched; compare `@protected` directly unless wrappers are regenerated.
- LLVM `range(i64 0, 4294967296)` is half-open `[0, 2^32)`, not an off-by-one by itself.
