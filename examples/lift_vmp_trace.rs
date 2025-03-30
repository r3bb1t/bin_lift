use inkwell::context::Context;
use std::error::Error;
use std::time::Instant;
use zydis::{AllOperands, Decoder};
use zydis2llvmir::compiler::Compiler;

/// THis is an example of lifting some simple function to LLVM IR
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

    let decoder = Decoder::new64();
    // 17 883 instructions
    let raw_bytes = std::include_bytes!("files/raw_instr_trace.bin");
    //let raw_bytes = std::include_bytes!("another_trace_35.bin");
    // This one is the wrong trace, but

    let now = Instant::now();
    let mut instructions = Vec::with_capacity(17_833);
    let mut last_is_rep = false;
    for instruction_info in decoder.decode_all::<AllOperands>(raw_bytes, 0) {
        let (_ip, _raw_bytes, instruction) = instruction_info?;

        let curr_ins_attributes = instruction
            .attributes
            .contains(zydis::InstructionAttributes::HAS_REP);
        if !last_is_rep {
            instructions.push(instruction);
        }
        last_is_rep = curr_ins_attributes;
    }

    let mode = zydis::MachineMode::LONG_64;
    let instrs_count = instructions.len();

    //let lifter = LifterX86::new(&context, mode);
    const START_ADDRESS: u64 = 0x1400118d9;
    let compiler = Compiler::new_with_x86_lifter(&context, mode, START_ADDRESS)?;
    //let lifter = LifterX86::new(&context, mode)?;
    compiler.lift_function(&instructions)?;
    let elapsed = now.elapsed();

    println!("Lifted vec with {instrs_count} instructions. Took {elapsed:?}");

    let now = Instant::now();
    compiler.lifter.module.print_to_file("lifted.ll")?;
    let elapsed = now.elapsed();
    println!("Took {elapsed:?} to dump output to file");

    Ok(())
}
