use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum, IntType};
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

// TODO: Check if this even correct
pub fn get_type_store_size_in_bits(ty: BasicTypeEnum) -> u32 {
    let s = ty.size_of().unwrap().get_type().get_bit_width();

    // if let BasicTypeEnum::IntType(i) = ty {}
    //match ty {
    //    BasicTypeEnum::FloatType(f) => f.size_of().get_type().get_bit_width(),
    //    BasicTypeEnum::IntType(i) => get_size_of(i),
    //    BasicTypeEnum::VectorType(v) => v.get_size(),
    //    BasicTypeEnum::PointerType(p) => p.size_of().get_type().get_bit_width(),
    //    _ => unimplemented!("{ty:?}"),
    //};
    //
    //fn get_size_of<'ctx, T>(val: T) -> u32
    //where
    //    T: Into<IntValue<'ctx>>,
    //{
    //    let val: IntValue = val.into();
    //    val.get_type().get_bit_width()
    //}
    //panic!("{s}");
    s
}

pub fn get_int_ty(context: &Context, size: u32) -> IntType {
    match size {
        8 => context.i8_type(),
        16 => context.i16_type(),
        32 => context.i32_type(),
        64 => context.i64_type(),
        128 => context.i128_type(),
        _ => unimplemented!("Unexpected size {}", size),
    }
}

pub(crate) fn get_int_n_ty(context: &Context, num_bits: u32) -> IntType {
    get_int_ty(context, num_bits)
}
