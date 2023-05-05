#[derive(Debug, Clone, Copy)]
pub enum Addressing {
  Accumulator, Absolute, AbsoluteX, AbsoluteY, Immediate, Implied, Indirect,
  IndirectX, IndirectY, Relative, ZeroPage, ZeroPageX, ZeroPageY,
}

#[derive(Debug)]
pub enum OpCode {
  // Official
  ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL,
  BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX, CPY,
  DEC, DEX, DEY, EOR, INC, INX, INY, JAM, JMP, JSR,
  LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP,
  ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX,
  STY, TAX, TAY, TSX, TXA, TXS, TYA,

  // Unofficial
  XALR, XANC, XANE, XARR, XDCP, XISC, XLAS,
  XLAX, XLXA, XNOP, XRLA, XRRA, XSAX, XSBC,
  XSBX, XSHA, XSHX, XSHY, XSLO, XSRE, XTAS,
}

#[derive(Debug, Clone, Copy)]
pub struct OperandAddress(pub u16, pub Addressing, pub bool); // (address, source, page crossed)

#[derive(Debug, Clone, Copy)]
pub struct Operand(pub OperandAddress, pub u8); // (operand address, data)

pub struct Instruction {
  pub mode: Addressing,
  pub opcode: OpCode,
  pub cycles: u8,
  pub extra: u8,
}

impl Instruction {
  fn new(mode: Addressing, opcode: OpCode, cycles: u8, extra: u8) -> Self {
    Instruction {
      mode,
      opcode,
      cycles,
      extra,
    }
  }

