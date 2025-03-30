use crate::lifter::semantics::Lifter;
use crate::lifter::LifterX86;
use crate::miscellaneous::ExtendedRegisterEnum;

use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::FunctionValue;
use inkwell::{context::Context, values::IntValue};
use zydis::{FullInstruction, InstructionAttributes, MachineMode, Register};

pub mod contexts;

pub(super) mod error;
pub(crate) use error::Result;

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    mode: MachineMode,
    pub lifter: LifterX86<'ctx>,
    func_value: FunctionValue<'ctx>,
}

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
    // FIXME: Replace with flags
    ExtendedRegisterEnum::RFLAGS,
];

pub(crate) const ALL_REGS_IN_MIN_SIZE: [Register; 17] = [
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
    pub fn new_with_x86_lifter(
        context: &'ctx Context,
        mode: MachineMode,
        runtime_address: u64,
    ) -> Result<Self> {
        let module = context.create_module("protected");
        let func_value = create_func(&mode, context, &module);
        let lifter = LifterX86::new(context, mode, func_value, module, runtime_address).unwrap();

        let compiler = Self {
            context,
            mode,
            lifter,
            func_value,
        };
        Ok(compiler)
    }

    pub fn lift_function(
        &self,
        instructions: &Vec<FullInstruction>,
        //) -> Result<&FunctionValue> {
        //) -> Result<&FunctionValue> {
    ) -> Result<()> {
        #[cfg(debug_assertions)]
        let formatter = zydis::Formatter::intel();

        #[cfg(debug_assertions)]
        let mut problems_hs = std::collections::HashSet::new();

        #[cfg(debug_assertions)]
        let mut missed_instructions_count = 0;
        #[cfg(debug_assertions)]
        let mut lifted_instructions_count = 0;

        let mut last_ins_attr = zydis::InstructionAttributes::empty();

        for instruction in instructions {
            if [
                //InstructionCategory::CALL,
                //InstructionCategory::COND_BR,
                //InstructionCategory::UNCOND_BR,
                //InstructionCategory::RET,
            ]
            .contains(&instruction.meta.category)
            {
                #[cfg(debug_assertions)]
                eprintln!("Skipping {}", instruction.mnemonic);
            } else {
                //if self.lifter.lift_instr(instruction.clone()).is_err() {
                //    #[cfg(debug_assertions)]
                //    problems_hs.insert((instruction.mnemonic, instruction.meta.category));
                //    missed_instructions_count += 1;
                //}
                if instruction
                    .attributes
                    .contains(InstructionAttributes::HAS_REP)
                    && last_ins_attr.contains(InstructionAttributes::HAS_REP)
                {
                    last_ins_attr = instruction.attributes;
                    continue;
                }
                match self.lifter.lift_instr(instruction) {
                    Ok(_) => {
                        #[cfg(debug_assertions)]
                        {
                            lifted_instructions_count += 1;
                        }
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        {
                            match e {
                                crate::lifter::Error::UnsupportedInstr(_) => {
                                    problems_hs
                                        .insert((instruction.mnemonic, instruction.meta.category));
                                    missed_instructions_count += 1;
                                }
                                _ => panic!("{e}"),
                            }
                        }
                    }
                }
                last_ins_attr = instruction.attributes;
            }

            //match self.lifter.lift_instr(instruction) {
            //    Ok(_) => {
            //        #[cfg(debug_assertions)]
            //        {
            //            lifted_instructions_count += 1;
            //        }
            //    }
            //    Err(_) => {
            //        #[cfg(debug_assertions)]
            //        {
            //            problems_hs.insert((instruction.mnemonic, instruction.meta.category));
            //            missed_instructions_count += 1;
            //        }
            //    }
            //}
        }

        #[cfg(debug_assertions)]
        {
            dbg!(problems_hs.len());
            dbg!(problems_hs.len());
            dbg!(problems_hs);
            dbg!(missed_instructions_count);
            dbg!(lifted_instructions_count);
        }

        let rax = zydis::Register::AX.largest_enclosing(self.mode);

        if let Ok(rax_val) = self.lifter.load_register_value(&rax) {
            let rax_as_int: IntValue<'ctx> = rax_val.try_into()?;
            if let Some(BasicTypeEnum::IntType(expected_retval_type)) =
                self.func_value.get_type().get_return_type()
            {
                let rax_with_correct_size = self
                    .lifter
                    .create_z_ext_or_trunc(rax_as_int, expected_retval_type)?;
                self.lifter
                    .builder
                    .build_return(Some(&rax_with_correct_size))?;
            } else {
                self.lifter.builder.build_return(Some(&rax_as_int))?;
            }
        }

        //Ok(&self.func_value)
        Ok(())
    }

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

    //// Compiles stuff into an LLVM `FunctionValue`.
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
}

pub(crate) fn create_func<'ctx>(
    mode: &MachineMode,
    context: &'ctx Context,
    module: &Module<'ctx>,
) -> FunctionValue<'ctx> {
    let example_reg = Register::AX.largest_enclosing(*mode); // random rax for convenience
    let int_type = context.custom_width_int_type(example_reg.width(*mode).into());
    //let int_type = get_int_type(context, &example_reg, mode);

    const ARGS_COUNT: usize = ALL_REGS_IN_MIN_SIZE.len() + CPU_FLAGS.len();
    let regs_args: [BasicMetadataTypeEnum; ALL_REGS_IN_MIN_SIZE.len()] =
        core::array::from_fn(|_| int_type.into());

    //let flags_args: [BasicMetadataTypeEnum; CPU_FLAGS.len()] =
    //    core::array::from_fn(|_| context.i8_type().into());
    let flags_args: [BasicMetadataTypeEnum; CPU_FLAGS.len()] =
        core::array::from_fn(|_| context.bool_type().into());

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
