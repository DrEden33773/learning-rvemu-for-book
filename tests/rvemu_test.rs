use std::{fs, ops::Not};

use rvemu_for_book::{self, TestTools};

// BUG: `code.lines().count()` only works if there is no `branch/jump/link` instruction.
// TODO: Dynamically detect the existence of `branch/jump/link` instruction.
// TODO: `n_clock = code.lines().count() if not [branch/jump/link] else DRAM_END`
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
    TestTools::step_into_temp_folder();
    match TestTools::rv_helper(code, test_name, n_clock) {
        Ok(cpu) => cmp_iter.for_each(|(reg, expect)| {
            assert_eq!(cpu.observe_reg(reg), expect);
        }),
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
    for suffix in ["", ".s", ".bin"] {
        fs::remove_file(
            std::env::current_dir()
                .unwrap()
                .join(format!("{test_name}{suffix}")),
        )
        .unwrap();
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
fn test_add_with_neg() {
    let code = "
        addi x29, x0, -5
        addi x30, x0, 37
        add x31, x30, x29
    ";
    let cmp_iter = [("x31", 32)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_add_with_neg", cmp_iter);
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

#[test]
fn test_and() {
    let code = "
        addi x29, x0, 0b1010
        addi x30, x0, 0b1100
        and x31, x30, x29
    ";
    let cmp_iter = [("x31", 0b1000)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_and", cmp_iter);
}

#[test]
fn test_or() {
    let code = "
        addi x29, x0, 0b1010
        addi x30, x0, 0b1100
        or x31, x30, x29
    ";
    let cmp_iter = [("x31", 0b1110)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_or", cmp_iter);
}

#[test]
fn test_xor() {
    let code = "
        addi x29, x0, 0b1010
        addi x30, x0, 0b1100
        xor x31, x30, x29
    ";
    let cmp_iter = [("x31", 0b0110)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_xor", cmp_iter);
}

#[test]
fn test_sb_lb() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, 0x10
        sb x30, 0(x29)
        lb x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x10)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sb_lb", cmp_iter);
}

#[test]
fn test_sh_lh() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, 0x100
        sh x30, 0(x29)
        lh x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x100)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sh_lh", cmp_iter);
}

#[test]
fn test_sw_lw() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, 0x200
        sw x30, 0(x29)
        lw x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x200)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sw_lw", cmp_iter);
}

#[test]
fn test_sd_ld() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, 0x200
        sd x30, 0(x29)
        ld x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x200)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sd_ld", cmp_iter);
}

#[test]
fn test_sw_lw_with_negative() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, -0x200
        sw x30, 0(x29)
        lw x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x200_u64.not().wrapping_add(1))].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sw_lw_with_negative", cmp_iter);
}

#[test]
fn test_sw_lwu_with_negative() {
    let code = "
        addi x29, x0, 0x100
        addi x30, x0, -0x200
        sw x30, 0(x29)
        lwu x31, 0(x29)
    ";
    let cmp_iter = [("x31", 0x200_u32.not().wrapping_add(1) as u64)].into_iter();
    run_from_asm_snippet_with_auto_clock(code, "test_sw_lwu_with_negative", cmp_iter);
}
