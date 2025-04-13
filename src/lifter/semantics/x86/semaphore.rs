use super::{LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::IntPredicate;
use zydis::{Instruction, Operands};

impl LifterX86<'_> {
    pub(super) fn lift_xadd<O: Operands>(&self, instr: &Instruction<O>) -> Result<()> {
        let builder = &self.builder;

        let operands = instr.operands();
        let dest = &operands[0];
        let src = &operands[1];

        let lhs = self.load_single_int_op(dest, dest.size)?;
        let rhs = self.load_single_int_op(src, dest.size)?;

        let l_value_ty = lhs.get_type();

        let temp = builder.build_int_add(lhs, rhs, "xadd_sum_")?;

        self.store_op(src, lhs)?;
        self.store_op(dest, temp)?;

        let cf = builder.build_or(
            builder.build_int_compare(IntPredicate::ULT, temp, lhs, "")?,
            builder.build_int_compare(IntPredicate::ULT, temp, rhs, "")?,
            "",
        )?;

        let af = {
            let lower_nibble_mask = l_value_ty.const_int(0xF, false);
            let dest_lower_nibble = builder.build_and(lhs, lower_nibble_mask, "xadddst_")?;
            let src_lower_nibble = builder.build_and(rhs, lower_nibble_mask, "xaddsrc_")?;
            let sum_lower_nibble =
                builder.build_int_add(dest_lower_nibble, src_lower_nibble, "")?;

            builder.build_int_compare(
                IntPredicate::ULT,
                sum_lower_nibble,
                lower_nibble_mask,
                "xadd_af_",
            )?
        };

        let lhs_ty_zero = l_value_ty.const_zero();
        let result_sign = builder.build_int_compare(IntPredicate::SLT, temp, lhs_ty_zero, "")?;
        let dest_sign = builder.build_int_compare(IntPredicate::SLT, lhs, lhs_ty_zero, "")?;
        let src_sign =
            builder.build_int_compare(IntPredicate::SLT, rhs, rhs.get_type().const_zero(), "")?;

        let input_same_sign =
            builder.build_int_compare(IntPredicate::EQ, dest_sign, src_sign, "")?;
        let of = builder.build_and(
            input_same_sign,
            builder.build_int_compare(IntPredicate::NE, dest_sign, result_sign, "")?,
            "",
        )?;

        let pf = self.compute_parity_flag(temp)?;
        let sf = self.compute_sign_flag(temp)?;
        let zf = self.compute_zero_flag(temp)?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af);
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf);
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of);

        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf);
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf);
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf);

        Ok(())
    }
}
