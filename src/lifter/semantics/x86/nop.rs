use super::Result;
use crate::lifter::LifterX86;

impl<'m, 'ctx> LifterX86<'m, 'ctx> {
    #[inline]
    pub(super) fn lift_nop(&mut self) -> Result<()> {
        Ok(())
    }
}
