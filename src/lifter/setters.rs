use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegister;
use inkwell::builder::BuilderError;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use zydis::ffi::{DecodedOperandKind, MemoryInfo};
use zydis::Register;

impl<'ctx> LifterX86<'ctx> {
    pub fn store_op<T>(&self, op_kind: &DecodedOperandKind, val: T) -> Result<(), BuilderError>
    where
        BasicValueEnum<'ctx>: From<T>,
    {
        match op_kind {
            DecodedOperandKind::Reg(reg) => self.store_reg(*reg, val),
            DecodedOperandKind::Mem(mem_info) => {
                self.store_mem(mem_info.clone(), &BasicValueEnum::from(val))?
            }
            DecodedOperandKind::Imm(_) | DecodedOperandKind::Unused => unreachable!(),
            _ => unimplemented!(),
        };
        Ok(())
    }

    fn store_mem(&self, mem_info: MemoryInfo, val: &BasicValueEnum) -> Result<(), BuilderError> {
        //let addr = self.calc_mem_operand(&mem_info)?;
        let addr = self.retdec_calc_mem_operand(&mem_info)?;

        //let i64_type = self.context.i64_type();
        let addr_to_ptr = self.builder.build_int_to_ptr(
            //addr.into_int_value(),
            addr,
            self.context.ptr_type(AddressSpace::default()),
            //i64_type.ptr_type(AddressSpace::default()),
            "mem_ptr_for_store_",
        )?;
        self.builder.build_store(addr_to_ptr, *val)?;

        Ok(())
    }

    pub(crate) fn store_reg<T>(&self, reg: Register, val: T)
    where
        BasicValueEnum<'ctx>: From<T>,
    {
        let val = BasicValueEnum::from(val);

        let largest_enclosing = reg.largest_enclosing(self.mode);
        let mut regs_hashmap = self.regs_hashmap.borrow_mut();

        // let cached = regs_hashmap.get(&largest_enclosing).unwrap();
        // let cached_size = cached.get_type();
        // let new_value = self.builder.build_load(cached_size,cached.into_pointer_value(), "" ).unwrap();

        regs_hashmap.insert(largest_enclosing.into(), val);
    }

    pub(crate) fn store_cpu_flag<T>(&self, cpu_flag: ExtendedRegister, val: T)
    where
        T: Into<BasicValueEnum<'ctx>>,
    {
        let mut regs_hashmap = self.regs_hashmap.borrow_mut();
        regs_hashmap.insert(cpu_flag, val.into());
    }

    pub(crate) fn store_cpu_flag_bool(&self, cpu_flag: ExtendedRegister, val: bool) {
        let bool_ty = &self.context.bool_type();
        let value = match val {
            true => bool_ty.const_int(1, false),
            false => bool_ty.const_zero(),
        };
        let mut regs_hashmap = self.regs_hashmap.borrow_mut();
        regs_hashmap.insert(cpu_flag, value.into());
    }

    // region:  flags related stuff stolen from retdec
    // #[allow(non_snake_case)]
    // pub(crate) fn storeRegistersPlusSflags(
    //     &self,
    //     sflagsVal: BasicValueEnum<'ctx>,
    //     regs: &[(ExtendedRegister, IntValue<'ctx>)],
    // ) -> Result<(), BuilderError> {
    //     for (reg, val) in regs {
    //         let val = val.as_basic_value_enum();
    //         self.store_cpu_flag(*reg, val);
    //     }

    //     let sflagsVal_as_int = sflagsVal.into_int_value();
    //     self.store_cpu_flag(
    //         ExtendedRegister::ZF,
    //         self.generateZeroFlag(sflagsVal_as_int)?
    //             .as_basic_value_enum(),
    //     );
    //     self.store_cpu_flag(
    //         ExtendedRegister::SF,
    //         self.generateSignFlag(sflagsVal_as_int)?
    //             .as_basic_value_enum(),
    //     );
    //     self.store_cpu_flag(
    //         ExtendedRegister::PF,
    //         self.generateParityFlag(sflagsVal_as_int)?
    //             .as_basic_value_enum(),
    //     );
    //     Ok(())
    // }

    // #[allow(non_snake_case)]
    // fn generateZeroFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
    //     let zero = val.get_type().const_int(0, false);
    //     self.builder
    //         .build_int_compare(IntPredicate::EQ, val, zero, "")
    // }

    // #[allow(non_snake_case)]
    // fn generateSignFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
    //     let zero = val.get_type().const_int(0, false);
    //     self.builder
    //         .build_int_compare(IntPredicate::SLT, val, zero, "")
    // }

    // #[allow(non_snake_case)]
    // fn generateParityFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
    //     let builder = &self.builder;
    //     let i8t = self.context.i8_type();
    //     let trunc = self.builder.build_int_truncate(val, i8t, "")?;
    //     let f = inkwell::intrinsics::Intrinsic::find("llvm.ctpop")
    //         .expect("Can't find 'llvm.ctpop' intrinsic while calculating Parity Flag (PF)");

    //     let f_func = f.get_declaration(&self.module, &[i8t.into()]).unwrap();
    //     let c = builder.build_call(f_func, &[trunc.into()], "")?;

    //     let c_retval = c.try_as_basic_value().left().unwrap().into_int_value();

    //     let a = builder
    //         // TODO: Check if "false" really fits
    //         .build_and(c_retval, c_retval.get_type().const_int(1, false), "")?;

    //     // TODO: Check if false fits here as well
    //     builder.build_int_compare(IntPredicate::EQ, a, a.get_type().const_int(0, false), "")
    // }
    // endregion:  flags related stuff stolen from retdec
}
