use std::io::{self, Read};

use crate::prog;
use crate::prog::Program;
use crate::prog::Instruction;


pub fn count_divs(instrs: &Vec<Instruction>) -> u32 {
    let mut c: u32 = 0;

    for instr in instrs {
        match instr {
            Instruction::ValueOperation { op, dest: _, type_: _, args: _, funcs: _, labels: _ } => {
                if op == "div" {
                    c += 1;
                }
            }
            Instruction::EffectOperation { op, args: _, funcs: _, labels: _ } => {
                if op == "div" {
                    c += 1;
                }
            }
            _ => continue
        }
    }

    return c;
}
