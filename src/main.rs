#![allow(dead_code)]

use std::io;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub code: Vec<u8>,
}

impl Cpu {
    /// Read 32bit instruction from a memory
    fn fetch(&self) -> u32 {
        todo!()
    }
    /// Decode an instruction and execute it
    fn execute(&mut self, inst: u32) {
        todo!()
    }
}

fn main() -> io::Result<()> {
    unimplemented!()
}
