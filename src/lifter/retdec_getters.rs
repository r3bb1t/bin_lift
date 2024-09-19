use crate::lifter::{eOpConv, LifterX86};
use crate::util::{get_int_ty, get_type_store_size_in_bits};
use inkwell::builder::BuilderError;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use inkwell::AddressSpace;
use zydis::ffi::{DecodedOperand, DecodedOperandKind};
use zydis::Register;

impl<'ctx> LifterX86<'ctx> {
    pub fn retdec_loadOpUnary(
        &self,
        ops: &[DecodedOperand],
        load_type: Option<BasicTypeEnum<'ctx>>,
        dst_type: Option<BasicTypeEnum<'ctx>>,
        ct: Option<eOpConv>,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        let ct = ct.unwrap_or(eOpConv::Throw);
        let op = self.retdec__load_ops(ops, 1, true, load_type, dst_type, ct)?[0];
        Ok(op)
    }

    pub fn retdec_loadOpBinary(
        &self,
        operands: &[DecodedOperand],
        if_not_equal: eOpConv,
    ) -> Result<[BasicValueEnum<'ctx>; 2], BuilderError> {
        //let ops = self.retdec__load_ops(ops, 2, true, None, None, ct)?;
        //dbg!(&ops);
        //Ok([ops[0], ops[1]])
        let lhs = self.retdec_load_operand(&operands[0], &None, false)?;

        let rhs = self.retdec_load_operand(&operands[1], &None, false)?;

        // let conversioned = self.retdec_generate_type_conversion(lhs, Some(rhs.get_type()), if_not_equal)?;

        if rhs.is_int_value() {
            let rhs = rhs.into_int_value();
            let lhs = lhs.into_int_value();

            let rhs_final = match if_not_equal {
                eOpConv::Nothing => rhs,
                eOpConv::SEXT_TRUNC_OR_BITCAST => {
                    //bldr.build_int_s_extend_or_bit_cast(lhs, rhs.get_type(), "")?
                    self.create_s_ext_or_trunc(rhs, lhs.get_type())?
                }

                eOpConv::ZEXT_TRUNC_OR_BITCAST => {
                    //self.builder
                    //    .build_int_s_extend_or_bit_cast(lhs, rhs.get_type(), "")?
                    self.create_z_ext_or_trunc(rhs, lhs.get_type())?
                }
                _ => unimplemented!(),
            };

            Ok([lhs.as_basic_value_enum(), rhs_final.as_basic_value_enum()])
        } else {
            unimplemented!()
        }
    }

    fn retdec__load_ops(
        &self,
        ops: &[DecodedOperand],
        op_count: usize,
        strict: bool,
        load_type: Option<BasicTypeEnum<'ctx>>,
        mut dst_type: Option<BasicTypeEnum<'ctx>>,
        ct: eOpConv,
    ) -> Result<Vec<BasicValueEnum<'ctx>>, BuilderError> {
        // We don't need this from retdec
        //if (strict && ops.len() != op_count) || (ops.len() < op_count) {
        //    panic!(
        //        "Trying to load {} operands from instruction with {} operands.",
        //        op_count,
        //        ops.len()
        //    )
        //}

        let mut start_op = ops.len() - op_count;
        let mut operands = vec![];

        if let Some(dst_type) = dst_type {
            let ty = self.retdec_check_type_conversion(dst_type, ct)?;
            if ty != dst_type {
                panic!("Invalid combination of destination type and conversion type")
            }
        } else {
            let op0 = self.retdec_load_op(ops, start_op, load_type, dst_type, ct)?;
            dst_type = Some(op0.get_type());
            let dst_type = Some(self.retdec_check_type_conversion(dst_type.unwrap(), ct)?);
            let op0_2 = self.retdec_generate_type_conversion(op0, dst_type, ct)?;
            start_op += 1;
            operands.push(op0_2);
        }

        while start_op < op_count {
            let op = self.retdec_load_op(ops, start_op, load_type, dst_type, ct)?;
            operands.push(op);
            start_op += 1;
        }

