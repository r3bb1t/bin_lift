use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegister;
use crate::util::get_int_type;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::cell::RefCell;
use std::collections::HashMap;
use zydis::{Instruction, MachineMode, Operands, Register};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    mode: &'a MachineMode,
}

const CPU_FLAGS: [ExtendedRegister; 17] = [
    ExtendedRegister::CF,
    ExtendedRegister::PF,
    ExtendedRegister::AF,
    ExtendedRegister::ZF,
    ExtendedRegister::SF,
    ExtendedRegister::TF,
    ExtendedRegister::IF,
    ExtendedRegister::DF,
    ExtendedRegister::OF,
    ExtendedRegister::IOPL,
    ExtendedRegister::NT,
    ExtendedRegister::RF,
    ExtendedRegister::VM,
    ExtendedRegister::AC,
    ExtendedRegister::VIF,
    ExtendedRegister::VIP,
    ExtendedRegister::ID,
];

const ALL_REGS_IN_MIN_SIZE: [Register; 18] = [
    Register::AX,
    Register::BX,
    Register::CX,
    Register::DX,
    Register::SI,
    Register::DI,
    Register::BP,
    Register::SP,
    Register::BP,
    Register::R8B,
    Register::R9B,
    Register::R10B,
    Register::R11B,
    Register::R12B,
    Register::R13B,
    Register::R14B,
    Register::R15B,
    Register::IP,
];

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder` and `Module`.
    pub fn compile<O: Operands>(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        instructions: Vec<Instruction<O>>,
        mode: &'a MachineMode,
    ) -> Result<FunctionValue<'ctx>, BuilderError> {
        let compiler = Compiler {
            context,
            builder,
            module,
            mode,
        };

        let compiled_fn = compiler.compile_fn(instructions)?;

        Ok(compiled_fn)
    }

    /// Compiles stuff into an LLVM `FunctionValue`.
    fn compile_fn<O: Operands>(
        &self,
        instructions: Vec<Instruction<O>>,
    ) -> Result<FunctionValue<'ctx>, BuilderError> {
        let func_value = self.create_func(self.mode);

        let regs_hashmap = prep_regs_hashmap(&func_value, self.mode);

        let entry_basic_block = self.context.append_basic_block(func_value, "entry");
        self.builder.position_at_end(entry_basic_block);

        let lifter = LifterX86 {
            context: self.context,
            builder: self.builder,
            module: self.module,
            regs_hashmap: RefCell::new(regs_hashmap),
            mode: self.mode,
        };

        for ins in instructions {
            lifter.lift_instr(ins)?;
        }

        Ok(func_value)
    }

    fn create_func(&self, mode: &MachineMode) -> FunctionValue<'ctx> {
        let example_reg = Register::AX.largest_enclosing(*mode); // random rax for convenience
        let int_type = get_int_type(self.context, &example_reg, mode);

        const ARGS_COUNT: usize = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
        let regs_args: [BasicMetadataTypeEnum; ALL_REGS_IN_MIN_SIZE.len()] =
            core::array::from_fn(|_| int_type.into());

        let flags_args: [BasicMetadataTypeEnum; CPU_FLAGS.len()] =
            core::array::from_fn(|_| self.context.i8_type().into());

        let mut args = Vec::with_capacity(ARGS_COUNT);
        args.extend_from_slice(&regs_args);
        args.extend_from_slice(&flags_args);

        let fn_type = int_type.fn_type(&args, false);
        let fn_val = self.module.add_function("protected", fn_type, None);

        /// Inner function for converting register names
        fn get_reg_name_for_mode(reg: Register, mode: MachineMode) -> &'static str {
            reg.largest_enclosing(mode).static_string().unwrap()
        }

        let mode = *self.mode;

        // Set names for regular regs
        for (id, reg) in ALL_REGS_IN_MIN_SIZE.into_iter().enumerate() {
            fn_val
                .get_nth_param(id as u32)
                .unwrap()
                .set_name(get_reg_name_for_mode(reg, mode));
        }

        // Set names for CPU flags
        for (id, cpu_flag) in CPU_FLAGS.into_iter().enumerate() {
            fn_val
                .get_nth_param((ALL_REGS_IN_MIN_SIZE.len() + id) as u32)
                .unwrap()
                .set_name(&format!("{cpu_flag:?}"));
        }

        fn_val
    }
}

/// After the function was created, use this one to store everything in a hashmap
fn prep_regs_hashmap<'ctx>(
    fn_val: &FunctionValue<'ctx>,
    mode: &MachineMode,
) -> HashMap<ExtendedRegister, BasicValueEnum<'ctx>> {
    let mut registers_hashmap = HashMap::new();
    let regs: [Register; 18] = ALL_REGS_IN_MIN_SIZE.map(|reg| reg.largest_enclosing(*mode));

    for (id, reg) in regs.into_iter().enumerate() {
        registers_hashmap.insert(reg.into(), fn_val.get_nth_param(id as u32).unwrap());
    }
    // Also insert flags separately
    let mut last_index = regs.len() - 1;
    for cpu_flag in CPU_FLAGS {
        last_index += 1;
        registers_hashmap.insert(cpu_flag, fn_val.get_nth_param(last_index as u32).unwrap());
    }

    registers_hashmap
}
