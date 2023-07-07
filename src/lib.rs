pub mod bus;
pub mod cpu;
pub mod dram;
pub mod param;

#[cfg(test)]
mod rvemu_test {
    use super::cpu::*;
    use super::param::*;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn test_add_addi() -> io::Result<()> {
        let mut file = File::open("add-addi.bin")?;
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
}
