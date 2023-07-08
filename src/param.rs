/* DRAM Params */
pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;
pub const DRAM_END: u64 = DRAM_SIZE + DRAM_BASE - 1;

/* Branch Inst */
pub const BRANCH_OP: u32 = 0b1100011;
pub const BEQ: u32 = 0b000;
pub const BNE: u32 = 0b001;
pub const BLT: u32 = 0b100;
pub const BGE: u32 = 0b101;
pub const BLTU: u32 = 0b110;
pub const BGEU: u32 = 0b111;

/* Load Inst */
pub const LOAD_OP: u32 = 0b0000011;
pub const LB: u32 = 0b000;
pub const LH: u32 = 0b001;
pub const LW: u32 = 0b010;
pub const LD: u32 = 0b011;
pub const LBU: u32 = 0b100;
pub const LHU: u32 = 0b101;
pub const LWU: u32 = 0b110;

/* Store Inst */
pub const STORE_OP: u32 = 0b0100011;
pub const SB: u32 = 0b000;
pub const SH: u32 = 0b001;
pub const SW: u32 = 0b010;
pub const SD: u32 = 0b011;

/* RType Inst */
pub const R_TYPE_OP: u32 = 0b0110011;
pub const ADD_SUB: u32 = 0b000;
pub const SLL: u32 = 0b001;
pub const SLT: u32 = 0b010;
pub const SLTU: u32 = 0b011;
pub const XOR: u32 = 0b100;
pub const SRL_SRA: u32 = 0b101;
pub const OR: u32 = 0b110;
pub const AND: u32 = 0b111;

/* IType Inst */
pub const I_TYPE_OP: u32 = 0b0010011;
pub const ADDI: u32 = 0b000;
pub const SLTI: u32 = 0b010;
pub const SLTIU: u32 = 0b011;
pub const XORI: u32 = 0b100;
pub const ORI: u32 = 0b110;
pub const ANDI: u32 = 0b111;
pub const SLLI: u32 = 0b001;
pub const SRLI_SRAI: u32 = 0b101;

/* Machine-level CSRs. */
pub const MHARTID: usize = 0xF14;
/// Machine status register.
pub const MSTATUS: usize = 0x300;
/// Machine exception delefation register.
pub const MEDELEG: usize = 0x302;
/// Machine interrupt delefation register.
pub const MIDELEG: usize = 0x303;
/// Machine interrupt-enable register.
pub const MIE: usize = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: usize = 0x305;
/// Machine counter enable.
pub const MCOUNTEREN: usize = 0x306;
/// Scratch register for machine trap handlers.
pub const MSCRATCH: usize = 0x340;
/// Machine exception program counter.
pub const MEPC: usize = 0x341;
/// Machine trap cause.
pub const MCAUSE: usize = 0x342;
/// Machine bad address or instruction.
pub const MTVAL: usize = 0x343;
/// Machine interrupt pending.
pub const MIP: usize = 0x344;

/* Supervisor-level CSRs. */
/// Supervisor status register.
pub const SSTATUS: usize = 0x100;
/// Supervisor interrupt-enable register.
pub const SIE: usize = 0x104;
/// Supervisor trap handler base address.
pub const STVEC: usize = 0x105;
/// Scratch register for supervisor trap handlers.
pub const SSCRATCH: usize = 0x140;
/// Supervisor exception program counter.
pub const SEPC: usize = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: usize = 0x142;
/// Supervisor bad address or instruction.
pub const STVAL: usize = 0x143;
/// Supervisor interrupt pending.
pub const SIP: usize = 0x144;
/// Supervisor address translation and protection.
pub const SATP: usize = 0x180;

/* `mstatus` and `sstatus` field mask */
pub const MASK_SIE: u64 = 1 << 1;
pub const MASK_MIE: u64 = 1 << 3;
pub const MASK_SPIE: u64 = 1 << 5;
pub const MASK_UBE: u64 = 1 << 6;
pub const MASK_MPIE: u64 = 1 << 7;
pub const MASK_SPP: u64 = 1 << 8;
pub const MASK_VS: u64 = 0b11 << 9;
pub const MASK_MPP: u64 = 0b11 << 11;
pub const MASK_FS: u64 = 0b11 << 13;
pub const MASK_XS: u64 = 0b11 << 15;
pub const MASK_MPRV: u64 = 1 << 17;
pub const MASK_SUM: u64 = 1 << 18;
pub const MASK_MXR: u64 = 1 << 19;
pub const MASK_TVM: u64 = 1 << 20;
pub const MASK_TW: u64 = 1 << 21;
pub const MASK_TSR: u64 = 1 << 22;
pub const MASK_UXL: u64 = 0b11 << 32;
pub const MASK_SXL: u64 = 0b11 << 34;
pub const MASK_SBE: u64 = 1 << 36;
pub const MASK_MBE: u64 = 1 << 37;
pub const MASK_SD: u64 = 1 << 63;
pub const MASK_SSTATUS: u64 = MASK_SIE
    | MASK_SPIE
    | MASK_UBE
    | MASK_SPP
    | MASK_FS
    | MASK_XS
    | MASK_SUM
    | MASK_MXR
    | MASK_UXL
    | MASK_SD;

/* `MIP` / `SIP` field mask */
pub const MASK_SSIP: u64 = 1 << 1;
pub const MASK_MSIP: u64 = 1 << 3;
pub const MASK_STIP: u64 = 1 << 5;
pub const MASK_MTIP: u64 = 1 << 7;
pub const MASK_SEIP: u64 = 1 << 9;
pub const MASK_MEIP: u64 = 1 << 11;
