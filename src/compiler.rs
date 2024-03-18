use crate::lifter::LifterX86;
use crate::util::{get_int_type, prep_regs_hashmap};
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use std::cell::RefCell;
use zydis::{Instruction, MachineMode, Operands, Register};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    mode: &'a MachineMode,
}
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
            // println!("{} {:?}", ins.mnemonic, ins.meta.category);
            lifter.lift_instr(ins)?;
        }

        Ok(func_value)
    }

    fn create_func(&self, mode: &MachineMode) -> FunctionValue<'ctx> {
        let example_reg = Register::AX.largest_enclosing(*mode); // random rax for convenience
        let int_type = get_int_type(self.context, &example_reg, mode);
        let fn_type = self.context.void_type().fn_type(
            &[
                int_type.into(), // RAX
                int_type.into(), // RBX
                int_type.into(), // RCX
                int_type.into(), // RDX
                int_type.into(), // RSI
                int_type.into(), // RDI
                int_type.into(), // RBP
                int_type.into(), // RSP
                int_type.into(), // R8
                int_type.into(), // R9
                int_type.into(), // R10
                int_type.into(), // R11
                int_type.into(), // R12
                int_type.into(), // R13
                int_type.into(), // R14
                int_type.into(), // R15
                int_type.into(), // RFLAGS
                int_type.into(), // RIP
            ],
            false,
        );
        let fn_val = self.module.add_function("protected", fn_type, None);

        /// Inner function for converting register names
        fn get_reg_name_for_mode(reg: Register, mode: MachineMode) -> &'static str {
            reg.largest_enclosing(mode).static_string().unwrap()
        }

        let mode = *self.mode;
        // TODO: Replace with struct
        fn_val
            .get_nth_param(0)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::AX, mode));
        fn_val
            .get_nth_param(1)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::BX, mode));
        fn_val
            .get_nth_param(2)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::CX, mode));
        fn_val
            .get_nth_param(3)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::DX, mode));
        fn_val
            .get_nth_param(4)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::SI, mode));
        fn_val
            .get_nth_param(5)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::DI, mode));
        fn_val
            .get_nth_param(6)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::BP, mode));
        fn_val
            .get_nth_param(7)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::SP, mode));
        fn_val
            .get_nth_param(8)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R8B, mode));
        fn_val
            .get_nth_param(9)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R9B, mode));
        fn_val
            .get_nth_param(10)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R10B, mode));
        fn_val
            .get_nth_param(11)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R11B, mode));
        fn_val
            .get_nth_param(12)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R12B, mode));
        fn_val
            .get_nth_param(13)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R13B, mode));
        fn_val
            .get_nth_param(14)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R14B, mode));
        fn_val
            .get_nth_param(15)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::R15B, mode));
        fn_val
            .get_nth_param(16)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::FLAGS, mode));
        fn_val
            .get_nth_param(17)
            .unwrap()
            .set_name(get_reg_name_for_mode(Register::IP, mode));

        fn_val
    }
}
