use rvemu_for_book::{self, TestBenchTools};

#[inline]
fn run_rv_test_with_auto_clock<'a>(
    code: &str,
    test_name: &str,
    cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
    run_rv_test(code, test_name, code.lines().count(), cmp_iter)
}

#[inline]
fn run_rv_test<'a>(
    code: &str,
    test_name: &str,
    n_clock: usize,
    cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
    match TestBenchTools::rv_helper(code, test_name, n_clock) {
        Ok(cpu) => cmp_iter.for_each(|(reg, expect)| {
            assert_eq!(cpu.observe_reg(reg), expect);
        }),
        Err(e) => {
            eprintln!("error: {}", e);
            unreachable!()
        }
    }
}

#[test]
fn test_add_addi() {
    let code = "
        addi x29, x0, 5
        addi x30, x0, 37
        add x31, x30, x29
    ";
    let cmp_iter = [("x31", 42)].into_iter();
    run_rv_test_with_auto_clock(code, "rvemu_test_add_addi", cmp_iter);
}

#[test]
fn test_sub() {
    let code = "
        addi x29, x0, 5
        addi x30, x0, 37
        sub x31, x30, x29
    ";
    let cmp_iter = [("x31", 32)].into_iter();
    run_rv_test_with_auto_clock(code, "rvemu_test_sub", cmp_iter);
}
