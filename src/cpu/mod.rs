pub mod register;
pub mod memory;
pub mod opcode;

use register::{Registers, Register, Flag};
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
        // AND
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),

        // ASL
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(&opcode.mode),

        // BIT
        0x24 | 0x2C => self.bit(&opcode.mode),

        // BRK
        0x00 => return,

        // CLC
        0x18 => self.registers.set_flag(&Flag::Carry, false),

        // CLD
        0xD8 => self.registers.set_flag(&Flag::Decimal, false),

        // CLI
        0x58 => self.registers.set_flag(&Flag::InterruptDisable, false),

        // CLV
        0xB8 => self.registers.set_flag(&Flag::Overflow, false),

        // CMP
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.compare(&opcode.mode, &Register::A),

        // CPX
        0xE0 | 0xE4 | 0xEC => self.compare(&opcode.mode, &Register::X),

        // CPY
        0xC0 | 0xC4 | 0xCC => self.compare(&opcode.mode, &Register::Y),

        // DEC
        0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode),

        // DEX
        0xCA => self.decrement_reg(&Register::X),

        // DEY
        0x88 => self.decrement_reg(&Register::Y),

        // EOR
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&opcode.mode),

        // INC
        0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(&opcode.mode),

        // INX
        0xE8 => self.increment_reg(&Register::X),

        // INY
        0xC8 => self.increment_reg(&Register::Y),

        // LDA
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.load_reg(&opcode.mode, &Register::A),

        // LDX
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.load_reg(&opcode.mode, &Register::X),

        // LDY
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.load_reg(&opcode.mode, &Register::Y),

        // LSR
        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(&opcode.mode),

        // NOP
        0xEA => (),

        // ORA
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&opcode.mode),

        // SEC
        0x38 => self.registers.set_flag(&Flag::Carry, true),

        // SED
        0xF8 => self.registers.set_flag(&Flag::Decimal, true),

        // SEI
        0x78 => self.registers.set_flag(&Flag::InterruptDisable, true),

        // STA
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.store_reg(&opcode.mode, &Register::A),

        // STX
        0x86 | 0x96 | 0x8E => self.store_reg(&opcode.mode, &Register::X),

        // STY
        0x84 | 0x94 | 0x8C => self.store_reg(&opcode.mode, &Register::Y),

        // TAX
        0xAA => self.transfer_reg(&Register::A, &Register::X),

        // TAY
        0xA8 => self.transfer_reg(&Register::A, &Register::Y),

        // TSX
        0xBA => self.transfer_reg(&Register::SP, &Register::X),

        // TXA
        0x8A => self.transfer_reg(&Register::X, &Register::A),

        // TXS
        0x9A => self.transfer_reg(&Register::X, &Register::SP),

        // TYA
        0x98 => self.transfer_reg(&Register::Y, &Register::A),

        _ => todo!(""),
      }

      self.increment_pc((opcode.len - 1) as u16);
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
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(&Register::X)) as u16,
      Addressing::ZeroPageY =>
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(&Register::Y)) as u16,
      Addressing::AbsoluteX =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(&Register::X) as u16),
      Addressing::AbsoluteY =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(&Register::Y) as u16),
      Addressing::IndirectX => {
        let addr = self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(&Register::X));
        self.memory.readu16(addr as u16)
      }
      Addressing::IndirectY => {
        let addr = self.memory.read(self.registers.get_pc());
        self.memory.readu16(addr as u16).wrapping_add(self.registers.get(&Register::Y) as u16)
      }
      Addressing::Implied => panic!("Implied addressing doesn't yield an address."),
    }
  }

  fn update_zero_negative(&mut self, res: u8) {
    if res == 0 {
      self.registers.set_flag(&Flag::Zero, true);
    } else {
      self.registers.set_flag(&Flag::Zero, false);
    }

    if (res >> 7) & 0b1 != 0 {
      self.registers.set_flag(&Flag::Negative, true);
    } else {
      self.registers.set_flag(&Flag::Negative, false);
    }
  }

  // fn check_overflow(&mut self, x: u8, y: u8) {
  //   match x.checked_add(y) {
  //     None => self.registers.set_flag(6, false),
  //     Some(_) => self.registers.set_flag(6, true),
  //   }
  // }

  fn and(&mut self, mode: &Addressing) {
    let byte = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(&Register::A, self.registers.get(&Register::A) & byte);

    self.update_zero_negative(self.registers.get(&Register::A));
  }

  fn asl(&mut self, mode: &Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(&Register::A);
        self.registers.set_flag(&Flag::Carry, (data >> 7) & 0b1 != 0);

        self.registers.set(&Register::A, data << 1);
        self.update_zero_negative(self.registers.get(&Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        self.registers.set_flag(&Flag::Carry, (data >> 7) & 0b1 != 0);

        self.memory.write(addr, data << 1);
        self.update_zero_negative(self.memory.read(addr));
      }
    }
  }

  fn bit(&mut self, mode: &Addressing) {
    let data = self.memory.read(self.get_operand_addr(mode));

    self.registers.set_flag(&Flag::Zero, data & self.registers.get(&Register::A) == 0);
    self.registers.set_flag(&Flag::Overflow, (data >> 6) & 0b1 != 0);
    self.registers.set_flag(&Flag::Negative, (data >> 7) & 0b1 != 0);
  }

  fn compare(&mut self, mode: &Addressing, reg: &Register) {
    let data = self.memory.read(self.get_operand_addr(mode));
    let comparable = self.registers.get(reg);

    self.registers.set_flag(&Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data))
  }

  fn dec(&mut self, mode: &Addressing) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);

    self.memory.write(addr, data.wrapping_sub(1));
    self.update_zero_negative(self.memory.read(addr));
  }

  fn decrement_reg(&mut self, reg: &Register) {
    self.registers.set(reg, self.registers.get(reg).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(reg));
  }

  fn eor(&mut self, mode: &Addressing) {
    let byte = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(&Register::A, self.registers.get(&Register::A) ^ byte);

    self.update_zero_negative(self.registers.get(&Register::A));
  }

  fn inc(&mut self, mode: &Addressing) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);

    self.memory.write(addr, data.wrapping_add(1));
    self.update_zero_negative(self.memory.read(addr));
  }

  fn increment_reg(&mut self, reg: &Register) {
    self.registers.set(reg, self.registers.get(reg).wrapping_add(1));
    self.update_zero_negative(self.registers.get(reg));
  }

  fn load_reg(&mut self, mode: &Addressing, reg: &Register) {
    let val = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(reg, val);

    self.update_zero_negative(val);
  }

  fn lsr(&mut self, mode: &Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(&Register::A);
        self.registers.set_flag(&Flag::Carry, data & 0b1 != 0);

        self.registers.set(&Register::A, data >> 1);
        self.update_zero_negative(self.registers.get(&Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        self.registers.set_flag(&Flag::Carry, data & 0b1 != 0);

        self.memory.write(addr, data >> 1);
        self.update_zero_negative(self.memory.read(addr));
      }
    }
  }

  fn ora(&mut self, mode: &Addressing) {
    let byte = self.memory.read(self.get_operand_addr(mode));
    self.registers.set(&Register::A, self.registers.get(&Register::A) | byte);

    self.update_zero_negative(self.registers.get(&Register::A));
  }

  fn store_reg(&mut self, mode: &Addressing, reg: &Register) {
    self.memory.write(self.get_operand_addr(mode), self.registers.get(reg));
  }

  fn transfer_reg(&mut self, from: &Register, to: &Register) {
    self.registers.set(to, self.registers.get(from));
    self.update_zero_negative(self.registers.get(to));
  }
}