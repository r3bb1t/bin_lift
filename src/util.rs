use inkwell::context::Context;
use inkwell::types::IntType;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::collections::HashMap;
use zydis::{MachineMode, Register};

/// After the function was created, use this one to store everything in a hashmap
pub fn prep_regs_hashmap<'ctx>(
    fn_val: &FunctionValue<'ctx>,
    mode: &MachineMode,
) -> HashMap<Register, BasicValueEnum<'ctx>> {
    let mut registers_hashmap = HashMap::new();

    registers_hashmap.insert(
        Register::AX.largest_enclosing(*mode),
        fn_val.get_nth_param(0).unwrap(),
    );
    registers_hashmap.insert(
        Register::BX.largest_enclosing(*mode),
        fn_val.get_nth_param(1).unwrap(),
    );
    registers_hashmap.insert(
        Register::CX.largest_enclosing(*mode),
        fn_val.get_nth_param(2).unwrap(),
    );
    registers_hashmap.insert(
        Register::DX.largest_enclosing(*mode),
        fn_val.get_nth_param(3).unwrap(),
    );
    registers_hashmap.insert(
        Register::SI.largest_enclosing(*mode),
        fn_val.get_nth_param(4).unwrap(),
    );
    registers_hashmap.insert(
        Register::DI.largest_enclosing(*mode),
        fn_val.get_nth_param(5).unwrap(),
    );
    registers_hashmap.insert(
        Register::BP.largest_enclosing(*mode),
        fn_val.get_nth_param(6).unwrap(),
    );
    registers_hashmap.insert(
        Register::SP.largest_enclosing(*mode),
        fn_val.get_nth_param(7).unwrap(),
    );
    registers_hashmap.insert(
        Register::R8B.largest_enclosing(*mode),
        fn_val.get_nth_param(8).unwrap(),
    );
    registers_hashmap.insert(
        Register::R9B.largest_enclosing(*mode),
        fn_val.get_nth_param(9).unwrap(),
    );
    registers_hashmap.insert(
        Register::R10B.largest_enclosing(*mode),
        fn_val.get_nth_param(10).unwrap(),
    );
    registers_hashmap.insert(
        Register::R11B.largest_enclosing(*mode),
        fn_val.get_nth_param(11).unwrap(),
    );
    registers_hashmap.insert(
        Register::R12B.largest_enclosing(*mode),
        fn_val.get_nth_param(12).unwrap(),
    );
    registers_hashmap.insert(
        Register::R13B.largest_enclosing(*mode),
        fn_val.get_nth_param(13).unwrap(),
    );
    registers_hashmap.insert(
        Register::R14B.largest_enclosing(*mode),
        fn_val.get_nth_param(14).unwrap(),
    );
    registers_hashmap.insert(
        Register::R15B.largest_enclosing(*mode),
        fn_val.get_nth_param(15).unwrap(),
    );
    registers_hashmap.insert(
        Register::FLAGS.largest_enclosing(*mode),
        fn_val.get_nth_param(16).unwrap(),
    );
    registers_hashmap.insert(
        Register::IP.largest_enclosing(*mode),
        fn_val.get_nth_param(17).unwrap(),
    );

    registers_hashmap
}

pub fn get_int_type<'ctx>(
    context: &'ctx Context,
    register: &Register,
    mode: &MachineMode,
) -> IntType<'ctx> {
    match register.width(*mode) {
        128 => context.i128_type(), // Never really expected to be constructed
        64 => context.i64_type(),
        32 => context.i32_type(),
        16 => context.i16_type(),
        8 => context.i8_type(),
        _ => unimplemented!(), // TODO: consider changing to unreachable
    }
}
