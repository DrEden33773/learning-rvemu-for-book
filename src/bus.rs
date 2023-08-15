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

  pub fn fetch_inst(&self, addr: u64) -> Result<u8, Exception> {
    match addr {
      DRAM_BASE..=DRAM_END => Ok(self.dram.fetch_inst(addr)),
      _ => Err(Exception::InstructionAccessFault(addr)),
    }
  }

  pub fn load(&mut self, addr: u64, size: SizeType) -> Result<u64, Exception> {
    let addr = addr + DRAM_BASE;
    match addr {
      DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
      _ => Err(Exception::LoadAccessFault(addr)),
    }
  }
  pub fn store(&mut self, addr: u64, size: SizeType, value: u64) -> Result<(), Exception> {
    let addr = addr + DRAM_BASE;
    match addr {
      DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
      _ => Err(Exception::StoreAMOAccessFault(addr)),
    }
  }
  pub fn load_u(&mut self, addr: u64, size: SizeType) -> Result<u64, Exception> {
    let addr = addr + DRAM_BASE;
    match addr {
      DRAM_BASE..=DRAM_END => self.dram.load_u(addr, size),
      _ => Err(Exception::LoadAccessFault(addr)),
    }
  }
}
