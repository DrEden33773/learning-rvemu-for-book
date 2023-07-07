use crate::dram::*;
use crate::exception::*;
use crate::param::*;

pub struct Bus {
    pub dram: Dram,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
        }
    }

    pub fn fetch(&self, addr: u64) -> Result<u8, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => Ok(self.dram.fetch(addr)),
            _ => Err(Exception::InstructionAccessFault(addr)),
        }
    }
    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }
    pub fn load_u(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.load_u(addr, size),
            _ => Err(Exception::LoadAccessFault(addr)),
        }
    }
}
