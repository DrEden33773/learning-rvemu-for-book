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
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(()),
        }
    }
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        match size {
            8 => {
                self.store8(addr, value);
                Ok(())
            }
            16 => {
                self.store16(addr, value);
                Ok(())
            }
            32 => {
                self.store32(addr, value);
                Ok(())
            }
            64 => {
                self.store64(addr, value);
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn load8n(&self, addr: u64, n: usize) -> u64 {
        let index = (addr - DRAM_BASE) as usize;
        let mut value = 0;
        for i in 0..n {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        value
    }
    #[inline]
    fn load8(&self, addr: u64) -> u64 {
        Dram::load8n(self, addr, 1)
    }
    #[inline]
    fn load16(&self, addr: u64) -> u64 {
        Dram::load8n(self, addr, 2)
    }
    #[inline]
    fn load32(&self, addr: u64) -> u64 {
        Dram::load8n(self, addr, 4)
    }
    #[inline]
    fn load64(&self, addr: u64) -> u64 {
        Dram::load8n(self, addr, 8)
    }

    fn store8n(&mut self, addr: u64, value: u64, n: usize) {
        let index = (addr - DRAM_BASE) as usize;
        for i in 0..n {
            self.dram[index + i] = ((value >> (8 * i)) & 0xff) as u8;
        }
    }
    #[inline]
    fn store8(&mut self, addr: u64, value: u64) {
        Dram::store8n(self, addr, value, 1)
    }
    #[inline]
    fn store16(&mut self, addr: u64, value: u64) {
        Dram::store8n(self, addr, value, 2)
    }
    #[inline]
    fn store32(&mut self, addr: u64, value: u64) {
        Dram::store8n(self, addr, value, 4)
    }
    #[inline]
    fn store64(&mut self, addr: u64, value: u64) {
        Dram::store8n(self, addr, value, 8)
    }
}
