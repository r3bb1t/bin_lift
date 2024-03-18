use crate::lifter::LifterX86;
use inkwell::builder::BuilderError;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use zydis::ffi::{DecodedOperandKind, MemoryInfo};
use zydis::Register;

impl<'a, 'b, 'ctx> LifterX86<'a, 'b, 'ctx> {
    pub fn store_op<T>(&self, op_kind: DecodedOperandKind, val: T)
    where
        BasicValueEnum<'b>: From<T>,
    {
        let val = BasicValueEnum::from(val);
        match op_kind {
            DecodedOperandKind::Reg(reg) => self.store_reg(reg, val),
            DecodedOperandKind::Mem(mem_info) => self.store_mem(mem_info, &val).unwrap(),
            DecodedOperandKind::Imm(_) | DecodedOperandKind::Unused => unreachable!(),
            _ => unimplemented!(),
        }
    }

    fn store_mem(&self, mem_info: MemoryInfo, val: &BasicValueEnum) -> Result<(), BuilderError> {
        let addr = self.calc_mem_operand(&mem_info)?;

        let i64_type = self.context.i64_type();
        let addr_to_ptr = self.builder.build_int_to_ptr(
            addr.into_int_value(),
            i64_type.ptr_type(AddressSpace::default()),
            "mem_ptr_for_store_",
        )?;
        self.builder.build_store(addr_to_ptr, *val)?;

        Ok(())
    }

    fn store_reg(&self, reg: Register, val: BasicValueEnum<'b>) {
        let largest_enclosing = reg.largest_enclosing(*self.mode);
        let mut regs_hashmap = self.regs_hashmap.borrow_mut();

        // let cached = regs_hashmap.get(&largest_enclosing).unwrap();
        // let cached_size = cached.get_type();
        // let new_value = self.builder.build_load(cached_size,cached.into_pointer_value(), "" ).unwrap();

        regs_hashmap.insert(largest_enclosing, val);
    }
}
