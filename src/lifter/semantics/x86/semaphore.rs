use super::Result;
use crate::lifter::{Error, LifterX86};
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{AtomicOrdering, AtomicRMWBinOp, AtomicRMWConfig, IntDyn, IntValue, SyncScope};
use zydis::{ffi::DecodedOperandKind, Instruction, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_xadd<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        if operands.len() < 2 {
            return Err(Error::UnsupportedInstr("xadd requires two operands"));
        }
        let dest = &operands[0];
        let src = &operands[1];
        if !matches!(&src.kind, DecodedOperandKind::Reg(_)) {
            return Err(Error::UnsupportedInstr("xadd source must be register"));
        }

        let rhs = self.load_single_int_op(src, dest.size)?;
        let (lhs, result) = match &dest.kind {
            DecodedOperandKind::Mem(mem) => {
                let pointer = self.mergen_calculate_memory_operand(mem)?;
                let old_inst = self.builder()?.build_atomicrmw(
                    AtomicRMWBinOp::Add,
                    pointer,
                    rhs,
                    AtomicRMWConfig::new(AtomicOrdering::SequentiallyConsistent, SyncScope::System),
                    "xadd_atomic",
                )?;
                let lhs: IntValue<'ctx, IntDyn> = old_inst.as_value().try_into()?;
                let result = self
                    .builder()?
                    .build_int_add::<IntDyn, _, _, _>(lhs, rhs, "xadd_sum")?;
                self.store_op(src, lhs)?;
                (lhs, result)
            }
            DecodedOperandKind::Reg(_) => {
                let lhs = self.load_single_int_op(dest, dest.size)?;
                let result = self
                    .builder()?
                    .build_int_add::<IntDyn, _, _, _>(lhs, rhs, "xadd_sum")?;
                self.store_op(src, lhs)?;
                self.store_op(dest, result)?;
                (lhs, result)
            }
            _ => {
                return Err(Error::UnsupportedInstr(
                    "xadd destination must be register or memory",
                ))
            }
        };

        self.store_xadd_flags(lhs, rhs, result)
    }

    fn store_xadd_flags(
        &mut self,
        lhs: IntValue<'ctx, IntDyn>,
        rhs: IntValue<'ctx, IntDyn>,
        result: IntValue<'ctx, IntDyn>,
    ) -> Result<()> {
        let cf_lhs =
            self.builder()?
                .build_icmp_ult::<IntDyn, _, _, _>(result, lhs, "xadd_cf_lhs")?;
        let cf_rhs =
            self.builder()?
                .build_icmp_ult::<IntDyn, _, _, _>(result, rhs, "xadd_cf_rhs")?;
        let cf = self
            .builder()?
            .build_int_or::<bool, _, _, _>(cf_lhs, cf_rhs, "xadd_cf")?;

        let xor_inputs =
            self.builder()?
                .build_int_xor::<IntDyn, _, _, _>(lhs, rhs, "xadd_af_inputs")?;
        let xor_result = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
            xor_inputs,
            result,
            "xadd_af_result",
        )?;
        let aux_bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            xor_result,
            result.ty().const_int_raw(0x10, false)?,
            "xadd_af_bit",
        )?;
        let af = self.builder()?.build_icmp_ne::<IntDyn, _, _, _>(
            aux_bit,
            result.ty().const_zero(),
            "xadd_af",
        )?;

        let zero = result.ty().const_zero();
        let result_sign =
            self.builder()?
                .build_icmp_slt::<IntDyn, _, _, _>(result, zero, "xadd_result_sign")?;
        let lhs_sign =
            self.builder()?
                .build_icmp_slt::<IntDyn, _, _, _>(lhs, zero, "xadd_lhs_sign")?;
        let rhs_sign =
            self.builder()?
                .build_icmp_slt::<IntDyn, _, _, _>(rhs, zero, "xadd_rhs_sign")?;
        let same_input_sign =
            self.builder()?
                .build_icmp_eq::<bool, _, _, _>(lhs_sign, rhs_sign, "xadd_same_sign")?;
        let sign_changed = self.builder()?.build_icmp_ne::<bool, _, _, _>(
            lhs_sign,
            result_sign,
            "xadd_sign_changed",
        )?;
        let of = self.builder()?.build_int_and::<bool, _, _, _>(
            same_input_sign,
            sign_changed,
            "xadd_of",
        )?;

        let pf = self.xadd_even_parity(result)?;
        let sf = self
            .builder()?
            .build_icmp_slt::<IntDyn, _, _, _>(result, zero, "xadd_sf")?;
        let zf = self
            .builder()?
            .build_icmp_eq::<IntDyn, _, _, _>(result, zero, "xadd_zf")?;

        self.store_cpu_flag(ExtendedRegisterEnum::AF, af.as_dyn())?;
        self.store_cpu_flag(ExtendedRegisterEnum::CF, cf.as_dyn())?;
        self.store_cpu_flag(ExtendedRegisterEnum::OF, of.as_dyn())?;
        self.store_cpu_flag(ExtendedRegisterEnum::PF, pf.as_dyn())?;
        self.store_cpu_flag(ExtendedRegisterEnum::SF, sf.as_dyn())?;
        self.store_cpu_flag(ExtendedRegisterEnum::ZF, zf.as_dyn())
    }

    fn xadd_even_parity(&self, value: IntValue<'ctx, IntDyn>) -> Result<IntValue<'ctx, bool>> {
        let i8_ty = self.module.i8_type().as_dyn();
        let mut folded = self.create_z_ext_or_trunc(value, i8_ty)?;
        for shift in [4_u64, 2, 1] {
            let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                folded,
                i8_ty.const_int_raw(shift, false)?,
                "xadd_parity_shift",
            )?;
            folded = self.builder()?.build_int_xor::<IntDyn, _, _, _>(
                folded,
                shifted,
                "xadd_parity_fold",
            )?;
        }
        let low_bit = self.builder()?.build_int_and::<IntDyn, _, _, _>(
            folded,
            i8_ty.const_int_raw(1, false)?,
            "xadd_parity_bit",
        )?;
        Ok(self.builder()?.build_icmp_eq::<IntDyn, _, _, _>(
            low_bit,
            i8_ty.const_zero(),
            "xadd_pf",
        )?)
    }
}
