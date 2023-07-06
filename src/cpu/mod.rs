pub struct Cpu {
    pub reges: [u64; 32],
    pub pc: u64,
    pub dram: Vec<u8>,
}

impl Cpu {
    /// Create a new CPU with some existing codes
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            reges: [0; 32],
            pc: 0,
            dram: code,
        }
    }

    /// ## Fetch
    ///
    /// ### Read 32bit instruction from a memory
    ///
    /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
    pub fn fetch(&self) -> u32 {
        let curr_pc = self.pc as usize;
        (self.dram[curr_pc] as u32)
            | ((self.dram[curr_pc + 1] as u32) << 8)
            | ((self.dram[curr_pc + 2] as u32) << 16)
            | ((self.dram[curr_pc + 3] as u32) << 24)
    }

    /// ## execute
    ///
    /// ### Decode an instruction and execute it.
    ///
    /// ![RISC-V base instruction formats](https://book.rvemu.app/img/1-1-2.png)
    pub fn execute(&mut self, inst: u32) {
        /* 7bits opcode */
        let opcode = inst & 0x7f;
        /* 5bits rd */
        let rd = ((inst >> 7) & 0x1f) as usize;
        /* 5bits rf */
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        /* 5bits rf */
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        // TODO: Implement all instructions
        match opcode {
            0x13 => {
                /* addi */
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.reges[rd] = self.reges[rs1].wrapping_add(imm);
            }
            0x33 => {
                /* add */
                self.reges[rd] = self.reges[rs1].wrapping_add(self.reges[rs2]);
            }
            _ => {
                dbg!("{opcode} is not implemented yet!");
            }
        }
    }

    pub fn dump_registers(&self) {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];

        let mut output = String::new();
        let mut values = Vec::new();
        for i in (0..32).step_by(4) {
            values.push(format!(
                "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                i,
                abi[i],
                self.reges[i],
                i + 1,
                abi[i + 1],
                self.reges[i + 1],
                i + 2,
                abi[i + 2],
                self.reges[i + 2],
                i + 3,
                abi[i + 3],
                self.reges[i + 3],
            ));
        }
        output = values.join("\n");
        println!("{}", output);
    }
}