  pub fn get(code: u8) -> Self {
    match code {
      // ADC
      0x69 => Instruction::new(Addressing::Immediate, OpCode::ADC, 2, 0),
      0x65 => Instruction::new(Addressing::ZeroPage, OpCode::ADC, 3, 0),
      0x75 => Instruction::new(Addressing::ZeroPageX, OpCode::ADC, 4, 0),
      0x6D => Instruction::new(Addressing::Absolute, OpCode::ADC, 4, 0),
      0x7D => Instruction::new(Addressing::AbsoluteX, OpCode::ADC, 4, 1),
      0x79 => Instruction::new(Addressing::AbsoluteY, OpCode::ADC, 4, 1),
      0x61 => Instruction::new(Addressing::IndirectX, OpCode::ADC, 6, 0),
      0x71 => Instruction::new(Addressing::IndirectY, OpCode::ADC, 5, 1),

      // *ALR
      0x4B => Instruction::new(Addressing::Immediate, OpCode::XALR, 2, 0),

      // *ANC
      0x0B => Instruction::new(Addressing::Immediate, OpCode::XANC, 2, 0),
      0x2B => Instruction::new(Addressing::Immediate, OpCode::XANC, 2, 0),

      // AND
      0x29 => Instruction::new(Addressing::Immediate, OpCode::AND, 2, 0),
      0x25 => Instruction::new(Addressing::ZeroPage, OpCode::AND, 3, 0),
      0x35 => Instruction::new(Addressing::ZeroPageX, OpCode::AND, 4, 0),
      0x2D => Instruction::new(Addressing::Absolute, OpCode::AND, 4, 0),
      0x3D => Instruction::new(Addressing::AbsoluteX, OpCode::AND, 4, 1),
      0x39 => Instruction::new(Addressing::AbsoluteY, OpCode::AND, 4, 1),
      0x21 => Instruction::new(Addressing::IndirectX, OpCode::AND, 6, 0),
      0x31 => Instruction::new(Addressing::IndirectY, OpCode::AND, 5, 1),

      // *ANE
      0x8B => Instruction::new(Addressing::Immediate, OpCode::XANE, 2, 0),

      // *ARR
      0x6B => Instruction::new(Addressing::Immediate, OpCode::XARR, 2, 0),

      // ASL
      0x0A => Instruction::new(Addressing::Accumulator, OpCode::ASL, 2, 0),
      0x06 => Instruction::new(Addressing::ZeroPage, OpCode::ASL, 5, 0),
      0x16 => Instruction::new(Addressing::ZeroPageX, OpCode::ASL, 6, 0),
      0x0E => Instruction::new(Addressing::Absolute, OpCode::ASL, 6, 0),
      0x1E => Instruction::new(Addressing::AbsoluteX, OpCode::ASL, 7, 0),

      // BCC
      0x90 => Instruction::new(Addressing::Relative, OpCode::BCC, 2, 1),

      // BCS
      0xB0 => Instruction::new(Addressing::Relative, OpCode::BCS, 2, 1),

      // BEQ
      0xF0 => Instruction::new(Addressing::Relative, OpCode::BEQ, 2, 1),

      // BIT
      0x24 => Instruction::new(Addressing::ZeroPage, OpCode::BIT, 3, 0),
      0x2C => Instruction::new(Addressing::Absolute, OpCode::BIT, 4, 0),

      // BMI
      0x30 => Instruction::new(Addressing::Relative, OpCode::BMI, 2, 1),

      // BNE
      0xD0 => Instruction::new(Addressing::Relative, OpCode::BNE, 2, 1),

      // BPL
      0x10 => Instruction::new(Addressing::Relative, OpCode::BPL, 2, 1),

      // BRK
      0x00 => Instruction::new(Addressing::Implied, OpCode::BRK, 7, 0),

      // BVC
      0x50 => Instruction::new(Addressing::Relative, OpCode::BVC, 2, 1),

      // BVS
      0x70 => Instruction::new(Addressing::Relative, OpCode::BVS, 2, 1),

      // CLC
      0x18 => Instruction::new(Addressing::Implied, OpCode::CLC, 2, 0),

      // CLD
      0xD8 => Instruction::new(Addressing::Implied, OpCode::CLD, 2, 0),

      // CLI
      0x58 => Instruction::new(Addressing::Implied, OpCode::CLI, 2, 0),

      // CLV
      0xB8 => Instruction::new(Addressing::Implied, OpCode::CLV, 2, 0),

      // CMP
      0xC9 => Instruction::new(Addressing::Immediate, OpCode::CMP, 2, 0),
      0xC5 => Instruction::new(Addressing::ZeroPage, OpCode::CMP, 3, 0),
      0xD5 => Instruction::new(Addressing::ZeroPageX, OpCode::CMP, 4, 0),
      0xCD => Instruction::new(Addressing::Absolute, OpCode::CMP, 4, 0),
      0xDD => Instruction::new(Addressing::AbsoluteX, OpCode::CMP, 4, 1),
      0xD9 => Instruction::new(Addressing::AbsoluteY, OpCode::CMP, 4, 1),
      0xC1 => Instruction::new(Addressing::IndirectX, OpCode::CMP, 6, 0),
      0xD1 => Instruction::new(Addressing::IndirectY, OpCode::CMP, 5, 0),

      // CPX
      0xE0 => Instruction::new(Addressing::Immediate, OpCode::CPX, 2, 0),
      0xE4 => Instruction::new(Addressing::ZeroPage, OpCode::CPX, 3, 0),
      0xEC => Instruction::new(Addressing::Absolute, OpCode::CPX, 4, 0),

      // CPY
      0xC0 => Instruction::new(Addressing::Immediate, OpCode::CPY, 2, 0),
      0xC4 => Instruction::new(Addressing::ZeroPage, OpCode::CPY, 3, 0),
      0xCC => Instruction::new(Addressing::Absolute, OpCode::CPY, 4, 0),

      // *DCP
      0xC7 => Instruction::new(Addressing::ZeroPage, OpCode::XDCP, 5, 0),
      0xD7 => Instruction::new(Addressing::ZeroPageX, OpCode::XDCP, 6, 0),
      0xCF => Instruction::new(Addressing::Absolute, OpCode::XDCP, 6, 0),
      0xDF => Instruction::new(Addressing::AbsoluteX, OpCode::XDCP, 7, 0),
      0xDB => Instruction::new(Addressing::AbsoluteY, OpCode::XDCP, 7, 0),
      0xC3 => Instruction::new(Addressing::IndirectX, OpCode::XDCP, 8, 0),
      0xD3 => Instruction::new(Addressing::IndirectY, OpCode::XDCP, 8, 0),

      // DEC
      0xC6 => Instruction::new(Addressing::ZeroPage, OpCode::DEC, 5, 0),
      0xD6 => Instruction::new(Addressing::ZeroPageX, OpCode::DEC, 6, 0),
      0xCE => Instruction::new(Addressing::Absolute, OpCode::DEC, 6, 0),
      0xDE => Instruction::new(Addressing::AbsoluteX, OpCode::DEC, 6, 0),

      // DEX
      0xCA => Instruction::new(Addressing::Implied, OpCode::DEX, 2, 0),

      // DEY
      0x88 => Instruction::new(Addressing::Implied, OpCode::DEY, 2, 0),

      // EOR
      0x49 => Instruction::new(Addressing::Immediate, OpCode::EOR, 2, 0),
      0x45 => Instruction::new(Addressing::ZeroPage, OpCode::EOR, 3, 0),
      0x55 => Instruction::new(Addressing::ZeroPageX, OpCode::EOR, 4, 0),
      0x4D => Instruction::new(Addressing::Absolute, OpCode::EOR, 4, 0),
      0x5D => Instruction::new(Addressing::AbsoluteX, OpCode::EOR, 4, 1),
      0x59 => Instruction::new(Addressing::AbsoluteY, OpCode::EOR, 4, 1),
      0x41 => Instruction::new(Addressing::IndirectX, OpCode::EOR, 6, 0),
      0x51 => Instruction::new(Addressing::IndirectY, OpCode::EOR, 5, 1),

      // INC
      0xE6 => Instruction::new(Addressing::ZeroPage, OpCode::INC, 5, 0),
      0xF6 => Instruction::new(Addressing::ZeroPageX, OpCode::INC, 6, 0),
      0xEE => Instruction::new(Addressing::Absolute, OpCode::INC, 6, 0),
      0xFE => Instruction::new(Addressing::AbsoluteX, OpCode::INC, 7, 0),

      // INX
      0xE8 => Instruction::new(Addressing::Implied, OpCode::INX, 2, 0),

      // INY
      0xC8 => Instruction::new(Addressing::Implied, OpCode::INY, 2, 0),

      // *ISC
      0xE7 => Instruction::new(Addressing::ZeroPage, OpCode::XISC, 5, 0),
      0xF7 => Instruction::new(Addressing::ZeroPageX, OpCode::XISC, 6, 0),
      0xEF => Instruction::new(Addressing::Absolute, OpCode::XISC, 6, 0),
      0xFF => Instruction::new(Addressing::AbsoluteX, OpCode::XISC, 7, 0),
      0xFB => Instruction::new(Addressing::AbsoluteY, OpCode::XISC, 7, 0),
      0xE3 => Instruction::new(Addressing::IndirectX, OpCode::XISC, 8, 0),
      0xF3 => Instruction::new(Addressing::IndirectY, OpCode::XISC, 8, 0),

      // JAM
      0x02 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x12 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x22 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x32 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x42 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x52 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x62 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x72 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0x92 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0xB2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0xD2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),
      0xF2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2, 0),

