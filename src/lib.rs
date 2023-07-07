pub mod exception;
pub mod bus;
pub mod cpu;
pub mod dram;
pub mod param;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use cpu::*;
use param::*;

pub fn run_with(mut file: File) -> io::Result<()> {
    let mut code = vec![];

    // read code into file
    file.read_to_end(&mut code)?;
    // init cpu with code
    let mut cpu = Cpu::new(code);

    while cpu.pc < DRAM_END {
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(_e) => {
                // eprintln!("{_e}");
                break;
            }
        };
        match cpu.execute(inst) {
            Ok(new_pc) => cpu.pc = new_pc,
            Err(_e) => {
                // eprintln!("{_e}");
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
    fn test_add_addi() -> io::Result<()> {
        let file = File::open("add-addi.bin")?;
        run_with(file)
    }
}
