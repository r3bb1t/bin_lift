use inkwell::context::Context;
use std::error::Error;
use zydis::Decoder;
use zydis2llvmir::lifter::LifterX86;


/// This is an example of lifting some simple function to LLVM IR
/// It simply lifts the following code
/// ```cpp
/// __int64 __fastcall add(int a, int b, int c)
/// {
///   j___CheckForDebuggerJustMyCode(&_5923EECC_simple_target_cpp);
///   return (c + b + a);
/// }
/// ```
///
/// But the call to "j___CheckForDebuggerJustMyCode" is skipped for now
///
/// Right now you have to manually create a @main" function which invokes @protected to be able to
/// compile the outputted llvm IR.
/// Please, Use llvm 18
fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    //let builder = context.create_builder();
    //let module = context.create_module("new_protected");

    let mode = zydis::MachineMode::LONG_64;
    let decoder = Decoder::new64();

    let mut all_instructions: Vec<zydis::FullInstruction> = vec![];
    for instruction_info in decoder.decode_all(&TEST_ADDITION_NOT_PATCHED_64, 0) {
        let (_ip, _raw_bytes, instruction) = instruction_info?;
        all_instructions.push(instruction);
    }

    let lifter = LifterX86::new(&context, mode);
    lifter.lift_basic_block(&all_instructions)?;

    // Append ret void at the end of the basic block
    lifter.builder.build_return(None)?;

    lifter.module.print_to_stderr();

    lifter
        .module
        .print_to_file("a.ll")
        .expect("unable to print to file");

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
const TEST_ADDITION_NOT_PATCHED_64: [u8; 71] = [
    0x44, 0x89, 0x44, 0x24, 0x18, 0x89, 0x54, 0x24, 0x10, 0x89, 0x4C, 0x24, 0x08, 0x55, 0x57, 0x48,
    0x81, 0xEC, 0xE8, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x6C, 0x24, 0x20, 0x48, 0x8D, 0x0D, 0x56, 0xF8,
    0x00, 0x00, 0xE8, 0xAA, 0xFB, 0xFF, 0xFF, 0x8B, 0x85, 0xE8, 0x00, 0x00, 0x00, 0x8B, 0x8D, 0xE0,
    0x00, 0x00, 0x00, 0x03, 0xC8, 0x8B, 0xC1, 0x03, 0x85, 0xF0, 0x00, 0x00, 0x00, 0x48, 0x8D, 0xA5,
    0xC8, 0x00, 0x00, 0x00, 0x5F, 0x5D, 0xC3,
];
