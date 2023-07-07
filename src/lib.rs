pub mod bus;
pub mod cpu;
pub mod csr;
pub mod dram;
pub mod exception;
pub mod param;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::Command;

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

pub struct TestBenchGenTools;

impl TestBenchGenTools {
    pub fn generate_rv_assembly(c_src: &str) {
        let cc = "clang";
        let output = Command::new(cc)
            .arg("-S")
            .arg(c_src)
            .arg("-nostdlib")
            .arg("-march=rv64g")
            .arg("-mabi=lp64")
            .arg("--target=riscv64")
            .arg("-mno-relax")
            .output()
            .expect("Failed to generate rv assembly");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    pub fn generate_rv_obj(assembly: &str) {
        let cc = "clang";
        let pieces: Vec<&str> = assembly.split('.').collect();
        let output = Command::new(cc)
            .arg("-Wl,-Ttext=0x0")
            .arg("-nostdlib")
            .arg("-march=rv64g")
            .arg("-mabi=lp64")
            .arg("--target=riscv64")
            .arg("-mno-relax")
            .arg("-o")
            .arg(pieces[0])
            .arg(assembly)
            .output()
            .expect("Failed to generate rv object");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    pub fn generate_rv_binary(obj: &str) {
        let objcopy = "llvm-objcopy";
        let output = Command::new(objcopy)
            .arg("-O")
            .arg("binary")
            .arg(obj)
            .arg(obj.to_owned() + ".bin")
            .output()
            .expect("Failed to generate rv binary");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(test)]
mod rvemu_test {
    use super::*;

    #[test]
    fn add_addi() -> io::Result<()> {
        run_with(File::open("asm/add-addi.bin")?)
    }

    #[test]
    fn sub() -> io::Result<()> {
        run_with(File::open("asm/sub.bin")?)
    }
}
