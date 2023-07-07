use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return Ok(());
    }
    let file = File::open(&args[1])?;
    rvemu_for_book::run_with(file)
}
