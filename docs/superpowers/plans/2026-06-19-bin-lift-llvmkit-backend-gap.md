# bin_lift llvmkit Backend Gap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Preserve bin_lift's user-facing ability to get optimized/native output after migrating IR construction to llvmkit, without pretending llvmkit is a target backend.

**Architecture:** Keep llvmkit as the IR construction and verification layer. Add an explicit optional post-processing boundary that consumes verified `.ll` text and either returns it unchanged or invokes an external LLVM toolchain configured by the caller. This keeps the core lifter free of `llvm-sys`/Inkwell while making backend dependence visible.

**Tech Stack:** Rust 2021, llvmkit textual IR output, optional external `opt`/`llc`/`clang` commands through `std::process::Command`, temporary files, and explicit backend configuration structs.

---

## File Structure

- Create: `src/backend.rs` — backend mode/config and external-command post-processing.
- Modify: `src/lib.rs` — export backend module.
- Modify: `src/compiler/mod.rs` — expose `lift_to_ir_text` and optional `lift_with_backend` wrapper.
- Modify: `examples/simple_add.rs` — show IR-only output by default.
- Create: `examples/optimized_ir.rs` — show explicit external LLVM optimization when configured.
- Modify: `Readme.md` — document the split between llvmkit IR generation and external backend processing.
- Test: `tests/backend_config.rs` — pure config tests that do not require LLVM tools installed.

---

### Task 1: Add backend config types without running external tools

**Files:**
- Create: `src/backend.rs`
- Modify: `src/lib.rs`
- Test: `tests/backend_config.rs`

- [ ] **Step 1: Create `src/backend.rs`**

```rust
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendOutputKind {
    IrText,
    OptimizedIrText,
    ObjectFile,
    AssemblyText,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalLlvmTools {
    pub opt: PathBuf,
    pub llc: PathBuf,
    pub clang: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendConfig {
    IrOnly,
    ExternalLlvm {
        tools: ExternalLlvmTools,
        opt_pipeline: String,
        output: BackendOutputKind,
        extra_args: Vec<OsString>,
    },
}

impl BackendConfig {
    pub fn ir_only() -> Self {
        Self::IrOnly
    }

    pub fn external_o2_ir(tools: ExternalLlvmTools) -> Self {
        Self::ExternalLlvm {
            tools,
            opt_pipeline: "default<O2>".to_owned(),
            output: BackendOutputKind::OptimizedIrText,
            extra_args: Vec::new(),
        }
    }
}
```

- [ ] **Step 2: Export backend module**

In `src/lib.rs`:

```rust
pub mod backend;
pub mod compiler;
pub mod lifter;
pub mod miscellaneous;
pub mod util;
```

Preserve any existing module exports in the file.

- [ ] **Step 3: Add config tests**

Create `tests/backend_config.rs`:

```rust
use std::path::PathBuf;

use zydis2llvmir::backend::{BackendConfig, BackendOutputKind, ExternalLlvmTools};

#[test]
fn ir_only_config_is_default_backend_boundary() {
    assert_eq!(BackendConfig::ir_only(), BackendConfig::IrOnly);
}

#[test]
fn external_o2_ir_config_names_pipeline_explicitly() {
    let tools = ExternalLlvmTools {
        opt: PathBuf::from("opt"),
        llc: PathBuf::from("llc"),
        clang: PathBuf::from("clang"),
    };

    let config = BackendConfig::external_o2_ir(tools);

    match config {
        BackendConfig::ExternalLlvm {
            opt_pipeline,
            output,
            extra_args,
            ..
        } => {
            assert_eq!(opt_pipeline, "default<O2>");
            assert_eq!(output, BackendOutputKind::OptimizedIrText);
            assert!(extra_args.is_empty());
        }
        BackendConfig::IrOnly => panic!("expected external backend config"),
    }
}
```

- [ ] **Step 4: Run pure backend tests**

```bash
cargo test --test backend_config
```

Expected: both tests pass without requiring LLVM binaries.

---

### Task 2: Add explicit backend result and error types

**Files:**
- Modify: `src/backend.rs`
- Modify: `src/compiler/error.rs`

- [ ] **Step 1: Add result enum**

Append to `src/backend.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendOutput {
    IrText(String),
    OptimizedIrText(String),
    ObjectFile(Vec<u8>),
    AssemblyText(String),
}
```

- [ ] **Step 2: Add backend error variant**

In `src/compiler/error.rs`, add:

```rust
#[error("backend command `{program}` failed: {stderr}")]
BackendCommandFailed { program: String, stderr: String },

#[error("backend command `{program}` could not be started: {reason}")]
BackendCommandSpawnFailed { program: String, reason: String },
```

- [ ] **Step 3: Check error formatting**

Add to `tests/backend_config.rs`:

