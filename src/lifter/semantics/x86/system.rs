use super::Result;
use crate::lifter::LifterX86;

use llvmkit::ir::{IntDyn, IntValue, IntrinsicId, IrError, Value};
use zydis::Register;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    pub(super) fn lift_rdtsc(&mut self) -> Result<()> {
        // FIXME: Keep this on llvmkit's intrinsic path and revisit once its intrinsic API stabilizes.
        let name = "llvm.readcyclecounter";
        let timestamp: IntValue<'ctx, IntDyn> = self
            .builder()?
            .build_intrinsic_call_by_id(
                IntrinsicId::READCYCLECOUNTER,
                name,
                core::iter::empty::<Value<'ctx>>(),
                "rdtsc_val",
            )?
            .return_value()
            .ok_or(IrError::InvalidOperation {
                message: "llvm.readcyclecounter returned void",
            })?
            .try_into()?;

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
