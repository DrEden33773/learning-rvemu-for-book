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
