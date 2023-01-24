pub mod register;
pub mod instruction;

use crate::memory::{Memory, cartridge::Cartridge};
use instruction::{Addressing, Instruction, OpCode};
use register::{Flag, Register, Registers};

pub struct CPU {
  pub registers: Registers,
  pub memory: Memory,
}

impl CPU {
  const STACK_START: u16 = 0x0100;

  pub fn new(cartridge: Cartridge) -> Self {
    CPU {
      registers: Registers::new(),
      memory: Memory::new(cartridge),
    }
  }

  fn reset(&mut self) {
    self.registers = Registers::new();
    self.registers.set_pc(self.memory.readu16(0xFFFC)); // Check after
    // self.registers.set_pc(0x0600);
    self.registers.set_pc(0xC000);
  }

  pub fn start<F>(&mut self, callback: F)
  where
    F: FnMut(&mut CPU),
  {
    self.reset();
    self.run(callback);
  }

  fn stack_push(&mut self, data: u8) {
    if self.registers.get(Register::SP) == 0 {
      panic!("Attempted to store beyone stack capacity.");
    }

    self.memory.write(CPU::STACK_START.wrapping_add(self.registers.get(Register::SP) as u16), data);
    self.registers.set(Register::SP, self.registers.get(Register::SP).wrapping_sub(1));
  }

