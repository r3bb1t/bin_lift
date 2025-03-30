use super::Result;
use crate::lifter::LifterX86;

impl LifterX86<'_> {
    pub(super) fn lift_nop(&self) -> Result<()> {
        Ok(())
    }
}
