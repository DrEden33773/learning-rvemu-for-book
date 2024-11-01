# Learning & Building `rvemu` (Rust RISC-V Emulator)

## Brief

This project is a RISC-V emulator written in Rust.

With [rvemu-for-book](https://github.com/d0iasm/rvemu-for-book), [rvemu: RISC-V Emulator](https://github.com/d0iasm/rvemu) and [rare](https://siriusdemon.github.io/Rare/) as `reference`, it becomes `not that tough` for me to rapidly implement this project.

## More Information

1. An `little-endian`, `64-bit` RISC-V emulator
2. Support `RV32I(basic)` & `RV64I(basic)` instruction set
3. Won't support `pipeline-model`, as this is `nothing more than an emulator`

## Requirements

Here's necessary requirements to run this project:

1. `rust` toolchain (at least support `rust-2021-edition`)
2. `llvm` toolchain (at least contains `clang, lld` which are in support of `riscv64` arch)
