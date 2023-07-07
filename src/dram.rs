use crate::exception::*;
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
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::LoadAccessFault(addr));
        }
        let n = size as usize / 8;
        let index = (addr - DRAM_BASE) as usize;
        let mut value: u64 = if (self.dram[index + n - 1] >> 7) & 1 == 1 {
            0xffffffffffffffff
        } else {
            0
        };
        for i in 0..n {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        Ok(value)
    }
    pub fn load_u(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::LoadAccessFault(addr));
        }
        let n = size as usize / 8;
        let index = (addr - DRAM_BASE) as usize;
        let mut value = 0;
        for i in 0..n {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        Ok(value)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if ![8, 16, 32, 64].contains(&size) {
            return Err(Exception::StoreAMOAccessFault(addr));
        }
        let n = size as usize / 8;
        let index = (addr - DRAM_BASE) as usize;
        for i in 0..n {
            self.dram[index + i] = ((value >> (8 * i)) & 0xff) as u8;
        }
        Ok(())
    }
}
