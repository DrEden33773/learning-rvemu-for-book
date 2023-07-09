use std::env;
use std::fs::File;
use std::io;

#[inline]
fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return Ok(());
    }
    let file = File::open(&args[1])?;
    rvemu_for_book::emulator::run_with(file)
}

fn main() -> io::Result<()> {
    run()
}
