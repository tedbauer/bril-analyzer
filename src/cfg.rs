use std::collections::HashMap;
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
    
    pub fn get_instr(&self, idx: usize) -> &Instruction {
        self.instrs[idx]
    }
}

fn gen_fresh_name(prefix: &String, name_map: &HashMap<String, &Block>) -> String {
    let mut i: u32 = 0;
    let mut name = prefix.to_owned() + &i.to_string();

    while name_map.keys().any(|k| k.to_string() == name) {
        i += 1;
        name = prefix.to_owned() + &i.to_string();
    }

    return name;
}

fn gen_name_map<'a>(blocks: &'a Vec<Block>) -> HashMap<String, &'a Block<'a>> {
    let mut name_map: HashMap<String, &Block> = HashMap::new();
    
    for block in blocks {
        match block.get_instr(0) {
            Instruction::Label { label } => {
                name_map.insert(label.to_string(), &block);
            }
            _ => {
                name_map.insert(gen_fresh_name(&"$".to_string(), &name_map), block);
            }
        }
    }

    return name_map;
}

fn gen_blocks(instrs: &Vec<Instruction>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
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
                if op == "jmp" || op == "br" || op == "ret" {
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
    let name_to_block = gen_name_map(&blocks);

    Ok(())
}
