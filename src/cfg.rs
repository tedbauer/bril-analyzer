use std::io::{self, Read};

use crate::prog;
use crate::prog::Program;
use crate::prog::Instruction;

pub struct CFG {
    
}

pub struct Block<'a> {
    instrs: Vec<&'a Instruction>
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        Self { instrs: Vec::new() }
    }

    pub fn add_instr(&mut self, i: &'a Instruction) {
        self.instrs.push(i);
    }
}

fn gen_blocks(instrs: &Vec<Instruction>) -> Vec<Block> {
    let mut blocks:Vec<Block> = Vec::new();
    blocks.push(Block::new());

    for instr in instrs {
        match instr {
            Instruction::Label { label: _ } => {
                blocks.push(Block::new());
                blocks.last_mut().unwrap().add_instr(&instr);
            }
            Instruction::Constant { op: _, dest: _, type_: _, value: _ } => {
                blocks.last_mut().unwrap().add_instr(&instr);
            }
            Instruction::ValueOperation { op, dest: _, type_: _, args: _, funcs: _, labels: _ } => {
                if op == "ret" {
                    blocks.last_mut().unwrap().add_instr(&instr);
                    blocks.push(Block::new());
                } else {
                    blocks.last_mut().unwrap().add_instr(&instr);
                }
            }
            Instruction::EffectOperation { op, args: _, funcs: _, labels: _ } => {
                if op == "jmp" || op == "br" {
                    blocks.last_mut().unwrap().add_instr(&instr);
                    blocks.push(Block::new());
                } else {
                    blocks.last_mut().unwrap().add_instr(&instr);
                }
            }
        }
    }

    return blocks;

}

pub fn gen_cfg(p: Program) -> io::Result<()> {

    let blocks = gen_blocks(&p.functions[0].instrs);

    Ok(())
}
