use crate::lifter::semantics::{LiftControl, Lifter};
use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegisterEnum;

use llvmkit::ir::{FunctionValue, IntDyn, IntValue, IrError, IrResult, Linkage, Module, Type};
use zydis::{FullInstruction, MachineMode, Register};

pub mod error;
pub use error::{Error, Result};

pub(crate) const CPU_FLAGS: [ExtendedRegisterEnum; 18] = [
    ExtendedRegisterEnum::CF,
    ExtendedRegisterEnum::PF,
    ExtendedRegisterEnum::AF,
    ExtendedRegisterEnum::ZF,
    ExtendedRegisterEnum::SF,
    ExtendedRegisterEnum::TF,
    ExtendedRegisterEnum::IF,
    ExtendedRegisterEnum::DF,
    ExtendedRegisterEnum::OF,
    ExtendedRegisterEnum::IOPL,
    ExtendedRegisterEnum::NT,
    ExtendedRegisterEnum::RF,
    ExtendedRegisterEnum::VM,
    ExtendedRegisterEnum::AC,
    ExtendedRegisterEnum::VIF,
    ExtendedRegisterEnum::VIP,
    ExtendedRegisterEnum::ID,
    ExtendedRegisterEnum::RFLAGS,
];

pub(crate) const BASE_REGS_IN_MIN_SIZE: [Register; 8] = [
    Register::AX,
    Register::BX,
    Register::CX,
    Register::DX,
    Register::SI,
    Register::DI,
    Register::SP,
    Register::BP,
];

pub(crate) const X64_REGS_IN_MIN_SIZE: [Register; 8] = [
    Register::R8B,
    Register::R9B,
    Register::R10B,
    Register::R11B,
    Register::R12B,
    Register::R13B,
    Register::R14B,
    Register::R15B,
];

pub(crate) fn register_arg_count(mode: MachineMode) -> usize {
    BASE_REGS_IN_MIN_SIZE.len()
        + if mode == MachineMode::LONG_64 {
            X64_REGS_IN_MIN_SIZE.len()
        } else {
            0
        }
        + 1
}

pub(crate) fn register_args_in_min_size(mode: MachineMode) -> impl Iterator<Item = Register> {
    let extended: &[Register] = if mode == MachineMode::LONG_64 {
        &X64_REGS_IN_MIN_SIZE
    } else {
        &[]
    };

    BASE_REGS_IN_MIN_SIZE
        .into_iter()
        .chain(extended.iter().copied())
        .chain([Register::IP])
}

pub struct Compiler<'m, 'ctx> {
    mode: MachineMode,
    pub lifter: LifterX86<'m, 'ctx>,
    func_value: FunctionValue<'ctx, IntDyn>,
}

impl<'m, 'ctx> Compiler<'m, 'ctx> {
    pub fn new_with_x86_lifter(
        module: &'m Module<'ctx>,
        mode: MachineMode,
        runtime_address: Option<u64>,
    ) -> Result<Self> {
        let func_value = create_func(module, mode)?;
        let lifter = LifterX86::new(module, mode, func_value, runtime_address)?;

        Ok(Self {
            mode,
            lifter,
            func_value,
        })
    }

    pub fn lift_function(&mut self, instructions: &[FullInstruction]) -> Result<()> {
        for instruction in instructions {
            match self.lifter.lift_instr(instruction) {
                Ok(LiftControl::Continue) => {}
                Ok(LiftControl::FunctionTerminated) => return Ok(()),
                Err(e) => return Err(Error::LifterError(e)),
            }
        }

        if self.lifter.builder.is_some() {
            let rax = Register::AX.largest_enclosing(self.mode);
            let rax_val = self.lifter.load_register_value(&rax)?;
            let rax_as_int: IntValue<'ctx, IntDyn> = rax_val.try_into()?;
            let expected_retval_type = self.func_value.return_int_type();
            let rax_with_correct_size = self
                .lifter
                .create_z_ext_or_trunc(rax_as_int, expected_retval_type)?;
            let builder = self.lifter.take_builder()?;
            builder.build_ret(rax_with_correct_size)?;
        }

        Ok(())
    }
}

pub fn lift_to_ir_text<'ctx>(
    module: Module<'ctx>,
    instructions: &[FullInstruction],
    mode: MachineMode,
    runtime_address: Option<u64>,
) -> Result<String> {
    {
        let mut compiler = Compiler::new_with_x86_lifter(&module, mode, runtime_address)?;
        compiler.lift_function(instructions)?;
    }
    let verified = module.verify()?;
    Ok(format!("{verified}"))
}

pub(crate) fn create_func<'ctx>(
    module: &Module<'ctx>,
    mode: MachineMode,
) -> IrResult<FunctionValue<'ctx, IntDyn>> {
    let example_reg = Register::AX.largest_enclosing(mode);
    let int_type = module.custom_width_int_type(example_reg.width(mode).into())?;

    let register_arg_count = register_arg_count(mode);
    let mut args: Vec<Type<'ctx>> = Vec::with_capacity(register_arg_count + CPU_FLAGS.len());
    for _ in register_args_in_min_size(mode) {
        args.push(int_type.as_type());
    }
    for _ in CPU_FLAGS {
        args.push(module.bool_type().as_type());
    }

    let fn_type = module.fn_type(int_type.as_type(), args, false);
    let mut builder = module
        .function_builder::<IntDyn, _>("protected", fn_type)
        .linkage(Linkage::External);

    for (id, reg) in register_args_in_min_size(mode).enumerate() {
        let slot = u32::try_from(id).map_err(|_| IrError::InvalidOperation {
            message: "register argument index exceeds u32::MAX",
        })?;
        let name =
            reg.largest_enclosing(mode)
                .static_string()
                .ok_or(IrError::InvalidOperation {
                    message: "register name missing",
                })?;
        builder = builder.param_name(slot, name);
    }
    for (id, cpu_flag) in CPU_FLAGS.into_iter().enumerate() {
        let raw_slot = register_arg_count + id;
        let slot = u32::try_from(raw_slot).map_err(|_| IrError::InvalidOperation {
            message: "flag argument index exceeds u32::MAX",
        })?;
        builder = builder.param_name(slot, format!("{cpu_flag:?}"));
    }
    builder.build()
}
