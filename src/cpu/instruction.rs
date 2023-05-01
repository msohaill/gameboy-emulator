use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Copy)]
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
  _ALR, _ANC, _ANE, _ARR, _DCP, _ISC, _LAS,
  _LAX, _LXA, _NOP, _RLA, _RRA, _SAX, _SBC,
  _SBX, _SHA, _SHX, _SHY, _SLO, _SRE, _TAS,
}

#[derive(Debug, Clone, Copy)]
pub struct Operand(pub u16, pub u8, pub u8); // (data, base cycles, additional cycles from page crossing)

impl Display for OpCode {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Debug::fmt(self, f)
}
}

pub struct Instruction {
  pub len: u8,
  pub mode: Addressing,
  pub opcode: OpCode,
}

impl Instruction {
  fn new(mode: Addressing, opcode: OpCode) -> Self {
    Instruction {
      len: match mode {
        Addressing::Accumulator | Addressing::Implied => 1,
        Addressing::Immediate | Addressing::ZeroPage | Addressing::ZeroPageX |
        Addressing::ZeroPageY | Addressing::IndirectX | Addressing::IndirectY | Addressing::Relative => 2,
        Addressing::Absolute | Addressing::AbsoluteX | Addressing::AbsoluteY |
        Addressing::Indirect => 3,
      },
      mode,
      opcode,
    }
  }