```rust
use zydis2llvmir::compiler::error::Error;

#[test]
fn backend_error_names_failing_program() {
    let error = Error::BackendCommandFailed {
        program: "opt".to_owned(),
        stderr: "bad input".to_owned(),
    };

    assert_eq!(
        error.to_string(),
        "backend command `opt` failed: bad input"
    );
}
```

- [ ] **Step 4: Run backend tests**

```bash
cargo test --test backend_config
```

Expected: all backend config/error tests pass.

---

### Task 3: Add IR-only backend processing

**Files:**
- Modify: `src/backend.rs`
- Test: `tests/backend_config.rs`

- [ ] **Step 1: Add processing function**

Append to `src/backend.rs`:

```rust
use crate::compiler::error::{Error, Result};

pub fn process_verified_ir(ir: String, config: &BackendConfig) -> Result<BackendOutput> {
    match config {
        BackendConfig::IrOnly => Ok(BackendOutput::IrText(ir)),
        BackendConfig::ExternalLlvm { .. } => run_external_backend(ir, config),
    }
}

fn run_external_backend(ir: String, config: &BackendConfig) -> Result<BackendOutput> {
    let BackendConfig::ExternalLlvm { output, .. } = config else {
        return Ok(BackendOutput::IrText(ir));
    };

    match output {
        BackendOutputKind::OptimizedIrText => Ok(BackendOutput::OptimizedIrText(ir)),
        BackendOutputKind::IrText => Ok(BackendOutput::IrText(ir)),
        BackendOutputKind::ObjectFile => Err(Error::OptimizationBackendUnavailable),
        BackendOutputKind::AssemblyText => Err(Error::OptimizationBackendUnavailable),
    }
}
```

This intentionally does not spawn tools yet. It wires the API boundary first.

- [ ] **Step 2: Add IR-only processing test**

Add to `tests/backend_config.rs`:

```rust
use zydis2llvmir::backend::{process_verified_ir, BackendOutput};

#[test]
fn ir_only_backend_returns_input_ir() {
    let output = process_verified_ir("define void @f() { ret void }".to_owned(), &BackendConfig::IrOnly)
        .expect("ir-only backend");

    assert_eq!(
        output,
        BackendOutput::IrText("define void @f() { ret void }".to_owned())
    );
}
```

- [ ] **Step 3: Run backend tests**

```bash
cargo test --test backend_config
```

Expected: backend boundary tests pass.

---

### Task 4: Connect compiler output to backend config

**Files:**
- Modify: `src/compiler/mod.rs`
- Test: `tests/backend_config.rs`

- [ ] **Step 1: Import backend types**

At the top of `src/compiler/mod.rs`:

```rust
use crate::backend::{process_verified_ir, BackendConfig, BackendOutput};
```

- [ ] **Step 2: Add backend wrapper**

In `impl Compiler` after `lift_to_ir_text`:

```rust
pub fn lift_with_backend(
    module: Module<'ctx, B, Unverified>,
    instructions: &[FullInstruction],
    mode: MachineMode,
    runtime_address: Option<u64>,
    backend: &BackendConfig,
) -> Result<BackendOutput> {
    let ir = Self::lift_to_ir_text(module, instructions, mode, runtime_address)?;
    Ok(process_verified_ir(ir, backend)?)
}
```

- [ ] **Step 3: Add compiler/backend API test without decoding**

If the IR migration test already exists, add this assertion there after generating `ir`:

```rust
let output = zydis2llvmir::backend::process_verified_ir(
    ir.clone(),
    &zydis2llvmir::backend::BackendConfig::ir_only(),
)?;
assert_eq!(output, zydis2llvmir::backend::BackendOutput::IrText(ir));
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test backend_config
cargo test --test llvmkit_ir_generation
```

Expected: both pass.

---

### Task 5: Implement external `opt` for optimized IR text

**Files:**
- Modify: `src/backend.rs`
- Test: no mandatory test requiring LLVM tools.

- [ ] **Step 1: Add command helper imports**

At the top of `src/backend.rs`:

```rust
use std::io::Write;
use std::process::{Command, Stdio};
```

- [ ] **Step 2: Add `run_opt` helper**

```rust
fn run_opt(ir: &str, tools: &ExternalLlvmTools, pipeline: &str, extra_args: &[OsString]) -> Result<String> {
    let mut command = Command::new(&tools.opt);
    command.arg(format!("-passes={pipeline}"));
    command.arg("-S");
    for arg in extra_args {
        command.arg(arg);
    }
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let program = tools.opt.display().to_string();
    let mut child = command.spawn().map_err(|error| Error::BackendCommandSpawnFailed {
        program: program.clone(),
        reason: error.to_string(),
    })?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(ir.as_bytes()).map_err(|error| Error::BackendCommandSpawnFailed {
            program: program.clone(),
            reason: error.to_string(),
        })?;
    }

    let output = child.wait_with_output().map_err(|error| Error::BackendCommandSpawnFailed {
        program: program.clone(),
        reason: error.to_string(),
    })?;

    if !output.status.success() {
        return Err(Error::BackendCommandFailed {
            program,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
```

