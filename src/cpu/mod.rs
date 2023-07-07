use crate::bus::*;
use crate::param::*;

/// RISC-V CPU
///
/// - Little-endian
/// - 64bit
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
    pub fn fetch(&self) -> Result<u32, ()> {
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
    pub fn execute(&mut self, inst: u32) -> bool {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        const ADDI_OP: u32 = 0x13;
        const ADD_OP: u32 = 0x33;

        // TODO: Implement all instructions
        match opcode {
            ADDI_OP => {
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.gpr[rd] = self.gpr[rs1].wrapping_add(imm);
                true
            }
            ADD_OP => {
                self.gpr[rd] = self.gpr[rs1].wrapping_add(self.gpr[rs2]);
                true
            }
            _ => {
                // dbg!("{} is not implemented yet!", opcode);
                false
            }
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
        eprintln!("\n{}", output);
    }
}
