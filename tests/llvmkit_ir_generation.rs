use bin_lift::compiler;
use llvmkit::ir::Module;
use zydis::{Decoder, MachineMode};

const ADD_BYTES: [u8; 2] = [0x01, 0xD8];
const MOV_AL_IMM_BYTES: [u8; 2] = [0xB0, 0x01];
const CMP_PUSHFQ_BYTES: [u8; 3] = [0x39, 0xD8, 0x9C];
const OLD_DISPATCH_BYTES: [u8; 10] = [
    0x11, 0xD8, // adc eax, ebx
    0x0F, 0x94, 0xC0, // setz al
    0x0F, 0x44, 0xC3, // cmove eax, ebx
    0xD1, 0xE0, // shl eax, 1
];
const MOV_BL_AH_BYTES: [u8; 2] = [0x88, 0xE3];
const RDTSC_BYTES: [u8; 2] = [0x0F, 0x31];

fn lift_bytes_with_mode(
    bytes: &[u8],
    decoder: Decoder,
    mode: MachineMode,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut instructions = Vec::new();
    for item in decoder.decode_all(bytes, 0) {
        let (_ip, _raw, instruction) = item?;
        instructions.push(instruction);
    }

    Ok(Module::with_new("protected", |module| {
        compiler::lift_to_ir_text(module, &instructions, mode, None)
    })?)
}

fn lift_bytes(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    lift_bytes_with_mode(bytes, Decoder::new64(), MachineMode::LONG_64)
}

#[test]
fn lifts_add_to_verified_llvmkit_ir() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&ADD_BYTES)?;

    assert!(ir.contains("define"), "{ir}");
    assert!(ir.contains("@protected"), "{ir}");
    assert!(ir.contains(" add ") || ir.contains("add i"), "{ir}");
    assert!(ir.contains("ret"), "{ir}");
    Ok(())
}

#[test]
fn stack_memory_allocates_4096_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&ADD_BYTES)?;

    assert!(ir.contains("alloca i8, i128 4096"), "{ir}");
    assert!(!ir.contains("alloca i128, i128 4096"), "{ir}");
    Ok(())
}

#[test]
fn long_compat_32_signature_omits_64_bit_only_registers() -> Result<(), Box<dyn std::error::Error>>
{
    let ir = lift_bytes_with_mode(&ADD_BYTES, Decoder::new32(), MachineMode::LONG_COMPAT_32)?;

    assert!(ir.contains("define i32 @protected("), "{ir}");
    assert!(ir.contains("i32 %eip"), "{ir}");
    assert!(!ir.contains("%r8d"), "{ir}");
    assert!(!ir.contains("%r15d"), "{ir}");
    Ok(())
}

#[test]
fn preserves_upper_bits_on_byte_register_write() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&MOV_AL_IMM_BYTES)?;

    assert!(ir.contains("and i64 %rax"), "{ir}");
    assert!(ir.contains("or i64"), "{ir}");
    assert!(ir.contains("ret i64 %newreg_"), "{ir}");
    Ok(())
}

#[test]
fn pushfq_uses_composed_rflags_value() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&CMP_PUSHFQ_BYTES)?;

    assert!(ir.contains("computed_zf"), "{ir}");
    assert!(ir.contains("computed_sf"), "{ir}");
    assert!(ir.contains("cmp_cf"), "{ir}");
    assert!(ir.contains("shl i64"), "{ir}");
    assert!(ir.contains("or i64"), "{ir}");
    assert!(!ir.contains("store i64 0, ptr"), "{ir}");
    Ok(())
}

#[test]
fn old_dispatch_families_still_lift() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&OLD_DISPATCH_BYTES)?;

    assert!(ir.contains("adc"), "{ir}");
    assert!(ir.contains("setcc"), "{ir}");
    assert!(ir.contains("cmov"), "{ir}");
    assert!(ir.contains("shl"), "{ir}");
    assert!(ir.contains("ret"), "{ir}");
    Ok(())
}

#[test]
fn reads_high_byte_register_bits() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&MOV_BL_AH_BYTES)?;

    assert!(ir.contains("high_byte"), "{ir}");
    assert!(ir.contains("lshr i64 %rax, 8"), "{ir}");
    Ok(())
}

#[test]
fn rdtsc_uses_readcyclecounter_intrinsic() -> Result<(), Box<dyn std::error::Error>> {
    let ir = lift_bytes(&RDTSC_BYTES)?;

    assert!(ir.contains("ret"), "{ir}");
    assert!(ir.contains("rdtsc_val"), "{ir}");
    assert!(!ir.contains("asm sideeffect"), "{ir}");
    assert!(ir.contains("llvm.readcyclecounter"), "{ir}");
    Ok(())
}