  pub fn get(code: u8) -> Self {
    match code {
      // ADC
      0x69 => Instruction::new(Addressing::Immediate, OpCode::ADC),
      0x65 => Instruction::new(Addressing::ZeroPage, OpCode::ADC),
      0x75 => Instruction::new(Addressing::ZeroPageX, OpCode::ADC),
      0x6D => Instruction::new(Addressing::Absolute, OpCode::ADC),
      0x7D => Instruction::new(Addressing::AbsoluteX, OpCode::ADC),
      0x79 => Instruction::new(Addressing::AbsoluteY, OpCode::ADC),
      0x61 => Instruction::new(Addressing::IndirectX, OpCode::ADC),
      0x71 => Instruction::new(Addressing::IndirectY, OpCode::ADC),

      // *ALR
      0x4B => Instruction::new(Addressing::Immediate, OpCode::_ALR),

      // *ANC
      0x0B => Instruction::new(Addressing::Immediate, OpCode::_ANC),
      0x2B => Instruction::new(Addressing::Immediate, OpCode::_ANC),

      // AND
      0x29 => Instruction::new(Addressing::Immediate, OpCode::AND),
      0x25 => Instruction::new(Addressing::ZeroPage, OpCode::AND),
      0x35 => Instruction::new(Addressing::ZeroPageX, OpCode::AND),
      0x2D => Instruction::new(Addressing::Absolute, OpCode::AND),
      0x3D => Instruction::new(Addressing::AbsoluteX, OpCode::AND),
      0x39 => Instruction::new(Addressing::AbsoluteY, OpCode::AND),
      0x21 => Instruction::new(Addressing::IndirectX, OpCode::AND),
      0x31 => Instruction::new(Addressing::IndirectY, OpCode::AND),

      // *ANE
      0x8B => Instruction::new(Addressing::Immediate, OpCode::_ANE),

      // *ARR
      0x6B => Instruction::new(Addressing::Immediate, OpCode::_ARR),

      // ASL
      0x0A => Instruction::new(Addressing::Accumulator, OpCode::ASL),
      0x06 => Instruction::new(Addressing::ZeroPage, OpCode::ASL),
      0x16 => Instruction::new(Addressing::ZeroPageX, OpCode::ASL),
      0x0E => Instruction::new(Addressing::Absolute, OpCode::ASL),
      0x1E => Instruction::new(Addressing::AbsoluteX, OpCode::ASL),

      // BCC
      0x90 => Instruction::new(Addressing::Relative, OpCode::BCC),

      // BCS
      0xB0 => Instruction::new(Addressing::Relative, OpCode::BCS),

      // BEQ
      0xF0 => Instruction::new(Addressing::Relative, OpCode::BEQ),

      // BIT
      0x24 => Instruction::new(Addressing::ZeroPage, OpCode::BIT),
      0x2C => Instruction::new(Addressing::Absolute, OpCode::BIT),

      // BMI
      0x30 => Instruction::new(Addressing::Relative, OpCode::BMI),

      // BNE
      0xD0 => Instruction::new(Addressing::Relative, OpCode::BNE),

      // BPL
      0x10 => Instruction::new(Addressing::Relative, OpCode::BPL),

      // BRK
      0x00 => Instruction::new(Addressing::Implied, OpCode::BRK),

      // BVC
      0x50 => Instruction::new(Addressing::Relative, OpCode::BVC),

      // BVS
      0x70 => Instruction::new(Addressing::Relative, OpCode::BVS),

      // CLC
      0x18 => Instruction::new(Addressing::Implied, OpCode::CLC),

      // CLD
      0xD8 => Instruction::new(Addressing::Implied, OpCode::CLD),

      // CLI
      0x58 => Instruction::new(Addressing::Implied, OpCode::CLI),

      // CLV
      0xB8 => Instruction::new(Addressing::Implied, OpCode::CLV),

      // CMP
      0xC9 => Instruction::new(Addressing::Immediate, OpCode::CMP),
      0xC5 => Instruction::new(Addressing::ZeroPage, OpCode::CMP),
      0xD5 => Instruction::new(Addressing::ZeroPageX, OpCode::CMP),
      0xCD => Instruction::new(Addressing::Absolute, OpCode::CMP),
      0xDD => Instruction::new(Addressing::AbsoluteX, OpCode::CMP),
      0xD9 => Instruction::new(Addressing::AbsoluteY, OpCode::CMP),
      0xC1 => Instruction::new(Addressing::IndirectX, OpCode::CMP),
      0xD1 => Instruction::new(Addressing::IndirectY, OpCode::CMP),

      // CPX
      0xE0 => Instruction::new(Addressing::Immediate, OpCode::CPX),
      0xE4 => Instruction::new(Addressing::ZeroPage, OpCode::CPX),
      0xEC => Instruction::new(Addressing::Absolute, OpCode::CPX),

      // CPY
      0xC0 => Instruction::new(Addressing::Immediate, OpCode::CPY),
      0xC4 => Instruction::new(Addressing::ZeroPage, OpCode::CPY),
      0xCC => Instruction::new(Addressing::Absolute, OpCode::CPY),

      // *DCP
      0xC7 => Instruction::new(Addressing::ZeroPage, OpCode::_DCP),
      0xD7 => Instruction::new(Addressing::ZeroPageX, OpCode::_DCP),
      0xCF => Instruction::new(Addressing::Absolute, OpCode::_DCP),
      0xDF => Instruction::new(Addressing::AbsoluteX, OpCode::_DCP),
      0xDB => Instruction::new(Addressing::AbsoluteY, OpCode::_DCP),
      0xC3 => Instruction::new(Addressing::IndirectX, OpCode::_DCP),
      0xD3 => Instruction::new(Addressing::IndirectY, OpCode::_DCP),

      // DEC
      0xC6 => Instruction::new(Addressing::ZeroPage, OpCode::DEC),
      0xD6 => Instruction::new(Addressing::ZeroPageX, OpCode::DEC),
      0xCE => Instruction::new(Addressing::Absolute, OpCode::DEC),
      0xDE => Instruction::new(Addressing::AbsoluteX, OpCode::DEC),

      // DEX
      0xCA => Instruction::new(Addressing::Implied, OpCode::DEX),

      // DEY
      0x88 => Instruction::new(Addressing::Implied, OpCode::DEY),

      // EOR
      0x49 => Instruction::new(Addressing::Immediate, OpCode::EOR),
      0x45 => Instruction::new(Addressing::ZeroPage, OpCode::EOR),
      0x55 => Instruction::new(Addressing::ZeroPageX, OpCode::EOR),
      0x4D => Instruction::new(Addressing::Absolute, OpCode::EOR),
      0x5D => Instruction::new(Addressing::AbsoluteX, OpCode::EOR),
      0x59 => Instruction::new(Addressing::AbsoluteY, OpCode::EOR),
      0x41 => Instruction::new(Addressing::IndirectX, OpCode::EOR),
      0x51 => Instruction::new(Addressing::IndirectY, OpCode::EOR),

      // INC
      0xE6 => Instruction::new(Addressing::ZeroPage, OpCode::INC),
      0xF6 => Instruction::new(Addressing::ZeroPageX, OpCode::INC),
      0xEE => Instruction::new(Addressing::Absolute, OpCode::INC),
      0xFE => Instruction::new(Addressing::AbsoluteX, OpCode::INC),

      // INX
      0xE8 => Instruction::new(Addressing::Implied, OpCode::INX),

      // INY
      0xC8 => Instruction::new(Addressing::Implied, OpCode::INY),

      // *ISC
      0xE7 => Instruction::new(Addressing::ZeroPage, OpCode::_ISC),
      0xF7 => Instruction::new(Addressing::ZeroPageX, OpCode::_ISC),
      0xEF => Instruction::new(Addressing::Absolute, OpCode::_ISC),
      0xFF => Instruction::new(Addressing::AbsoluteX, OpCode::_ISC),
      0xFB => Instruction::new(Addressing::AbsoluteY, OpCode::_ISC),
      0xE3 => Instruction::new(Addressing::IndirectX, OpCode::_ISC),
      0xF3 => Instruction::new(Addressing::IndirectY, OpCode::_ISC),

      // JAM
      0x02 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x12 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x22 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x32 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x42 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x52 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x62 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x72 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0x92 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0xB2 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0xD2 => Instruction::new(Addressing::Implied, OpCode::JAM),
      0xF2 => Instruction::new(Addressing::Implied, OpCode::JAM),

      // JMP
      0x4C => Instruction::new(Addressing::Absolute, OpCode::JMP),
      0x6C => Instruction::new(Addressing::Indirect, OpCode::JMP),

      // JSR
      0x20 => Instruction::new(Addressing::Absolute, OpCode::JSR),

      // *LAS
      0xBB => Instruction::new(Addressing::AbsoluteY, OpCode::_LAS),

      // *LAX
      0xA7 => Instruction::new(Addressing::ZeroPage, OpCode::_LAX),
      0xB7 => Instruction::new(Addressing::ZeroPageY, OpCode::_LAX),
      0xAF => Instruction::new(Addressing::Absolute, OpCode::_LAX),
      0xBF => Instruction::new(Addressing::AbsoluteY, OpCode::_LAX),
      0xA3 => Instruction::new(Addressing::IndirectX, OpCode::_LAX),
      0xB3 => Instruction::new(Addressing::IndirectY, OpCode::_LAX),

      // LDA
      0xA9 => Instruction::new(Addressing::Immediate, OpCode::LDA),
      0xA5 => Instruction::new(Addressing::ZeroPage, OpCode::LDA),
      0xB5 => Instruction::new(Addressing::ZeroPageX, OpCode::LDA),
      0xAD => Instruction::new(Addressing::Absolute, OpCode::LDA),
      0xBD => Instruction::new(Addressing::AbsoluteX, OpCode::LDA),
      0xB9 => Instruction::new(Addressing::AbsoluteY, OpCode::LDA),
      0xA1 => Instruction::new(Addressing::IndirectX, OpCode::LDA),
      0xB1 => Instruction::new(Addressing::IndirectY, OpCode::LDA),

      // LDX
      0xA2 => Instruction::new(Addressing::Immediate, OpCode::LDX),
      0xA6 => Instruction::new(Addressing::ZeroPage, OpCode::LDX),
      0xB6 => Instruction::new(Addressing::ZeroPageY, OpCode::LDX),
      0xAE => Instruction::new(Addressing::Absolute, OpCode::LDX),
      0xBE => Instruction::new(Addressing::AbsoluteY, OpCode::LDX),

      // LDY
      0xA0 => Instruction::new(Addressing::Immediate, OpCode::LDY),
      0xA4 => Instruction::new(Addressing::ZeroPage, OpCode::LDY),
      0xB4 => Instruction::new(Addressing::ZeroPageX, OpCode::LDY),
      0xAC => Instruction::new(Addressing::Absolute, OpCode::LDY),
      0xBC => Instruction::new(Addressing::AbsoluteX, OpCode::LDY),

      // LSR
      0x4A => Instruction::new(Addressing::Accumulator, OpCode::LSR),
      0x46 => Instruction::new(Addressing::ZeroPage, OpCode::LSR),
      0x56 => Instruction::new(Addressing::ZeroPageX, OpCode::LSR),
      0x4E => Instruction::new(Addressing::Absolute, OpCode::LSR),
      0x5E => Instruction::new(Addressing::AbsoluteX, OpCode::LSR),

      // *LXA
      0xAB => Instruction::new(Addressing::Immediate, OpCode::_LXA),

      // NOP
      0xEA => Instruction::new(Addressing::Implied, OpCode::NOP),

      // *NOP
      0x1A => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0x3A => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0x5A => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0x7A => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0xDA => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0xFA => Instruction::new(Addressing::Implied, OpCode::_NOP),
      0x80 => Instruction::new(Addressing::Immediate, OpCode::_NOP),
      0x82 => Instruction::new(Addressing::Immediate, OpCode::_NOP),
      0x89 => Instruction::new(Addressing::Immediate, OpCode::_NOP),
      0xC2 => Instruction::new(Addressing::Immediate, OpCode::_NOP),
      0xE2 => Instruction::new(Addressing::Immediate, OpCode::_NOP),
      0x04 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP),
      0x44 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP),
      0x64 => Instruction::new(Addressing::ZeroPage, OpCode::_NOP),
      0x14 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0x34 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0x54 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0x74 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0xD4 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0xF4 => Instruction::new(Addressing::ZeroPageX, OpCode::_NOP),
      0x0C => Instruction::new(Addressing::Absolute, OpCode::_NOP),
      0x1C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),
      0x3C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),
      0x5C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),
      0x7C => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),
      0xDC => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),
      0xFC => Instruction::new(Addressing::AbsoluteX, OpCode::_NOP),

      // ORA
      0x09 => Instruction::new(Addressing::Immediate, OpCode::ORA),
      0x05 => Instruction::new(Addressing::ZeroPage, OpCode::ORA),
      0x15 => Instruction::new(Addressing::ZeroPageX, OpCode::ORA),
      0x0D => Instruction::new(Addressing::Absolute, OpCode::ORA),
      0x1D => Instruction::new(Addressing::AbsoluteX, OpCode::ORA),
      0x19 => Instruction::new(Addressing::AbsoluteY, OpCode::ORA),
      0x01 => Instruction::new(Addressing::IndirectX, OpCode::ORA),
      0x11 => Instruction::new(Addressing::IndirectY, OpCode::ORA),

      // PHA
      0x48 => Instruction::new(Addressing::Implied, OpCode::PHA),

      // PHP
      0x08 => Instruction::new(Addressing::Implied, OpCode::PHP),

      // PLA
      0x68 => Instruction::new(Addressing::Implied, OpCode::PLA),

      // PLP
      0x28 => Instruction::new(Addressing::Implied, OpCode::PLP),

      // *RLA
      0x27 => Instruction::new(Addressing::ZeroPage, OpCode::_RLA),
      0x37 => Instruction::new(Addressing::ZeroPageX, OpCode::_RLA),
      0x2F => Instruction::new(Addressing::Absolute, OpCode::_RLA),
      0x3F => Instruction::new(Addressing::AbsoluteX, OpCode::_RLA),
      0x3B => Instruction::new(Addressing::AbsoluteY, OpCode::_RLA),
      0x23 => Instruction::new(Addressing::IndirectX, OpCode::_RLA),
      0x33 => Instruction::new(Addressing::IndirectY, OpCode::_RLA),

      // ROL
      0x2A => Instruction::new(Addressing::Accumulator, OpCode::ROL),
      0x26 => Instruction::new(Addressing::ZeroPage, OpCode::ROL),
      0x36 => Instruction::new(Addressing::ZeroPageX, OpCode::ROL),
      0x2E => Instruction::new(Addressing::Absolute, OpCode::ROL),
      0x3E => Instruction::new(Addressing::AbsoluteX, OpCode::ROL),

      // ROR
      0x6A => Instruction::new(Addressing::Accumulator, OpCode::ROR),
      0x66 => Instruction::new(Addressing::ZeroPage, OpCode::ROR),
      0x76 => Instruction::new(Addressing::ZeroPageX, OpCode::ROR),
      0x6E => Instruction::new(Addressing::Absolute, OpCode::ROR),
      0x7E => Instruction::new(Addressing::AbsoluteX, OpCode::ROR),

      // *RRA
      0x67 => Instruction::new(Addressing::ZeroPage, OpCode::_RRA),
      0x77 => Instruction::new(Addressing::ZeroPageX, OpCode::_RRA),
      0x6F => Instruction::new(Addressing::Absolute, OpCode::_RRA),
      0x7F => Instruction::new(Addressing::AbsoluteX, OpCode::_RRA),
      0x7B => Instruction::new(Addressing::AbsoluteY, OpCode::_RRA),
      0x63 => Instruction::new(Addressing::IndirectX, OpCode::_RRA),
      0x73 => Instruction::new(Addressing::IndirectY, OpCode::_RRA),

      // RTI
      0x40 => Instruction::new(Addressing::Implied, OpCode::RTI),

      // RTS
      0x60 => Instruction::new(Addressing::Implied, OpCode::RTS),

      // *SAX
      0x87 => Instruction::new(Addressing::ZeroPage, OpCode::_SAX),
      0x97 => Instruction::new(Addressing::ZeroPageY, OpCode::_SAX),
      0x8F => Instruction::new(Addressing::Absolute, OpCode::_SAX),
      0x83 => Instruction::new(Addressing::IndirectX, OpCode::_SAX),

      // *SBC
      0xEB => Instruction::new(Addressing::Immediate, OpCode::_SBC),

      // SBC
      0xE9 => Instruction::new(Addressing::Immediate, OpCode::SBC),
      0xE5 => Instruction::new(Addressing::ZeroPage, OpCode::SBC),
      0xF5 => Instruction::new(Addressing::ZeroPageX, OpCode::SBC),
      0xED => Instruction::new(Addressing::Absolute, OpCode::SBC),
      0xFD => Instruction::new(Addressing::AbsoluteX, OpCode::SBC),
      0xF9 => Instruction::new(Addressing::AbsoluteY, OpCode::SBC),
      0xE1 => Instruction::new(Addressing::IndirectX, OpCode::SBC),
      0xF1 => Instruction::new(Addressing::IndirectY, OpCode::SBC),

      // *SBX
      0xCB => Instruction::new(Addressing::Immediate, OpCode::_SBX),

      // SEC
      0x38 => Instruction::new(Addressing::Implied, OpCode::SEC),

      // SED
      0xF8 => Instruction::new(Addressing::Implied, OpCode::SED),

      // SEI
      0x78 => Instruction::new(Addressing::Implied, OpCode::SEI),

      // *SHA
      0x9F => Instruction::new(Addressing::AbsoluteY, OpCode::_SHA),
      0x93 => Instruction::new(Addressing::IndirectY, OpCode::_SHA),

      // *SHX
      0x9C => Instruction::new(Addressing::AbsoluteY, OpCode::_SHX),

      // *SHY
      0x9E => Instruction::new(Addressing::AbsoluteX, OpCode::_SHY),

      // *SLO
      0x07 => Instruction::new(Addressing::ZeroPage, OpCode::_SLO),
      0x17 => Instruction::new(Addressing::ZeroPageX, OpCode::_SLO),
      0x0F => Instruction::new(Addressing::Absolute, OpCode::_SLO),
      0x1F => Instruction::new(Addressing::AbsoluteX, OpCode::_SLO),
      0x1B => Instruction::new(Addressing::AbsoluteY, OpCode::_SLO),
      0x03 => Instruction::new(Addressing::IndirectX, OpCode::_SLO),
      0x13 => Instruction::new(Addressing::IndirectY, OpCode::_SLO),

      // *SRE
      0x47 => Instruction::new(Addressing::ZeroPage, OpCode::_SRE),
      0x57 => Instruction::new(Addressing::ZeroPageX, OpCode::_SRE),
      0x4F => Instruction::new(Addressing::Absolute, OpCode::_SRE),
      0x5F => Instruction::new(Addressing::AbsoluteX, OpCode::_SRE),
      0x5B => Instruction::new(Addressing::AbsoluteY, OpCode::_SRE),
      0x43 => Instruction::new(Addressing::IndirectX, OpCode::_SRE),
      0x53 => Instruction::new(Addressing::IndirectY, OpCode::_SRE),

      // STA
      0x85 => Instruction::new(Addressing::ZeroPage, OpCode::STA),
      0x95 => Instruction::new(Addressing::ZeroPageX, OpCode::STA),
      0x8D => Instruction::new(Addressing::Absolute, OpCode::STA),
      0x9D => Instruction::new(Addressing::AbsoluteX, OpCode::STA),
      0x99 => Instruction::new(Addressing::AbsoluteY, OpCode::STA),
      0x81 => Instruction::new(Addressing::IndirectX, OpCode::STA),
      0x91 => Instruction::new(Addressing::IndirectY, OpCode::STA),

      // STX
      0x86 => Instruction::new(Addressing::ZeroPage, OpCode::STX),
      0x96 => Instruction::new(Addressing::ZeroPageY, OpCode::STX),
      0x8E => Instruction::new(Addressing::Absolute, OpCode::STX),

      // STY
      0x84 => Instruction::new(Addressing::ZeroPage, OpCode::STY),
      0x94 => Instruction::new(Addressing::ZeroPageX, OpCode::STY),
      0x8C => Instruction::new(Addressing::Absolute, OpCode::STY),

      // *TAS
      0x9B => Instruction::new(Addressing::AbsoluteY, OpCode::_TAS),

      // TAX
      0xAA => Instruction::new(Addressing::Implied, OpCode::TAX),

      // TAY
      0xA8 => Instruction::new(Addressing::Implied, OpCode::TAY),

      // TSX
      0xBA => Instruction::new(Addressing::Implied, OpCode::TSX),

      // TXA
      0x8A => Instruction::new(Addressing::Implied, OpCode::TXA),

      // TXS
      0x9A => Instruction::new(Addressing::Implied, OpCode::TXS),

      // TYA
      0x98 => Instruction::new(Addressing::Implied, OpCode::TYA),
    }
  }
}
