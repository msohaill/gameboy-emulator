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

    // AND
    map.insert(0x29, OpCode::new(0x29, 2, Addressing::Immediate));
    map.insert(0x25, OpCode::new(0x25, 2, Addressing::ZeroPage));
    map.insert(0x35, OpCode::new(0x35, 2, Addressing::ZeroPageX));
    map.insert(0x2D, OpCode::new(0x2D, 3, Addressing::Absolute));
    map.insert(0x3D, OpCode::new(0x3D, 3, Addressing::AbsoluteX));
    map.insert(0x39, OpCode::new(0x39, 3, Addressing::AbsoluteY));
    map.insert(0x21, OpCode::new(0x21, 2, Addressing::IndirectX));
    map.insert(0x31, OpCode::new(0x31, 2, Addressing::IndirectY));

    // ASL
    map.insert(0x0A, OpCode::new(0x0A, 1, Addressing::Implied));
    map.insert(0x06, OpCode::new(0x06, 2, Addressing::ZeroPage));
    map.insert(0x16, OpCode::new(0x16, 2, Addressing::ZeroPageX));
    map.insert(0x0E, OpCode::new(0x0E, 3, Addressing::Absolute));
    map.insert(0x1E, OpCode::new(0x1E, 3, Addressing::AbsoluteX));

    // BIT
    map.insert(0x24, OpCode::new(0x24, 2, Addressing::ZeroPage));
    map.insert(0x2C, OpCode::new(0x2C, 3, Addressing::Absolute));

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

    // CMP
    map.insert(0xC9, OpCode::new(0xC9, 2, Addressing::Immediate));
    map.insert(0xC5, OpCode::new(0xC5, 2, Addressing::ZeroPage));
    map.insert(0xD5, OpCode::new(0xD5, 2, Addressing::ZeroPageX));
    map.insert(0xCD, OpCode::new(0xCD, 3, Addressing::Absolute));
    map.insert(0xDD, OpCode::new(0xDD, 3, Addressing::AbsoluteX));
    map.insert(0xD9, OpCode::new(0xD9, 3, Addressing::AbsoluteY));
    map.insert(0xC1, OpCode::new(0xC1, 2, Addressing::IndirectX));
    map.insert(0xD1, OpCode::new(0xD1, 2, Addressing::IndirectY));

    // CPX
    map.insert(0xE0, OpCode::new(0xE0, 2, Addressing::Immediate));
    map.insert(0xE4, OpCode::new(0xE4, 2, Addressing::ZeroPage));
    map.insert(0xEC, OpCode::new(0xEc, 3, Addressing::Absolute));

    // CPY
    map.insert(0xC0, OpCode::new(0xC0, 2, Addressing::Immediate));
    map.insert(0xC4, OpCode::new(0xC4, 2, Addressing::ZeroPage));
    map.insert(0xCC, OpCode::new(0xCc, 3, Addressing::Absolute));

    // DEC
    map.insert(0xC6, OpCode::new(0xC6, 2, Addressing::ZeroPage));
    map.insert(0xD6, OpCode::new(0xD6, 2, Addressing::ZeroPageX));
    map.insert(0xCE, OpCode::new(0xCE, 3, Addressing::Absolute));
    map.insert(0xDE, OpCode::new(0xDE, 3, Addressing::AbsoluteX));

    // DEX
    map.insert(0xCA, OpCode::new(0xCA, 1, Addressing::Implied));

    // DEY
    map.insert(0x88, OpCode::new(0x88, 1, Addressing::Implied));

    // EOR
    map.insert(0x49, OpCode::new(0x49, 2, Addressing::Immediate));
    map.insert(0x45, OpCode::new(0x45, 2, Addressing::ZeroPage));
    map.insert(0x55, OpCode::new(0x55, 2, Addressing::ZeroPageX));
    map.insert(0x4D, OpCode::new(0x4D, 3, Addressing::Absolute));
    map.insert(0x5D, OpCode::new(0x5D, 3, Addressing::AbsoluteX));
    map.insert(0x59, OpCode::new(0x59, 3, Addressing::AbsoluteY));
    map.insert(0x41, OpCode::new(0x41, 2, Addressing::IndirectX));
    map.insert(0x51, OpCode::new(0x51, 2, Addressing::IndirectX));

    // INC
    map.insert(0xE6, OpCode::new(0xE6, 2, Addressing::ZeroPage));
    map.insert(0xF6, OpCode::new(0xF6, 2, Addressing::ZeroPageX));
    map.insert(0xEE, OpCode::new(0xEE, 3, Addressing::Absolute));
    map.insert(0xFE, OpCode::new(0xFE, 3, Addressing::AbsoluteX));

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

    // LSR
    map.insert(0x4A, OpCode::new(0x4A, 1, Addressing::Implied));
    map.insert(0x46, OpCode::new(0x46, 2, Addressing::ZeroPage));
    map.insert(0x56, OpCode::new(0x56, 2, Addressing::ZeroPageX));
    map.insert(0x4E, OpCode::new(0x4E, 3, Addressing::Absolute));
    map.insert(0x5E, OpCode::new(0x5E, 3, Addressing::AbsoluteX));

    // NOP
    map.insert(0xEA, OpCode::new(0xEA, 1, Addressing::Implied));

    // ORA
    map.insert(0x09, OpCode::new(0x09, 2, Addressing::Immediate));
    map.insert(0x05, OpCode::new(0x05, 2, Addressing::ZeroPage));
    map.insert(0x15, OpCode::new(0x15, 2, Addressing::ZeroPageX));
    map.insert(0x0D, OpCode::new(0x0D, 3, Addressing::Absolute));
    map.insert(0x1D, OpCode::new(0x1D, 3, Addressing::AbsoluteX));
    map.insert(0x19, OpCode::new(0x19, 3, Addressing::AbsoluteY));
    map.insert(0x01, OpCode::new(0x01, 2, Addressing::IndirectX));
    map.insert(0x11, OpCode::new(0x11, 2, Addressing::IndirectX));

    // ROL
    map.insert(0x2A, OpCode::new(0x2A, 1, Addressing::Implied));
    map.insert(0x26, OpCode::new(0x26, 2, Addressing::ZeroPage));
    map.insert(0x36, OpCode::new(0x36, 2, Addressing::ZeroPageX));
    map.insert(0x2E, OpCode::new(0x2E, 3, Addressing::Absolute));
    map.insert(0x3E, OpCode::new(0x3E, 3, Addressing::AbsoluteX));

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
    map.insert(0x98, OpCode::new(0x98, 1, Addressing::Implied));

    map
  };
}