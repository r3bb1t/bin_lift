use inkwell::context::Context;
use std::error::Error;
use zydis::{FullInstruction, Mnemonic};

use zydis2llvmir::compiler::Compiler;
use zydis2llvmir::test_sequences;


fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("new_protected");

    let mode = zydis::MachineMode::LONG_64;
    let instructions =
        test_sequences::decode_sample_bytes(&TEST_ADDITION_NOT_PATCHED_64, &mode)?;

    let instructions: Vec<FullInstruction> = instructions
        .into_iter()
        // .filter(|ins| ins.mnemonic == Mnemonic::PUSH)
        .filter(|ins| ins.mnemonic != Mnemonic::RET)
        .filter(|ins| ins.mnemonic != Mnemonic::CALL)
        .collect();
    // instructions.retain(|ins| ins.mnemonic != Mnemonic::LEA && ins.mnemonic != Mnemonic::CALL);
    // dbg!(&instructions);

    // let fmt = zydis::Formatter::intel();
    // for ins in &instructions {
    //     println!("{}", fmt.format(None, ins).unwrap());
    //     // println!("{:?}", ins.meta.category)
    // }
    // todo!();

    let compiled = Compiler::compile(&context, &builder, &module, instructions, &mode)?;

    // Append ret void at the end of the basic block
    builder.build_return(None)?;

    compiled.print_to_stderr();

    Ok(())
}

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
pub const TEST_ADDITION_NOT_PATCHED_64: [u8; 71] = [
    0x44, 0x89, 0x44, 0x24, 0x18, 0x89, 0x54, 0x24, 0x10, 0x89, 0x4C, 0x24, 0x08, 0x55, 0x57, 0x48,
    0x81, 0xEC, 0xE8, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x6C, 0x24, 0x20, 0x48, 0x8D, 0x0D, 0x56, 0xF8,
    0x00, 0x00, 0xE8, 0xAA, 0xFB, 0xFF, 0xFF, 0x8B, 0x85, 0xE8, 0x00, 0x00, 0x00, 0x8B, 0x8D, 0xE0,
    0x00, 0x00, 0x00, 0x03, 0xC8, 0x8B, 0xC1, 0x03, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x48, 0x8D, 0xA5,
    0xC8, 0x00, 0x00, 0x00, 0x5F, 0x5D, 0xC3,
];
