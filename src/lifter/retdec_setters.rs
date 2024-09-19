use core::panic;

use inkwell::{
    builder::BuilderError,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, IntValue},
    AddressSpace, IntPredicate,
};
use zydis::{
    ffi::{DecodedOperand, DecodedOperandKind},
    Register,
};

use crate::miscellaneous::ExtendedRegister;

use super::{eOpConv, LifterX86};

impl<'ctx> LifterX86<'ctx> {
    pub fn retdec_store_op<T>(
        &self,
        op: &DecodedOperand,
        val: T,
        ct: Option<eOpConv>,
    ) -> Result<(), BuilderError>
    where
        BasicValueEnum<'ctx>: From<T>,
    {
        let op_kind: &DecodedOperandKind = &op.kind;
        let ct = ct.unwrap_or(eOpConv::ZEXT_TRUNC_OR_BITCAST);
        //let val = BasicValueEnum::from(val);

        match op_kind {
            DecodedOperandKind::Reg(reg) => self.retdec_store_register(*reg, val, Some(ct))?,
            DecodedOperandKind::Mem(mem) => {
                let builder = &self.builder;

                let addr = self.retdec_calc_mem_operand(mem)?;

                let mut val = BasicValueEnum::from(val);
                let tt = if val.is_int_value() {
                    self.retdec_get_integer_type_from_byte_size((op.size / 8).into())
                } else if val.is_float_value() {
                    unimplemented!("Float type isn't implemented yet")
                } else {
                    dbg!(val);

                    self.module.print_to_stderr();
                    unreachable!()
                };

                val =
                    self.retdec_generate_type_conversion(val, Some(tt.as_basic_type_enum()), ct)?;

                // TODO: Replace hardcoded AddressSpace
                let pt = self.context.ptr_type(AddressSpace::default());
                let addr_ptr = builder.build_int_to_ptr(addr, pt, "")?;
                builder.build_store(addr_ptr, val)?;
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    // TODO: Support floating point regs.
    // Rn if we will get a floating point reg it will most likely panic
    pub fn retdec_store_register<T>(
        &self,
        r: Register,
        val: T,
        ct: Option<eOpConv>,
    ) -> Result<(), BuilderError>
    where
        BasicValueEnum<'ctx>: From<T>,
    {
        let ct = ct.unwrap_or(eOpConv::ZEXT_TRUNC_OR_BITCAST);
        let mut val = BasicValueEnum::from(val);
        let bldr = &self.builder;
        let ctx = self.context;

        let rt = self.get_register_type(r);
        let rt_size = r.width(self.mode);

        let pr = r.largest_enclosing(self.mode);
        let pr_size = pr.width(self.mode);

        if let (BasicTypeEnum::IntType(rt_), BasicValueEnum::IntValue(val_)) = (rt, val) {
            let val_t = val_.get_type();
            if val_t.get_bit_width() > (r.width(self.mode)).into() {
                val = bldr
                    .build_int_truncate(val_, rt_, "")?
                    .as_basic_value_enum();
            }
        }

        let reg = self
            .retdec_get_register(pr)
            .unwrap_or_else(|| panic!("Unhandled reg: {r}"));
        // TODO: really 'rt'? (Probably yes)
        val = self.retdec_generate_type_conversion(val, Some(reg.get_type()), ct)?;

        if r == pr || (pr_size == 64 && r.width(self.mode) == 32) {
            self.retdec_set_register(r, val);
            Ok(())
        } else {
            // Honestly, i don't fully understand what's going on in this else block

            let l = self.retdec_get_register(r).unwrap();
            let l_ty = l.get_type();

            let l_size = l_ty.into_int_type().get_bit_width();

            if let BasicTypeEnum::IntType(l_ty_) = l_ty {
                //let bit_width = l_ty_.get_bit_width();
                //dbg!(bit_width);

                [16, 32, 64].contains(&l_ty_.get_bit_width())
            } else {
                panic!("Unexpected parent type.")
            };

            let mut and_c = None;

            if [Register::AH, Register::CH, Register::DH, Register::BH].contains(&r) {
                let v = match l_size {
                    16 => ctx.i16_type().const_int(0x00ff, false),
                    32 => ctx.i32_type().const_int(0xffff00ff, false),
                    64 => ctx.i64_type().const_int(0xffffffffffff00ff, false),
                    _ => unreachable!(),
                };

                and_c = Some(v);

                let val_as_int = val.into_int_value();
                val = self
                    .builder
                    .build_left_shift(val_as_int, val_as_int.get_type().const_int(8, false), "")?
                    .as_basic_value_enum();
            } else {
                let v = match rt_size {
                    8 => match l_size {
                        16 => Some(ctx.i16_type().const_int(0xff00, false)),
                        32 => Some(ctx.i32_type().const_int(0xffffff00, false)),
                        64 => Some(ctx.i64_type().const_int(0xffffffffffffff00, false)),
                        _ => None,
                    },
                    16 => match l_size {
                        32 => Some(ctx.i32_type().const_int(0xffffff00, false)),
                        64 => Some(ctx.i64_type().const_int(0xffffffffffffff00, false)),
                        _ => unreachable!(),
                    },
                    32 => {
                        if l_size == 32 {
                            Some(ctx.i64_type().const_int(0xffffffff00000000, false))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if let Some(v) = v {
                    and_c = Some(v)
                }
            }

            let and_c = and_c.expect("Mask not initialized in storeRegister().");

            let l = bldr.build_and(l.into_int_value(), and_c, "")?;
            let o = bldr.build_or(l, val.into_int_value(), "")?;
            self.retdec_set_register(r, o.as_basic_value_enum());
            //let ptr_type = ctx.ptr_type(AddressSpace::default());
            //bldr.build_store(o.const_to_pointer(ptr_type), reg)
            Ok(())
        }
    }

    // Region: flags
    pub(super) fn retdec_store_registers_plus_sflags<T>(
        &self,
        sflags_val: T,
        regs: &[(ExtendedRegister, IntValue<'ctx>)],
    ) -> Result<(), BuilderError>
    where
        BasicValueEnum<'ctx>: From<T>,
    {
        let sflags_val = BasicValueEnum::from(sflags_val);
        for (reg, val) in regs {
            let val = val.as_basic_value_enum();
            self.store_cpu_flag(*reg, val);
        }

        let sflags_val_as_int = sflags_val.into_int_value();
        self.store_cpu_flag(
            ExtendedRegister::ZF,
            self.generateZeroFlag(sflags_val_as_int)?
                .as_basic_value_enum(),
        );
        self.store_cpu_flag(
            ExtendedRegister::SF,
            self.generateSignFlag(sflags_val_as_int)?
                .as_basic_value_enum(),
        );
        self.store_cpu_flag(
            ExtendedRegister::PF,
            self.generateParityFlag(sflags_val_as_int)?
                .as_basic_value_enum(),
        );
        Ok(())
    }

    #[allow(non_snake_case)]
    fn generateZeroFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
        let zero = val.get_type().const_int(0, false);
        self.builder
            .build_int_compare(IntPredicate::EQ, val, zero, "")
    }

    #[allow(non_snake_case)]
    fn generateSignFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
        let zero = val.get_type().const_int(0, false);
        self.builder
            .build_int_compare(IntPredicate::SLT, val, zero, "")
    }

    #[allow(non_snake_case)]
    fn generateParityFlag(&self, val: IntValue<'ctx>) -> Result<IntValue<'ctx>, BuilderError> {
        let builder = &self.builder;
        let i8t = self.context.i8_type();
        let trunc = self.builder.build_int_truncate(val, i8t, "")?;
        let f = inkwell::intrinsics::Intrinsic::find("llvm.ctpop")
            .expect("Can't find 'llvm.ctpop' intrinsic while calculating Parity Flag (PF)");

        let f_func = f.get_declaration(&self.module, &[i8t.into()]).unwrap();
        let c = builder.build_call(f_func, &[trunc.into()], "")?;

        let c_retval = c.try_as_basic_value().left().unwrap().into_int_value();

        let a = builder
            // TODO: Check if "false" really fits
            .build_and(c_retval, c_retval.get_type().const_int(1, false), "")?;

        // TODO: Check if false fits here as well
        builder.build_int_compare(IntPredicate::EQ, a, a.get_type().const_int(0, false), "")
    }
    // Endregion: flags

    fn retdec_get_register(&self, r: Register) -> Option<BasicValueEnum<'ctx>> {
        let pr = r.largest_enclosing(self.mode);
        let regs_hashmap = self.regs_hashmap.borrow();
        regs_hashmap
            .get(&pr.into())
            .map(|v| v.as_basic_value_enum())
    }

    fn retdec_set_register(&self, r: Register, val: BasicValueEnum<'ctx>) {
        let pr = r.largest_enclosing(self.mode);
        let mut regs_hashmap = self.regs_hashmap.borrow_mut();
        regs_hashmap.insert(pr.into(), val);
    }
}
