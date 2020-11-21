use super::jump_table::*;
use super::memory::*;
use super::token::*;

use std::io;
use std::result::Result;

#[derive(Debug)]
pub struct Interpreter {
    memory: Memory,
}

impl Interpreter {
    pub fn new(mem_size: usize) -> Self {
        let memory = Memory::new(mem_size);
        Self { memory: memory }
    }

    pub fn run(&mut self, tokens: &Vec<Token>) -> Result<(), &'static str> {
        let len = tokens.len();
        let input = io::stdin();
        let jump_table = JumpTable::new(&tokens)?;
        let mut pc = 0;
        while pc < len {
            let token = tokens[pc];
            match token {
                Token::RShift => {
                    self.memory.rshift();
                }
                Token::LShift => {
                    self.memory.lshift();
                }
                Token::Inc => {
                    self.memory.inc();
                }
                Token::Dec => {
                    self.memory.dec();
                }
                Token::Read => {
                    let mut buf = String::new();
                    input.read_line(&mut buf).expect("read line error");
                    self.memory.read(buf.as_bytes()[0]);
                }
                Token::Write => {
                    print!("{}", self.memory.get() as char);
                }
                Token::LoopBegin => {
                    if self.memory.ready_loop_begin() {
                        if let Some(next) = jump_table.get(pc) {
                            pc = *next;
                        }
                    }
                }
                Token::LoopEnd => {
                    if self.memory.ready_loop_end() {
                        if let Some(next) = jump_table.get(pc) {
                            pc = *next;
                        }
                    }
                }
                _ => (),
            }
            pc += 1;
        }
        Ok(())
    }
}
