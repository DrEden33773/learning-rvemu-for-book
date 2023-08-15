use crate::bus::*;
use crate::csr::*;
use crate::dram::SizeType;
use crate::exception::*;
use crate::param::*;

const ABI: [&str; 32] = [
  "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ", " a1 ",
  " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ", " s6 ", " s7 ",
  " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
];

/// RISC-V CPU
///
/// - Little-Endian
/// - 64-bit
pub struct Cpu {
  pub gpr: [u64; 32],
  pub pc: u64,
  pub bus: Bus,
  pub csr: Csr,
}

impl Cpu {
  /// Create a new CPU with some existing codes
  pub fn new(code: Vec<u8>) -> Self {
    let mut gpr = [0; 32];
    gpr[2] = DRAM_END;
    Self {
      gpr,
      pc: DRAM_BASE,
      bus: Bus::new(code),
      csr: Csr::default(),
    }
  }

  /// Read 32bit instruction from a memory
  ///
  /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
  pub fn fetch(&self) -> Result<u32, Exception> {
    let curr_pc = self.pc;
    let curr_code = (self.bus.fetch_inst(curr_pc)? as u32)
      | ((self.bus.fetch_inst(curr_pc + 1)? as u32) << 8)
      | ((self.bus.fetch_inst(curr_pc + 2)? as u32) << 16)
      | ((self.bus.fetch_inst(curr_pc + 3)? as u32) << 24);
    Ok(curr_code)
  }

