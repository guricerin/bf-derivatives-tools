use std::result::Result;

#[derive(Debug)]
pub struct Memory {
    cells: Vec<u8>,
    ptr: usize,
}

impl Memory {
    pub fn new(mem_size: usize) -> Self {
        Self {
            cells: vec![0; mem_size],
            ptr: 0,
        }
    }

    pub fn get(&self) -> u8 {
        self.cells[self.ptr]
    }

    pub fn set(&mut self, b: u8) {
        self.cells[self.ptr] = b;
    }

    pub fn rshift(&mut self) -> Result<(), &'static str> {
        let p = self.ptr;
        if p == self.cells.len() {
            return Err("cell pointer overflow!");
        }
        self.ptr = p + 1;
        Ok(())
    }

    pub fn lshift(&mut self) -> Result<(), &'static str> {
        let p = self.ptr;
        if p == 0 {
            return Err("cell pointer underflow!");
        }
        self.ptr = p - 1;
        Ok(())
    }

    pub fn inc(&mut self) -> Result<(), &'static str> {
        let n = self.get();
        if n == u8::MAX {
            return Err("cell value overflow!");
        }
        self.set(n + 1);
        Ok(())
    }

    pub fn dec(&mut self) -> Result<(), &'static str> {
        let n = self.get();
        if n == u8::MIN {
            return Err("cell value underflow!");
        }
        self.set(n - 1);
        Ok(())
    }

    pub fn ready_loop_begin(&self) -> bool {
        self.get() == 0
    }

    pub fn ready_loop_end(&self) -> bool {
        self.get() != 0
    }
}
