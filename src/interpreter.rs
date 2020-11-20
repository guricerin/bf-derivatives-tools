use super::jump_table::*;
use super::memory::*;
use super::token::*;
use std::io;
use std::result::Result;

#[derive(Debug)]
pub struct Interpreter {
    tokens: Vec<Token>,
    jump_table: JumpTable,
}

impl Interpreter {
    pub fn parse(program: &str) -> Result<Self, &'static str> {
        let tokens = vec![];
        let jump_table = JumpTable::try_new(&tokens)?;
        let res = Self {
            tokens: tokens,
            jump_table: jump_table,
        };
        Ok(res)
    }

    pub fn run(&self, mem_size: usize) -> Result<(), &'static str> {
        let len = self.tokens.len();
        let input = io::stdin();
        let mut memory = Memory::new(mem_size);
        let mut pc = 0;
        while pc < len {
            let token = self.tokens[pc];
            match token {
                Token::RShift => {
                    memory.rshift();
                }
                Token::LShift => {
                    memory.lshift();
                }
                Token::Inc => {
                    memory.inc();
                }
                Token::Dec => {
                    memory.dec();
                }
                Token::Read => {
                    let mut buf = String::new();
                    input.read_line(&mut buf).expect("read line error");
                    memory.read(buf.as_bytes()[0]);
                }
                Token::Write => {
                    print!("{}", memory.get() as char);
                }
                Token::LoopBegin => {
                    if memory.ready_loop_begin() {
                        if let Some(next) = self.jump_table.get(pc) {
                            pc = *next;
                        }
                    }
                }
                Token::LoopEnd => {
                    if memory.ready_loop_end() {
                        if let Some(next) = self.jump_table.get(pc) {
                            pc = *next;
                        }
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}
