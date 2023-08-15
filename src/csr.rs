use crate::param::*;

const NUM_CSRS: usize = 4096;

pub struct Csr {
  csrs: [u64; NUM_CSRS],
}

impl Csr {
  pub fn new() -> Csr {
    Self {
      csrs: [0; NUM_CSRS],
    }
  }

  pub fn load(&self, addr: usize) -> u64 {
    match addr {
      SIE => self.csrs[MIE] & self.csrs[MIDELEG],
      SIP => self.csrs[MIP] & self.csrs[MIDELEG],
      SSTATUS => self.csrs[MSTATUS] & MASK_SSTATUS,
      _ => self.csrs[addr],
    }
  }

  pub fn store(&mut self, addr: usize, value: u64) {
    match addr {
      SIE => self.csrs[MIE] = (self.csrs[MIE] & !self.csrs[MIDELEG]) | (value & self.csrs[MIDELEG]),
      SIP => self.csrs[MIP] = (self.csrs[MIE] & !self.csrs[MIDELEG]) | (value & self.csrs[MIDELEG]),
      SSTATUS => self.csrs[MSTATUS] = (self.csrs[MSTATUS] & !MASK_SSTATUS) | (value & MASK_SSTATUS),
      _ => self.csrs[addr] = value,
    }
  }
}

impl Default for Csr {
  fn default() -> Self {
    Self::new()
  }
}
