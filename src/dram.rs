use crate::exception::*;
use crate::param::*;

/// # Dram
///
/// ## Brief
///
/// `Main Memory` which contains `Instructions` and `Data`
///
/// ## Seeking Unit
///
/// - `Byte`
///
/// ## Definition
///
/// - 0 ..DRAM_BASE : `Instructions`
/// - DRAM_BASE ..= DRAM_END : `Data`
pub struct Dram {
    pub dram: Vec<u8>,
}

pub enum SizeType {
    /// 8-bit
    Byte,
    /// 16-bit
    Half,
    /// 32-bit
    Word,
    /// 64-bit
    DoubleWord,
}

impl SizeType {
    #[inline]
    pub fn how_many_bytes(&self) -> usize {
        match self {
            SizeType::Byte => 1,
            SizeType::Half => 2,
            SizeType::Word => 4,
            SizeType::DoubleWord => 8,
        }
    }
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Dram {
        let mut dram: Vec<u8> = vec![0; DRAM_BASE as usize + DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());
        Self { dram }
    }

    pub fn fetch_inst(&self, addr: u64) -> u8 {
        let index = (addr - DRAM_BASE) as usize;
        self.dram[index]
    }

    pub fn load(&self, addr: u64, size: SizeType) -> Result<u64, Exception> {
        let n_bytes = size.how_many_bytes();
        let index = addr as usize;
        let mut value = ((((self.dram[index + n_bytes - 1] as u64) << (8 * 7)) as i64)
            >> (8 * (8 - n_bytes))) as u64;
        for i in 0..n_bytes {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        Ok(value)
    }
    pub fn load_u(&self, addr: u64, size: SizeType) -> Result<u64, Exception> {
        let n_bytes = size.how_many_bytes();
        let index = addr as usize;
        let mut value: u64 = 0;
        for i in 0..n_bytes {
            value |= (self.dram[index + i] as u64) << (8 * i);
        }
        Ok(value)
    }

    pub fn store(&mut self, addr: u64, size: SizeType, value: u64) -> Result<(), Exception> {
        let n_bytes = size.how_many_bytes();
        let index = addr as usize;
        for i in 0..n_bytes {
            self.dram[index + i] = ((value >> (8 * i)) & 0xff) as u8;
        }
        Ok(())
    }
}
