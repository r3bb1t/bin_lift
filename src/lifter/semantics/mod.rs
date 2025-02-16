use super::{Error, Result};
use zydis::FullInstruction;

mod x86;

pub trait Lifter {
    fn lift_instr(&self, instruction: &FullInstruction) -> Result<()>;
}
