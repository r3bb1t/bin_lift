use super::Result;
use zydis::FullInstruction;

mod x86;

pub(crate) enum LiftControl {
    Continue,
    FunctionTerminated,
}

pub trait Lifter {
    fn lift_instr(&mut self, instruction: &FullInstruction) -> Result<LiftControl>;
}
