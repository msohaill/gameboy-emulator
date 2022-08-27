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
  pub fn new(code: u8, len: u8, mode: Addressing) -> Self {
    OpCode { code, len, mode }
  }
}

lazy_static! {
  pub static ref OPCODE_MAP : HashMap<u8, OpCode> = {
    let mut map = HashMap::new();

    // BRK
    map.insert(0x00, OpCode::new(0x00, 1, Addressing::Implied));

    // CLC
    map.insert(0x18, OpCode::new(0x18, 1, Addressing::Implied));

    // CLD
    map.insert(0xD8, OpCode::new(0xD8, 1, Addressing::Implied));

    // CLI
    map.insert(0x58, OpCode::new(0x58, 1, Addressing::Implied));

    // CLV
    map.insert(0xB8, OpCode::new(0xB8, 1, Addressing::Implied));

    // DEX
    map.insert(0xCA, OpCode::new(0xCA, 1, Addressing::Implied));

    // DEY
    map.insert(0x88, OpCode::new(0x88, 1, Addressing::Implied));

    // INX
    map.insert(0xE8, OpCode::new(0xE8, 1, Addressing::Implied));

    // INY
    map.insert(0xC8, OpCode::new(0xC8, 1, Addressing::Implied));

    // LDA
    map.insert(0xA9, OpCode::new(0xA9, 2, Addressing::Immediate));
    map.insert(0xA5, OpCode::new(0xA5, 2, Addressing::ZeroPage));
    map.insert(0xB5, OpCode::new(0xB5, 2, Addressing::ZeroPageX));
    map.insert(0xAD, OpCode::new(0xAD, 3, Addressing::Absolute));
    map.insert(0xBD, OpCode::new(0xBD, 3, Addressing::AbsoluteX));
    map.insert(0xB9, OpCode::new(0xB9, 3, Addressing::AbsoluteY));
    map.insert(0xA1, OpCode::new(0xA1, 2, Addressing::IndirectX));
    map.insert(0xB1, OpCode::new(0xB1, 2, Addressing::IndirectY));

    // LDX
    map.insert(0xA2, OpCode::new(0xA2, 2, Addressing::Immediate));
    map.insert(0xA6, OpCode::new(0xA6, 2, Addressing::ZeroPage));
    map.insert(0xB6, OpCode::new(0xB6, 2, Addressing::ZeroPageY));
    map.insert(0xAE, OpCode::new(0xAE, 3, Addressing::Absolute));
    map.insert(0xBE, OpCode::new(0xBE, 3, Addressing::AbsoluteY));

    // LDY
    map.insert(0xA0, OpCode::new(0xA0, 2, Addressing::Immediate));
    map.insert(0xA4, OpCode::new(0xA4, 2, Addressing::ZeroPage));
    map.insert(0xB4, OpCode::new(0xB4, 2, Addressing::ZeroPageX));
    map.insert(0xAC, OpCode::new(0xAC, 3, Addressing::Absolute));
    map.insert(0xBC, OpCode::new(0xBC, 3, Addressing::AbsoluteX));

    // NOP
    map.insert(0xEA, OpCode::new(0xEA, 1, Addressing::Implied));

    // SEC
    map.insert(0x38, OpCode::new(0x38, 1, Addressing::Implied));

    // SED
    map.insert(0xF8, OpCode::new(0xF8, 1, Addressing::Implied));

    // SEI
    map.insert(0x78, OpCode::new(0x78, 1, Addressing::Implied));

    // STA
    map.insert(0x85, OpCode::new(0x85, 2, Addressing::ZeroPage));
    map.insert(0x95, OpCode::new(0x95, 2, Addressing::ZeroPageX));
    map.insert(0x8D, OpCode::new(0x8D, 3, Addressing::Absolute));
    map.insert(0x9D, OpCode::new(0x9D, 3, Addressing::AbsoluteX));
    map.insert(0x99, OpCode::new(0x99, 3, Addressing::AbsoluteY));

    // STX
    map.insert(0x86, OpCode::new(0x86, 2, Addressing::ZeroPage));
    map.insert(0x96, OpCode::new(0x96, 2, Addressing::ZeroPageY));
    map.insert(0x8E, OpCode::new(0x8E, 3, Addressing::Absolute));

    // STY
    map.insert(0x84, OpCode::new(0x84, 2, Addressing::ZeroPage));
    map.insert(0x94, OpCode::new(0x94, 2, Addressing::ZeroPageX));
    map.insert(0x8C, OpCode::new(0x8C, 3, Addressing::Absolute));

    // TAX
    map.insert(0xAA, OpCode::new(0xAA, 1, Addressing::Implied));

    // TAY
    map.insert(0xA8, OpCode::new(0xA8, 1, Addressing::Implied));

    // TSX
    map.insert(0xBA, OpCode::new(0xBA, 1, Addressing::Implied));

    // TXA
    map.insert(0x8A, OpCode::new(0x8A, 1, Addressing::Implied));

    // TXS
    map.insert(0x9A, OpCode::new(0x9A, 1, Addressing::Implied));

    // TYA
    map.insert(0x98, OpCode::new(0x9A, 1, Addressing::Implied));

    map
  };
}