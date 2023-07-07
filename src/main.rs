use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_for_book::cpu::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut code = vec![];

    // read code into file
    file.read_to_end(&mut code)?;
    // init cpu with code
    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.dram.len() as u64 {
        // 1. Fetch
        let inst = cpu.fetch();

        // 2. PC update
        cpu.pc += 4;

        // 3. decode
        // 4. execute
        cpu.execute(inst)
    }
    cpu.dump_registers();

    Ok(())
}