      // JMP
      0x4C => Instruction::new(Addressing::Absolute, OpCode::JMP, 3, 0),
      0x6C => Instruction::new(Addressing::Indirect, OpCode::JMP, 5, 0),

      // JSR
      0x20 => Instruction::new(Addressing::Absolute, OpCode::JSR, 6, 0),

      // *LAS
      0xBB => Instruction::new(Addressing::AbsoluteY, OpCode::XLAS, 4, 1),

      // *LAX
      0xA7 => Instruction::new(Addressing::ZeroPage, OpCode::XLAX, 3, 0),
      0xB7 => Instruction::new(Addressing::ZeroPageY, OpCode::XLAX, 4, 0),
      0xAF => Instruction::new(Addressing::Absolute, OpCode::XLAX, 4, 0),
      0xBF => Instruction::new(Addressing::AbsoluteY, OpCode::XLAX, 4, 1),
      0xA3 => Instruction::new(Addressing::IndirectX, OpCode::XLAX, 6, 0),
      0xB3 => Instruction::new(Addressing::IndirectY, OpCode::XLAX, 5, 1),

      // LDA
      0xA9 => Instruction::new(Addressing::Immediate, OpCode::LDA, 2, 0),
      0xA5 => Instruction::new(Addressing::ZeroPage, OpCode::LDA, 3, 0),
      0xB5 => Instruction::new(Addressing::ZeroPageX, OpCode::LDA, 4, 0),
      0xAD => Instruction::new(Addressing::Absolute, OpCode::LDA, 4, 0),
      0xBD => Instruction::new(Addressing::AbsoluteX, OpCode::LDA, 4, 1),
      0xB9 => Instruction::new(Addressing::AbsoluteY, OpCode::LDA, 4, 1),
      0xA1 => Instruction::new(Addressing::IndirectX, OpCode::LDA, 6, 0),
      0xB1 => Instruction::new(Addressing::IndirectY, OpCode::LDA, 5, 1),

