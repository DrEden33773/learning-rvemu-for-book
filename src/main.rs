use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rvemu_for_book::cpu::*;
use rvemu_for_book::param::*;

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

    while cpu.pc < DRAM_END {
        // 1. Fetch
        let inst = cpu.fetch().unwrap();

        // 2. PC update
        cpu.pc += 4;

        // 3. decode
        // 4. execute
        let if_impl = cpu.execute(inst);
        if !if_impl {
            break;
        }
    }
    cpu.dump_registers();

    Ok(())
}
