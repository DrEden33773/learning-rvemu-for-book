use std::env;
use std::fs::File;
use std::io;
use std::ops::Shr;

#[allow(unused)]
fn black_board() -> io::Result<()> {
    let i: i64 = -3;
    println!("{}", i.shr(2));
    println!("{}", i.wrapping_shr(2));
    let i = i as u64;
    println!("{}", i.shr(2));
    println!("{}", i.wrapping_shr(2));
    Ok(())
}

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
    rvemu_for_book::run_with(file)
}

fn main() -> io::Result<()> {
    // black_board()
    run()
}
