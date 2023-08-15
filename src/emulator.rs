use std::{
  fs::File,
  io::{self, prelude::*},
};

use crate::cpu::Cpu;
use crate::param::*;

pub fn run_with(mut file: File) -> io::Result<()> {
  eprintln!();

  let mut code = vec![];
  file.read_to_end(&mut code)?;

  let mut cpu = Cpu::new(code);

  while cpu.pc < DRAM_END {
    let inst = match cpu.fetch() {
      Ok(inst) => {
        if inst == 0 {
          eprintln!("End of program\n");
          break;
        };
        inst
      }
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
