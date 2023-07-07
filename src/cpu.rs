use crate::bus::*;
use crate::exception::*;
use crate::param::*;

/// RISC-V CPU
///
/// - Little-Endian
/// - 64-bit
pub struct Cpu {
    pub gpr: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    /// Create a new CPU with some existing codes
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            gpr: [0; 32],
            pc: DRAM_BASE,
            bus: Bus::new(code),
        }
    }

    /// Read 32bit instruction from a memory
    ///
    /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
    pub fn fetch(&self) -> Result<u32, Exception> {
        let curr_pc = self.pc;
        let curr_code = (self.bus.fetch(curr_pc)? as u32)
            | ((self.bus.fetch(curr_pc + 1)? as u32) << 8)
            | ((self.bus.fetch(curr_pc + 2)? as u32) << 16)
            | ((self.bus.fetch(curr_pc + 3)? as u32) << 24);
        Ok(curr_code)
    }

    /// Decode an instruction and execute it.
    ///
    /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
    pub fn execute(&mut self, inst: u32) -> Result<u64, Exception> {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        /* Branch */
        const BRANCH_OP: u32 = 0b1100011;
        const BEQ: u32 = 0b000;
        const BNE: u32 = 0b001;
        const BLT: u32 = 0b100;
        const BGE: u32 = 0b101;
        const BLTU: u32 = 0b110;
        const BGEU: u32 = 0b111;

        /* Load */
        const LOAD_OP: u32 = 0b0000011;
        const LB: u32 = 0b000;
        const LH: u32 = 0b001;
        const LW: u32 = 0b010;
        const LD: u32 = 0b011;
        const LBU: u32 = 0b100;
        const LHU: u32 = 0b101;
        const LWU: u32 = 0b110;

        /* Store */
        const STORE_OP: u32 = 0b0100011;
        const SB: u32 = 0b000;
        const SH: u32 = 0b001;
        const SW: u32 = 0b010;
        const SD: u32 = 0b011;

        /* RType */
        const R_TYPE_OP: u32 = 0b0110011;
        const ADD_SUB: u32 = 0b000;
        const SLL: u32 = 0b001;
        const SLT: u32 = 0b010;
        const SLTU: u32 = 0b011;
        const XOR: u32 = 0b100;
        const SRL_SRA: u32 = 0b101;
        const OR: u32 = 0b110;
        const AND: u32 = 0b111;

        /* IType */
        const I_TYPE_OP: u32 = 0b0010011;
        const ADDI: u32 = 0b000;
        const SLTI: u32 = 0b010;
        const SLTIU: u32 = 0b011;
        const XORI: u32 = 0b100;
        const ORI: u32 = 0b110;
        const ANDI: u32 = 0b111;
        const SLLI: u32 = 0b001;
        const SRLI_SRAI: u32 = 0b101;

        // TODO: Implement `RV32I` & `RV64I`
        match opcode {
            BRANCH_OP => {
                let _imm_12 = (inst >> 31) & 1;
                let _imm_11 = (inst >> 7) & 1;
                let _imm_10_5 = (inst >> 25) & 0x3f;
                let _imm_4_1 = (inst >> 8) & 0xf;
                let imm = ((_imm_12 << 12)
                    | (_imm_11 << 11)
                    | (_imm_10_5 << 5)
                    | (_imm_4_1 << 1)
                    | _imm_11) as i32 as i64;
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
                let imm = ((inst >> 20) & 0xfff) as i32 as i64 as u64;
                let addr = self.gpr[rs1].wrapping_add(imm);
                let value = match funct3 {
                    LB => self.bus.load(addr, 8)?,
                    LH => self.bus.load(addr, 16)?,
                    LW => self.bus.load(addr, 32)?,
                    LD => self.bus.load(addr, 64)?,
                    LBU => self.bus.load_u(addr, 8)?,
                    LHU => self.bus.load_u(addr, 16)?,
                    LWU => self.bus.load_u(addr, 32)?,
                    _ => return Err(Exception::IllegalInstruction(inst as u64)),
                };
                self.gpr[rd] = value;
                Ok(self.pc + 4)
            }
            STORE_OP => {
                let _imm_11_5 = (inst >> 25) & 0x7f;
                let _imm_4_0 = (inst >> 7) & 0x1f;
                let imm = ((_imm_11_5 << 5) | _imm_4_0) as u64;
                let addr = self.gpr[rs1].wrapping_add(imm);
                let value = self.gpr[rs2];
                match funct3 {
                    SB => self.bus.store(addr, value, 8)?,
                    SH => self.bus.store(addr, value, 16)?,
                    SW => self.bus.store(addr, value, 32)?,
                    SD => self.bus.store(addr, value, 64)?,
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
                        let shamt = self.gpr[rs2] & 0x3f;
                        (self.gpr[rs1]).wrapping_shl(shamt as u32) as i64
                    }
                    SRL_SRA => {
                        let shamt = self.gpr[rs2] & 0x3f;
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
            I_TYPE_OP => {
                let imm = ((inst >> 20) & 0xfff) as i32 as i64;
                let shamt = (imm & 0x3f) as u32;
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
            _ => Err(Exception::IllegalInstruction(inst as u64)),
        }
    }

    /// Dump all registers onto the screen
    pub fn dump_registers(&self) {
        const ABI: [&str; 32] = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
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
        eprintln!("\n{}\n", output);
    }
}