- [ ] **Step 3: Wire optimized IR path**

Replace `run_external_backend` with:

```rust
fn run_external_backend(ir: String, config: &BackendConfig) -> Result<BackendOutput> {
    let BackendConfig::ExternalLlvm {
        tools,
        opt_pipeline,
        output,
        extra_args,
    } = config else {
        return Ok(BackendOutput::IrText(ir));
    };

    match output {
        BackendOutputKind::IrText => Ok(BackendOutput::IrText(ir)),
        BackendOutputKind::OptimizedIrText => {
            let optimized = run_opt(&ir, tools, opt_pipeline, extra_args)?;
            Ok(BackendOutput::OptimizedIrText(optimized))
        }
        BackendOutputKind::ObjectFile => Err(Error::OptimizationBackendUnavailable),
        BackendOutputKind::AssemblyText => Err(Error::OptimizationBackendUnavailable),
    }
}
```

- [ ] **Step 4: Run tests without requiring LLVM tools**

```bash
cargo test --test backend_config
```

Expected: existing tests pass. Do not add CI tests that require `opt` installed.

---

### Task 6: Add explicit optimized example

**Files:**
- Create: `examples/optimized_ir.rs`
- Modify: `examples/Readme.md` if present.

- [ ] **Step 1: Create example**

Create `examples/optimized_ir.rs`:

```rust
use std::error::Error;
use std::path::PathBuf;

use llvmkit::ir::Module;
use zydis::Decoder;
use zydis2llvmir::backend::{BackendConfig, BackendOutput, ExternalLlvmTools};
use zydis2llvmir::compiler::Compiler;

const ADD_BYTES: [u8; 3] = [0x01, 0xD8, 0xC3];

fn main() -> Result<(), Box<dyn Error>> {
    let decoder = Decoder::new64();
    let mut instructions = Vec::new();
    for item in decoder.decode_all(&ADD_BYTES, 0) {
        let (_ip, _raw, instruction) = item?;
        instructions.push(instruction);
    }

    let tools = ExternalLlvmTools {
        opt: PathBuf::from("opt"),
        llc: PathBuf::from("llc"),
        clang: PathBuf::from("clang"),
    };
    let backend = BackendConfig::external_o2_ir(tools);

    let output = Module::with_new::<_, _, _>("protected", |module| {
        Compiler::lift_with_backend(
            module,
            &instructions,
            zydis::MachineMode::LONG_64,
            None,
            &backend,
        )
    })?;

    match output {
        BackendOutput::IrText(ir) | BackendOutput::OptimizedIrText(ir) | BackendOutput::AssemblyText(ir) => {
            println!("{ir}");
        }
        BackendOutput::ObjectFile(bytes) => {
            println!("object bytes: {}", bytes.len());
        }
    }

    Ok(())
}
```

- [ ] **Step 2: Run example only on machines with `opt`**

Manual command:

```bash
cargo run --example optimized_ir
```

Expected with LLVM tools installed: prints optimized LLVM IR. Expected without tools: returns `backend command` error naming `opt`.

---

### Task 7: Document backend split

**Files:**
- Modify: `Readme.md`
- Modify: `examples/Readme.md`

- [ ] **Step 1: Update root README**

Add this section:

```markdown
# llvmkit migration model

bin_lift uses llvmkit for LLVM IR construction and verification. llvmkit is not a target backend: it does not link libLLVM, run LLVM's `default<O2>` pipeline, emit object files, or JIT code.

Default output is verified textual LLVM IR:

```rust
let ir = Module::with_new::<_, _, _>("protected", |module| {
    Compiler::lift_to_ir_text(module, &instructions, mode, None)
})?;
```

Optimized IR or native output is an explicit external backend step. That keeps the core lifter independent from LLVM FFI while still allowing users with LLVM tools installed to post-process the generated `.ll`.
```

- [ ] **Step 2: Update examples README**

If `examples/Readme.md` exists, add:

```markdown
- `simple_add.rs` prints verified llvmkit IR.
- `optimized_ir.rs` invokes external LLVM `opt` when available. It is not part of the core lifter and may fail if LLVM tools are not on `PATH`.
```

- [ ] **Step 3: Verify docs and tests**

```bash
cargo test --test backend_config
cargo fmt -- --check
```

Expected: tests pass and formatting is clean.

---

## Execution Notes

- Do not reintroduce Inkwell as a hidden dependency in the lifter.
- External LLVM tooling belongs only in `src/backend.rs` and examples that opt into it.
- The default library API should always be able to produce verified textual IR with no LLVM installation.
- Keep object emission behind `BackendOutputKind::ObjectFile`; if implemented, it must be explicit and testable separately from IR generation.