      // LDX
      0xA2 => Instruction::new(Addressing::Immediate, OpCode::LDX, 2, 0),
      0xA6 => Instruction::new(Addressing::ZeroPage, OpCode::LDX, 3, 0),
      0xB6 => Instruction::new(Addressing::ZeroPageY, OpCode::LDX, 4, 0),
      0xAE => Instruction::new(Addressing::Absolute, OpCode::LDX, 4, 0),
      0xBE => Instruction::new(Addressing::AbsoluteY, OpCode::LDX, 4, 1),

      // LDY
      0xA0 => Instruction::new(Addressing::Immediate, OpCode::LDY, 2, 0),
      0xA4 => Instruction::new(Addressing::ZeroPage, OpCode::LDY, 3, 0),
      0xB4 => Instruction::new(Addressing::ZeroPageX, OpCode::LDY, 4, 0),
      0xAC => Instruction::new(Addressing::Absolute, OpCode::LDY, 4, 0),
      0xBC => Instruction::new(Addressing::AbsoluteX, OpCode::LDY, 4, 1),

      // LSR
      0x4A => Instruction::new(Addressing::Accumulator, OpCode::LSR, 2, 0),
      0x46 => Instruction::new(Addressing::ZeroPage, OpCode::LSR, 5, 0),
      0x56 => Instruction::new(Addressing::ZeroPageX, OpCode::LSR, 6, 0),
      0x4E => Instruction::new(Addressing::Absolute, OpCode::LSR, 6, 0),
      0x5E => Instruction::new(Addressing::AbsoluteX, OpCode::LSR, 7, 0),

      // *LXA
      0xAB => Instruction::new(Addressing::Immediate, OpCode::XLXA, 2, 0),

      // NOP
      0xEA => Instruction::new(Addressing::Implied, OpCode::NOP, 2, 0),

