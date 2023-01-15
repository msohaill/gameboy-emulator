pub mod register;
pub mod opcode;

use register::{Registers, Register, Flag};
use super::memory::rom::ROM;

use super::memory::Memory;
use opcode::{Addressing, OPCODE_MAP};

pub struct CPU {
  pub registers: Registers,
  pub memory: Memory,
}

impl CPU {
  const STACK_START: u16 = 0x0100;

  pub fn new(cartridge: ROM) -> Self {
    CPU {
      registers: Registers::new(),
      memory: Memory::new(cartridge),
    }
  }

  fn reset(&mut self) {
    self.registers = Registers::new();
    self.registers.set_pc(self.memory.readu16(0xFFFC)); // Check after
    // self.registers.set_pc(0x0600);
  }

  pub fn start<F>(&mut self, callback: F)
  where
    F: FnMut(&mut CPU),
  {
    self.reset();
    self.run(callback);
  }

  fn stack_push(&mut self, data: u8) {
    if self.registers.get(&Register::SP) == 0 {
      panic!("Attempted to store beyone stack capacity.");
    }

    self.memory.write(CPU::STACK_START.wrapping_add(self.registers.get(&Register::SP) as u16), data);
    self.registers.set(&Register::SP, self.registers.get(&Register::SP).wrapping_sub(1));
  }

  fn stack_pop(&mut self) -> u8 {
    if self.registers.get(&Register::SP) == 0x1E {
      panic!("Attempted to pop empty stack");
    }

    self.registers.set(&Register::SP, self.registers.get(&Register::SP).wrapping_add(1));
    self.memory.read(CPU::STACK_START.wrapping_add(self.registers.get(&Register::SP) as u16))
  }

  fn stack_pushu16(&mut self, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xFF) as u8;