  fn stack_pop(&mut self) -> u8 {
    if self.registers.get(Register::SP) == 0x1E {
      panic!("Attempted to pop empty stack");
    }

    self.registers.set(Register::SP, self.registers.get(Register::SP).wrapping_add(1));
    self.memory.read(CPU::STACK_START.wrapping_add(self.registers.get(Register::SP) as u16))
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

      let instruction = Instruction::get(self.memory.read(self.registers.get_pc()));
      self.increment_pc(1);
      let current_pc = self.registers.get_pc();

      match instruction.opcode {
        OpCode::ADC => self.adc(instruction.mode),

        OpCode::_ALR => self.alr(),

        OpCode::_ANC => self.anc(),

        OpCode::AND => self.and(instruction.mode),

        OpCode::_ANE => self.ane(),

        OpCode::_ARR => self.arr(),

        OpCode::ASL => self.asl(instruction.mode),

        OpCode::BCC => self.branch(!self.registers.get_flag(Flag::Carry)),

        OpCode::BCS => self.branch(self.registers.get_flag(Flag::Carry)),

        OpCode::BEQ => self.branch(self.registers.get_flag(Flag::Zero)),

        OpCode::BIT => self.bit(instruction.mode),

        OpCode::BMI => self.branch(self.registers.get_flag(Flag::Negative)),

        OpCode::BNE => self.branch(!self.registers.get_flag(Flag::Zero)),

        OpCode::BPL => self.branch(!self.registers.get_flag(Flag::Negative)),

        OpCode::BRK => return,

        OpCode::BVC => self.branch(!self.registers.get_flag(Flag::Overflow)),

        OpCode::BVS => self.branch(self.registers.get_flag(Flag::Overflow)),

        OpCode::CLC => self.registers.set_flag(Flag::Carry, false),

        OpCode::CLD => self.registers.set_flag(Flag::Decimal, false),

        OpCode::CLI => self.registers.set_flag(Flag::InterruptDisable, false),

        OpCode::CLV => self.registers.set_flag(Flag::Overflow, false),

        OpCode::CMP => self.compare(instruction.mode, Register::A),

        OpCode::CPX => self.compare(instruction.mode, Register::X),

        OpCode::CPY => self.compare(instruction.mode, Register::Y),

        OpCode::_DCP => self.dcp(instruction.mode),

        OpCode::DEC => self.dec(instruction.mode),

        OpCode::DEX => self.decrement_reg(Register::X),

        OpCode::DEY => self.decrement_reg(Register::Y),

        OpCode::EOR => self.eor(instruction.mode),

        OpCode::INC => self.inc(instruction.mode),

        OpCode::INX => self.increment_reg(Register::X),

        OpCode::INY => self.increment_reg(Register::Y),

        OpCode::_ISC => self.isc(instruction.mode),

        OpCode::JAM => (),

        OpCode::JMP => self.jmp(instruction.mode),

        OpCode::JSR => self.jsr(),

        OpCode::_LAS => self.las(),

        OpCode::_LAX => self.lax(instruction.mode),

        OpCode::LDA => self.load_reg(instruction.mode, Register::A),

        OpCode::LDX => self.load_reg(instruction.mode, Register::X),

        OpCode::LDY => self.load_reg(instruction.mode, Register::Y),

        OpCode::LSR => self.lsr(instruction.mode),

        OpCode::_LXA => self.lxa(),

        OpCode::NOP => (),

        OpCode::_NOP => (),

        OpCode::ORA => self.ora(instruction.mode),

        OpCode::PHA => self.stack_push(self.registers.get(Register::A)),

        OpCode::PHP => self.php(),

        OpCode::PLA => self.pla(),

        OpCode::PLP => self.plp(),

        OpCode::_RLA => self.rla(instruction.mode),

        OpCode::ROL => self.rol(instruction.mode),

        OpCode::ROR => self.ror(instruction.mode),

        OpCode::_RRA => self.rra(instruction.mode),

        OpCode::RTI => self.rti(),

        OpCode::RTS => self.rts(),

        OpCode::_SAX => self.sax(instruction.mode),

        OpCode::SBC | OpCode::_SBC => self.sbc(instruction.mode),

        OpCode::_SBX => self.sbx(),

        OpCode::SEC => self.registers.set_flag(Flag::Carry, true),

        OpCode::SED => self.registers.set_flag(Flag::Decimal, true),

        OpCode::SEI => self.registers.set_flag(Flag::InterruptDisable, true),

        OpCode::_SHA => self.sha(instruction.mode),

        OpCode::_SHX => self.shx(),

        OpCode::_SHY => self.shy(),

        OpCode::_SLO => self.slo(instruction.mode),

        OpCode::_SRE => self.sre(instruction.mode),

        OpCode::STA => self.store_reg(instruction.mode, Register::A),

        OpCode::STX => self.store_reg(instruction.mode, Register::X),

        OpCode::STY => self.store_reg(instruction.mode, Register::Y),

        OpCode::_TAS => self.tas(),

        OpCode::TAX => self.transfer_reg(Register::A, Register::X),

        OpCode::TAY => self.transfer_reg(Register::A, Register::Y),

        OpCode::TSX => self.transfer_reg(Register::SP, Register::X),

        OpCode::TXA => self.transfer_reg(Register::X, Register::A),

        OpCode::TXS => self.registers.set(Register::SP, self.registers.get(Register::X)),

        OpCode::TYA => self.transfer_reg(Register::Y, Register::A),
      }

      if current_pc == self.registers.get_pc() {
        self.increment_pc((instruction.len - 1) as u16);
      }
    }
  }

  pub fn get_operand_addr(&mut self, mode: Addressing) -> u16 {
    match mode {
      Addressing::Immediate =>
        self.registers.get_pc(),
      Addressing::ZeroPage =>
        self.memory.read(self.registers.get_pc()) as u16,
      Addressing::Absolute | Addressing::AbsoluteIndirect =>
        self.memory.readu16(self.registers.get_pc()),
      Addressing::ZeroPageX =>
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(Register::X)) as u16,
      Addressing::ZeroPageY =>
        self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(Register::Y)) as u16,
      Addressing::AbsoluteX =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(Register::X) as u16),
      Addressing::AbsoluteY =>
        self.memory.readu16(self.registers.get_pc()).wrapping_add(self.registers.get(Register::Y) as u16),
      Addressing::IndirectX => {
        let addr = self.memory.read(self.registers.get_pc()).wrapping_add(self.registers.get(Register::X));
        let lo = self.memory.read(addr as u16) as u16;
        let hi = self.memory.read(addr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
      }
      Addressing::IndirectY => {
        let addr = self.memory.read(self.registers.get_pc());
        let lo = self.memory.read(addr as u16) as u16;
        let hi = self.memory.read((addr).wrapping_add(1) as u16) as u16;
        ((hi << 8) | lo).wrapping_add(self.registers.get(Register::Y) as u16)
      }
      Addressing::Implied => panic!("Implied addressing doesn't yield an address."),
    }
  }

  fn update_zero_negative(&mut self, res: u8) {
    if res == 0 {
      self.registers.set_flag(Flag::Zero, true);
    } else {
      self.registers.set_flag(Flag::Zero, false);
    }

    if (res >> 7) & 0b1 != 0 {
      self.registers.set_flag(Flag::Negative, true);
    } else {
      self.registers.set_flag(Flag::Negative, false);
    }
  }

  fn adc(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let val = self.memory.read(addr);

    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add( if self.registers.get_flag(Flag::Carry) { 1 } else { 0 })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self.registers.set_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.set_flag(Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ val) & 0x80 != 0
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn alr(&mut self) {
    self.and(Addressing::Immediate);
    self.lsr(Addressing::Immediate);
  }

  fn anc(&mut self) {
    self.and(Addressing::Immediate);
    self.registers.set_flag(Flag::Carry, (self.registers.get(Register::A) >> 7) & 0b1 != 0);
  }

  fn and(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let byte = self.memory.read(addr);
    self.registers.set(Register::A, self.registers.get(Register::A) & byte);

    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn ane(&mut self) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    let addr = self.get_operand_addr(Addressing::Immediate);
    let data = self.memory.read(addr);

    self.registers.set(Register::A, a & x & data);
  }

  fn arr(&mut self) {
    self.and(Addressing::Immediate);
    self.ror(Addressing::Immediate);

    let b5 = (self.registers.get(Register::A) >> 5) & 1;
    let b6 = (self.registers.get(Register::A) >> 6) & 1;

    self.registers.set_flag(Flag::Carry, b5 != 0);
    self.registers.set_flag(Flag::Overflow, b5 ^ b6 != 0);
  }

  fn asl(&mut self, mode: Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(Register::A);
        self.registers.set_flag(Flag::Carry, (data >> 7) & 0b1 != 0);

        self.registers.set(Register::A, data << 1);
        self.update_zero_negative(self.registers.get(Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        self.registers.set_flag(Flag::Carry, (data >> 7) & 0b1 != 0);

        self.memory.write(addr, data << 1);
        self.update_zero_negative(data << 1);
      }
    }
  }

  fn bit(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);

    self.registers.set_flag(Flag::Zero, data & self.registers.get(Register::A) == 0);
    self.registers.set_flag(Flag::Overflow, (data >> 6) & 0b1 != 0);
    self.registers.set_flag(Flag::Negative, (data >> 7) & 0b1 != 0);
  }

  fn branch(&mut self, condition: bool) {
    if !condition { return };

    let delta = self.memory.read(self.registers.get_pc()) as i8;
    self.registers.set_pc(self.registers.get_pc().wrapping_add(1).wrapping_add(delta as u16));
  }

  fn compare(&mut self, mode: Addressing, reg: Register) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);
    let comparable = self.registers.get(reg);

    self.registers.set_flag(Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data));
  }

  fn dcp(&mut self, mode: Addressing) {
    self.dec(mode);
    self.compare(mode, Register::A);
  }

  fn dec(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);

    self.memory.write(addr, data.wrapping_sub(1));
    self.update_zero_negative(data.wrapping_sub(1));
  }

  fn decrement_reg(&mut self, reg: Register) {
    self.registers.set(reg, self.registers.get(reg).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(reg));
  }

  fn eor(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let byte = self.memory.read(addr);
    self.registers.set(Register::A, self.registers.get(Register::A) ^ byte);

    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn inc(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let data = self.memory.read(addr);

    self.memory.write(addr, data.wrapping_add(1));
    self.update_zero_negative(data.wrapping_add(1));
  }

  fn increment_reg(&mut self, reg: Register) {
    self.registers.set(reg, self.registers.get(reg).wrapping_add(1));
    self.update_zero_negative(self.registers.get(reg));
  }

  fn isc(&mut self, mode: Addressing) {
    self.inc(mode);
    self.sbc(mode);
  }

  fn jmp(&mut self, mode: Addressing) {
    match mode {
      Addressing::Absolute => {
        let addr = self.get_operand_addr(mode);
        self.registers.set_pc(addr);
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
    let addr = self.get_operand_addr(Addressing::Absolute);
    self.registers.set_pc(addr);
  }

  fn las(&mut self) {
    let addr = self.get_operand_addr(Addressing::AbsoluteY);
    let data = self.memory.read(addr) & self.registers.get(Register::SP);

    self.registers.set(Register::A, data);
    self.registers.set(Register::X, data);
    self.registers.set(Register::SP, data);

    self.update_zero_negative(data);
  }

  fn lax(&mut self, mode: Addressing) {
    self.load_reg(mode, Register::A);
    self.load_reg(mode, Register::X);
  }

  fn load_reg(&mut self, mode: Addressing, reg: Register) {
    let addr = self.get_operand_addr(mode);
    let val = self.memory.read(addr);
    self.registers.set(reg, val);

    self.update_zero_negative(val);
  }

  fn lsr(&mut self, mode: Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(Register::A);
        self.registers.set_flag(Flag::Carry, data & 0b1 != 0);

        self.registers.set(Register::A, data >> 1);
        self.update_zero_negative(self.registers.get(Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        self.registers.set_flag(Flag::Carry, data & 0b1 != 0);

        self.memory.write(addr, data >> 1);
        self.update_zero_negative(data >> 1);
      }
    }
  }

  fn lxa(&mut self) {
    let addr = self.get_operand_addr(Addressing::Immediate);
    let data = self.memory.read(addr);
    let res = self.registers.get(Register::A) & data;

    self.registers.set(Register::A, res);
    self.registers.set(Register::X, res);

    self.update_zero_negative(res);
  }

  fn ora(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let byte = self.memory.read(addr);
    self.registers.set(Register::A, self.registers.get(Register::A) | byte);

    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn php(&mut self) {
    let status = self.registers.get(Register::P) | (0b11 << 4);
    self.stack_push(status);
  }

  fn pla(&mut self) {
    let val = self.stack_pop();
    self.registers.set(Register::A, val);

    self.update_zero_negative(val);
  }

  fn plp(&mut self) {
    let status = (self.stack_pop() | (1 << 5)) & !(1 << 4);
    self.registers.set(Register::P, status);
  }

  fn rla(&mut self, mode: Addressing) {
    self.rol(mode);
    self.and(mode);
  }

  fn rol(&mut self, mode: Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(Register::A);
        let prev_carry = self.registers.get_flag(Flag::Carry);

        self.registers.set_flag(Flag::Carry, (data >> 7) & 0b1 != 0);

        self.registers.set(Register::A, (data << 1) | if prev_carry { 1 } else { 0 });
        self.update_zero_negative(self.registers.get(Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        let prev_carry = self.registers.get_flag(Flag::Carry);

        self.registers.set_flag(Flag::Carry, (data >> 7) & 0b1 != 0);

        let res = (data << 1) | if prev_carry { 1 } else { 0 };

        self.memory.write(addr, res);
        self.update_zero_negative(res);
      }
    }
  }

  fn ror(&mut self, mode: Addressing) {
    match mode {
      Addressing::Implied => {
        let data = self.registers.get(Register::A);
        let prev_carry = self.registers.get_flag(Flag::Carry);

        self.registers.set_flag(Flag::Carry, data & 0b1 != 0);

        self.registers.set(Register::A, (data >> 1) | if prev_carry { 1 << 7 } else { 0 });
        self.update_zero_negative(self.registers.get(Register::A));
      }
       _ => {
        let addr = self.get_operand_addr(mode);
        let data = self.memory.read(addr);
        let prev_carry = self.registers.get_flag(Flag::Carry);

        self.registers.set_flag(Flag::Carry, data & 0b1 != 0);

        let res = (data >> 1) | if prev_carry { 1 << 7 } else { 0 };

        self.memory.write(addr, res);
        self.update_zero_negative(res);
      }
    }
  }

  fn rra(&mut self, mode: Addressing) {
    self.ror(mode);
    self.adc(mode);
  }

  fn rti(&mut self) {
    let status = self.stack_pop();
    self.registers.set(Register::P, (status | (1 << 5)) & !(1 << 4));

    let pc = self.stack_popu16();
    self.registers.set_pc(pc);
  }

  fn rts(&mut self) {
    let pc = self.stack_popu16().wrapping_add(1);
    self.registers.set_pc(pc);
  }

  fn sax(&mut self, mode: Addressing) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    let addr = self.get_operand_addr(mode);

    self.memory.write(addr, a & x);
  }

  fn sbc(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    let val = self.memory.read(addr).wrapping_neg().wrapping_sub(1);

    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add( if self.registers.get_flag(Flag::Carry) { 1 } else { 0 })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self.registers.set_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.set_flag(Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ val) & 0x80 != 0
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn sbx(&mut self) {
    let addr = self.get_operand_addr(Addressing::Immediate);
    let data_a = self.memory.read(addr);
    let data_b = self.registers.get(Register::A) & self.registers.get(Register::X);

    let res = data_b.wrapping_sub(data_a);

    self.registers.set_flag(Flag::Carry, data_b <= data_a);
    self.update_zero_negative(res);
    self.registers.set(Register::X, res);
  }

  fn sha(&mut self, mode: Addressing) {
    let addr = self.get_operand_addr(mode);
    self.memory.write(addr,
      self.registers.get(Register::A) & self.registers.get(Register::X) & ((addr >> 8) as u8).wrapping_add(1));
  }

  fn shx(&mut self) {
    let addr = self.get_operand_addr(Addressing::AbsoluteY);
    let x = self.registers.get(Register::X);

    self.memory.write(addr, x  & ((addr >> 8) as u8).wrapping_add(1));
  }

  fn shy(&mut self) {
    let addr = self.get_operand_addr(Addressing::AbsoluteY);
    let y = self.registers.get(Register::Y);

    self.memory.write(addr, y  & ((addr >> 8) as u8).wrapping_add(1));
  }

  fn slo(&mut self, mode: Addressing) {
    self.asl(mode);
    self.ora(mode);
  }

  fn sre(&mut self, mode: Addressing) {
    self.lsr(mode);
    self.eor(mode);
  }

  fn store_reg(&mut self, mode: Addressing, reg: Register) {
    let addr = self.get_operand_addr(mode);
    self.memory.write(addr, self.registers.get(reg));
  }

  fn tas(&mut self) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    let addr = self.get_operand_addr(Addressing::AbsoluteY);


    self.registers.set(Register::SP, a & x);
    self.memory.write(addr, a & x & ((addr >> 8) as u8).wrapping_add(1));
  }

  fn transfer_reg(&mut self, from: Register, to: Register) {
    self.registers.set(to, self.registers.get(from));
    self.update_zero_negative(self.registers.get(to));
  }
}