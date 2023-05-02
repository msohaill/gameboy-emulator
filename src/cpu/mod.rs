pub mod instruction;
pub mod interrupt;
pub mod register;

use crate::bus::Bus;
use instruction::{Addressing, Instruction, OpCode, Operand};
use interrupt::{Interrupt, NMI, BRK};
use register::{Flag, Register, Registers};

pub struct CPU<'a> {
  pub registers: Registers,
  pub bus: Bus<'a>,
}

impl<'a> CPU<'a> {
  const STACK_START: u16 = 0x0100;

  pub fn new<'b>(bus: Bus<'b>) -> CPU<'b> {
    CPU {
      registers: Registers::new(),
      bus,
    }
  }

  fn reset(&mut self) {
    self.registers = Registers::new();
    self.registers.set_pc(self.bus.readu16(0xFFFC)); // Check after
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

    self.bus.write(
      CPU::STACK_START.wrapping_add(self.registers.get(Register::SP) as u16),
      data,
    );
    self.registers.set(
      Register::SP,
      self.registers.get(Register::SP).wrapping_sub(1),
    );
  }

  fn stack_pop(&mut self) -> u8 {
    if self.registers.get(Register::SP) == 0x1E {
      panic!("Attempted to pop empty stack");
    }

    self.registers.set(
      Register::SP,
      self.registers.get(Register::SP).wrapping_add(1),
    );
    self
      .bus
      .read(CPU::STACK_START.wrapping_add(self.registers.get(Register::SP) as u16))
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
    self
      .registers
      .set_pc(self.registers.get_pc().wrapping_add(i));
  }

  fn interrupt(&mut self, interrupt: Interrupt) {
    if self.registers.get_flag(Flag::InterruptDisable) && (interrupt == BRK) {
      return;
    }

    self
      .registers
      .change_flag(Flag::B1, interrupt.mask & 0b00010000 == 0b00010000);
    self
      .registers
      .change_flag(Flag::B2, interrupt.mask & 0b00100000 == 0b00100000);

    self.stack_pushu16(self.registers.get_pc());
    self.stack_push(self.registers.get(Register::P));

    self.registers.set_flag(Flag::InterruptDisable);

    self.bus.tick(interrupt.cycles);

    self
      .registers
      .set_pc(self.bus.readu16(interrupt.read_address));
  }

  fn run<F>(&mut self, mut callback: F)
  where
    F: FnMut(&mut CPU),
  {
    loop {
      if self.bus.poll_nmi() {
        self.bus.clear_nmi();
        self.interrupt(NMI);
      }

      callback(self);

      let instruction = Instruction::get(self.bus.read(self.registers.get_pc()));
      self.increment_pc(1);
      let current_pc = self.registers.get_pc();

      let cycles = self.execute_instr(&instruction);

      self.bus.tick(cycles);

      if current_pc == self.registers.get_pc() {
        self.increment_pc((instruction.len - 1) as u16);
      }
    }
  }

  fn execute_instr(&mut self, instruction: &Instruction) -> u8 {
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

      OpCode::BRK => self.brk(), // 7,

      OpCode::BVC => self.branch(!self.registers.get_flag(Flag::Overflow)),

      OpCode::BVS => self.branch(self.registers.get_flag(Flag::Overflow)),

      OpCode::CLC => self.clear(Flag::Carry),

      OpCode::CLD => self.clear(Flag::Decimal),

      OpCode::CLI => self.clear(Flag::InterruptDisable),

      OpCode::CLV => self.clear(Flag::Overflow),

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

      OpCode::JAM => panic!("Jammed!"),

      OpCode::JMP => self.jmp(instruction.mode),

      OpCode::JSR => self.jsr(),

      OpCode::_LAS => self.las(),

      OpCode::_LAX => self.lax(instruction.mode),

      OpCode::LDA => self.load_reg(instruction.mode, Register::A),

      OpCode::LDX => self.load_reg(instruction.mode, Register::X),

      OpCode::LDY => self.load_reg(instruction.mode, Register::Y),

      OpCode::LSR => self.lsr(instruction.mode),

      OpCode::_LXA => self.lxa(),

      OpCode::NOP | OpCode::_NOP => self.nop(instruction.mode),

      OpCode::ORA => self.ora(instruction.mode),

      OpCode::PHA => self.pha(),

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

      OpCode::SEC => self.set(Flag::Carry),

      OpCode::SED => self.set(Flag::Decimal),

      OpCode::SEI => self.set(Flag::InterruptDisable),

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

      OpCode::TXS => self.txs(),

      OpCode::TYA => self.transfer_reg(Register::Y, Register::A),
    }
  }

  pub fn get_operand_addr(&mut self, mode: Addressing) -> Operand {
    match mode {
      Addressing::Accumulator => Operand(0, 1, 0),
      Addressing::Absolute => Operand(self.bus.readu16(self.registers.get_pc()), 3, 0),
      Addressing::AbsoluteX => {
        let base_addr = self.bus.readu16(self.registers.get_pc());
        let addr = base_addr.wrapping_add(self.registers.get(Register::X) as u16);
        let additional_cyc = ((addr & 0xff00) != (base_addr & 0xff00)) as u8;

        Operand(addr, 3, additional_cyc)
      }
      Addressing::AbsoluteY => {
        let base_addr = self.bus.readu16(self.registers.get_pc());
        let addr = base_addr.wrapping_add(self.registers.get(Register::Y) as u16);
        let additional_cyc = ((addr & 0xff00) != (base_addr & 0xff00)) as u8;

        Operand(addr, 3, additional_cyc)
      }
      Addressing::Immediate => Operand(self.registers.get_pc(), 1, 0),
      Addressing::Implied => Operand(0, 0, 0),
      Addressing::Indirect => {
        let base_addr = self.bus.readu16(self.registers.get_pc());
        let addr = if base_addr & 0x00FF == 0x00FF {
          let lo = self.bus.read(base_addr) as u16;
          let hi = self.bus.read(base_addr & 0xFF00) as u16;

          (hi << 8) | lo
        } else {
          self.bus.readu16(base_addr)
        };

        Operand(addr, 5, 0)
      }
      Addressing::IndirectX => {
        let fetch_addr = self
          .bus
          .read(self.registers.get_pc())
          .wrapping_add(self.registers.get(Register::X));
        let lo = self.bus.read(fetch_addr as u16) as u16;
        let hi = self.bus.read(fetch_addr.wrapping_add(1) as u16) as u16;
        let addr = (hi << 8) | lo;

        Operand(addr, 5, 0)
      }
      Addressing::IndirectY => {
        let fetch_addr = self.bus.read(self.registers.get_pc());
        let lo = self.bus.read(fetch_addr as u16) as u16;
        let hi = self.bus.read((fetch_addr).wrapping_add(1) as u16) as u16;
        let base_addr = (hi << 8) | lo;
        let addr = base_addr.wrapping_add(self.registers.get(Register::Y) as u16);
        let additional_cyc = ((addr & 0xff00) != (base_addr & 0xff00)) as u8;

        Operand(addr, 4, additional_cyc)
      }
      Addressing::Relative => {
        let delta = self.bus.read(self.registers.get_pc()) as i8;
        let base_addr = self.registers.get_pc().wrapping_add(1);
        let addr = base_addr.wrapping_add(delta as u16);
        let additional_cyc = ((addr & 0xff00) != (base_addr & 0xff00)) as u8;

        Operand(addr, 2, additional_cyc)
      }
      Addressing::ZeroPage => Operand(self.bus.read(self.registers.get_pc()) as u16, 2, 0),
      Addressing::ZeroPageX => Operand(
        self
          .bus
          .read(self.registers.get_pc())
          .wrapping_add(self.registers.get(Register::X)) as u16,
        3,
        0,
      ),
      Addressing::ZeroPageY => Operand(
        self
          .bus
          .read(self.registers.get_pc())
          .wrapping_add(self.registers.get(Register::Y)) as u16,
        3,
        0,
      ),
    }
  }

  fn get_operand(&mut self, mode: Addressing) -> (Operand, u8) {
    match mode {
      Addressing::Accumulator => (self.get_operand_addr(mode), self.registers.get(Register::A)),

      Addressing::Implied | Addressing::Indirect | Addressing::Relative => {
        (self.get_operand_addr(mode), 0)
      }
      _ => {
        let Operand(addr, cyc, additional_cyc) = self.get_operand_addr(mode);
        (Operand(addr, cyc, additional_cyc), self.bus.read(addr))
      }
    }
  }

  fn update_zero_negative(&mut self, res: u8) {
    self.registers.change_flag(Flag::Zero, res == 0);
    self
      .registers
      .change_flag(Flag::Negative, (res >> 7) & 0b1 != 0);
  }

  fn adc(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, additional_cyc), val) = self.get_operand(mode);

    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add(if self.registers.get_flag(Flag::Carry) {
        1
      } else {
        0
      })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self
      .registers
      .change_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.change_flag(
      Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ val) & 0x80 != 0,
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));

    1 + cyc + additional_cyc
  }

  fn alr(&mut self) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(Addressing::Immediate);

    self.and(Addressing::Immediate);
    self.lsr(Addressing::Immediate);

    1 + cyc
  }

  fn anc(&mut self) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(Addressing::Immediate);

    self.and(Addressing::Immediate);
    self.registers.change_flag(
      Flag::Carry,
      (self.registers.get(Register::A) >> 7) & 0b1 != 0,
    );

    1 + cyc
  }

  fn and(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, additional_cyc), byte) = self.get_operand(mode);

    self
      .registers
      .set(Register::A, self.registers.get(Register::A) & byte);
    self.update_zero_negative(self.registers.get(Register::A));

    1 + cyc + additional_cyc
  }

  fn ane(&mut self) -> u8 {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);

    let (Operand(_, cyc, _), data) = self.get_operand(Addressing::Immediate);

    self.registers.set(Register::A, a & x & data);

    1 + cyc
  }

  fn arr(&mut self) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(Addressing::Immediate);

    self.and(Addressing::Immediate);
    self.ror(Addressing::Immediate);

    let b5 = (self.registers.get(Register::A) >> 5) & 1;
    let b6 = (self.registers.get(Register::A) >> 6) & 1;

    self.registers.change_flag(Flag::Carry, b5 != 0);
    self.registers.change_flag(Flag::Overflow, b5 ^ b6 != 0);

    1 + cyc
  }

  fn asl(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    self
      .registers
      .change_flag(Flag::Carry, (data >> 7) & 0b1 != 0);
    self.update_zero_negative(data << 1);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, data << 1);
        1 + cyc
      }
      Addressing::AbsoluteX => {
        self.bus.write(addr, data << 1);
        4 + cyc
      }
      _ => {
        self.bus.write(addr, data << 1);
        3 + cyc
      }
    }
  }

  fn bit(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, _), data) = self.get_operand(mode);

    self
      .registers
      .change_flag(Flag::Zero, data & self.registers.get(Register::A) == 0);
    self
      .registers
      .change_flag(Flag::Overflow, (data >> 6) & 0b1 != 0);
    self
      .registers
      .change_flag(Flag::Negative, (data >> 7) & 0b1 != 0);

    1 + cyc
  }

  fn branch(&mut self, condition: bool) -> u8 {
    let Operand(addr, cyc, additional_cyc) = self.get_operand_addr(Addressing::Relative);

    if !condition {
      cyc
    } else {
      self.registers.set_pc(addr);
      1 + cyc + additional_cyc
    }
  }

  fn brk(&mut self) -> u8 {
    self.increment_pc(1);
    self.interrupt(BRK);
    7
  }

  fn clear(&mut self, flag: Flag) -> u8 {
    self.registers.unset_flag(flag);
    2
  }

  fn compare(&mut self, mode: Addressing, reg: Register) -> u8 {
    let (Operand(_, cyc, additional_cyc), data) = self.get_operand(mode);
    let comparable = self.registers.get(reg);

    self.registers.change_flag(Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data));

    1 + cyc + additional_cyc
  }

  fn dcp(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.dec(mode);
    self.compare(mode, Register::A);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn dec(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    self.bus.write(addr, data.wrapping_sub(1));
    self.update_zero_negative(data.wrapping_sub(1));

    match mode {
      Addressing::AbsoluteX => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn decrement_reg(&mut self, reg: Register) -> u8 {
    self
      .registers
      .set(reg, self.registers.get(reg).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(reg));
    2
  }

  fn eor(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, additional_cyc), byte) = self.get_operand(mode);

    self
      .registers
      .set(Register::A, self.registers.get(Register::A) ^ byte);
    self.update_zero_negative(self.registers.get(Register::A));

    1 + cyc + additional_cyc
  }

  fn inc(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    self.bus.write(addr, data.wrapping_add(1));
    self.update_zero_negative(data.wrapping_add(1));

    match mode {
      Addressing::AbsoluteX => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn increment_reg(&mut self, reg: Register) -> u8 {
    self
      .registers
      .set(reg, self.registers.get(reg).wrapping_add(1));
    self.update_zero_negative(self.registers.get(reg));
    2
  }

  fn isc(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.inc(mode);
    self.sbc(mode);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn jmp(&mut self, mode: Addressing) -> u8 {
    let Operand(addr, cyc, _) = self.get_operand_addr(mode);
    self.registers.set_pc(addr);
    cyc
  }

  fn jsr(&mut self) -> u8 {
    self.stack_pushu16(self.registers.get_pc() + 1);
    let Operand(addr, cyc, _) = self.get_operand_addr(Addressing::Absolute);
    self.registers.set_pc(addr);
    3 + cyc
  }

  fn las(&mut self) -> u8 {
    let (Operand(_, cyc, additional_cyc), base_data) = self.get_operand(Addressing::AbsoluteY);
    let data = base_data & self.registers.get(Register::SP);

    self.registers.set(Register::A, data);
    self.registers.set(Register::X, data);
    self.registers.set(Register::SP, data);

    self.update_zero_negative(data);

    1 + cyc + additional_cyc
  }

  fn lax(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, additional_cyc) = self.get_operand_addr(mode);

    self.load_reg(mode, Register::A);
    self.load_reg(mode, Register::X);

    1 + cyc + additional_cyc
  }

  fn load_reg(&mut self, mode: Addressing, reg: Register) -> u8 {
    let (Operand(_, cyc, additional_cyc), val) = self.get_operand(mode);

    self.registers.set(reg, val);
    self.update_zero_negative(val);

    1 + cyc + additional_cyc
  }

  fn lsr(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    self.registers.change_flag(Flag::Carry, data & 0b1 != 0);
    self.update_zero_negative(data >> 1);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, data >> 1);
        1 + cyc
      }
      Addressing::AbsoluteX => {
        self.bus.write(addr, data >> 1);
        4 + cyc
      }
      _ => {
        self.bus.write(addr, data >> 1);
        3 + cyc
      }
    }
  }

  fn lxa(&mut self) -> u8 {
    let (Operand(_, cyc, _), data) = self.get_operand(Addressing::Immediate);
    let res = self.registers.get(Register::A) & data;

    self.registers.set(Register::A, res);
    self.registers.set(Register::X, res);
    self.update_zero_negative(res);

    1 + cyc
  }

  fn nop(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, additional_cyc) = self.get_operand_addr(mode);

    match mode {
      Addressing::Implied => 2 + cyc,
      _ => 1 + cyc + additional_cyc,
    }
  }

  fn ora(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, additional_cyc), byte) = self.get_operand(mode);

    self
      .registers
      .set(Register::A, self.registers.get(Register::A) | byte);
    self.update_zero_negative(self.registers.get(Register::A));

    1 + cyc + additional_cyc
  }

  fn pha(&mut self) -> u8 {
    self.stack_push(self.registers.get(Register::A));
    3
  }

  fn php(&mut self) -> u8 {
    let status = self.registers.get(Register::P) | (0b11 << 4);
    self.stack_push(status);
    3
  }

  fn pla(&mut self) -> u8 {
    let val = self.stack_pop();
    self.registers.set(Register::A, val);
    self.update_zero_negative(val);
    4
  }

  fn plp(&mut self) -> u8 {
    let status = (self.stack_pop() | (1 << 5)) & !(1 << 4);
    self.registers.set(Register::P, status);
    4
  }

  fn rla(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.rol(mode);
    self.and(mode);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn rol(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    let prev_carry = self.registers.get_flag(Flag::Carry);
    self
      .registers
      .change_flag(Flag::Carry, (data >> 7) & 0b1 != 0);

    let res = (data << 1) | if prev_carry { 1 } else { 0 };
    self.update_zero_negative(res);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, res);
        1 + cyc
      }
      Addressing::AbsoluteX => {
        self.bus.write(addr, res);
        4 + cyc
      }
      _ => {
        self.bus.write(addr, res);
        3 + cyc
      }
    }
  }

  fn ror(&mut self, mode: Addressing) -> u8 {
    let (Operand(addr, cyc, _), data) = self.get_operand(mode);

    let prev_carry = self.registers.get_flag(Flag::Carry);
    self.registers.change_flag(Flag::Carry, data & 0b1 != 0);

    let res = (data >> 1) | if prev_carry { 1 << 7 } else { 0 };
    self.update_zero_negative(res);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, res);
        1 + cyc
      }
      Addressing::AbsoluteX => {
        self.bus.write(addr, res);
        4 + cyc
      }
      _ => {
        self.bus.write(addr, res);
        3 + cyc
      }
    }
  }

  fn rra(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.ror(mode);
    self.adc(mode);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn rti(&mut self) -> u8 {
    let status = self.stack_pop();
    self
      .registers
      .set(Register::P, (status | (1 << 5)) & !(1 << 4));

    let pc = self.stack_popu16();
    self.registers.set_pc(pc);

    6
  }

  fn rts(&mut self) -> u8 {
    let pc = self.stack_popu16().wrapping_add(1);
    self.registers.set_pc(pc);

    6
  }

  fn sax(&mut self, mode: Addressing) -> u8 {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    let Operand(addr, cyc, _) = self.get_operand_addr(mode);

    self.bus.write(addr, a & x);

    1 + cyc
  }

  fn sbc(&mut self, mode: Addressing) -> u8 {
    let (Operand(_, cyc, additional_cyc), base_val) = self.get_operand(mode);

    let val = base_val.wrapping_neg().wrapping_sub(1);

    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add(if self.registers.get_flag(Flag::Carry) {
        1
      } else {
        0
      })
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self
      .registers
      .change_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.change_flag(
      Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ val) & 0x80 != 0,
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));

    1 + cyc + additional_cyc
  }

  fn sbx(&mut self) -> u8 {
    let (Operand(_, cyc, _), data_a) = self.get_operand(Addressing::Immediate);
    let data_b = self.registers.get(Register::A) & self.registers.get(Register::X);

    let res = data_b.wrapping_sub(data_a);

    self.registers.change_flag(Flag::Carry, data_b <= data_a);
    self.update_zero_negative(res);
    self.registers.set(Register::X, res);

    1 + cyc
  }

  fn set(&mut self, flag: Flag) -> u8 {
    self.registers.set_flag(flag);
    2
  }

  fn sha(&mut self, mode: Addressing) -> u8 {
    let Operand(addr, cyc, _) = self.get_operand_addr(mode);
    self.bus.write(
      addr,
      self.registers.get(Register::A)
        & self.registers.get(Register::X)
        & ((addr >> 8) as u8).wrapping_add(1),
    );

    2 + cyc
  }

  fn shx(&mut self) -> u8 {
    let Operand(addr, cyc, _) = self.get_operand_addr(Addressing::AbsoluteY);
    let x = self.registers.get(Register::X);

    self
      .bus
      .write(addr, x & ((addr >> 8) as u8).wrapping_add(1));

    2 + cyc
  }

  fn shy(&mut self) -> u8 {
    let Operand(addr, cyc, _) = self.get_operand_addr(Addressing::AbsoluteY);
    let y = self.registers.get(Register::Y);

    self
      .bus
      .write(addr, y & ((addr >> 8) as u8).wrapping_add(1));

    2 + cyc
  }

  fn slo(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.asl(mode);
    self.ora(mode);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn sre(&mut self, mode: Addressing) -> u8 {
    let Operand(_, cyc, _) = self.get_operand_addr(mode);

    self.lsr(mode);
    self.eor(mode);

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 4 + cyc,
      _ => 3 + cyc,
    }
  }

  fn store_reg(&mut self, mode: Addressing, reg: Register) -> u8 {
    let Operand(addr, cyc, _) = self.get_operand_addr(mode);
    self.bus.write(addr, self.registers.get(reg));

    match mode {
      Addressing::AbsoluteX | Addressing::AbsoluteY | Addressing::IndirectY => 2 + cyc,
      _ => 1 + cyc,
    }
  }

  fn tas(&mut self) -> u8 {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    let Operand(addr, cyc, _) = self.get_operand_addr(Addressing::AbsoluteY);

    self.registers.set(Register::SP, a & x);
    self
      .bus
      .write(addr, a & x & ((addr >> 8) as u8).wrapping_add(1));

    2 + cyc
  }

  fn transfer_reg(&mut self, from: Register, to: Register) -> u8 {
    self.registers.set(to, self.registers.get(from));
    self.update_zero_negative(self.registers.get(to));

    2
  }

  fn txs(&mut self) -> u8 {
    self
      .registers
      .set(Register::SP, self.registers.get(Register::X));
    2
  }
}
