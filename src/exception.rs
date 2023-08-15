use std::fmt;

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

impl fmt::Display for Exception {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Exception::*;
    match self {
      InstructionAddrMisaligned(addr) => {
        write!(f, "InstructionAddrMisaligned: 0x{:016x}", addr)
      }
      InstructionAccessFault(addr) => write!(f, "InstructionAccessFault: 0x{:016x}", addr),
      IllegalInstruction(inst) => write!(f, "IllegalInstruction: 0x{:016x}", inst),
      Breakpoint(pc) => write!(f, "Breakpoint: 0x{:016x}", pc),
      LoadAccessMisaligned(addr) => write!(f, "LoadAccessMisaligned: 0x{:016x}", addr),
      LoadAccessFault(addr) => write!(f, "LoadAccessFault: 0x{:016x}", addr),
      StoreAMOAddrMisaligned(addr) => write!(f, "StoreAMOAddrMisaligned: 0x{:016x}", addr),
      StoreAMOAccessFault(addr) => write!(f, "StoreAMOAccessFault: 0x{:016x}", addr),
      EnvironmentCallFromUMode(pc) => write!(f, "EnvironmentCallFromUMode: 0x{:016x}", pc),
      EnvironmentCallFromSMode(pc) => write!(f, "EnvironmentCallFromSMode: 0x{:016x}", pc),
      EnvironmentCallFromMMode(pc) => write!(f, "EnvironmentCallFromMMode: 0x{:016x}", pc),
      InstructionPageFault(addr) => write!(f, "InstructionPageFault: 0x{:016x}", addr),
      LoadPageFault(addr) => write!(f, "LoadPageFault: 0x{:016x}", addr),
      StoreAMOPageFault(addr) => write!(f, "StoreAMOPageFault: 0x{:016x}", addr),
    }
  }
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
    matches!(
      self,
      InstructionAddrMisaligned(_)
        | InstructionAccessFault(_)
        | LoadAccessFault(_)
        | StoreAMOAddrMisaligned(_)
        | StoreAMOAccessFault(_)
        | IllegalInstruction(_)
    )
  }
}
