use std::fmt::{Debug, Display, Formatter, Result};

pub enum Addressing {
  Immediate, ZeroPage, ZeroPageX, ZeroPageY, Absolute, AbsoluteIndirect,
  AbsoluteX, AbsoluteY, IndirectX, IndirectY, Implied,
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
  _ALR, _ANC, _ANE, _ARR, _DCP, _ISC, _LAS,
  _LAX, _LXA, _NOP, _RLA, _RRA, _SAX, _SBC,
  _SBX, _SHA, _SHX, _SHY, _SLO, _SRE, _TAS,
}

impl Display for OpCode {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Debug::fmt(self, f)
}
}

pub struct Instruction {
  pub len: u8,
  pub mode: Addressing,
  pub opcode: OpCode,
  pub cycles: u8,
}

impl Instruction {
  fn new(mode: Addressing, opcode: OpCode, cycles: u8) -> Self {
    Instruction {
      len: match mode {
        Addressing::Implied => 1,
        Addressing::Immediate | Addressing::ZeroPage | Addressing::ZeroPageX |
        Addressing::ZeroPageY | Addressing::IndirectX | Addressing::IndirectY => 2,
        Addressing::Absolute | Addressing::AbsoluteX | Addressing::AbsoluteY |
        Addressing::AbsoluteIndirect => 3,
      },
      mode,
      opcode,
      cycles,
    }
  }