  /// Decode an instruction and execute it.
  ///
  /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
  pub fn execute(&mut self, inst: u32) -> Result<u64, Exception> {
    let opcode = inst & 0x7F;
    let rd = ((inst >> 7) & 0x1F) as usize;
    let rs1 = ((inst >> 15) & 0x1F) as usize;
    let rs2 = ((inst >> 20) & 0x1F) as usize;
    let funct3 = (inst >> 12) & 0x7;
    let funct7 = (inst >> 25) & 0x7F;

    // TODO: Implement `RV32I` & `RV64I`
    match opcode {
      BRANCH_OP => {
        let _imm_12 = (inst & 0x8000_0000) as i32 >> 31;
        let _imm_11 = (inst & 0x80) as i32 >> 7;
        let _imm_10_5 = (inst & 0x7E00_0000) as i32 >> 25;
        let _imm_4_1 = (inst & 0xf00) as i32 >> 8;
        let imm =
          ((_imm_12 << 12) | (_imm_11 << 11) | (_imm_10_5 << 5) | (_imm_4_1 << 1) | _imm_11) as i64;
        let if_jump = match funct3 {
          BEQ => self.gpr[rs1] == self.gpr[rs2],
          BNE => self.gpr[rs1] != self.gpr[rs2],
          BLT => (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64),
          BGE => (self.gpr[rs1] as i64) >= (self.gpr[rs2] as i64),
          BLTU => self.gpr[rs1] < self.gpr[rs2],
          BGEU => self.gpr[rs1] >= self.gpr[rs2],
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        let next_pc = if if_jump {
          (self.pc as i64).wrapping_add(imm) as u64
        } else {
          self.pc + 4
        };
        Ok(next_pc)
      }
      LOAD_OP => {
        let imm = ((inst & 0xFFF0_0000) as i32 >> 20) as i64;
        let addr = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;
        let value = match funct3 {
          LB => self.bus.load(addr, SizeType::Byte)?,
          LH => self.bus.load(addr, SizeType::Half)?,
          LW => self.bus.load(addr, SizeType::Word)?,
          LD => self.bus.load(addr, SizeType::DoubleWord)?,
          LBU => self.bus.load_u(addr, SizeType::Byte)?,
          LHU => self.bus.load_u(addr, SizeType::Half)?,
          LWU => self.bus.load_u(addr, SizeType::Word)?,
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        self.gpr[rd] = value;
        Ok(self.pc + 4)
      }
      STORE_OP => {
        let _imm_11_5 = (inst & 0xFE00_0000) as i32 >> 25;
        let _imm_4_0 = (inst & 0xF80) as i32 >> 7;
        let imm = ((_imm_11_5 << 5) | _imm_4_0) as i64;
        let addr = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;
        let value = self.gpr[rs2];
        match funct3 {
          SB => self.bus.store(addr, SizeType::Byte, value)?,
          SH => self.bus.store(addr, SizeType::Half, value)?,
          SW => self.bus.store(addr, SizeType::Word, value)?,
          SD => self.bus.store(addr, SizeType::DoubleWord, value)?,
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        Ok(self.pc + 4)
      }
      R_TYPE_OP => {
        let result = match funct3 {
          ADD_SUB => {
            if funct7 == 0 {
              (self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64)
            } else {
              (self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64)
            }
          }
          SLL => {
            let shamt = self.gpr[rs2] & 0x3F;
            (self.gpr[rs1]).wrapping_shl(shamt as u32) as i64
          }
          SRL_SRA => {
            let shamt = self.gpr[rs2] & 0x3F;
            if funct7 == 0 {
              (self.gpr[rs1]).wrapping_shr(shamt as u32) as i64
            } else {
              (self.gpr[rs1] as i64).wrapping_shr(shamt as u32)
            }
          }
          SLT => ((self.gpr[rs1] as i64) < (self.gpr[rs2] as i64)) as i64,
          SLTU => (self.gpr[rs1] < self.gpr[rs2]) as i64,
          XOR => (self.gpr[rs1] ^ self.gpr[rs2]) as i64,
          OR => (self.gpr[rs1] | self.gpr[rs2]) as i64,
          AND => (self.gpr[rs1] & self.gpr[rs2]) as i64,
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        self.gpr[rd] = result as u64;
        Ok(self.pc + 4)
      }
      R_W_TYPE_OP => {
        let result = match funct3 {
          ADDW_SUBW => {
            if funct7 == 0 {
              (self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64)
            } else {
              (self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64)
            }
          }
          SLLW => {
            let shamt = self.gpr[rs2] & 0x3F;
            ((self.gpr[rs1]) << shamt) as i64
          }
          SRLW_SRAW => {
            let shamt = self.gpr[rs2] & 0x3F;
            if funct7 == 0 {
              ((self.gpr[rs1]) >> shamt) as i64
            } else {
              self.gpr[rs1] as i64 >> shamt
            }
          }
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        self.gpr[rd] = result as u64;
        Ok(self.pc + 4)
      }
      I_TYPE_OP => {
        let imm = ((inst & 0xFFF0_0000) as i32 >> 20) as i64;
        let shamt = (imm & 0x3F) as u32;
        let result = match funct3 {
          ADDI => (self.gpr[rs1] as i64).wrapping_add(imm),
          SLTI => ((self.gpr[rs1] as i64) < imm) as i64,
          SLTIU => (self.gpr[rs1] < imm as u64) as i64,
          XORI => (self.gpr[rs1] ^ imm as u64) as i64,
          ORI => (self.gpr[rs1] | imm as u64) as i64,
          ANDI => (self.gpr[rs1] & imm as u64) as i64,
          SLLI => (self.gpr[rs1]).wrapping_shl(shamt) as i64,
          SRLI_SRAI => {
            if funct7 == 0 {
              (self.gpr[rs1]).wrapping_shr(shamt) as i64
            } else {
              (self.gpr[rs1] as i64).wrapping_shr(shamt)
            }
          }
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        self.gpr[rd] = result as u64;
        Ok(self.pc + 4)
      }
      I_W_TYPE_OP => {
        let imm = ((inst & 0xFFF0_0000) as i32 >> 20) as i64;
        let shamt = (imm & 0x3F) as u32;
        let result = match funct3 {
          ADDIW => (self.gpr[rs1] as i64).wrapping_add(imm),
          SLLIW => (self.gpr[rs1]).wrapping_shl(shamt) as i64,
          SRLIW_SRAIW => {
            if funct7 == 0 {
              (self.gpr[rs1]).wrapping_shr(shamt) as i64
            } else {
              (self.gpr[rs1] as i64).wrapping_shr(shamt)
            }
          }
          _ => return Err(Exception::IllegalInstruction(inst as u64)),
        };
        self.gpr[rd] = result as u64;
        Ok(self.pc + 4)
      }
      _ => Err(Exception::IllegalInstruction(inst as u64)),
    }
  }

  /// Dump all registers onto the screen
  pub fn dump_registers(&self) {
    let mut values = Vec::new();
    for i in (0..32).step_by(4) {
      values.push(format!(
        "x{:02}({})={:>#18x}  x{:02}({})={:>#18x}  x{:02}({})={:>#18x}  x{:02}({})={:>#18x}",
        i,
        ABI[i],
        self.gpr[i],
        i + 1,
        ABI[i + 1],
        self.gpr[i + 1],
        i + 2,
        ABI[i + 2],
        self.gpr[i + 2],
        i + 3,
        ABI[i + 3],
        self.gpr[i + 3],
      ));
    }
    let output = values.join("\n");
    eprintln!("{}", output);
  }

  pub fn observe_reg(&self, r: &str) -> u64 {
    match ABI.iter().position(|&x| x == r) {
      Some(i) => self.gpr[i],
      None => match r {
        "pc" => self.pc,
        "fp" => self.observe_reg("s0"),
        r if r.starts_with('x') => {
          if let Ok(i) = r[1..].parse::<usize>() {
            if i <= 31 {
              return self.gpr[i];
            }
            panic!("Invalid register {r}");
          }
          panic!("Invalid register {r}");
        }
        _ => panic!("Invalid register {r}"),
      },
    }
  }
}
