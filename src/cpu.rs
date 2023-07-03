pub mod instruction;
pub mod interrupt;
pub mod register;

use crate::system::System;
use instruction::{Addressing, Instruction, OpCode, Operand, OperandAddress};
use interrupt::Interrupt;
use register::{Flag, Register, Registers};

pub struct CPU<'a> {
  pub registers: Registers,
  pub system: System<'a>,
  branched: bool,
}

impl<'a> CPU<'a> {
  const STACK_START: u16 = 0x0100;

  pub fn new<'b>(system: System<'b>) -> CPU<'b> {
    CPU {
      registers: Registers::new(),
      system,
      branched: false,
    }
  }

  fn reset(&mut self) {
    self.registers = Registers::new();
    self.registers.set_pc(self.system.readu16(0xFFFC));
  }

  pub fn start<F>(&mut self, callback: F)
  where
    F: FnMut(&mut CPU),
  {
    self.reset();
    self.run(callback);
  }

  fn run<F>(&mut self, mut callback: F)
  where
    F: FnMut(&mut CPU),
  {
    loop {
      if self.system.poll_nmi() {
        self.system.clear_nmi();
        self.interrupt(Interrupt::NMI);
      }

      callback(self);

      let instruction = Instruction::get(self.read());
      let operand = self.get_operand(instruction.mode, instruction.needs_data());

      self.execute_instr(instruction.opcode, operand);

      let cycles = instruction.cycles + instruction.extra * (operand.0.2 as u8) + self.branched();

      self.system.tick(cycles);
    }
  }

  fn interrupt(&mut self, interrupt: Interrupt) {
    if self.registers.get_flag(Flag::InterruptDisable) && (interrupt == Interrupt::BRK) {
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

    self.system.tick(interrupt.cycles);

    self
      .registers
      .set_pc(self.system.readu16(interrupt.read_address));
  }

  fn read(&mut self) -> u8 {
    let res = self.system.read(self.registers.get_pc());
    self.increment_pc(1);
    res
  }

  fn readu16(&mut self) -> u16 {
    let res = self.system.readu16(self.registers.get_pc());
    self.increment_pc(2);
    res
  }

  fn stack_push(&mut self, data: u8) {
    self.system.write(
      CPU::STACK_START.wrapping_add(self.registers.get(Register::SP) as u16),
      data,
    );
    self.registers.set(
      Register::SP,
      self.registers.get(Register::SP).wrapping_sub(1),
    );
  }

  fn stack_pop(&mut self) -> u8 {
    self.registers.set(
      Register::SP,
      self.registers.get(Register::SP).wrapping_add(1),
    );
    self
      .system
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

  fn branched(&mut self) -> u8 {
    if self.branched {
      self.branched = false;
      1
    } else {
      0
    }
  }

  fn update_zero_negative(&mut self, res: u8) {
    self.registers.change_flag(Flag::Zero, res == 0x00);
    self
      .registers
      .change_flag(Flag::Negative, res & 0x80 == 0x80);
  }

  fn get_operand_addr(&mut self, mode: Addressing) -> OperandAddress {
    match mode {
      Addressing::Accumulator => OperandAddress(0, mode, false),
      Addressing::Absolute => OperandAddress(self.readu16(), mode, false),
      Addressing::AbsoluteX => {
        let base_addr = self.readu16();
        let addr = base_addr.wrapping_add(self.registers.get(Register::X) as u16);
        let extra = (addr & 0xFF00) != (base_addr & 0xFF00);

        OperandAddress(addr, mode, extra)
      }
      Addressing::AbsoluteY => {
        let base_addr = self.readu16();
        let addr = base_addr.wrapping_add(self.registers.get(Register::Y) as u16);
        let extra = (addr & 0xFF00) != (base_addr & 0xFF00);

        OperandAddress(addr, mode, extra)
      }
      Addressing::Immediate => OperandAddress(self.registers.get_pc(), mode, false),
      Addressing::Implied => OperandAddress(0, mode, false),
      Addressing::Indirect => {
        let base_addr = self.readu16();
        let addr = if base_addr & 0x00FF == 0x00FF {
          let lo = self.system.read(base_addr) as u16;
          let hi = self.system.read(base_addr & 0xFF00) as u16;

          (hi << 8) | lo
        } else {
          self.system.readu16(base_addr)
        };

        OperandAddress(addr, mode, false)
      }
      Addressing::IndirectX => {
        let fetch_addr = self.read().wrapping_add(self.registers.get(Register::X));
        let lo = self.system.read(fetch_addr as u16) as u16;
        let hi = self.system.read(fetch_addr.wrapping_add(1) as u16) as u16;
        let addr = (hi << 8) | lo;

        OperandAddress(addr, mode, false)
      }
      Addressing::IndirectY => {
        let fetch_addr = self.read();
        let lo = self.system.read(fetch_addr as u16) as u16;
        let hi = self.system.read((fetch_addr).wrapping_add(1) as u16) as u16;
        let base_addr = (hi << 8) | lo;
        let addr = base_addr.wrapping_add(self.registers.get(Register::Y) as u16);
        let extra = (addr & 0xFF00) != (base_addr & 0xFF00);

        OperandAddress(addr, mode, extra)
      }
      Addressing::Relative => {
        let delta = self.read() as i8;
        let base_addr = self.registers.get_pc();
        let addr = base_addr.wrapping_add(delta as u16);
        let extra = (addr & 0xFF00) != (base_addr & 0xFF00);

        OperandAddress(addr, mode, extra)
      }
      Addressing::ZeroPage => OperandAddress(self.read() as u16, mode, false),
      Addressing::ZeroPageX => OperandAddress(
        self.read().wrapping_add(self.registers.get(Register::X)) as u16,
        mode,
        false,
      ),
      Addressing::ZeroPageY => OperandAddress(
        self.read().wrapping_add(self.registers.get(Register::Y)) as u16,
        mode,
        false,
      ),
    }
  }

  fn get_operand(&mut self, mode: Addressing, read_data: bool) -> Operand {
    match (mode, read_data) {
      (Addressing::Accumulator, _) => {
        Operand(self.get_operand_addr(mode), self.registers.get(Register::A))
      }
      (Addressing::Implied | Addressing::Indirect | Addressing::Relative, _) => {
        Operand(self.get_operand_addr(mode), 0x0)
      }
      (Addressing::Immediate, _) => Operand(self.get_operand_addr(mode), self.read()),
      (_, false) => Operand(self.get_operand_addr(mode), 0x0),
      _ => {
        let OperandAddress(addr, mode, extra) = self.get_operand_addr(mode);
        Operand(OperandAddress(addr, mode, extra), self.system.read(addr))
      }
    }
  }

  fn execute_instr(&mut self, opcode: OpCode, operand: Operand) {
    match opcode {
      OpCode::ADC   =>  self.adc(operand),
      OpCode::XALR  =>  self.alr(operand),
      OpCode::XANC  =>  self.anc(operand),
      OpCode::AND   =>  self.and(operand),
      OpCode::XANE  =>  self.ane(operand),
      OpCode::XARR  =>  self.arr(operand),
      OpCode::ASL   =>  self.asl(operand),
      OpCode::BCC   =>  self.bcc(operand),
      OpCode::BCS   =>  self.bcs(operand),
      OpCode::BEQ   =>  self.beq(operand),
      OpCode::BIT   =>  self.bit(operand),
      OpCode::BMI   =>  self.bmi(operand),
      OpCode::BNE   =>  self.bne(operand),
      OpCode::BPL   =>  self.bpl(operand),
      OpCode::BRK   =>  self.brk(),
      OpCode::BVC   =>  self.bvc(operand),
      OpCode::BVS   =>  self.bvs(operand),
      OpCode::CLC   =>  self.clc(),
      OpCode::CLD   =>  self.cld(),
      OpCode::CLI   =>  self.cli(),
      OpCode::CLV   =>  self.clv(),
      OpCode::CMP   =>  self.cmp(operand),
      OpCode::CPX   =>  self.cpx(operand),
      OpCode::CPY   =>  self.cpy(operand),
      OpCode::XDCP  =>  self.dcp(operand),
      OpCode::DEC   =>  self.dec(operand),
      OpCode::DEX   =>  self.dex(),
      OpCode::DEY   =>  self.dey(),
      OpCode::EOR   =>  self.eor(operand),
      OpCode::INC   =>  self.inc(operand),
      OpCode::INX   =>  self.inx(),
      OpCode::INY   =>  self.iny(),
      OpCode::XISC  =>  self.isc(operand),
      OpCode::JAM   =>  panic!("Console was jammed, please reboot."),
      OpCode::JMP   =>  self.jmp(operand),
      OpCode::JSR   =>  self.jsr(operand),
      OpCode::XLAS  =>  self.las(operand),
      OpCode::XLAX  =>  self.lax(operand),
      OpCode::LDA   =>  self.lda(operand),
      OpCode::LDX   =>  self.ldx(operand),
      OpCode::LDY   =>  self.ldy(operand),
      OpCode::LSR   =>  self.lsr(operand),
      OpCode::XLXA  =>  self.lxa(operand),
      OpCode::NOP   =>  self.nop(),
      OpCode::XNOP  =>  self.nop(),
      OpCode::ORA   =>  self.ora(operand),
      OpCode::PHA   =>  self.pha(),
      OpCode::PHP   =>  self.php(),
      OpCode::PLA   =>  self.pla(),
      OpCode::PLP   =>  self.plp(),
      OpCode::XRLA  =>  self.rla(operand),
      OpCode::ROL   =>  self.rol(operand),
      OpCode::ROR   =>  self.ror(operand),
      OpCode::XRRA  =>  self.rra(operand),
      OpCode::RTI   =>  self.rti(),
      OpCode::RTS   =>  self.rts(),
      OpCode::XSAX  =>  self.sax(operand),
      OpCode::SBC   =>  self.sbc(operand),
      OpCode::XSBC  =>  self.sbc(operand),
      OpCode::XSBX  =>  self.sbx(operand),
      OpCode::SEC   =>  self.sec(),
      OpCode::SED   =>  self.sed(),
      OpCode::SEI   =>  self.sei(),
      OpCode::XSHA  =>  self.sha(operand),
      OpCode::XSHX  =>  self.shx(operand),
      OpCode::XSHY  =>  self.shy(operand),
      OpCode::XSLO  =>  self.slo(operand),
      OpCode::XSRE  =>  self.sre(operand),
      OpCode::STA   =>  self.sta(operand),
      OpCode::STX   =>  self.stx(operand),
      OpCode::STY   =>  self.sty(operand),
      OpCode::XTAS  =>  self.tas(operand),
      OpCode::TAX   =>  self.tax(),
      OpCode::TAY   =>  self.tay(),
      OpCode::TSX   =>  self.tsx(),
      OpCode::TXA   =>  self.txa(),
      OpCode::TXS   =>  self.txs(),
      OpCode::TYA   =>  self.tya(),
    }
  }

  fn adc(&mut self, Operand(_, data): Operand) {
    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add(self.registers.get_flag(Flag::Carry) as u16)
      .wrapping_add(data as u16);

    let res = raw_res as u8;

    self
      .registers
      .change_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.change_flag(
      Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ data) & 0x80 != 0x00,
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn alr(&mut self, operand: Operand) {
    self.and(operand);
    self.lsr(operand);
  }

  fn anc(&mut self, operand: Operand) {
    self.and(operand);
    self
      .registers
      .change_flag(Flag::Carry, self.registers.get(Register::A) & 0x80 == 0x80);
  }

  fn and(&mut self, Operand(_, data): Operand) {
    self
      .registers
      .set(Register::A, self.registers.get(Register::A) & data);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn ane(&mut self, Operand(_, data): Operand) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);
    self.registers.set(Register::A, a & x & data);
  }

  fn arr(&mut self, operand: Operand) {
    self.and(operand);
    self.ror(operand);

    let b5 = self.registers.get(Register::A) & 0x20 == 0x20;
    let b6 = self.registers.get(Register::A) & 0x40 == 0x40;

    self.registers.change_flag(Flag::Carry, b5);
    self.registers.change_flag(Flag::Overflow, b5 ^ b6);
  }

  fn asl(&mut self, Operand(OperandAddress(addr, mode, _), data): Operand) {
    self.registers.change_flag(Flag::Carry, data & 0x80 == 0x80);
    self.update_zero_negative(data << 1);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, data << 1);
      }
      _ => {
        self.system.write(addr, data << 1);
      }
    }
  }

  fn bcc(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if !self.registers.get_flag(Flag::Carry) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn bcs(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if self.registers.get_flag(Flag::Carry) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn beq(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if self.registers.get_flag(Flag::Zero) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn bit(&mut self, Operand(_, data): Operand) {
    self
      .registers
      .change_flag(Flag::Zero, data & self.registers.get(Register::A) == 0);
    self
      .registers
      .change_flag(Flag::Overflow, data & 0x40 == 0x40);
    self
      .registers
      .change_flag(Flag::Negative, data & 0x80 == 0x80);
  }

  fn bmi(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if self.registers.get_flag(Flag::Negative) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn bne(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if !self.registers.get_flag(Flag::Zero) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn bpl(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if !self.registers.get_flag(Flag::Negative) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn brk(&mut self) {
    self.increment_pc(1);
    self.interrupt(Interrupt::BRK);
  }

  fn bvc(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if !self.registers.get_flag(Flag::Overflow) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn bvs(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    if self.registers.get_flag(Flag::Overflow) {
      self.registers.set_pc(addr);
      self.branched = true;
    }
  }

  fn clc(&mut self) {
    self.registers.unset_flag(Flag::Carry);
  }

  fn cld(&mut self) {
    self.registers.unset_flag(Flag::Decimal);
  }

  fn cli(&mut self) {
    self.registers.unset_flag(Flag::InterruptDisable);
  }

  fn clv(&mut self) {
    self.registers.unset_flag(Flag::Overflow);
  }

  fn cmp(&mut self, Operand(_, data): Operand) {
    let comparable = self.registers.get(Register::A);
    self.registers.change_flag(Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data));
  }

  fn cpx(&mut self, Operand(_, data): Operand) {
    let comparable = self.registers.get(Register::X);
    self.registers.change_flag(Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data));
  }

  fn cpy(&mut self, Operand(_, data): Operand) {
    let comparable = self.registers.get(Register::Y);
    self.registers.change_flag(Flag::Carry, data <= comparable);
    self.update_zero_negative(comparable.wrapping_sub(data));
  }

  fn dcp(&mut self, operand: Operand) {
    self.dec(operand);
    self.cmp(operand);
  }

  fn dec(&mut self, Operand(OperandAddress(addr, _, _), data): Operand) {
    self.system.write(addr, data.wrapping_sub(1));
    self.update_zero_negative(data.wrapping_sub(1));
  }

  fn dex(&mut self) {
    self
      .registers
      .set(Register::X, self.registers.get(Register::X).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(Register::X));
  }

  fn dey(&mut self) {
    self
      .registers
      .set(Register::Y, self.registers.get(Register::Y).wrapping_sub(1));
    self.update_zero_negative(self.registers.get(Register::Y));
  }

  fn eor(&mut self, Operand(_, data): Operand) {
    self
      .registers
      .set(Register::A, self.registers.get(Register::A) ^ data);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn inc(&mut self, Operand(OperandAddress(addr, _, _), data): Operand) {
    self.system.write(addr, data.wrapping_add(1));
    self.update_zero_negative(data.wrapping_add(1));
  }

  fn inx(&mut self) {
    self
      .registers
      .set(Register::X, self.registers.get(Register::X).wrapping_add(1));
    self.update_zero_negative(self.registers.get(Register::X));
  }

  fn iny(&mut self) {
    self
      .registers
      .set(Register::Y, self.registers.get(Register::Y).wrapping_add(1));
    self.update_zero_negative(self.registers.get(Register::Y));
  }

  fn isc(&mut self, operand: Operand) {
    self.inc(operand);
    self.sbc(operand);
  }

  fn jmp(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.registers.set_pc(addr);
  }

  fn jsr(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.stack_pushu16(self.registers.get_pc().wrapping_sub(1));
    self.registers.set_pc(addr);
  }

  fn las(&mut self, Operand(_, data): Operand) {
    let data = data & self.registers.get(Register::SP);

    self.registers.set(Register::A, data);
    self.registers.set(Register::X, data);
    self.registers.set(Register::SP, data);

    self.update_zero_negative(data);
  }

  fn lax(&mut self, operand: Operand) {
    self.lda(operand);
    self.ldx(operand);
  }

  fn lda(&mut self, Operand(_, data): Operand) {
    self.registers.set(Register::A, data);
    self.update_zero_negative(data);
  }

  fn ldx(&mut self, Operand(_, data): Operand) {
    self.registers.set(Register::X, data);
    self.update_zero_negative(data);
  }

  fn ldy(&mut self, Operand(_, data): Operand) {
    self.registers.set(Register::Y, data);
    self.update_zero_negative(data);
  }

  fn lsr(&mut self, Operand(OperandAddress(addr, mode, _), data): Operand) {
    self.registers.change_flag(Flag::Carry, data & 0x01 == 0x01);
    self.update_zero_negative(data >> 1);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, data >> 1);
      }
      _ => {
        self.system.write(addr, data >> 1);
      }
    }
  }

  fn lxa(&mut self, Operand(_, data): Operand) {
    let res = self.registers.get(Register::A) & data;

    self.registers.set(Register::A, res);
    self.registers.set(Register::X, res);
    self.update_zero_negative(res);
  }

  fn nop(&mut self) {}

  fn ora(&mut self, Operand(_, data): Operand) {
    self
      .registers
      .set(Register::A, self.registers.get(Register::A) | data);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn pha(&mut self) {
    self.stack_push(self.registers.get(Register::A));
  }

  fn php(&mut self) {
    let status = self.registers.get(Register::P) | 0x30;
    self.stack_push(status);
  }

  fn pla(&mut self) {
    let val = self.stack_pop();
    self.registers.set(Register::A, val);
    self.update_zero_negative(val);
  }

  fn plp(&mut self) {
    let status = self.stack_pop() & 0xEF | 0x20;
    self.registers.set(Register::P, status);
  }

  fn rla(&mut self, operand: Operand) {
    self.rol(operand);
    self.and(operand);
  }

  fn rol(&mut self, Operand(OperandAddress(addr, mode, _), data): Operand) {
    let prev_carry = self.registers.get_flag(Flag::Carry);
    self.registers.change_flag(Flag::Carry, data & 0x80 == 0x80);

    let res = (data << 1) | (prev_carry as u8);
    self.update_zero_negative(res);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, res);
      }
      _ => {
        self.system.write(addr, res);
      }
    }
  }

  fn ror(&mut self, Operand(OperandAddress(addr, mode, _), data): Operand) {
    let prev_carry = self.registers.get_flag(Flag::Carry);
    self.registers.change_flag(Flag::Carry, data & 0x01 == 0x01);

    let res = (data >> 1) | (prev_carry as u8) << 7;
    self.update_zero_negative(res);

    match mode {
      Addressing::Accumulator => {
        self.registers.set(Register::A, res);
      }
      _ => {
        self.system.write(addr, res);
      }
    }
  }

  fn rra(&mut self, operand: Operand) {
    self.ror(operand);
    self.adc(operand);
  }

  fn rti(&mut self) {
    let status = self.stack_pop();
    self.registers.set(Register::P, status & 0xEF | 0x20);

    let pc = self.stack_popu16();
    self.registers.set_pc(pc);
  }

  fn rts(&mut self) {
    let pc = self.stack_popu16().wrapping_add(1);
    self.registers.set_pc(pc);
  }

  fn sax(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);

    self.system.write(addr, a & x);
  }

  fn sbc(&mut self, Operand(_, data): Operand) {
    let val = data.wrapping_neg().wrapping_sub(1);

    let raw_res = (self.registers.get(Register::A) as u16)
      .wrapping_add(self.registers.get_flag(Flag::Carry) as u16)
      .wrapping_add(val as u16);

    let res = raw_res as u8;

    self
      .registers
      .change_flag(Flag::Carry, (res as u16) < raw_res);
    self.registers.change_flag(
      Flag::Overflow,
      (res ^ self.registers.get(Register::A)) & (res ^ val) & 0x80 != 0x00,
    );

    self.registers.set(Register::A, res);
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn sbx(&mut self, Operand(_, data_a): Operand) {
    let data_b = self.registers.get(Register::A) & self.registers.get(Register::X);

    let res = data_b.wrapping_sub(data_a);

    self.registers.change_flag(Flag::Carry, data_b <= data_a);
    self.update_zero_negative(res);
    self.registers.set(Register::X, res);
  }

  fn sec(&mut self) {
    self.registers.set_flag(Flag::Carry);
  }

  fn sed(&mut self) {
    self.registers.set_flag(Flag::Decimal);
  }

  fn sei(&mut self) {
    self.registers.set_flag(Flag::InterruptDisable);
  }

  fn sha(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(
      addr,
      self.registers.get(Register::A)
        & self.registers.get(Register::X)
        & ((addr >> 8) as u8).wrapping_add(1),
    );
  }

  fn shx(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(
      addr,
      self.registers.get(Register::X) & ((addr >> 8) as u8).wrapping_add(1),
    );
  }

  fn shy(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(
      addr,
      self.registers.get(Register::Y) & ((addr >> 8) as u8).wrapping_add(1),
    );
  }

  fn slo(&mut self, operand: Operand) {
    self.asl(operand);
    self.ora(operand);
  }

  fn sre(&mut self, operand: Operand) {
    self.lsr(operand);
    self.eor(operand);
  }

  fn sta(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(addr, self.registers.get(Register::A));
  }

  fn stx(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(addr, self.registers.get(Register::X));
  }

  fn sty(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    self.system.write(addr, self.registers.get(Register::Y));
  }

  fn tas(&mut self, Operand(OperandAddress(addr, _, _), _): Operand) {
    let a = self.registers.get(Register::A);
    let x = self.registers.get(Register::X);

    self.registers.set(Register::SP, a & x);
    self
      .system
      .write(addr, a & x & ((addr >> 8) as u8).wrapping_add(1));
  }

  fn tax(&mut self) {
    self
      .registers
      .set(Register::X, self.registers.get(Register::A));
    self.update_zero_negative(self.registers.get(Register::X));
  }

  fn tay(&mut self) {
    self
      .registers
      .set(Register::Y, self.registers.get(Register::A));
    self.update_zero_negative(self.registers.get(Register::Y));
  }

  fn tsx(&mut self) {
    self
      .registers
      .set(Register::X, self.registers.get(Register::SP));
    self.update_zero_negative(self.registers.get(Register::X));
  }

  fn txa(&mut self) {
    self
      .registers
      .set(Register::A, self.registers.get(Register::X));
    self.update_zero_negative(self.registers.get(Register::A));
  }

  fn txs(&mut self) {
    self
      .registers
      .set(Register::SP, self.registers.get(Register::X));
  }

  fn tya(&mut self) {
    self
      .registers
      .set(Register::A, self.registers.get(Register::Y));
    self.update_zero_negative(self.registers.get(Register::A));
  }
}
