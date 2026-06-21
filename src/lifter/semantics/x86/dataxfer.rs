use super::Result;
use crate::lifter::{Error, LifterX86};

use llvmkit::ir::{IntDyn, IntValue};
use zydis::{ffi::DecodedOperandKind, Instruction, Mnemonic, Operands};

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_bswap<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = operands
            .first()
            .ok_or(Error::UnsupportedInstr("bswap missing destination operand"))?;
        if !matches!(&dest.kind, DecodedOperandKind::Reg(_)) {
            return Err(Error::UnsupportedInstr(
                "bswap requires register destination",
            ));
        }
        let value = self.load_single_int_op(dest, dest.size)?;
        let ty = value.ty();

        match dest.size {
            16 => return self.store_op(dest, ty.const_zero()),
            32 | 64 => {}
            _ => return Err(Error::UnsupportedInstr("unsupported bswap operand size")),
        }

        let byte_count = u32::from(dest.size / 8);
        let mut result: IntValue<'ctx, IntDyn> = ty.const_zero().as_value().try_into()?;

        for index in 0..byte_count {
            let source_shift = u64::from(index * 8);
            let dest_shift = u64::from((byte_count - 1 - index) * 8);
            let shifted = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
                value,
                ty.const_int_raw(source_shift, false)?,
                "bswap_src",
            )?;
            let byte = self.builder()?.build_int_and::<IntDyn, _, _, _>(
                shifted,
                ty.const_int_raw(0xff, false)?,
                "bswap_byte",
            )?;
            let positioned = self.builder()?.build_int_shl::<IntDyn, _, _, _>(
                byte,
                ty.const_int_raw(dest_shift, false)?,
                "bswap_dst",
            )?;
            result = self
                .builder()?
                .build_int_or::<IntDyn, _, _, _>(result, positioned, "bswap")?;
        }

        self.store_op(dest, result)
    }

    pub(super) fn lift_mov<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let dest = operands
            .first()
            .ok_or(Error::UnsupportedInstr("mov missing destination operand"))?;
        let src = operands
            .get(1)
            .ok_or(Error::UnsupportedInstr("mov missing source operand"))?;
        match instr.mnemonic {
            Mnemonic::MOV => {
                if !matches!(
                    &dest.kind,
                    DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
                ) {
                    return Err(Error::UnsupportedInstr(
                        "mov requires register or memory destination",
                    ));
                }
                if !matches!(
                    &src.kind,
                    DecodedOperandKind::Reg(_)
                        | DecodedOperandKind::Mem(_)
                        | DecodedOperandKind::Imm(_)
                ) {
                    return Err(Error::UnsupportedInstr(
                        "mov requires register, memory, or immediate source",
                    ));
                }
                if matches!(&dest.kind, DecodedOperandKind::Mem(_))
                    && matches!(&src.kind, DecodedOperandKind::Mem(_))
                {
                    return Err(Error::UnsupportedInstr(
                        "mov cannot transfer memory to memory",
                    ));
                }
            }
            Mnemonic::MOVSX | Mnemonic::MOVSXD | Mnemonic::MOVZX => {
                if !matches!(&dest.kind, DecodedOperandKind::Reg(_)) {
                    return Err(Error::UnsupportedInstr(
                        "movsx/movzx require register destination",
                    ));
                }
                if !matches!(
                    &src.kind,
                    DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
                ) {
                    return Err(Error::UnsupportedInstr(
                        "movsx/movzx require register or memory source",
                    ));
                }
            }
            _ => return Err(Error::UnsupportedInstr("unsupported move instruction")),
        }

        let src_value = self.load_single_int_op(src, src.size)?;
        let dest_ty = self.module.custom_width_int_type(u32::from(dest.size))?;

        let value = match instr.mnemonic {
            Mnemonic::MOV => self.create_z_ext_or_trunc(src_value, dest_ty)?,
            Mnemonic::MOVSX | Mnemonic::MOVSXD => {
                if src_value.ty().bit_width() < dest_ty.bit_width() {
                    self.builder()?
                        .build_sext_dyn(src_value, dest_ty, "movsx")?
                } else {
                    self.create_z_ext_or_trunc(src_value, dest_ty)?
                }
            }
            Mnemonic::MOVZX => self.create_z_ext_or_trunc(src_value, dest_ty)?,
            _ => return Err(Error::UnsupportedInstr("unsupported move instruction")),
        };

        self.store_op(dest, value)
    }

    pub(super) fn lift_xchg<O: Operands>(&mut self, instr: &Instruction<O>) -> Result<()> {
        let operands = instr.operands();
        let lhs = operands
            .first()
            .ok_or(Error::UnsupportedInstr("xchg missing first operand"))?;
        let rhs = operands
            .get(1)
            .ok_or(Error::UnsupportedInstr("xchg missing second operand"))?;
        let lhs_supported = matches!(
            &lhs.kind,
            DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
        );
        let rhs_supported = matches!(
            &rhs.kind,
            DecodedOperandKind::Reg(_) | DecodedOperandKind::Mem(_)
        );
        let lhs_register = matches!(&lhs.kind, DecodedOperandKind::Reg(_));
        let rhs_register = matches!(&rhs.kind, DecodedOperandKind::Reg(_));
        if !lhs_supported || !rhs_supported || (!lhs_register && !rhs_register) {
            return Err(Error::UnsupportedInstr(
                "xchg requires one register operand and one register or memory operand",
            ));
        }
        if lhs.size != rhs.size {
            return Err(Error::UnsupportedInstr(
                "xchg operands must have equal size",
            ));
        }
        let lhs_value = self.load_single_op(lhs, lhs.size)?;
        let rhs_value = self.load_single_op(rhs, rhs.size)?;

        self.store_op(lhs, rhs_value)?;
        self.store_op(rhs, lhs_value)
    }
}