  pub fn get(code: u8) -> Self {
    match code {
      // ADC
      0x69 => Instruction::new(Addressing::Immediate, OpCode::ADC, 2),
      0x65 => Instruction::new(Addressing::ZeroPage, OpCode::ADC, 3),
      0x75 => Instruction::new(Addressing::ZeroPageX, OpCode::ADC, 4 ),
      0x6D => Instruction::new(Addressing::Absolute, OpCode::ADC, 4),
      0x7D => Instruction::new(Addressing::AbsoluteX, OpCode::ADC, 4),
      0x79 => Instruction::new(Addressing::AbsoluteY, OpCode::ADC, 4),
      0x61 => Instruction::new(Addressing::IndirectX, OpCode::ADC, 6),
      0x71 => Instruction::new(Addressing::IndirectY, OpCode::ADC, 5),

      // *ALR
      0x4B => Instruction::new(Addressing::Immediate, OpCode::_ALR, 2),

      // *ANC
      0x0B => Instruction::new(Addressing::Immediate, OpCode::_ANC, 2),
      0x2B => Instruction::new(Addressing::Immediate, OpCode::_ANC, 2),

      // AND
      0x29 => Instruction::new(Addressing::Immediate, OpCode::AND, 2),
      0x25 => Instruction::new(Addressing::ZeroPage, OpCode::AND, 3),
      0x35 => Instruction::new(Addressing::ZeroPageX, OpCode::AND, 4),
      0x2D => Instruction::new(Addressing::Absolute, OpCode::AND, 4),
      0x3D => Instruction::new(Addressing::AbsoluteX, OpCode::AND, 4),
      0x39 => Instruction::new(Addressing::AbsoluteY, OpCode::AND, 4),
      0x21 => Instruction::new(Addressing::IndirectX, OpCode::AND, 6),
      0x31 => Instruction::new(Addressing::IndirectY, OpCode::AND, 5),

      // *ANE
      0x8B => Instruction::new(Addressing::Immediate, OpCode::_ANE, 2),

      // *ARR
      0x6B => Instruction::new(Addressing::Immediate, OpCode::_ARR, 2),

      // ASL
      0x0A => Instruction::new(Addressing::Implied, OpCode::ASL, 2),
      0x06 => Instruction::new(Addressing::ZeroPage, OpCode::ASL, 5),
      0x16 => Instruction::new(Addressing::ZeroPageX, OpCode::ASL, 6),
      0x0E => Instruction::new(Addressing::Absolute, OpCode::ASL, 6),
      0x1E => Instruction::new(Addressing::AbsoluteX, OpCode::ASL, 7),

      // BCC
      0x90 => Instruction::new(Addressing::Immediate, OpCode::BCC, 2),

      // BCS
      0xB0 => Instruction::new(Addressing::Immediate, OpCode::BCS, 2),

      // BEQ
      0xF0 => Instruction::new(Addressing::Immediate, OpCode::BEQ, 2),

      // BIT
      0x24 => Instruction::new(Addressing::ZeroPage, OpCode::BIT, 3),
      0x2C => Instruction::new(Addressing::Absolute, OpCode::BIT, 4),

      // BMI
      0x30 => Instruction::new(Addressing::Immediate, OpCode::BMI, 2),

      // BNE
      0xD0 => Instruction::new(Addressing::Immediate, OpCode::BNE, 2),

      // BPL
      0x10 => Instruction::new(Addressing::Immediate, OpCode::BPL, 2),

      // BRK
      0x00 => Instruction::new(Addressing::Implied, OpCode::BRK, 7),

      // BVC
      0x50 => Instruction::new(Addressing::Immediate, OpCode::BVC, 2),

      // BVS
      0x70 => Instruction::new(Addressing::Immediate, OpCode::BVS, 2),

      // CLC
      0x18 => Instruction::new(Addressing::Implied, OpCode::CLC, 2),

      // CLD
      0xD8 => Instruction::new(Addressing::Implied, OpCode::CLD, 2),

      // CLI
      0x58 => Instruction::new(Addressing::Implied, OpCode::CLI, 2),

      // CLV
      0xB8 => Instruction::new(Addressing::Implied, OpCode::CLV, 2),

      // CMP
      0xC9 => Instruction::new(Addressing::Immediate, OpCode::CMP, 2),
      0xC5 => Instruction::new(Addressing::ZeroPage, OpCode::CMP, 3),
      0xD5 => Instruction::new(Addressing::ZeroPageX, OpCode::CMP, 4),
      0xCD => Instruction::new(Addressing::Absolute, OpCode::CMP, 4),
      0xDD => Instruction::new(Addressing::AbsoluteX, OpCode::CMP, 4),
      0xD9 => Instruction::new(Addressing::AbsoluteY, OpCode::CMP, 4),
      0xC1 => Instruction::new(Addressing::IndirectX, OpCode::CMP, 6),
      0xD1 => Instruction::new(Addressing::IndirectY, OpCode::CMP, 5),

      // CPX
      0xE0 => Instruction::new(Addressing::Immediate, OpCode::CPX, 2),
      0xE4 => Instruction::new(Addressing::ZeroPage, OpCode::CPX, 3),
      0xEC => Instruction::new(Addressing::Absolute, OpCode::CPX, 4),

      // CPY
      0xC0 => Instruction::new(Addressing::Immediate, OpCode::CPY, 2),
      0xC4 => Instruction::new(Addressing::ZeroPage, OpCode::CPY, 3),
      0xCC => Instruction::new(Addressing::Absolute, OpCode::CPY, 4),

      // *DCP
      0xC7 => Instruction::new(Addressing::ZeroPage, OpCode::_DCP, 5),
      0xD7 => Instruction::new(Addressing::ZeroPageX, OpCode::_DCP, 6),
      0xCF => Instruction::new(Addressing::Absolute, OpCode::_DCP, 6),
      0xDF => Instruction::new(Addressing::AbsoluteX, OpCode::_DCP, 7),
      0xDB => Instruction::new(Addressing::AbsoluteY, OpCode::_DCP, 7),
      0xC3 => Instruction::new(Addressing::IndirectX, OpCode::_DCP, 8),
      0xD3 => Instruction::new(Addressing::IndirectY, OpCode::_DCP, 8),

      // DEC
      0xC6 => Instruction::new(Addressing::ZeroPage, OpCode::DEC, 5),
      0xD6 => Instruction::new(Addressing::ZeroPageX, OpCode::DEC, 6),
      0xCE => Instruction::new(Addressing::Absolute, OpCode::DEC, 6),
      0xDE => Instruction::new(Addressing::AbsoluteX, OpCode::DEC, 7),

      // DEX
      0xCA => Instruction::new(Addressing::Implied, OpCode::DEX, 2),

      // DEY
      0x88 => Instruction::new(Addressing::Implied, OpCode::DEY, 2),

      // EOR
      0x49 => Instruction::new(Addressing::Immediate, OpCode::EOR, 2),
      0x45 => Instruction::new(Addressing::ZeroPage, OpCode::EOR, 3),
      0x55 => Instruction::new(Addressing::ZeroPageX, OpCode::EOR, 4),
      0x4D => Instruction::new(Addressing::Absolute, OpCode::EOR, 4),
      0x5D => Instruction::new(Addressing::AbsoluteX, OpCode::EOR, 4),
      0x59 => Instruction::new(Addressing::AbsoluteY, OpCode::EOR, 4),
      0x41 => Instruction::new(Addressing::IndirectX, OpCode::EOR, 6),
      0x51 => Instruction::new(Addressing::IndirectY, OpCode::EOR, 5),

      // INC
      0xE6 => Instruction::new(Addressing::ZeroPage, OpCode::INC, 5),
      0xF6 => Instruction::new(Addressing::ZeroPageX, OpCode::INC, 6),
      0xEE => Instruction::new(Addressing::Absolute, OpCode::INC, 6),
      0xFE => Instruction::new(Addressing::AbsoluteX, OpCode::INC, 7),

      // INX
      0xE8 => Instruction::new(Addressing::Implied, OpCode::INX, 2),

      // INY
      0xC8 => Instruction::new(Addressing::Implied, OpCode::INY, 2),

      // *ISC
      0xE7 => Instruction::new(Addressing::ZeroPage, OpCode::_ISC, 5),
      0xF7 => Instruction::new(Addressing::ZeroPageX, OpCode::_ISC, 6),
      0xEF => Instruction::new(Addressing::Absolute, OpCode::_ISC, 6),
      0xFF => Instruction::new(Addressing::AbsoluteX, OpCode::_ISC, 7),
      0xFB => Instruction::new(Addressing::AbsoluteY, OpCode::_ISC, 7),
      0xE3 => Instruction::new(Addressing::IndirectX, OpCode::_ISC, 8),
      0xF3 => Instruction::new(Addressing::IndirectY, OpCode::_ISC, 8),

      // JAM
      0x02 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x12 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x22 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x32 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x42 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x52 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x62 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x72 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0x92 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0xB2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0xD2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),
      0xF2 => Instruction::new(Addressing::Implied, OpCode::JAM, 2),

      // JMP
      0x4C => Instruction::new(Addressing::Absolute, OpCode::JMP, 3),
      0x6C => Instruction::new(Addressing::AbsoluteIndirect, OpCode::JMP, 5),

      // JSR
      0x20 => Instruction::new(Addressing::Absolute, OpCode::JSR, 6),

      // *LAS
      0xBB => Instruction::new(Addressing::AbsoluteY, OpCode::_LAS, 4),

      // *LAX
      0xA7 => Instruction::new(Addressing::ZeroPage, OpCode::_LAX, 3),
      0xB7 => Instruction::new(Addressing::ZeroPageY, OpCode::_LAX, 4),
      0xAF => Instruction::new(Addressing::Absolute, OpCode::_LAX, 4),
      0xBF => Instruction::new(Addressing::AbsoluteY, OpCode::_LAX, 4),
      0xA3 => Instruction::new(Addressing::IndirectX, OpCode::_LAX, 6),
      0xB3 => Instruction::new(Addressing::IndirectY, OpCode::_LAX, 5),

      // LDA
      0xA9 => Instruction::new(Addressing::Immediate, OpCode::LDA, 2),
      0xA5 => Instruction::new(Addressing::ZeroPage, OpCode::LDA, 3),
      0xB5 => Instruction::new(Addressing::ZeroPageX, OpCode::LDA, 4),
      0xAD => Instruction::new(Addressing::Absolute, OpCode::LDA, 4),
      0xBD => Instruction::new(Addressing::AbsoluteX, OpCode::LDA, 4),
      0xB9 => Instruction::new(Addressing::AbsoluteY, OpCode::LDA, 4),
      0xA1 => Instruction::new(Addressing::IndirectX, OpCode::LDA, 6),
      0xB1 => Instruction::new(Addressing::IndirectY, OpCode::LDA, 5),

      // LDX
      0xA2 => Instruction::new(Addressing::Immediate, OpCode::LDX, 2),
      0xA6 => Instruction::new(Addressing::ZeroPage, OpCode::LDX, 3),
      0xB6 => Instruction::new(Addressing::ZeroPageY, OpCode::LDX, 4),
      0xAE => Instruction::new(Addressing::Absolute, OpCode::LDX, 4),
      0xBE => Instruction::new(Addressing::AbsoluteY, OpCode::LDX, 4),

      // LDY
      0xA0 => Instruction::new(Addressing::Immediate, OpCode::LDY, 2),
      0xA4 => Instruction::new(Addressing::ZeroPage, OpCode::LDY, 3),
      0xB4 => Instruction::new(Addressing::ZeroPageX, OpCode::LDY, 4),
      0xAC => Instruction::new(Addressing::Absolute, OpCode::LDY, 4),
      0xBC => Instruction::new(Addressing::AbsoluteX, OpCode::LDY, 4),

      // LSR
      0x4A => Instruction::new(Addressing::Implied, OpCode::LSR, 2),
      0x46 => Instruction::new(Addressing::ZeroPage, OpCode::LSR, 5),
      0x56 => Instruction::new(Addressing::ZeroPageX, OpCode::LSR, 6),
      0x4E => Instruction::new(Addressing::Absolute, OpCode::LSR, 6),
      0x5E => Instruction::new(Addressing::AbsoluteX, OpCode::LSR, 7),

      // *LXA
      0xAB => Instruction::new(Addressing::Immediate, OpCode::_LXA, 2),

      // NOP
      0xEA => Instruction::new(Addressing::Implied, OpCode::NOP, 2),

      // *NOP
      0x1A => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0x3A => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0x5A => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0x7A => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0xDA => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0xFA => Instruction::new(Addressing::Implied, OpCode::_NOP, 2),
      0x80 => Instruction::new(Addressing::Immediate, OpCode::_NOP, 2),
      0x82 => Instruction::new(Addressing::Immediate, OpCode::_NOP, 2),
      0x89 => Instruction::new(Addressing::Immediate, OpCode::_NOP, 2),
      0xC2 => Instruction::new(Addressing::Immediate, OpCode::_NOP, 2),
      0xE2 => Instruction::new(Addressing::Immediate, OpCode::_NOP, 2),
      0x04 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP, 3),
      0x44 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP, 3),
      0x64 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP, 3),
      0x14 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0x34 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0x54 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0x74 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0xD4 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0xF4 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP, 4),
      0x0C => Instruction::new(Addressing::Absolute, OpCode::_NOP, 4),
      0x1C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),
      0x3C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),
      0x5C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),
      0x7C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),
      0xDC => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),
      0xFC => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP, 4),

      // ORA
      0x09 => Instruction::new(Addressing::Immediate, OpCode::ORA, 2),
      0x05 => Instruction::new(Addressing::ZeroPage, OpCode::ORA, 3),
      0x15 => Instruction::new(Addressing::ZeroPageX, OpCode::ORA, 4),
      0x0D => Instruction::new(Addressing::Absolute, OpCode::ORA, 4),
      0x1D => Instruction::new(Addressing::AbsoluteX, OpCode::ORA, 4),
      0x19 => Instruction::new(Addressing::AbsoluteY, OpCode::ORA, 4),
      0x01 => Instruction::new(Addressing::IndirectX, OpCode::ORA, 6),
      0x11 => Instruction::new(Addressing::IndirectY, OpCode::ORA, 5),

      // PHA
      0x48 => Instruction::new(Addressing::Implied, OpCode::PHA, 3),

      // PHP
      0x08 => Instruction::new(Addressing::Implied, OpCode::PHP, 3),

      // PLA
      0x68 => Instruction::new(Addressing::Implied, OpCode::PLA, 4),

      // PLP
      0x28 => Instruction::new(Addressing::Implied, OpCode::PLP, 4),

      // *RLA
      0x27 => Instruction::new(Addressing::ZeroPage, OpCode::_RLA, 5),
      0x37 => Instruction::new(Addressing::ZeroPageX, OpCode::_RLA, 6),
      0x2F => Instruction::new(Addressing::Absolute, OpCode::_RLA, 6),
      0x3F => Instruction::new(Addressing::AbsoluteX, OpCode::_RLA, 7),
      0x3B => Instruction::new(Addressing::AbsoluteY, OpCode::_RLA, 7),
      0x23 => Instruction::new(Addressing::IndirectX, OpCode::_RLA, 8),
      0x33 => Instruction::new(Addressing::IndirectY, OpCode::_RLA, 8),

      // ROL
      0x2A => Instruction::new(Addressing::Implied, OpCode::ROL, 2),
      0x26 => Instruction::new(Addressing::ZeroPage, OpCode::ROL, 5),
      0x36 => Instruction::new(Addressing::ZeroPageX, OpCode::ROL, 6),
      0x2E => Instruction::new(Addressing::Absolute, OpCode::ROL, 6),
      0x3E => Instruction::new(Addressing::AbsoluteX, OpCode::ROL, 7),

      // ROR
      0x6A => Instruction::new(Addressing::Implied, OpCode::ROR, 2),
      0x66 => Instruction::new(Addressing::ZeroPage, OpCode::ROR, 5),
      0x76 => Instruction::new(Addressing::ZeroPageX, OpCode::ROR, 6),
      0x6E => Instruction::new(Addressing::Absolute, OpCode::ROR, 6),
      0x7E => Instruction::new(Addressing::AbsoluteX, OpCode::ROR, 7),

      // *RRA
      0x67 => Instruction::new(Addressing::ZeroPage, OpCode::_RRA, 5),
      0x77 => Instruction::new(Addressing::ZeroPageX, OpCode::_RRA, 6),
      0x6F => Instruction::new(Addressing::Absolute, OpCode::_RRA, 6),
      0x7F => Instruction::new(Addressing::AbsoluteX, OpCode::_RRA, 7),
      0x7B => Instruction::new(Addressing::AbsoluteY, OpCode::_RRA, 7),
      0x63 => Instruction::new(Addressing::IndirectX, OpCode::_RRA, 8),
      0x73 => Instruction::new(Addressing::IndirectY, OpCode::_RRA, 8),

      // RTI
      0x40 => Instruction::new(Addressing::Implied, OpCode::RTI, 6),

      // RTS
      0x60 => Instruction::new(Addressing::Implied, OpCode::RTS, 6),

      // *SAX
      0x87 => Instruction::new(Addressing::ZeroPage, OpCode::_SAX, 3),
      0x97 => Instruction::new(Addressing::ZeroPageY, OpCode::_SAX, 4),
      0x8F => Instruction::new(Addressing::Absolute, OpCode::_SAX, 4),
      0x83 => Instruction::new(Addressing::IndirectX, OpCode::_SAX, 6),

      // *SBC
      0xEB => Instruction::new(Addressing::Immediate, OpCode::_SBC, 2),

      // SBC
      0xE9 => Instruction::new(Addressing::Immediate, OpCode::SBC, 2),
      0xE5 => Instruction::new(Addressing::ZeroPage, OpCode::SBC, 3),
      0xF5 => Instruction::new(Addressing::ZeroPageX, OpCode::SBC, 4),
      0xED => Instruction::new(Addressing::Absolute, OpCode::SBC, 4),
      0xFD => Instruction::new(Addressing::AbsoluteX, OpCode::SBC, 4),
      0xF9 => Instruction::new(Addressing::AbsoluteY, OpCode::SBC, 4),
      0xE1 => Instruction::new(Addressing::IndirectX, OpCode::SBC, 6),
      0xF1 => Instruction::new(Addressing::IndirectY, OpCode::SBC, 5),

      // *SBX
      0xCB => Instruction::new(Addressing::Immediate, OpCode::_SBX, 2),

      // SEC
      0x38 => Instruction::new(Addressing::Implied, OpCode::SEC, 2),

      // SED
      0xF8 => Instruction::new(Addressing::Implied, OpCode::SED, 2),

      // SEI
      0x78 => Instruction::new(Addressing::Implied, OpCode::SEI, 2),

      // *SHA
      0x9F => Instruction::new(Addressing::AbsoluteY, OpCode::_SHA, 5),
      0x93 => Instruction::new(Addressing::IndirectY, OpCode::_SHA, 6),

      // *SHX
      0x9C => Instruction::new(Addressing::AbsoluteY, OpCode::_SHX, 5),

      // *SHY
      0x9E => Instruction::new(Addressing::AbsoluteX, OpCode::_SHY, 5),

      // *SLO
      0x07 => Instruction::new(Addressing::ZeroPage, OpCode::_SLO, 5),
      0x17 => Instruction::new(Addressing::ZeroPageX, OpCode::_SLO, 6),
      0x0F => Instruction::new(Addressing::Absolute, OpCode::_SLO, 6),
      0x1F => Instruction::new(Addressing::AbsoluteX, OpCode::_SLO, 7),
      0x1B => Instruction::new(Addressing::AbsoluteY, OpCode::_SLO, 7),
      0x03 => Instruction::new(Addressing::IndirectX, OpCode::_SLO, 8),
      0x13 => Instruction::new(Addressing::IndirectY, OpCode::_SLO, 8),

      // *SRE
      0x47 => Instruction::new(Addressing::ZeroPage, OpCode::_SRE, 5),
      0x57 => Instruction::new(Addressing::ZeroPageX, OpCode::_SRE, 6),
      0x4F => Instruction::new(Addressing::Absolute, OpCode::_SRE, 6),
      0x5F => Instruction::new(Addressing::AbsoluteX, OpCode::_SRE, 7),
      0x5B => Instruction::new(Addressing::AbsoluteY, OpCode::_SRE, 7),
      0x43 => Instruction::new(Addressing::IndirectX, OpCode::_SRE, 8),
      0x53 => Instruction::new(Addressing::IndirectY, OpCode::_SRE, 8),

      // STA
      0x85 => Instruction::new(Addressing::ZeroPage, OpCode::STA, 3),
      0x95 => Instruction::new(Addressing::ZeroPageX, OpCode::STA, 4),
      0x8D => Instruction::new(Addressing::Absolute, OpCode::STA, 4),
      0x9D => Instruction::new(Addressing::AbsoluteX, OpCode::STA, 5),
      0x99 => Instruction::new(Addressing::AbsoluteY, OpCode::STA, 5),
      0x81 => Instruction::new(Addressing::IndirectX, OpCode::STA, 6),
      0x91 => Instruction::new(Addressing::IndirectY, OpCode::STA, 6),

      // STX
      0x86 => Instruction::new(Addressing::ZeroPage, OpCode::STX, 3),
      0x96 => Instruction::new(Addressing::ZeroPageY, OpCode::STX, 4),
      0x8E => Instruction::new(Addressing::Absolute, OpCode::STX, 4),

      // STY
      0x84 => Instruction::new(Addressing::ZeroPage, OpCode::STY, 3),
      0x94 => Instruction::new(Addressing::ZeroPageX, OpCode::STY, 4),
      0x8C => Instruction::new(Addressing::Absolute, OpCode::STY, 4),

      // *TAS
      0x9B => Instruction::new(Addressing::AbsoluteY, OpCode::_TAS, 5),

      // TAX
      0xAA => Instruction::new(Addressing::Implied, OpCode::TAX, 2),

      // TAY
      0xA8 => Instruction::new(Addressing::Implied, OpCode::TAY, 2),

      // TSX
      0xBA => Instruction::new(Addressing::Implied, OpCode::TSX, 2),

      // TXA
      0x8A => Instruction::new(Addressing::Implied, OpCode::TXA, 2),

      // TXS
      0x9A => Instruction::new(Addressing::Implied, OpCode::TXS, 2),

      // TYA
      0x98 => Instruction::new(Addressing::Implied, OpCode::TYA, 2),
    }
  }
}
