// use crate::ToStore;
use inkwell::context::Context;
use inkwell::types::IntType;
use zydis::{MachineMode, Register};

pub fn get_int_type<'ctx>(
    context: &'ctx Context,
    register: &Register,
    mode: &MachineMode,
) -> IntType<'ctx> {
    match register.width(*mode) {
        64 => context.i64_type(),
        32 => context.i32_type(),
        16 => context.i16_type(),
        8 => context.i8_type(),
        _ => unreachable!(),
    }
}
