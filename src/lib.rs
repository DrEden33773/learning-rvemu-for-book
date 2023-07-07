pub mod bus;
pub mod cpu;
pub mod dram;
pub mod exception;
pub mod param;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use cpu::*;
use param::*;

pub fn run_with(mut file: File) -> io::Result<()> {
    eprintln!();

    let mut code = vec![];
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);
    while cpu.pc < DRAM_END {
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };
        match cpu.execute(inst) {
            Ok(new_pc) => cpu.pc = new_pc,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };
    }

    cpu.dump_registers();
    Ok(())
}

#[cfg(test)]
mod rvemu_test {
    use super::*;

    #[test]
    fn add_addi() -> io::Result<()> {
        let file = File::open("asm/add-addi.bin")?;
        run_with(file)
    }
}
