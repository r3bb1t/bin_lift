use llvmkit::ir::Module;
use std::error::Error;
use std::time::Instant;
use zydis::Decoder;

fn main() -> Result<(), Box<dyn Error>> {
    let decoder = Decoder::new64();
    let raw_bytes = std::include_bytes!("files/newest_trace.bin");

    let now = Instant::now();
    let mut instructions = Vec::with_capacity(17_833);
    let mut last_is_rep = false;
    for instruction_info in decoder.decode_all(raw_bytes, 0) {
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
    const START_ADDRESS: u64 = 0x1400118d9;

    let ir = Module::with_new("protected", |module| {
        zydis2llvmir::compiler::lift_to_ir_text(module, &instructions, mode, Some(START_ADDRESS))
    })?;
    let elapsed = now.elapsed();
    println!("Lifted vec with {instrs_count} instructions. Took {elapsed:?}");

    let now = Instant::now();
    std::fs::write("lifted.ll", ir)?;
    let elapsed = now.elapsed();
    println!("Took {elapsed:?} to dump output to file");

    Ok(())
}
