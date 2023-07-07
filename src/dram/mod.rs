use crate::param::*;

pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Dram {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());
        Self { dram }
    }

    pub fn fetch(&self, addr: u64) -> u8 {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index]
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(());
        }
        Dram::load8n(self, addr, size as usize / 8)
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(());
        }
        Dram::store8n(self, addr, value, size as usize / 8)
    }

    fn load8n(&self, addr: u64, n: usize) -> Result<u64, ()> {
        if ![1, 2, 3, 4].contains(&n) {
            return Err(());
        }
        let index = (addr - DRAM_BASE) as usize;
        let mut value = 0;
        for i in 0..n {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        Ok(value)
    }
    fn store8n(&mut self, addr: u64, value: u64, n: usize) -> Result<(), ()> {
        if ![1, 2, 3, 4].contains(&n) {
            return Err(());
        }
        let index = (addr - DRAM_BASE) as usize;
        for i in 0..n {
            self.dram[index + i] = ((value >> (8 * i)) & 0xff) as u8;
        }
        Ok(())
    }
}
