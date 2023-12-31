use std::{ops::Not, sync::Arc};

use rvemu_for_book::{self, param::*, utils::test_framework::TestFramework};

#[inline]
fn test_from_asm_snippet_with_auto_clock<'a>(
  code: &str,
  test_name: &str,
  cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
  let disable_auto_clock: Arc<[&str]> =
    Arc::new(["beq", "bne", "blt", "bge", "bltu", "bgeu", "jal", "jalr"]);
  let should_disable_auto_clock = || -> bool {
    if test_name
      .lines()
      .any(|line| disable_auto_clock.iter().any(|&inst| line.contains(inst)))
    {
      true
    } else {
      code
        .lines()
        .any(|line| disable_auto_clock.iter().any(|&inst| line.contains(inst)))
    }
  };
  let n_clock = if should_disable_auto_clock() {
    DRAM_END
  } else {
    code.lines().count() as u64
  };
  test_from_asm_snippet(code, test_name, n_clock, cmp_iter)
}

#[inline]
fn test_from_asm_snippet<'a>(
  code: &str,
  test_name: &str,
  n_clock: u64,
  cmp_iter: impl Iterator<Item = (&'a str, u64)>,
) {
  match TestFramework::test_from_asm(code, test_name, n_clock) {
    Ok(cpu) => cmp_iter.for_each(|(reg, expect)| {
      assert_eq!(cpu.observe_reg(reg), expect);
    }),
    Err(e) => {
      eprintln!("error: {}", e);
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
  test_from_asm_snippet_with_auto_clock(code, "test_add_addi", cmp_iter);
}

#[test]
fn test_add_with_neg() {
  let code = "
    addi x29, x0, -5
    addi x30, x0, 37
    add x31, x30, x29
  ";
  let cmp_iter = [("x31", 32)].into_iter();
  test_from_asm_snippet_with_auto_clock(code, "test_add_with_neg", cmp_iter);
}

#[test]
fn test_sub() {
  let code = "
    addi x29, x0, 5
    addi x30, x0, 37
    sub x31, x30, x29
  ";
  let cmp_iter = [("x31", 32)].into_iter();
  test_from_asm_snippet_with_auto_clock(code, "test_sub", cmp_iter);
}

#[test]
fn test_and() {
  let code = "
    addi x29, x0, 0b1010
    addi x30, x0, 0b1100
    and x31, x30, x29
  ";
  let cmp_iter = [("x31", 0b1000)].into_iter();
  test_from_asm_snippet_with_auto_clock(code, "test_and", cmp_iter);
}

#[test]
fn test_or() {
  let code = "
    addi x29, x0, 0b1010
    addi x30, x0, 0b1100
    or x31, x30, x29
  ";
  let cmp_iter = [("x31", 0b1110)].into_iter();
  test_from_asm_snippet_with_auto_clock(code, "test_or", cmp_iter);
}

#[test]
fn test_xor() {
  let code = "
    addi x29, x0, 0b1010
    addi x30, x0, 0b1100
    xor x31, x30, x29
  ";
  let cmp_iter = [("x31", 0b0110)].into_iter();
  test_from_asm_snippet_with_auto_clock(code, "test_xor", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sb_lb", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sh_lh", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sw_lw", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sd_ld", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sw_lw_with_negative", cmp_iter);
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
  test_from_asm_snippet_with_auto_clock(code, "test_sw_lwu_with_negative", cmp_iter);
}
