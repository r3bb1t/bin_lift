use crate::miscellaneous::ExtendedRegisterEnum;

use super::{LifterX86, Result};

use inkwell::values::IntValue;
use zydis::{ffi::DecodedOperandKind, Instruction, InstructionAttributes, Mnemonic, Operands};

impl LifterX86<'_> {
    // NOTE: checked
    pub(super) fn lift_bswap<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ops = instr.operands();
        let dest = &ops[0];

        let l_value: IntValue<'_> = self.load_single_op(dest, dest.size)?.try_into()?;
        let l_value_ty = l_value.get_type();

        if dest.size == 16 {
            let zero = l_value_ty.const_zero();
            self.store_op(dest, zero)?;
            return Ok(());
        }

        let mut new_swapped_value = l_value_ty.const_zero();
        let mut mask = l_value_ty.const_int(0xff, false);

        for i in 0..l_value_ty.get_bit_width() / 8 {
            let byte = builder.build_right_shift(
                builder.build_and(l_value, mask, "shlresultmsb")?,
                l_value_ty.const_int((i * 8).into(), false),
                false,
                "",
            )?;
            let shift_by = l_value_ty.get_bit_width() - (i + 1) * 8;
            let new_pos_byte =
                builder.build_left_shift(byte, l_value_ty.const_int(shift_by.into(), false), "")?;

            new_swapped_value = builder.build_or(new_swapped_value, new_pos_byte, "")?;
            mask = builder.build_left_shift(mask, mask.get_type().const_int(8, false), "")?;
        }

        self.store_op(dest, new_swapped_value)?;
        Ok(())
    }

    // NOTE: checked
    pub(super) fn lift_mov<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let operands = instr.operands();

        let dest = &operands[0];
        let src = &operands[1];

        let dst_size = &dest.size;
        let s_ext_ty = self.context.custom_width_int_type((*dst_size).into());

        // TODO: In future, double check that we indeed need src.size
        let r_value: IntValue<'_> = self.load_single_op(src, src.size)?.try_into()?;

        let mut s_ext_rhs = match instr.mnemonic {
            Mnemonic::MOV => r_value,
            Mnemonic::MOVSX | Mnemonic::MOVSXD => {
                let name = if instr.mnemonic == Mnemonic::MOVSXD {
                    "movsxd_"
                } else {
                    "movsx_"
                };
                builder.build_int_s_extend(r_value, s_ext_ty, name)?
            }
            Mnemonic::MOVZX => builder.build_int_z_extend(r_value, s_ext_ty, "movzx_")?,
            _ => unreachable!(),
        };

        if let DecodedOperandKind::Imm(_) = &dest.kind {
            s_ext_rhs = self.load_single_op(dest, *dst_size)?.try_into()?;
        }

        self.store_op(dest, s_ext_rhs)?;

        Ok(())
    }

    pub(super) fn lift_movs_x<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;
        let ctx = self.context;
        let ops = instr.operands();
        let size = &ops[1].size.clone();

        let mut dst_ptr_value = self.load_single_op(&ops[1], *size)?;
        self.store_op(&ops[0], dst_ptr_value)?;

        let is_rep = instr.attributes.contains(InstructionAttributes::HAS_REP);

        let df = self.load_flag(ExtendedRegisterEnum::DF)?;

        let byte_size_value: u64 = (*size).into();

        let src_op = if is_rep { &ops[2 + 1] } else { &ops[2] };
        let dst_op = if is_rep { &ops[3 + 1] } else { &ops[3] };

        let src_op_ty = ctx.custom_width_int_type(src_op.size.into());
        let direction = builder
            .build_select(
                df,
                src_op_ty.const_int(byte_size_value, false),
                src_op_ty.const_int(u64::MAX.wrapping_mul(byte_size_value), false),
                "",
            )?
            .into_int_value();

        //let direction = builder
        //    .build_select(
        //        df,
        //        ctx.custom_width_int_type(src_op.size.into())
        //            .const_int(1 * byte_size_value, false),
        //        ctx.custom_width_int_type(src_op.size.into())
        //            //.const_int(u64::MAX * byte_size_value, false),
        //            .const_int(u64::MAX.wrapping_mul(byte_size_value), false),
        //        "",
        //    )?
        //    .into_int_value();

        let src_value: IntValue<'_> = self.load_single_op(src_op, src_op.size)?.try_into()?;
        let dst_value: IntValue<'_> = self.load_single_op(dst_op, dst_op.size)?.try_into()?;

        if is_rep {
            let count_ci: IntValue<'_> = self.load_single_op(&ops[2], ops[2].size)?.try_into()?;
            debug_assert!(count_ci.is_constant_int(), "fix rep");
            let mut update_src_value = src_value;
            let mut update_dst_value = dst_value;
            let looptime = count_ci.get_zero_extended_constant().unwrap();

            for _ in 0..looptime {
                dst_ptr_value = self.load_single_op(&ops[1], *size)?;
                self.store_op(&ops[0], dst_ptr_value)?;

                update_src_value = builder.build_int_add(update_src_value, direction, "")?;
                update_dst_value = builder.build_int_add(update_dst_value, direction, "")?;

                self.store_op(src_op, update_src_value)?;
                self.store_op(dst_op, update_dst_value)?;
            }

            self.store_op(&ops[2], count_ci.get_type().const_zero())?;
        }

        let update_src_value = builder.build_int_add(src_value, direction, "")?;
        let update_dst_value = builder.build_int_add(dst_value, direction, "")?;

        self.store_op(src_op, update_src_value)?;
        self.store_op(dst_op, update_dst_value)?;

        Ok(())
    }

    pub(super) fn lift_xchg<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let ops = instr.operands();
        let dst = &ops[0];
        let src = &ops[1];

        let rhs: IntValue<'_> = self.load_single_op(src, src.size)?.try_into()?;
        let lhs: IntValue<'_> = self.load_single_op(dst, dst.size)?.try_into()?;

        self.store_op(dst, rhs)?;
        self.store_op(src, lhs)?;

        Ok(())
    }
}
