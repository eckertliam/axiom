use crate::{
    ast::{Instruction, Program, Value},
    ir::{Block, Module, Opcode},
};

pub fn build_ir(program: Program) -> Module {
    let mut module = Module::new();

    // first we go through and add all labels as blocks
    for instruction in &program.instructions {
        if instruction.instr == "label" {
            match &instruction.args[0] {
                Value::Symbol(label) => {
                    module.add_block(Block {
                        label: label.clone(),
                        ops: Vec::new(),
                    });
                }
                _ => {
                    eprintln!("Invalid label: {}", instruction.args[0]);
                }
            }
        }
    }

    // then we set the current block to the entry block
    module.set_current_block("entry");

    // then we go through and add all instructions to the current block
    for instruction in &program.instructions {
        match instruction.instr.as_str() {
            "jump" => parse_jump(&instruction, &mut module),
            "jump_if" => parse_jump_if(&instruction, &mut module),
            "ret" => module.add_op(Opcode::Ret),
            "ipush" => parse_ipush(&instruction, &mut module),
            "iload" => module.add_op(Opcode::ILoad),
            "istore" => module.add_op(Opcode::IStore),
            "iadd" => module.add_op(Opcode::IAdd),
            "isub" => module.add_op(Opcode::ISub),
            "imul" => module.add_op(Opcode::IMul),
            "idiv" => module.add_op(Opcode::IDiv),
            "imod" => module.add_op(Opcode::IMod),
            "ipow" => module.add_op(Opcode::IPow),
            "icomp" => module.add_op(Opcode::IComp),
            "iprint" => module.add_op(Opcode::IPrint),
            "fpush" => parse_fpush(&instruction, &mut module),
            "fload" => module.add_op(Opcode::FLoad),
            "fstore" => module.add_op(Opcode::FStore),
            "fadd" => module.add_op(Opcode::FAdd),
            "fsub" => module.add_op(Opcode::FSub),
            "fmul" => module.add_op(Opcode::FMul),
            "fdiv" => module.add_op(Opcode::FDiv),
            "fcomp" => module.add_op(Opcode::FComp),
            "fprint" => module.add_op(Opcode::FPrint),
            "ppush" => parse_ppush(&instruction, &mut module),
            "pload" => module.add_op(Opcode::PLoad),
            "pstore" => module.add_op(Opcode::PStore),
            "padd" => module.add_op(Opcode::PAdd),
            "psub" => module.add_op(Opcode::PSub),
            "pmul" => module.add_op(Opcode::PMul),
            "pdiv" => module.add_op(Opcode::PDiv),
            "pmod" => module.add_op(Opcode::PMod),
            "pcomp" => module.add_op(Opcode::PComp),
            "pprint" => module.add_op(Opcode::PPrint),
            "bpush" => parse_bpush(&instruction, &mut module),
            "bload" => module.add_op(Opcode::BLoad),
            "bstore" => module.add_op(Opcode::BStore),
            "band" => module.add_op(Opcode::BAnd),
            "bor" => module.add_op(Opcode::BOr),
            "bnot" => module.add_op(Opcode::BNot),
            "bprint" => module.add_op(Opcode::BPrint),
            "cpush" => parse_cpush(&instruction, &mut module),
            "cload" => module.add_op(Opcode::CLoad),
            "cstore" => module.add_op(Opcode::CStore),
            "ccomp" => module.add_op(Opcode::CComp),
            "cprint" => module.add_op(Opcode::CPrint),
            _ => todo!(),
        }
    }

    module
}

fn parse_jump(instruction: &Instruction, module: &mut Module) {
    match &instruction.args[0] {
        Value::Symbol(label) => {
            if let Some(block_index) = module.get_block(label) {
                module.add_op(Opcode::Jump(block_index as u64));
            } else {
                eprintln!("Label not found: {}", label);
            }
        }
        _ => {
            eprintln!("Invalid jump: {}", instruction.args[0]);
        }
    }
}

fn parse_jump_if(instruction: &Instruction, module: &mut Module) {
    match &instruction.args[0..2] {
        [Value::Symbol(label), Value::Symbol(label2)] => {
            if let Some(block_index) = module.get_block(label) {
                if let Some(block_index2) = module.get_block(label2) {
                    module.add_op(Opcode::JumpIf(block_index as u64, block_index2 as u64));
                } else {
                    eprintln!("Label not found: {}", label2);
                }
            } else {
                eprintln!("Label not found: {}", label);
            }
        }
        _ => {
            eprintln!("Invalid jump_if: {}", instruction.args[0]);
        }
    }
}

macro_rules! push_fn {
    ($name:ident, $type:ident, $opcode:ident) => {
        fn $name(instruction: &Instruction, module: &mut Module) {
            match &instruction.args[0] {
                Value::$type(value) => {
                    module.add_op(Opcode::$opcode(*value));
                }
                _ => {
                    eprintln!("Invalid {} push: {}", stringify!($type), instruction.args[0]);
                }
            }
        }
    }
}


push_fn!(parse_ipush, Integer, IPush);
push_fn!(parse_fpush, Float, FPush);
push_fn!(parse_bpush, Boolean, BPush);
push_fn!(parse_cpush, Char, CPush);
push_fn!(parse_ppush, Pointer, PPush);



