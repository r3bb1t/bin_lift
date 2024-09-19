use inkwell::builder::BuilderError;
use zydis::FullInstruction;

mod x86;

/// Defines a trait for every architecture lifter
pub(super) trait Lifter {
    fn lift_instr(&self, instruction: FullInstruction) -> Result<(), BuilderError>;
}
