use brainfuck_maker::interpreter::*;
use std::result::Result;

fn main() -> Result<(), &'static str> {
    let program = "";
    Interpreter::parse(program)?.run(30000)
}
