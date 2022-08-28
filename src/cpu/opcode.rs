use std::collections::HashMap;
use lazy_static::lazy_static;

pub enum Addressing {
  Immediate, ZeroPage, ZeroPageX, ZeroPageY, Absolute,
  AbsoluteX, AbsoluteY, IndirectX, IndirectY, Implied,
}

pub struct OpCode {
  pub code: u8,
  pub len: u8,
  pub mode: Addressing,
}

impl OpCode {
  pub fn new(code: u8, mode: Addressing) -> Self {
    OpCode {
      code,
      len: match mode {
        Addressing::Implied => 1,
        Addressing::Immediate | Addressing::ZeroPage | Addressing::ZeroPageX |
        Addressing::ZeroPageY | Addressing::IndirectX | Addressing::IndirectY => 2,
        Addressing::Absolute | Addressing::AbsoluteX | Addressing::AbsoluteY => 3,
      },
      mode
    }
  }
}

lazy_static! {
  pub static ref OPCODE_MAP : HashMap<u8, OpCode> = {
    let mut map = HashMap::new();

    // AND
    map.insert(0x29, OpCode::new(0x29, Addressing::Immediate));
    map.insert(0x25, OpCode::new(0x25, Addressing::ZeroPage));
    map.insert(0x35, OpCode::new(0x35, Addressing::ZeroPageX));
    map.insert(0x2D, OpCode::new(0x2D, Addressing::Absolute));
    map.insert(0x3D, OpCode::new(0x3D, Addressing::AbsoluteX));
    map.insert(0x39, OpCode::new(0x39, Addressing::AbsoluteY));
    map.insert(0x21, OpCode::new(0x21, Addressing::IndirectX));
    map.insert(0x31, OpCode::new(0x31, Addressing::IndirectY));

    // ASL
    map.insert(0x0A, OpCode::new(0x0A, Addressing::Implied));
    map.insert(0x06, OpCode::new(0x06, Addressing::ZeroPage));
    map.insert(0x16, OpCode::new(0x16, Addressing::ZeroPageX));
    map.insert(0x0E, OpCode::new(0x0E, Addressing::Absolute));
    map.insert(0x1E, OpCode::new(0x1E, Addressing::AbsoluteX));

    // BCC
    map.insert(0x90, OpCode::new(0x90, Addressing::Immediate));

    // BIT
    map.insert(0x24, OpCode::new(0x24, Addressing::ZeroPage));
    map.insert(0x2C, OpCode::new(0x2C, Addressing::Absolute));

    // BRK
    map.insert(0x00, OpCode::new(0x00, Addressing::Implied));

    // CLC
    map.insert(0x18, OpCode::new(0x18, Addressing::Implied));

    // CLD
    map.insert(0xD8, OpCode::new(0xD8, Addressing::Implied));

    // CLI
    map.insert(0x58, OpCode::new(0x58, Addressing::Implied));

    // CLV
    map.insert(0xB8, OpCode::new(0xB8, Addressing::Implied));

    // CMP
    map.insert(0xC9, OpCode::new(0xC9, Addressing::Immediate));
    map.insert(0xC5, OpCode::new(0xC5, Addressing::ZeroPage));
    map.insert(0xD5, OpCode::new(0xD5, Addressing::ZeroPageX));
    map.insert(0xCD, OpCode::new(0xCD, Addressing::Absolute));
    map.insert(0xDD, OpCode::new(0xDD, Addressing::AbsoluteX));
    map.insert(0xD9, OpCode::new(0xD9, Addressing::AbsoluteY));
    map.insert(0xC1, OpCode::new(0xC1, Addressing::IndirectX));
    map.insert(0xD1, OpCode::new(0xD1, Addressing::IndirectY));

    // CPX
    map.insert(0xE0, OpCode::new(0xE0, Addressing::Immediate));
    map.insert(0xE4, OpCode::new(0xE4, Addressing::ZeroPage));
    map.insert(0xEC, OpCode::new(0xEc, Addressing::Absolute));

    // CPY
    map.insert(0xC0, OpCode::new(0xC0, Addressing::Immediate));
    map.insert(0xC4, OpCode::new(0xC4, Addressing::ZeroPage));
    map.insert(0xCC, OpCode::new(0xCc, Addressing::Absolute));

    // DEC
    map.insert(0xC6, OpCode::new(0xC6, Addressing::ZeroPage));
    map.insert(0xD6, OpCode::new(0xD6, Addressing::ZeroPageX));
    map.insert(0xCE, OpCode::new(0xCE, Addressing::Absolute));
    map.insert(0xDE, OpCode::new(0xDE, Addressing::AbsoluteX));

    // DEX
    map.insert(0xCA, OpCode::new(0xCA, Addressing::Implied));

    // DEY
    map.insert(0x88, OpCode::new(0x88, Addressing::Implied));

    // EOR
    map.insert(0x49, OpCode::new(0x49, Addressing::Immediate));
    map.insert(0x45, OpCode::new(0x45, Addressing::ZeroPage));
    map.insert(0x55, OpCode::new(0x55, Addressing::ZeroPageX));
    map.insert(0x4D, OpCode::new(0x4D, Addressing::Absolute));
    map.insert(0x5D, OpCode::new(0x5D, Addressing::AbsoluteX));
    map.insert(0x59, OpCode::new(0x59, Addressing::AbsoluteY));
    map.insert(0x41, OpCode::new(0x41, Addressing::IndirectX));
    map.insert(0x51, OpCode::new(0x51, Addressing::IndirectX));

    // INC
    map.insert(0xE6, OpCode::new(0xE6, Addressing::ZeroPage));
    map.insert(0xF6, OpCode::new(0xF6, Addressing::ZeroPageX));
    map.insert(0xEE, OpCode::new(0xEE, Addressing::Absolute));
    map.insert(0xFE, OpCode::new(0xFE, Addressing::AbsoluteX));

    // INX
    map.insert(0xE8, OpCode::new(0xE8, Addressing::Implied));

    // INY
    map.insert(0xC8, OpCode::new(0xC8, Addressing::Implied));

    // LDA
    map.insert(0xA9, OpCode::new(0xA9, Addressing::Immediate));
    map.insert(0xA5, OpCode::new(0xA5, Addressing::ZeroPage));
    map.insert(0xB5, OpCode::new(0xB5, Addressing::ZeroPageX));
    map.insert(0xAD, OpCode::new(0xAD, Addressing::Absolute));
    map.insert(0xBD, OpCode::new(0xBD, Addressing::AbsoluteX));
    map.insert(0xB9, OpCode::new(0xB9, Addressing::AbsoluteY));
    map.insert(0xA1, OpCode::new(0xA1, Addressing::IndirectX));
    map.insert(0xB1, OpCode::new(0xB1, Addressing::IndirectY));

    // LDX
    map.insert(0xA2, OpCode::new(0xA2, Addressing::Immediate));
    map.insert(0xA6, OpCode::new(0xA6, Addressing::ZeroPage));
    map.insert(0xB6, OpCode::new(0xB6, Addressing::ZeroPageY));
    map.insert(0xAE, OpCode::new(0xAE, Addressing::Absolute));
    map.insert(0xBE, OpCode::new(0xBE, Addressing::AbsoluteY));

    // LDY
    map.insert(0xA0, OpCode::new(0xA0, Addressing::Immediate));
    map.insert(0xA4, OpCode::new(0xA4, Addressing::ZeroPage));
    map.insert(0xB4, OpCode::new(0xB4, Addressing::ZeroPageX));
    map.insert(0xAC, OpCode::new(0xAC, Addressing::Absolute));
    map.insert(0xBC, OpCode::new(0xBC, Addressing::AbsoluteX));

    // LSR
    map.insert(0x4A, OpCode::new(0x4A, Addressing::Implied));
    map.insert(0x46, OpCode::new(0x46, Addressing::ZeroPage));
    map.insert(0x56, OpCode::new(0x56, Addressing::ZeroPageX));
    map.insert(0x4E, OpCode::new(0x4E, Addressing::Absolute));
    map.insert(0x5E, OpCode::new(0x5E, Addressing::AbsoluteX));

    // NOP
    map.insert(0xEA, OpCode::new(0xEA, Addressing::Implied));

    // ORA
    map.insert(0x09, OpCode::new(0x09, Addressing::Immediate));
    map.insert(0x05, OpCode::new(0x05, Addressing::ZeroPage));
    map.insert(0x15, OpCode::new(0x15, Addressing::ZeroPageX));
    map.insert(0x0D, OpCode::new(0x0D, Addressing::Absolute));
    map.insert(0x1D, OpCode::new(0x1D, Addressing::AbsoluteX));
    map.insert(0x19, OpCode::new(0x19, Addressing::AbsoluteY));
    map.insert(0x01, OpCode::new(0x01, Addressing::IndirectX));
    map.insert(0x11, OpCode::new(0x11, Addressing::IndirectX));

    // ROL
    map.insert(0x2A, OpCode::new(0x2A, Addressing::Implied));
    map.insert(0x26, OpCode::new(0x26, Addressing::ZeroPage));
    map.insert(0x36, OpCode::new(0x36, Addressing::ZeroPageX));
    map.insert(0x2E, OpCode::new(0x2E, Addressing::Absolute));
    map.insert(0x3E, OpCode::new(0x3E, Addressing::AbsoluteX));

    // ROR
    map.insert(0x6A, OpCode::new(0x2A, Addressing::Implied));
    map.insert(0x66, OpCode::new(0x26, Addressing::ZeroPage));
    map.insert(0x76, OpCode::new(0x36, Addressing::ZeroPageX));
    map.insert(0x6E, OpCode::new(0x2E, Addressing::Absolute));
    map.insert(0x7E, OpCode::new(0x3E, Addressing::AbsoluteX));

    // SEC
    map.insert(0x38, OpCode::new(0x38, Addressing::Implied));

    // SED
    map.insert(0xF8, OpCode::new(0xF8, Addressing::Implied));

    // SEI
    map.insert(0x78, OpCode::new(0x78, Addressing::Implied));

    // STA
    map.insert(0x85, OpCode::new(0x85, Addressing::ZeroPage));
    map.insert(0x95, OpCode::new(0x95, Addressing::ZeroPageX));
    map.insert(0x8D, OpCode::new(0x8D, Addressing::Absolute));
    map.insert(0x9D, OpCode::new(0x9D, Addressing::AbsoluteX));
    map.insert(0x99, OpCode::new(0x99, Addressing::AbsoluteY));

    // STX
    map.insert(0x86, OpCode::new(0x86, Addressing::ZeroPage));
    map.insert(0x96, OpCode::new(0x96, Addressing::ZeroPageY));
    map.insert(0x8E, OpCode::new(0x8E, Addressing::Absolute));

    // STY
    map.insert(0x84, OpCode::new(0x84, Addressing::ZeroPage));
    map.insert(0x94, OpCode::new(0x94, Addressing::ZeroPageX));
    map.insert(0x8C, OpCode::new(0x8C, Addressing::Absolute));

    // TAX
    map.insert(0xAA, OpCode::new(0xAA, Addressing::Implied));

    // TAY
    map.insert(0xA8, OpCode::new(0xA8, Addressing::Implied));

    // TSX
    map.insert(0xBA, OpCode::new(0xBA, Addressing::Implied));

    // TXA
    map.insert(0x8A, OpCode::new(0x8A, Addressing::Implied));

    // TXS
    map.insert(0x9A, OpCode::new(0x9A, Addressing::Implied));

    // TYA
    map.insert(0x98, OpCode::new(0x98, Addressing::Implied));

    map
  };
}