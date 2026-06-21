use llvmkit::ir::Module;
use std::{error::Error, time::Instant};
use zydis::Decoder;

/// Lifts a simple x86-64 function to verified textual LLVM IR.
///
/// It lifts the following source-level shape:
/// ```cpp
/// __int64 __fastcall add(int a, int b, int c)
/// {
///   j___CheckForDebuggerJustMyCode(&_5923EECC_simple_target_cpp);
///   return (c + b + a);
/// }
/// ```
///
/// The call is kept in the instruction stream and modeled by the current call
/// lifter as a return-address stack update; this example prints the protected
/// function IR, not a complete executable `main`.
fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mode = zydis::MachineMode::LONG_64;
    let decoder = Decoder::new64();

    let mut all_instructions = Vec::new();
    for instruction_info in decoder.decode_all(&TEST_ADDITION_NOT_PATCHED_64, 0) {
        let (_ip, _raw_bytes, instruction) = instruction_info?;
        all_instructions.push(instruction);
    }

    let ir = Module::with_new("protected", |module| {
        zydis2llvmir::compiler::lift_to_ir_text(module, &all_instructions, mode, None)
    })?;
    println!("{ir}");
    println!("Elapsed: {:?}", start_time.elapsed());

    Ok(())
}

/// Original x86-64 input:
/// ```assembly
/// mov [rsp+0x18], r8d
/// mov [rsp+0x10], edx
/// mov [rsp+0x08], ecx
/// push rbp
/// push rdi
/// sub rsp, 0xE8
/// lea rbp, [rsp+0x20]
/// lea rcx, [0x000000000000F85D]
/// call 0xFFFFFFFFFFFFFBAF
/// mov eax, [rbp+0xE8]
/// mov ecx, [rbp+0xE0]
/// add ecx, eax
/// mov eax, ecx
/// add eax, [rbp+0xF0]
/// lea rsp, [rbp+0xC8]
/// pop rdi
/// pop rbp
/// ret
/// ```
const TEST_ADDITION_NOT_PATCHED_64: [u8; 71] = [
    0x44, 0x89, 0x44, 0x24, 0x18, 0x89, 0x54, 0x24, 0x10, 0x89, 0x4C, 0x24, 0x08, 0x55, 0x57, 0x48,
    0x81, 0xEC, 0xE8, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x6C, 0x24, 0x20, 0x48, 0x8D, 0x0D, 0x56, 0xF8,
    0x00, 0x00, 0xE8, 0xAA, 0xFB, 0xFF, 0xFF, 0x8B, 0x85, 0xE8, 0x00, 0x00, 0x00, 0x8B, 0x8D, 0xE0,
    0x00, 0x00, 0x00, 0x03, 0xC8, 0x8B, 0xC1, 0x03, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x48, 0x8D, 0xA5,
    0xC8, 0x00, 0x00, 0x00, 0x5F, 0x5D, 0xC3,
];
