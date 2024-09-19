use crate::{
    lifter::{eOpConv, LifterX86},
    miscellaneous::ExtendedRegister,
};
use inkwell::builder::BuilderError;
use zydis::{Instruction, Operands};

impl<'ctx> LifterX86<'ctx> {
    pub(super) fn lift_btc<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let bldr = &self.builder;
        let ops = instr.operands();
        //let operands = self.load_2_operands(ops, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let operands = self.retdec_loadOpBinary(ops, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        //let operands = self.retdec_loadOpBinary(ops, eOpConv::SEXT_TRUNC_OR_BITCAST)?;
        let op0 = operands[0].into_int_value();

        let mut op1 = operands[1].into_int_value();
        let op0_bit_w = bldr
            .build_int_cast(op0, op0.get_type(), "")?
            .get_type()
            .get_bit_width();
        op1 = bldr.build_and(
            op1,
            op1.get_type().const_int((op0_bit_w - 1).into(), false),
            "",
        )?;

        let srl = bldr.build_right_shift(op0, op1, false, "")?;
        let and1 = bldr.build_and(srl, srl.get_type().const_int(1, false), "")?;
        let icmp = bldr.build_int_compare(
            inkwell::IntPredicate::NE,
            and1,
            and1.get_type().const_zero(),
            "",
        )?;
        self.store_cpu_flag(ExtendedRegister::CF, icmp);

        let shl = bldr.build_left_shift(op1.get_type().const_int(1, false), op1, "")?;
        // TODO: Check this. In original it has -1 and sign extend
        let xor1 = bldr.build_xor(shl, shl.get_type().const_int(u64::MAX, true), "")?;
        let and2 = bldr.build_int_add(op1, xor1, "")?;
        let xor2 = bldr.build_xor(and1, and1.get_type().const_int(1, false), "")?;
        let shl2 = bldr.build_left_shift(xor2, op1, "")?;
        let or1 = bldr.build_or(shl2, and2, "")?;

        //self.store_op(&ops[0].kind, or1)?;
        self.retdec_store_op(&ops[0], or1, None)?;

        Ok(())
    }

    pub(super) fn lift_bts<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        let bldr = &self.builder;
        let ops = instr.operands();
        //let operands = self.load_2_operands(ops, eOpConv::ZEXT_TRUNC_OR_BITCAST)?;
        let operands = self.retdec_loadOpBinary(ops, eOpConv::ZEXT_TRUNC_OR_BITCAST)?;

        let op0 = operands[0].into_int_value();
        let mut op1 = operands[0].into_int_value();

        let op0_bit_w = bldr
            .build_int_cast(op0, op0.get_type(), "")?
            .get_type()
            .get_bit_width();

        op1 = bldr.build_and(
            op1,
            op1.get_type().const_int((op0_bit_w - 1) as u64, false),
            "",
        )?;

        let shl = bldr.build_left_shift(op1.get_type().const_int(1, false), op1, "")?;
        let and = bldr.build_int_add(shl, op0, "")?;
        let icmp = bldr.build_int_compare(
            inkwell::IntPredicate::NE,
            and,
            and.get_type().const_zero(),
            "",
        )?;
        self.store_cpu_flag(ExtendedRegister::CF, icmp);

        let or1 = bldr.build_or(shl, op0, "")?;
        //self.store_op(&ops[0].kind, or1)?;
        self.retdec_store_op(&ops[0], or1, None)?;

        Ok(())
    }

    pub(super) fn lift_bt<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }

    pub(super) fn lift_bsf<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }

    pub(super) fn lift_btr<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }

    pub(super) fn lift_bsr<O: Operands>(&self, instr: Instruction<O>) -> Result<(), BuilderError> {
        todo!()
    }
    pub(super) fn lift_bswap<O: Operands>(
        &self,
        instr: Instruction<O>,
    ) -> Result<(), BuilderError> {
        todo!()
    }
}
