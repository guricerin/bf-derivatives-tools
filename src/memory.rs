#[derive(Debug)]
pub struct Memory {
    bytes: Vec<u8>,
    ptr: usize,
}

impl Memory {
    pub fn new(mem_size: usize) -> Self {
        Self {
            bytes: vec![0; mem_size],
            ptr: 0,
        }
    }

    pub fn get(&self) -> u8 {
        self.bytes[self.ptr]
    }

    pub fn rshift(&mut self) {
        self.ptr = self.ptr + 1;
    }

    pub fn lshift(&mut self) {
        self.ptr = self.ptr - 1;
    }

    pub fn inc(&mut self) {
        self.bytes[self.ptr] = self.get() + 1;
    }

    pub fn dec(&mut self) {
        self.bytes[self.ptr] = self.get() - 1;
    }

    pub fn read(&mut self, b: u8) {
        self.bytes[self.ptr] = b;
    }

    pub fn ready_loop_begin(&self) -> bool {
        self.get() == 0
    }

    pub fn ready_loop_end(&self) -> bool {
        self.get() != 0
    }
}