    self.stack_push(hi);
    self.stack_push(lo);
  }

  fn stack_popu16(&mut self) -> u16 {
    let lo = self.stack_pop() as u16;
    let hi = self.stack_pop() as u16;

    (hi << 8) | lo
  }

  fn increment_pc(&mut self, i: u16) {
    self.registers.set_pc(self.registers.get_pc().wrapping_add(i));
  }

  fn run<F>(&mut self, mut callback: F)
  where
    F: FnMut(&mut CPU),
  {
    loop {
      callback(self);

      let opcode = OPCODE_MAP
        .get(&self.memory.read(self.registers.get_pc()))
        .expect("opcode not recognized.");
      self.increment_pc(1);
      let current_pc = self.registers.get_pc();

      match opcode.code {
        // ADC
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&opcode.mode),

        // AND
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&opcode.mode),

        // ASL
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(&opcode.mode),

        // BCC
        0x90 => self.branch(!self.registers.get_flag(&Flag::Carry)),

        // BCS
        0xB0 => self.branch(self.registers.get_flag(&Flag::Carry)),

        // BEQ
        0xF0 => self.branch(self.registers.get_flag(&Flag::Zero)),

        // BIT
        0x24 | 0x2C => self.bit(&opcode.mode),

        // BMI
        0x30 => self.branch(self.registers.get_flag(&Flag::Negative)),

        // BNE
        0xD0 => self.branch(!self.registers.get_flag(&Flag::Zero)),

        // BPL
        0x10 => self.branch(!self.registers.get_flag(&Flag::Negative)),

        // BRK
        0x00 => return,

        // BVC
        0x50 => self.branch(!self.registers.get_flag(&Flag::Overflow)),

        // BCC
        0x70 => self.branch(self.registers.get_flag(&Flag::Carry)),

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

        // JMP
        0x4C | 0x6C => self.jmp(&opcode.mode),

        // JSR
        0x20 => self.jsr(),

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

        // PHA
        0x48 => self.stack_push(self.registers.get(&Register::A)),

        // PHP
        0x08 => self.php(),

        // PLA
        0x68 => self.pla(),

        // PLP
        0x28 => self.plp(),

        // ROL
        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(&opcode.mode),

        // ROR
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ror(&opcode.mode),

        // RTI
        0x40 => self.rti(),

        // RTS
        0x60 => self.rts(),

        // SBC
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&opcode.mode),

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

      if current_pc == self.registers.get_pc() {
        self.increment_pc((opcode.len - 1) as u16);
      }
    }
  }

  fn get_operand_addr(&self, mode: &Addressing) -> u16 {
    match mode {
      Addressing::Immediate =>
        self.registers.get_pc(),
      Addressing::ZeroPage =>
        self.memory.read(self.registers.get_pc()) as u16,
      Addressing::Absolute | Addressing::AbsoluteIndirect =>
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

  fn adc(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode));

    let raw_res = (self.registers.get(&Register::A) as u16)
      .wrapping_add( if self.registers.get_flag(&Flag::Carry) { 1 } else { 0 })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self.registers.set_flag(&Flag::Carry, (res as u16) < raw_res);
    self.registers.set_flag(&Flag::Overflow,
      (res ^ self.registers.get(&Register::A)) & (res ^ val) & 0x80 != 0
    );

    self.registers.set(&Register::A, res);
    self.update_zero_negative(self.registers.get(&Register::A));
  }

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

  fn branch(&mut self, condition: bool) {
    if !condition { return };

    let delta = self.memory.read(self.registers.get_pc()) as i8;
    self.registers.set_pc(self.registers.get_pc().wrapping_add(1).wrapping_add(delta as u16));
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

  fn jmp(&mut self, mode: &Addressing) {
    match mode {
      Addressing::Absolute => {
        self.registers.set_pc(self.get_operand_addr(mode));
      }
      Addressing::AbsoluteIndirect => {
        let addr = self.get_operand_addr(mode);
        self.registers.set_pc(
          if addr & 0x00FF == 0x00FF {
            let lo = self.memory.read(addr) as u16;
            let hi = self.memory.read(addr & 0xFF00) as u16;

            (hi << 8) | lo
          } else {
            self.memory.readu16(addr)
          }
        )
      }
      _ => panic!("Invalid mode for jump instructions"),
    }
  }

  fn jsr(&mut self) {
    self.stack_pushu16(self.registers.get_pc() + 1);
    self.registers.set_pc(self.get_operand_addr(&Addressing::Absolute));
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

  fn php(&mut self) {
    let status = self.registers.get(&Register::P) | (0b11 << 5);
    self.stack_push(status);
  }

  fn pla(&mut self) {
    let val = self.stack_pop();
    self.registers.set(&Register::A, val);

    self.update_zero_negative(val);
  }

  fn plp(&mut self) {
    let status = (self.stack_pop() | (1 << 5)) & !(1 << 4);
    self.registers.set(&Register::P, status);
  }

  fn rol(&mut self, mode: &Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(&Register::A);
        let prev_carry = self.registers.get_flag(&Flag::Carry);

        self.registers.set_flag(&Flag::Carry, (data >> 7) & 0b1 != 0);

        self.registers.set(&Register::A, (data << 1) | if prev_carry { 1 } else { 0 });
        self.update_zero_negative(self.registers.get(&Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        let prev_carry = self.registers.get_flag(&Flag::Carry);

        self.registers.set_flag(&Flag::Carry, (data >> 7) & 0b1 != 0);

        self.memory.write(addr, (data << 1) | if prev_carry { 1 } else { 0 });
        self.update_zero_negative(self.memory.read(addr));
      }
    }
  }

  fn ror(&mut self, mode: &Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(&Register::A);
        let prev_carry = self.registers.get_flag(&Flag::Carry);

        self.registers.set_flag(&Flag::Carry, data & 0b1 != 0);

        self.registers.set(&Register::A, (data >> 1) | if prev_carry { 1 << 7 } else { 0 });
        self.update_zero_negative(self.registers.get(&Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        let prev_carry = self.registers.get_flag(&Flag::Carry);

        self.registers.set_flag(&Flag::Carry, data & 0b1 != 0);

        self.memory.write(addr, (data >> 1) | if prev_carry { 1 << 7 } else { 0 });
        self.update_zero_negative(self.memory.read(addr));
      }
    }
  }

  fn rti(&mut self) {
    let status = self.stack_pop();
    self.registers.set(&Register::P, (status | (1 << 5)) & !(1 << 4));

    let pc = self.stack_popu16();
    self.registers.set_pc(pc);
  }

  fn rts(&mut self) {
    let pc = self.stack_popu16().wrapping_add(1);
    self.registers.set_pc(pc);
  }

  fn sbc(&mut self, mode: &Addressing) {
    let val = self.memory.read(self.get_operand_addr(mode)).wrapping_neg().wrapping_sub(1);

    let raw_res = (self.registers.get(&Register::A) as u16)
      .wrapping_add( if self.registers.get_flag(&Flag::Carry) { 1 } else { 0 })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self.registers.set_flag(&Flag::Carry, (res as u16) < raw_res);
    self.registers.set_flag(&Flag::Overflow,
      (res ^ self.registers.get(&Register::A)) & (res ^ val) & 0x80 != 0
    );

    self.registers.set(&Register::A, res);
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