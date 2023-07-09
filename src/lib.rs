pub mod bus;
pub mod cpu;
pub mod csr;
pub mod dram;
pub mod emulator;
pub mod exception;
pub mod param;

use std::{fs::File, io::prelude::*, process::Command};

use cpu::*;

pub struct TestBenchTools;

impl TestBenchTools {
    pub fn step_into_temp_folder() {
        let temp_dir = project_root::get_project_root().unwrap().join("temp");
        if std::env::current_dir().unwrap() != temp_dir {
            if !temp_dir.exists() {
                std::fs::create_dir(&temp_dir).unwrap();
            }
            std::env::set_current_dir(&temp_dir).unwrap();
        }
    }
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
        let raw_message = String::from_utf8_lossy(&output.stderr);
        if raw_message.contains("error") {
            eprintln!("{}", raw_message);
        }
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
        let raw_message = String::from_utf8_lossy(&output.stderr);
        if raw_message.contains("error") {
            eprintln!("{}", raw_message);
        }
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
        let raw_message = String::from_utf8_lossy(&output.stderr);
        if raw_message.contains("error") {
            eprintln!("{}", raw_message);
        }
    }
    pub fn rv_helper(code: &str, test_name: &str, n_clock: usize) -> Result<Cpu, std::io::Error> {
        // eprintln!();

        let filename = test_name.to_owned() + ".s";
        let mut file = File::create(&filename)?;
        file.write_all(code.as_bytes())?;
        TestBenchTools::generate_rv_obj(&filename);
        TestBenchTools::generate_rv_binary(test_name);

        let mut file_bin = File::open(test_name.to_owned() + ".bin")?;
        let mut code = vec![];
        file_bin.read_to_end(&mut code)?;
        let mut cpu = Cpu::new(code);

        for _i in 0..n_clock {
            let inst = match cpu.fetch() {
                Ok(inst) => {
                    if inst == 0 {
                        // eprintln!("End of program\n");
                        return Ok(cpu);
                    }
                    inst
                }
                Err(e) => {
                    eprintln!("{e}\n");
                    break;
                }
            };
            match cpu.execute(inst) {
                Ok(new_pc) => cpu.pc = new_pc,
                Err(e) => {
                    eprintln!("{e}\n");
                    break;
                }
            };
        }

        Ok(cpu)
    }
}
