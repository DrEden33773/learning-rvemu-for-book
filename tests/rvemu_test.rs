use rvemu_for_book::{self, TestBenchTools};

macro_rules! riscv_test {
    ($code:expr, $name:expr, $clock:expr, $($real:expr => $expect:expr), *) => {
        match TestBenchTools::rv_helper($code, $name, $clock) {
            Ok(cpu) => {
                $(assert_eq!(cpu.observe_reg($real), $expect);)*
            }
            Err(e) => { eprintln!("error: {}", e); assert!(false); }
        }
    };
}

#[test]
fn test_add_addi() {
    let code = "
        addi x29, x0, 5
        addi x30, x0, 37
        add x31, x30, x29
    ";
    riscv_test!(code, "rvemu_test_add_addi", code.lines().count(), "x31" => 42);
}

#[test]
fn test_sub() {
    let code = "
        addi x29, x0, 5
        addi x30, x0, 37
        sub x31, x30, x29
    ";
    riscv_test!(code, "rvemu_test_sub", code.lines().count(), "x31" => 32);
}
