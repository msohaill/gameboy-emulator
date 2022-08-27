pub mod register;
pub mod memory;
pub mod opcode;

use register::{Registers, RegisterType};
use memory::Memory;
use opcode::{Addressing, OPCODE_MAP};

pub struct CPU {
  pub registers: Registers,
  pub memory: Memory,
}

impl CPU {
  pub fn new() -> Self {
    CPU {
      registers: Registers::new(),
      memory: Memory::new(),
    }
  }

  fn increment_pc(&mut self, i: u16) {
    self.registers.set_pc(self.registers.get_pc().wrapping_add(i));
  }

  fn reset(&mut self) {
    self.registers = Registers::new();
    self.registers.set_pc(self.memory.readu16(0xFFFC));
  }

  fn load(&mut self, program: Vec<u8>) {
    self.memory.load(0x8000, program);
    self.memory.writeu16(0xFFFC, 0x8000);
  }

  pub fn start(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run();
  }

  fn run(&mut self) {
    loop {
      let opcode = OPCODE_MAP
        .get(&self.memory.read(self.registers.get_pc()))
        .expect("opcode not recognized.");
      self.increment_pc(1);

      match opcode.code {
        // BRK
        0x00 => return,

        // CLC
        0x18 => self.clc(),

        // CLD
        0xD8 => self.cld(),

        // CLI
        0x58 => self.cli(),

        // CLV
        0xB8 => self.clv(),

        // DEX
        0xCA => self.dex(),

        // DEY
        0x88 => self.dey(),

        // INX
        0xE8 => self.inx(),

        // INY
        0xC8 => self.iny(),

        // LDA
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
          self.lda(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // LDX
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
          self.ldx(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // LDY
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
          self.ldy(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // NOP
        0xEA => continue,

        // SEC
        0x38 => self.sec(),

        // SED
        0xF8 => self.sed(),

        // SEI
        0x78 => self.sei(),

        // STA
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
          self.sta(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // STX
        0x86 | 0x96 | 0x8E => {
          self.stx(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // STY
        0x84 | 0x94 | 0x8C => {
          self.sty(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // TAX
        0xAA => self.tax(),

        // TAY
        0xA8 => self.tay(),

        // TSX
        0xBA => self.tsx(),

        // TXA
        0x8A => self.txa(),

        // TXS
        0x9A => self.txs(),

        // TYA
        0x98 => self.tya(),

        _ => todo!(""),
      }
    }
  }

  fn get_operand_addr(&self, mode: &Addressing) -> u16 {
    match mode {
      Addressing::Immediate =>
        self.registers.get_pc(),
      Addressing::ZeroPage =>
        self.memory.read(self.registers.get_pc()) as u16,
      Addressing::Absolute =>
        self.memory.readu16(self.registers.get_pc()),
      Addressing::ZeroPageX =>
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(RegisterType::X)) as u16,
      Addressing::ZeroPageY =>
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(RegisterType::Y)) as u16,
      Addressing::AbsoluteX =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(RegisterType::X) as u16),
      Addressing::AbsoluteY =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(RegisterType::Y) as u16),
      Addressing::IndirectX => {
        let addr = self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(RegisterType::X));
        self.memory.readu16(addr as u16)
      }
      Addressing::IndirectY => {
        let addr = self.memory.read(self.registers.get_pc());
        self.memory.readu16(addr as u16).wrapping_add(self.registers.get(RegisterType::Y) as u16)
      }
      Addressing::Implied => panic!("Implied addressing doesn't yield an address."),
    }
  }

  fn update_zero_negative(&mut self, res: u8) {
    if res == 0 {
      self.registers.set_flag(1, true);
    } else {
      self.registers.set_flag(1, false);
    }

    if (res >> 7) & 0b1 != 0 {
      self.registers.set_flag(7, true);
    } else {
      self.registers.set_flag(7, false);
    }
  }

  // fn check_overflow(&mut self, x: u8, y: u8) {
  //   match x.checked_add(y) {
  //     None => self.registers.set_flag(6, false),
  //     Some(_) => self.registers.set_flag(6, true),
  //   }
  // }

  fn clc(&mut self) {
    self.registers.set_flag(0, false);
  }

  fn cld(&mut self) {
    self.registers.set_flag(3, false);
  }

  fn cli(&mut self) {
    self.registers.set_flag(2, false);
  }

  fn clv(&mut self) {
    self.registers.set_flag(6, false);
  }

  fn dex(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::X).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn dey(&mut self) {
    self.registers.set(RegisterType::Y, self.registers.get(RegisterType::Y).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(RegisterType::Y));
  }

  fn inx(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::X).wrapping_add(1));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn iny(&mut self) {
    self.registers.set(RegisterType::Y, self.registers.get(RegisterType::Y).wrapping_add(1));
    self.update_zero_negative(self.registers.get(RegisterType::Y));
  }

  fn lda(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(RegisterType::A, val);

    self.update_zero_negative(val);
  }

  fn ldx(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(RegisterType::X, val);

    self.update_zero_negative(val);
  }

  fn ldy(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(RegisterType::Y, val);

    self.update_zero_negative(val);
  }

  fn sec(&mut self) {
    self.registers.set_flag(0, true);
  }

  fn sed(&mut self) {
    self.registers.set_flag(3, true);
  }

  fn sei(&mut self) {
    self.registers.set_flag(2, true);
  }

  fn sta(&mut self, mode: &Addressing) {
    self.memory.write(self.get_operand_addr(mode), self.registers.get(RegisterType::A));
  }

  fn stx(&mut self, mode: &Addressing) {
    self.memory.write(self.get_operand_addr(mode), self.registers.get(RegisterType::X));
  }

  fn sty(&mut self, mode: &Addressing) {
    self.memory.write(self.get_operand_addr(mode), self.registers.get(RegisterType::Y));
  }

  fn tax(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::A));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn tay(&mut self) {
    self.registers.set(RegisterType::Y, self.registers.get(RegisterType::A));
    self.update_zero_negative(self.registers.get(RegisterType::Y));
  }

  fn tsx(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::SP));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn txa(&mut self) {
    self.registers.set(RegisterType::A, self.registers.get(RegisterType::X));
    self.update_zero_negative(self.registers.get(RegisterType::A));
  }

  fn txs(&mut self) {
    self.registers.set(RegisterType::SP, self.registers.get(RegisterType::X));
    self.update_zero_negative(self.registers.get(RegisterType::SP));
  }

  fn tya(&mut self) {
    self.registers.set(RegisterType::A, self.registers.get(RegisterType::Y));
    self.update_zero_negative(self.registers.get(RegisterType::A));
  }

}