      // *NOP
      0x1A => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0x3A => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0x5A => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0x7A => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0xDA => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0xFA => Instruction::new(Addressing::Implied, OpCode::XNOP, 2, 0),
      0x80 => Instruction::new(Addressing::Immediate, OpCode::XNOP, 2, 0),
      0x82 => Instruction::new(Addressing::Immediate, OpCode::XNOP, 2, 0),
      0x89 => Instruction::new(Addressing::Immediate, OpCode::XNOP, 2, 0),
      0xC2 => Instruction::new(Addressing::Immediate, OpCode::XNOP, 2, 0),
      0xE2 => Instruction::new(Addressing::Immediate, OpCode::XNOP, 2, 0),
      0x04 => Instruction::new(Addressing::ZeroPage, OpCode::XNOP, 3, 0),
      0x44 => Instruction::new(Addressing::ZeroPage, OpCode::XNOP, 3, 0),
      0x64 => Instruction::new(Addressing::ZeroPage, OpCode::XNOP, 3, 0),
      0x14 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0x34 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0x54 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0x74 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0xD4 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0xF4 => Instruction::new(Addressing::ZeroPageX, OpCode::XNOP, 4, 0),
      0x0C => Instruction::new(Addressing::Absolute, OpCode::XNOP, 4, 0),
      0x1C => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),
      0x3C => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),
      0x5C => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),
      0x7C => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),
      0xDC => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),
      0xFC => Instruction::new(Addressing::AbsoluteX, OpCode::XNOP, 4, 1),

      // ORA
      0x09 => Instruction::new(Addressing::Immediate, OpCode::ORA, 2, 0),
      0x05 => Instruction::new(Addressing::ZeroPage, OpCode::ORA, 3, 0),
      0x15 => Instruction::new(Addressing::ZeroPageX, OpCode::ORA, 4, 0),
      0x0D => Instruction::new(Addressing::Absolute, OpCode::ORA, 4, 0),
      0x1D => Instruction::new(Addressing::AbsoluteX, OpCode::ORA, 4, 1),
      0x19 => Instruction::new(Addressing::AbsoluteY, OpCode::ORA, 4, 1),
      0x01 => Instruction::new(Addressing::IndirectX, OpCode::ORA, 6, 0),
      0x11 => Instruction::new(Addressing::IndirectY, OpCode::ORA, 5, 1),

      // PHA
      0x48 => Instruction::new(Addressing::Implied, OpCode::PHA, 3, 0),

      // PHP
      0x08 => Instruction::new(Addressing::Implied, OpCode::PHP, 3, 0),

      // PLA
      0x68 => Instruction::new(Addressing::Implied, OpCode::PLA, 4, 0),

      // PLP
      0x28 => Instruction::new(Addressing::Implied, OpCode::PLP, 4, 0),

      // *RLA
      0x27 => Instruction::new(Addressing::ZeroPage, OpCode::XRLA, 5, 0),
      0x37 => Instruction::new(Addressing::ZeroPageX, OpCode::XRLA, 6, 0),
      0x2F => Instruction::new(Addressing::Absolute, OpCode::XRLA, 6, 0),
      0x3F => Instruction::new(Addressing::AbsoluteX, OpCode::XRLA, 7, 0),
      0x3B => Instruction::new(Addressing::AbsoluteY, OpCode::XRLA, 7, 0),
      0x23 => Instruction::new(Addressing::IndirectX, OpCode::XRLA, 8, 0),
      0x33 => Instruction::new(Addressing::IndirectY, OpCode::XRLA, 8, 0),

      // ROL
      0x2A => Instruction::new(Addressing::Accumulator, OpCode::ROL, 2, 0),
      0x26 => Instruction::new(Addressing::ZeroPage, OpCode::ROL, 5, 0),
      0x36 => Instruction::new(Addressing::ZeroPageX, OpCode::ROL, 6, 0),
      0x2E => Instruction::new(Addressing::Absolute, OpCode::ROL, 6, 0),
      0x3E => Instruction::new(Addressing::AbsoluteX, OpCode::ROL, 7, 0),

      // ROR
      0x6A => Instruction::new(Addressing::Accumulator, OpCode::ROR, 2, 0),
      0x66 => Instruction::new(Addressing::ZeroPage, OpCode::ROR, 5, 0),
      0x76 => Instruction::new(Addressing::ZeroPageX, OpCode::ROR, 6, 0),
      0x6E => Instruction::new(Addressing::Absolute, OpCode::ROR, 6, 0),
      0x7E => Instruction::new(Addressing::AbsoluteX, OpCode::ROR, 7, 0),

      // *RRA
      0x67 => Instruction::new(Addressing::ZeroPage, OpCode::XRRA, 5, 0),
      0x77 => Instruction::new(Addressing::ZeroPageX, OpCode::XRRA, 6, 0),
      0x6F => Instruction::new(Addressing::Absolute, OpCode::XRRA, 6, 0),
      0x7F => Instruction::new(Addressing::AbsoluteX, OpCode::XRRA, 7, 0),
      0x7B => Instruction::new(Addressing::AbsoluteY, OpCode::XRRA, 7, 0),
      0x63 => Instruction::new(Addressing::IndirectX, OpCode::XRRA, 8, 0),
      0x73 => Instruction::new(Addressing::IndirectY, OpCode::XRRA, 8, 0),

      // RTI
      0x40 => Instruction::new(Addressing::Implied, OpCode::RTI, 6, 0),

      // RTS
      0x60 => Instruction::new(Addressing::Implied, OpCode::RTS, 6, 0),

      // *SAX
      0x87 => Instruction::new(Addressing::ZeroPage, OpCode::XSAX, 3, 0),
      0x97 => Instruction::new(Addressing::ZeroPageY, OpCode::XSAX, 4, 0),
      0x8F => Instruction::new(Addressing::Absolute, OpCode::XSAX, 4, 0),
      0x83 => Instruction::new(Addressing::IndirectX, OpCode::XSAX, 6, 0),

      // *SBC
      0xEB => Instruction::new(Addressing::Immediate, OpCode::XSBC, 2, 0),

      // SBC
      0xE9 => Instruction::new(Addressing::Immediate, OpCode::SBC, 2, 0),
      0xE5 => Instruction::new(Addressing::ZeroPage, OpCode::SBC, 3, 0),
      0xF5 => Instruction::new(Addressing::ZeroPageX, OpCode::SBC, 4, 0),
      0xED => Instruction::new(Addressing::Absolute, OpCode::SBC, 4, 0),
      0xFD => Instruction::new(Addressing::AbsoluteX, OpCode::SBC, 4, 1),
      0xF9 => Instruction::new(Addressing::AbsoluteY, OpCode::SBC, 4, 1),
      0xE1 => Instruction::new(Addressing::IndirectX, OpCode::SBC, 6, 0),
      0xF1 => Instruction::new(Addressing::IndirectY, OpCode::SBC, 5, 1),

      // *SBX
      0xCB => Instruction::new(Addressing::Immediate, OpCode::XSBX, 2, 0),

      // SEC
      0x38 => Instruction::new(Addressing::Implied, OpCode::SEC, 2, 0),

      // SED
      0xF8 => Instruction::new(Addressing::Implied, OpCode::SED, 2, 0),

      // SEI
      0x78 => Instruction::new(Addressing::Implied, OpCode::SEI, 2, 0),

      // *SHA
      0x9F => Instruction::new(Addressing::AbsoluteY, OpCode::XSHA, 5, 0),
      0x93 => Instruction::new(Addressing::IndirectY, OpCode::XSHA, 6, 0),

      // *SHX
      0x9C => Instruction::new(Addressing::AbsoluteY, OpCode::XSHX, 5, 0),

      // *SHY
      0x9E => Instruction::new(Addressing::AbsoluteX, OpCode::XSHY, 5, 0),

      // *SLO
      0x07 => Instruction::new(Addressing::ZeroPage, OpCode::XSLO, 5, 0),
      0x17 => Instruction::new(Addressing::ZeroPageX, OpCode::XSLO, 6, 0),
      0x0F => Instruction::new(Addressing::Absolute, OpCode::XSLO, 6, 0),
      0x1F => Instruction::new(Addressing::AbsoluteX, OpCode::XSLO, 7, 0),
      0x1B => Instruction::new(Addressing::AbsoluteY, OpCode::XSLO, 7, 0),
      0x03 => Instruction::new(Addressing::IndirectX, OpCode::XSLO, 8, 0),
      0x13 => Instruction::new(Addressing::IndirectY, OpCode::XSLO, 8, 0),

      // *SRE
      0x47 => Instruction::new(Addressing::ZeroPage, OpCode::XSRE, 5, 0),
      0x57 => Instruction::new(Addressing::ZeroPageX, OpCode::XSRE, 6, 0),
      0x4F => Instruction::new(Addressing::Absolute, OpCode::XSRE, 6, 0),
      0x5F => Instruction::new(Addressing::AbsoluteX, OpCode::XSRE, 7, 0),
      0x5B => Instruction::new(Addressing::AbsoluteY, OpCode::XSRE, 7, 0),
      0x43 => Instruction::new(Addressing::IndirectX, OpCode::XSRE, 8, 0),
      0x53 => Instruction::new(Addressing::IndirectY, OpCode::XSRE, 8, 0),

      // STA
      0x85 => Instruction::new(Addressing::ZeroPage, OpCode::STA, 3, 0),
      0x95 => Instruction::new(Addressing::ZeroPageX, OpCode::STA, 4, 0),
      0x8D => Instruction::new(Addressing::Absolute, OpCode::STA, 4, 0),
      0x9D => Instruction::new(Addressing::AbsoluteX, OpCode::STA, 5, 0),
      0x99 => Instruction::new(Addressing::AbsoluteY, OpCode::STA, 5, 0),
      0x81 => Instruction::new(Addressing::IndirectX, OpCode::STA, 6, 0),
      0x91 => Instruction::new(Addressing::IndirectY, OpCode::STA, 6, 0),

      // STX
      0x86 => Instruction::new(Addressing::ZeroPage, OpCode::STX, 3, 0),
      0x96 => Instruction::new(Addressing::ZeroPageY, OpCode::STX, 4, 0),
      0x8E => Instruction::new(Addressing::Absolute, OpCode::STX, 4, 0),

      // STY
      0x84 => Instruction::new(Addressing::ZeroPage, OpCode::STY, 3, 0),
      0x94 => Instruction::new(Addressing::ZeroPageX, OpCode::STY, 4, 0),
      0x8C => Instruction::new(Addressing::Absolute, OpCode::STY, 4, 0),

      // *TAS
      0x9B => Instruction::new(Addressing::AbsoluteY, OpCode::XTAS, 5, 0),

      // TAX
      0xAA => Instruction::new(Addressing::Implied, OpCode::TAX, 2, 0),

      // TAY
      0xA8 => Instruction::new(Addressing::Implied, OpCode::TAY, 2, 0),

      // TSX
      0xBA => Instruction::new(Addressing::Implied, OpCode::TSX, 2, 0),

      // TXA
      0x8A => Instruction::new(Addressing::Implied, OpCode::TXA, 2, 0),

      // TXS
      0x9A => Instruction::new(Addressing::Implied, OpCode::TXS, 2, 0),

      // TYA
      0x98 => Instruction::new(Addressing::Implied, OpCode::TYA, 2, 0),
    }
  }

  pub fn needs_data(&self) -> bool {
    match self.opcode {
      OpCode::BCC
      | OpCode::BCS
      | OpCode::BEQ
      | OpCode::BMI
      | OpCode::BNE
      | OpCode::BPL
      | OpCode::BVC
      | OpCode::BVS
      | OpCode::JMP
      | OpCode::JSR
      | OpCode::XSAX
      | OpCode::XSHA
      | OpCode::XSHX
      | OpCode::XSHY
      | OpCode::STA
      | OpCode::STX
      | OpCode::STY
      | OpCode::XTAS => false,
      _ => true,
    }
  }
}
