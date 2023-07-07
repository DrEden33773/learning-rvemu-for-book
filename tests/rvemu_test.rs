use rvemu_for_book::{self, TestBenchTools};

#[inline]
fn run_from_asm_snippet_with_auto_clock<'a>(
    code: &str,
    test_name: &str,
    cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
    run_from_asm_snippet(code, test_name, code.lines().count(), cmp_iter)
}

#[inline]
fn run_from_asm_snippet<'a>(
    code: &str,
    test_name: &str,
    n_clock: usize,
    cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
    let final_test_name = if !test_name.starts_with("my_rvemu_") {
        "my_rvemu_".to_owned() + test_name
    } else {
        test_name.to_owned()
    };
    match TestBenchTools::rv_helper(code, &final_test_name, n_clock) {
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
    run_from_asm_snippet_with_auto_clock(code, "test_add_addi", cmp_iter);
}

#[test]
fn test_sub() {
    let code = "
        addi x29, x0, 5
        addi x30, x0, 37
        sub x31, x30, x29
    ";
    let cmp_iter = [("x31", 32)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sub", cmp_iter);
}
