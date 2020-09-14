use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::{self, Read};

mod prog;
use prog::Program;

mod cfg;

mod count_divs;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let program: Program = serde_json::from_str(&buffer).unwrap();
    //let div_count = count_divs::count_divs(&program.functions[0].instrs);
    let div_count = count_divs::count_divs(&program);
    println!("Counted {} div statements.", div_count);
    //cfg::gen_cfg(program);

    //println!("{}", program.functions[0].name);
    //println!("{}", serde_json::to_string(&program)?);
    Ok(())
}
