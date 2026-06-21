use super::Result;
use crate::lifter::LifterX86;

use llvmkit::ir::{IntDyn, IntrinsicId, Linkage, Value};
use zydis::Register;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_rdtsc(&mut self) -> Result<()> {
        // FIXME: Keep this on llvmkit's intrinsic path and revisit once its intrinsic API stabilizes.
        let name = "llvm.readcyclecounter";
        let rdtsc_func = if let Some(existing) = self.module.function_by_name_typed::<i64>(name)? {
            existing
        } else {
            let fn_ty = IntrinsicId::ReadCycleCounter.function_type(self.module, name)?;
            self.module
                .add_function::<i64, _>(name, fn_ty, Linkage::External)?
        };
        let timestamp = self
            .builder()?
            .build_call::<i64, _, _, _>(
                rdtsc_func,
                core::iter::empty::<Value<'ctx>>(),
                "rdtsc_val",
            )?
            .return_int_value()
            .as_dyn();

        let eax = self.create_z_ext_or_trunc(timestamp, self.module.i32_type().as_dyn())?;
        let edx_wide = self.builder()?.build_int_lshr::<IntDyn, _, _, _>(
            timestamp,
            timestamp.ty().const_int_raw(32, false)?,
            "rdtsc_high",
        )?;
        let edx = self.create_z_ext_or_trunc(edx_wide, self.module.i32_type().as_dyn())?;

        self.store_reg(Register::EDX, edx)?;
        self.store_reg(Register::EAX, eax)
    }
}
