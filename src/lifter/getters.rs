use std::borrow::Borrow;

use super::{error::Error, error::Result, ExtendedRegister};

use inkwell::values::IntValue;
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind, ImmediateInfo, MemoryInfo},
    MachineMode, Register, RegisterClass,
};

use super::{LifterX86, PossibleLLVMValueEnum};

impl<'ctx> LifterX86<'ctx> {
    // NOTE: Must be used only with instructions having exactly one operand if called from
    // semantics
    pub(super) fn load_single_op(
        &self,
        operand: &DecodedOperand,
        possible_size: u16,
    ) -> Result<PossibleLLVMValueEnum<'ctx>> {
        match operand.kind {
            DecodedOperandKind::Reg(register) => {
                let reg_val = self.load_reg_internal(&register)?;
                Ok(reg_val)
            }
            DecodedOperandKind::Mem(ref memory_info) => {
                let mem_val = self.load_mem(memory_info, possible_size, false)?;
                Ok(mem_val.into())
            }
            DecodedOperandKind::Imm(ref imm) => {
                //let imm_val = self.experimental_load_imm_internal(imm, possible_size);
                let imm_val = self.load_imm_internal(imm, possible_size);
                Ok(imm_val.into())
            }
            DecodedOperandKind::Unused | DecodedOperandKind::Ptr(_) => unreachable!(),
        }
    }

    pub(super) fn load_two_first_ops(
        &self,
        operands: &[DecodedOperand],
    ) -> Result<[PossibleLLVMValueEnum<'ctx>; 2]> {
        let first_op: IntValue<'_> = self
            .load_single_op(&operands[0], operands[0].size)?
            .try_into()?;
        let second_op: IntValue<'_> = self
            .load_single_op(&operands[1], operands[0].size)?
            .try_into()?;

        let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
        Ok([first_op.into(), second_op_zext.into()])
    }

    pub(super) fn load_two_first_ints(
        &self,
        ops: &[DecodedOperand],
    ) -> Result<[IntValue<'ctx>; 2]> {
        let first_op: IntValue<'_> = self.load_single_op(&ops[0], ops[0].size)?.try_into()?;
        let second_op: IntValue<'_> = self.load_single_op(&ops[1], ops[0].size)?.try_into()?;

        let second_op_zext = self.create_z_ext_or_trunc(second_op, first_op.get_type())?;
        Ok([first_op, second_op_zext])
    }

    //pub(super) fn load_flag<T: Borrow<ExtendedRegister>>(
    //    &self,
    //    cpu_flag: T,
    //) -> Result<IntValue<'ctx>> {
    //    let cpu_flag = cpu_flag.borrow();
    //    let regs_hashmap = self.regs_hashmap.get();
    //    let v = unsafe {
    //        (*regs_hashmap)
    //            .get(cpu_flag)
    //            .ok_or(Error::RegUnwrapError(*cpu_flag))?
    //            .to_owned()
    //            .try_into()?
    //    };
    //
    //    Ok(v)
    //}

    pub(super) fn load_flag<T: Borrow<ExtendedRegister>>(
        &self,
        cpu_flag: T,
    ) -> Result<IntValue<'ctx>> {
        let cpu_flag = cpu_flag.borrow();
        //let pr = r.largest_enclosing(self.mode);
        let regs_hashmap = self.regs_hashmap.get();
        let lookup_result = unsafe {
            (*regs_hashmap).get(cpu_flag).copied()
            //.ok_or(Error::RegUnwrapError(pr))
        };

        let reg_val = match lookup_result {
            Some(val) => val.try_into()?,
            //None => self.get_max_int_type().get_undef(),
            None => self.get_max_int_type().const_zero(),
        };

        Ok(reg_val)
    }

    fn load_mem(
        &self,
        mem: &MemoryInfo,
        first_operand_size: u16,
        lea: bool,
    ) -> Result<IntValue<'ctx>> {
        let builder = &self.builder;

        let mem_op_addr = self.retdec_calc_mem_operand(mem)?;

        if lea {
            Ok(mem_op_addr)
        } else {
            let t = self
                .context
                .custom_width_int_type(first_operand_size.into());
            //let pt = self.context.ptr_type(AddressSpace::default());
            //let addr2 = builder.build_int_to_ptr(mem_op_addr, pt, "")?;
            //let load = builder.build_load(t, addr2, "experimental_load_mem_")?;

            //let pointee = builder.build_alloca(t, "pointee_for_gep")?;

            //let gepped = unsafe {
            //    builder.build_gep(
            //        pointee.get_type(),
            //        self.stackmemory,
            //        &[mem_op_addr],
            //        "gep_loaded",
            //    )?
            //};
            //dbg!(gepped);

            //let loaded = builder.build_load(t, gepped, "loaded_from_gep")?;

            let pointer = unsafe {
                builder.build_gep(
                    self.context.i8_type(),
                    self.stackmemory,
                    &[mem_op_addr],
                    "GEPSTORE",
                )?
            };

            let retval = builder.build_load(t, pointer, "Loadxd")?.into_int_value();

            Ok(retval)
            //Ok(loaded.into_int_value())

            //Ok(load.into_int_value())
            //todo!()
        }
    }

    // FIXME: Taken from original lifter. Needs rework ig
    pub(crate) fn load_reg_internal(
        &self,
        register: &Register,
    ) -> Result<PossibleLLVMValueEnum<'ctx>> {
        let builder = &self.builder;

        if register.class() == RegisterClass::FLAGS {
            let flags_reg = self.get_rflags_value()?;
            return Ok(flags_reg.into());
        }

        if register == &Register::NONE {
            return Err(Error::RegUnwrapError((*register).into()));
        }
        let rt = self.get_register_type(*register);

        let pr = if [Register::RBP, Register::EBP, Register::BP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RBP,
                MachineMode::LEGACY_32 => Register::EBP,
                _ => unimplemented!("Only 32 and 64 bit supported right now"),
            }
        } else if [Register::RSP, Register::ESP, Register::SP].contains(register) {
            match self.mode {
                MachineMode::LONG_64 => Register::RSP,
                MachineMode::LEGACY_32 => Register::ESP,
                _ => unimplemented!("Only 32 and 64 bit supported right now"),
            }
        } else {
            register.largest_enclosing(self.mode)
        };

        //let regs_hashmap = self.regs_hashmap.get();
        //let reg = unsafe {
        //    (*regs_hashmap)
        //        .get(&pr.into())
        //        .ok_or(Error::RegUnwrapError(pr.into()))?
        //};
        let reg = self.get_register(pr)?;

        let reg = match reg {
            PossibleLLVMValueEnum::IntValue(int_value) => int_value,
            PossibleLLVMValueEnum::FloatValue(_) => unreachable!(),
        };

        let mut ret: PossibleLLVMValueEnum;

        // Honestly, i think we will be ignoring IP in future
        if register.class() == zydis::RegisterClass::IP {
            ret = self.get_register(*register)?;
            //ret = unsafe {
            //    (*regs_hashmap)
            //        .get(&(*register).into())
            //        .copied()
            //        .ok_or(Error::RegUnwrapError((*register).into()))?
            //}
        } else {
            //ret = (*reg).into();
            ret = reg.into();

            if register != &pr {
                if [Register::AH, Register::BH, Register::CH, Register::DH].contains(register) {
                    if let PossibleLLVMValueEnum::IntValue(ret2) = ret {
                        let ret3: PossibleLLVMValueEnum<'_> =
                            ret2.const_shl(ret2.get_type().const_int(8, false)).into();

                        ret = ret3;
                    } else {
                        panic!("message to myself: fix your retarded code!")
                    }
                }

                ret = match ret {
                    PossibleLLVMValueEnum::IntValue(int) => {
                        let truncated = self.create_z_ext_or_trunc(int, rt.try_into()?)?;
                        //let truncated = builder.build_int_truncate(int, rt.try_into()?, "")?;
                        truncated.into()
                    }
                    PossibleLLVMValueEnum::FloatValue(float) => {
                        // TODO: Check for possible bug
                        let truncated = builder.build_float_trunc(float, rt.try_into()?, "")?;
                        truncated.into()
                    }
                }
            }
        }

        Ok(ret)
    }

    fn load_imm_internal(&self, imm: &ImmediateInfo, possible_size: u16) -> IntValue<'ctx> {
        self.context
            .custom_width_int_type(possible_size.into())
            .const_int(imm.value, imm.is_signed)
    }
}