        Ok(operands)
    }

    pub(crate) fn retdec_load_op(
        &self,
        operands: &[DecodedOperand],
        idx: usize,
        load_type: Option<BasicTypeEnum<'ctx>>,
        dst_type: Option<BasicTypeEnum<'ctx>>,
        ct: eOpConv,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        let op = self.retdec_load_operand(&operands[idx], &load_type, false)?;
        self.retdec_generate_type_conversion(op, dst_type, ct)
    }

    /// Main loading function
    pub(crate) fn retdec_load_operand(
        &self,
        op: &DecodedOperand,
        ty: &Option<BasicTypeEnum<'ctx>>,
        lea: bool,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError> {
        match &op.kind {
            DecodedOperandKind::Reg(register) => {
                Ok(self.retdec_load_reg(register, None, None)?.unwrap())
            }
            DecodedOperandKind::Mem(mem) => {
                let builder = &self.builder;

                let addr = self.retdec_calc_mem_operand(mem)?;

                if lea {
                    Ok(addr.as_basic_value_enum())
                } else {
                    // TODO: it will likely panic when float
                    //let t = self.retdec_get_integer_type_from_byte_size((op.size / 8).into());
                    let pt = self.context.ptr_type(AddressSpace::default());
                    let addr2 = builder.build_int_to_ptr(addr, pt, "")?;
                    let load = builder.build_load(pt, addr2, "")?;

                    if let BasicValueEnum::PointerValue(ptr) = load {
                        let r = builder.build_ptr_to_int(ptr, addr.get_type(), "")?;
                        Ok(r.as_basic_value_enum())
                    } else {
                        unreachable!()
                    }
                }
            }
            DecodedOperandKind::Imm(imm) => Ok(self.load_imm(imm, op.size)),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn retdec_load_reg(
        &self,
        register: &Register,
        dst_type: Option<BasicTypeEnum<'ctx>>,
        ct: Option<eOpConv>,
    ) -> Result<Option<BasicValueEnum<'ctx>>, BuilderError> {
        let ct = ct.unwrap_or(eOpConv::Throw);
        let builder = &self.builder;

        if register == &Register::NONE {
            return Ok(None);
        }
        let rt = self.get_register_type(*register);
        let pr = register.largest_enclosing(self.mode);
        let regs_hashmap = self.regs_hashmap.borrow();
        let reg = regs_hashmap.get(&pr.into()).unwrap();

        let mut ret: BasicValueEnum;

        // Honestly, i think we will be ignoring IP in future
        if register.class() == zydis::RegisterClass::IP {
            ret = regs_hashmap.get(&(*register).into()).copied().unwrap();
        } else {
            //let ptr_type = self.context.ptr_type(AddressSpace::default());
            // FIXME: Crashing here. Found IntValue while asking for PointerValue
            //let reg_addr = reg.into_pointer_value();
            //ret = builder.build_load(ptr_type, reg_addr, "")?;
            ret = *reg;

            if register != &pr {
                if [Register::AH, Register::BH, Register::CH, Register::DH].contains(register) {
                    let ret2 = ret.into_int_value();

                    let ret3 = ret2
                        .const_shl(ret2.get_type().const_int(8, false))
                        .as_basic_value_enum();

                    ret = ret3;
                }

                if let BasicValueEnum::IntValue(int) = ret {
                    let truncated = builder.build_int_truncate(int, rt.into_int_type(), "")?;
                    ret = truncated.as_basic_value_enum();
                } else if let BasicValueEnum::FloatValue(float) = ret {
                    let truncated = builder.build_float_trunc(float, rt.into_float_type(), "")?;
                    ret = truncated.as_basic_value_enum()
                } else {
                    unreachable!()
                }
            }
        };

        let ret = self.retdec_generate_type_conversion(ret, dst_type, ct)?;

        Ok(Some(ret))
    }

    fn retdec_check_type_conversion(
        &self,
        to: BasicTypeEnum<'ctx>,
        ct: eOpConv,
    ) -> Result<BasicTypeEnum<'ctx>, BuilderError> {
        match ct {
            eOpConv::ZEXT_TRUNC_OR_BITCAST | eOpConv::SEXT_TRUNC_OR_BITCAST => {
                if to.is_int_type() {
                    let size = get_type_store_size_in_bits(to);
                    Ok(get_int_ty(self.context, size).as_basic_type_enum())
                } else {
                    Ok(to)
                }
            }
            eOpConv::FPCAST_OR_BITCAST | eOpConv::SITOFP_OR_FPCAST | eOpConv::UITOFP_OR_FPCAST => {
                unimplemented!("Floating point not supported yet")
            }
            eOpConv::Throw | eOpConv::Nothing => {
                // TODO: Replace with error
                Ok(to)
            }
        }
    }

    pub(crate) fn retdec_get_sp_pointer_reg_value(&self) -> Result<IntValue<'ctx>, BuilderError> {
        let sp = Register::SP.largest_enclosing(self.mode);
        if let Some(sp_val) = self.retdec_load_reg(&sp, None, None)? {
            Ok(sp_val.into_int_value())
        } else {
            unreachable!()
        }
    }

    pub(crate) fn retdec_load_sp_reg(&self) -> Register {
        Register::SP.largest_enclosing(self.mode)
    }
}
