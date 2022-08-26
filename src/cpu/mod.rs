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

        // NOP
        0xEA => continue,

        // LDA
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
          self.lda(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }

        // TAX
        0xAA => self.tax(),

        // TAY
        0xA8 => self.tay(),

        // INX
        0xE8 => self.inx(),

        // INY
        0xC8 => self.iny(),

        // DEX
        0xCA => self.dex(),

        // DEY
        0x88 => self.dey(),

        // STA
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
          self.sta(&opcode.mode);
          self.increment_pc((opcode.len - 1) as u16);
        }
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

    if res & 0b1000_0000 != 0 {
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

  fn lda(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(RegisterType::A, val);

    self.update_zero_negative(val);
  }

  fn tax(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::A));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn tay(&mut self) {
    self.registers.set(RegisterType::Y, self.registers.get(RegisterType::A));
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

  fn dex(&mut self) {
    self.registers.set(RegisterType::X, self.registers.get(RegisterType::X).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(RegisterType::X));
  }

  fn dey(&mut self) {
    self.registers.set(RegisterType::Y, self.registers.get(RegisterType::Y).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(RegisterType::Y));
  }

  fn sta(&mut self, mode: &Addressing) {
    self.memory.write(self.get_operand_addr(mode), self.registers.get(RegisterType::A));

  }

}