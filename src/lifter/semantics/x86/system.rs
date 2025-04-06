use super::{Error, LifterX86, Result};
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::intrinsics::Intrinsic;

const RDTSC_INTRINSIC: &str = "llvm.readcyclecounter";

impl LifterX86<'_> {
    pub(super) fn lift_rdtsc(&self) -> Result<()> {
        let builder = &self.builder;
        let rdtsc_intrinsic =
            Intrinsic::find(RDTSC_INTRINSIC).ok_or(Error::IntrinsicNotFound(RDTSC_INTRINSIC))?;

        let rdtsc_func = rdtsc_intrinsic.get_declaration(&self.module, &[]).unwrap();
        let rdtsc_call = builder.build_call(rdtsc_func, &[], "rdtsc_val")?;
        let rdtsc_retval = rdtsc_call
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();
        let edx_part = builder.build_right_shift(
            rdtsc_retval,
            rdtsc_retval.get_type().const_int(32, false),
            false,
            "to_edx",
        )?;
        let eax_part = self.create_z_ext_or_trunc(rdtsc_retval, self.context.i32_type())?;

        self.store_reg(ExtendedRegisterEnum::EDX.into(), edx_part)?;
        self.store_reg(ExtendedRegisterEnum::EAX.into(), eax_part)?;

        Ok(())
    }
}
