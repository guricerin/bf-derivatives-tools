use super::jump_table::*;
use super::memory::*;
use super::token::*;

use std::io::{self};
use std::result::Result;

#[derive(Debug)]
pub struct Interpreter<R, W> {
    memory: Memory,
    reader: R,
    writer: W,
}

impl<R: io::Read, W: io::Write> Interpreter<R, W> {
    pub fn new(mem_size: usize, reader: R, writer: W) -> Self {
        let memory = Memory::new(mem_size);
        Self {
            memory: memory,
            reader: reader,
            writer: writer,
        }
    }

    pub fn writer(&self) -> &W {
        &self.writer
    }

    pub fn run(&mut self, tokens: &Vec<Token>) -> Result<(), &'static str> {
        let len = tokens.len();
        let jump_table = JumpTable::new(&tokens)?;
        let mut pc = 0;
        while pc < len {
            match tokens[pc] {
                Token::RShift => {
                    self.memory.rshift()?;
                }
                Token::LShift => {
                    self.memory.lshift()?;
                }
                Token::Inc => {
                    self.memory.inc()?;
                }
                Token::Dec => {
                    self.memory.dec()?;
                }
                Token::Write => {
                    let b = self.memory.get();
                    self.writer.write_all(&[b]).expect("write error");
                }
                Token::Read => {
                    let mut buf = [0];
                    self.reader.read_exact(&mut buf).expect("read exact error");
                    self.memory.set(buf[0]);
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
