// TODO: Delete it all and move to examples maybe

use zydis::{AllOperands, Decoder, FullInstruction, MachineMode};

/// ```assembly
/// ADD RAX, RBX;
/// ADD RCX, RAX;
/// ADD RDX, RCX;
/// ADD RBX, RDX;
/// ```
#[allow(unused)]
pub(super) const ADD_FOUR_TIMES_64_BIT_VALUES: [u8; 12] = [
    0x48, 0x01, 0xD8, 0x48, 0x01, 0xC1, 0x48, 0x01, 0xCA, 0x48, 0x01, 0xD3,
];
/// ```assembly
/// ADD EAX, EBX
/// ADD EAX, EAX
/// ADD EDX, ECX
/// ADD EAX, EDX
/// ADD EBX, EBX
/// ```
#[allow(unused)]
pub(super) const ADD_5_TIMES_32_BIT_REGS: [u8; 10] =
    [0x01, 0xD8, 0x01, 0xC0, 0x01, 0xCA, 0x01, 0xD0, 0x01, 0xDB];

pub fn decode_sample_bytes(
    raw_bytes: &[u8],
    mode: &MachineMode,
) -> zydis::Result<Vec<FullInstruction>> {
    let mut all_instructions = vec![];

    let dec = match mode {
        MachineMode::LONG_64 => Decoder::new64(),
        MachineMode::LONG_COMPAT_32 => Decoder::new32(),
        _ => unimplemented!("Unimplemented machine mode"),
    };

    for instruction_info in dec.decode_all::<AllOperands>(raw_bytes, 0) {
        let (_ip, _raw_bytes, instruction) = instruction_info?;
        all_instructions.push(instruction);
    }

    Ok(all_instructions)
}
