#[derive(Debug, Copy, Clone)]
pub enum Exception {
    InstructionAddrMisaligned(u64),
    InstructionAccessFault(u64),
    IllegalInstruction(u64),
    Breakpoint(u64),
    LoadAccessMisaligned(u64),
    LoadAccessFault(u64),
    StoreAMOAddrMisaligned(u64),
    StoreAMOAccessFault(u64),
    EnvironmentCallFromUMode(u64),
    EnvironmentCallFromSMode(u64),
    EnvironmentCallFromMMode(u64),
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StoreAMOPageFault(u64),
}

impl Exception {
    pub fn value(self) -> u64 {
        use Exception::*;
        match self {
            InstructionAddrMisaligned(addr) => addr,
            InstructionAccessFault(addr) => addr,
            IllegalInstruction(inst) => inst,
            Breakpoint(pc) => pc,
            LoadAccessMisaligned(addr) => addr,
            LoadAccessFault(addr) => addr,
            StoreAMOAddrMisaligned(addr) => addr,
            StoreAMOAccessFault(addr) => addr,
            EnvironmentCallFromUMode(pc) => pc,
            EnvironmentCallFromSMode(pc) => pc,
            EnvironmentCallFromMMode(pc) => pc,
            InstructionPageFault(addr) => addr,
            LoadPageFault(addr) => addr,
            StoreAMOPageFault(addr) => addr,
        }
    }

    pub fn code(self) -> u64 {
        use Exception::*;
        match self {
            InstructionAddrMisaligned(_) => 0,
            InstructionAccessFault(_) => 1,
            IllegalInstruction(_) => 2,
            Breakpoint(_) => 3,
            LoadAccessMisaligned(_) => 4,
            LoadAccessFault(_) => 5,
            StoreAMOAddrMisaligned(_) => 6,
            StoreAMOAccessFault(_) => 7,
            EnvironmentCallFromUMode(_) => 8,
            EnvironmentCallFromSMode(_) => 9,
            EnvironmentCallFromMMode(_) => 11,
            InstructionPageFault(_) => 12,
            LoadPageFault(_) => 13,
            StoreAMOPageFault(_) => 15,
        }
    }

    pub fn is_fatal(self) -> bool {
        use Exception::*;
        match self {
            InstructionAddrMisaligned(_)
            | InstructionAccessFault(_)
            | LoadAccessFault(_)
            | StoreAMOAddrMisaligned(_)
            | StoreAMOAccessFault(_)
            | IllegalInstruction(_) => true,
            _else => false,
        }
    }
}
