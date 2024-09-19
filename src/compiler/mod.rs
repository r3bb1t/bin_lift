use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegister;
use crate::util::get_int_type;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::FunctionValue;
use zydis::{MachineMode, Register};

pub mod contexts;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    mode: MachineMode,
    pub lifter: LifterX86<'ctx>,
}

pub const CPU_FLAGS: [ExtendedRegister; 17] = [
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

pub const ALL_REGS_IN_MIN_SIZE: [Register; 17] = [
    Register::AX,
    Register::BX,
    Register::CX,
    Register::DX,
    Register::SI,
    Register::DI,
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

impl<'ctx> Compiler<'ctx> {
    //pub fn new(context: &'ctx Context, mode: MachineMode) -> Self {
    //    let lifter = LifterX86::new(context, mode);
    //
    //    Self {
    //        context,
    //        mode,
    //        lifter,
    //    }
    //}

    //pub fn compile_new<O: Operands>(
    //    &'ctx self,
    //    instructions: Vec<Instruction<O>>,
    //) -> Result<FunctionValue<'ctx>, BuilderError> {
    //    let func_value = Self::create_func(&self.mode, self.context, &self.module);
    //
    //    let entry_basic_block = self.context.append_basic_block(func_value, "entry");
    //    self.builder.position_at_end(entry_basic_block);
    //
    //    for ins in instructions {
    //        self.lifter.lift_instr(ins)?;
    //    }
    //
    //    Ok(func_value)
    //}
    //
    ///// Compiles the specified `Function` in the given `Context` and using the specified `Builder` and `Module`.
    //pub fn compile<O: Operands>(
    //    context: &'ctx Context,
    //    builder: &'ctx Builder<'ctx>,
    //    module: &'ctx Module<'ctx>,
    //    instructions: Vec<Instruction<O>>,
    //    mode: MachineMode,
    //) -> Result<FunctionValue<'ctx>, BuilderError> {
    //    let compiler = Compiler {
    //        context,
    //        builder,
    //        module,
    //        mode,
    //    };
    //
    //    let compiled_fn = compiler.compile_fn(instructions)?;
    //
    //    Ok(compiled_fn)
    //}

    /// Compiles stuff into an LLVM `FunctionValue`.
    //fn compile_fn<O: Operands>(
    //    &'ctx self,
    //    instructions: Vec<Instruction<O>>,
    //) -> Result<FunctionValue<'ctx>, BuilderError> {
    //    let func_value = self.create_func(&self.mode);
    //
    //    let regs_hashmap = prep_regs_hashmap(&func_value, &self.mode);
    //
    //    let entry_basic_block = self.context.append_basic_block(func_value, "entry");
    //    self.builder.position_at_end(entry_basic_block);
    //
    //    //let lifter = LifterX86 {
    //    //    context: self.context,
    //    //    builder: self.builder,
    //    //    module: self.module,
    //    //    regs_hashmap: RefCell::new(regs_hashmap),
    //    //    mode: self.mode.clone(),
    //    //};
    //
    //    let lifter = LifterX86::new(
    //        self.context,
    //        self.builder,
    //        self.module,
    //        self.mode,
    //        regs_hashmap,
    //    );
    //
    //    for ins in instructions {
    //        lifter.lift_instr(ins)?;
    //    }
    //
    //    Ok(func_value)
    //}
    fn create_func(
        mode: &MachineMode,
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        let example_reg = Register::AX.largest_enclosing(*mode); // random rax for convenience
        let int_type = get_int_type(context, &example_reg, mode);

        const ARGS_COUNT: usize = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
        let regs_args: [BasicMetadataTypeEnum; ALL_REGS_IN_MIN_SIZE.len()] =
            core::array::from_fn(|_| int_type.into());

        let flags_args: [BasicMetadataTypeEnum; CPU_FLAGS.len()] =
            core::array::from_fn(|_| context.i8_type().into());

        let mut args = Vec::with_capacity(ARGS_COUNT);
        args.extend_from_slice(&regs_args);
        args.extend_from_slice(&flags_args);

        let fn_type = int_type.fn_type(&args, false);
        let fn_val = module.add_function("protected", fn_type, None);

        /// Inner function for converting register names
        fn get_reg_name_for_mode(reg: Register, mode: MachineMode) -> &'static str {
            reg.largest_enclosing(mode).static_string().unwrap()
        }

        // Set names for regular regs
        for (id, reg) in ALL_REGS_IN_MIN_SIZE.into_iter().enumerate() {
            fn_val
                .get_nth_param(id as u32)
                .unwrap()
                .set_name(get_reg_name_for_mode(reg, *mode));
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
