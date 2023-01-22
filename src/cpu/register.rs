pub struct Registers {
  accumulator: u8,
  x_index: u8,
  y_index: u8,
  program_status: u8,
  program_counter: u16,
  stack_pointer: u8,
}

pub enum Register {
  A, X, Y, P, PC, SP,
}

pub enum Flag {
  Carry, Zero, InterruptDisable, Decimal,
  B1, B2, Overflow, Negative
}

impl Registers {
  pub fn new() -> Self {
    Registers {
      accumulator: 0,
      x_index: 0,
      y_index: 0,
      program_status: 0b0010_0100,
      program_counter: 0,
      stack_pointer: 0xFD,
    }
  }

  pub fn get(&self, reg: &Register) -> u8 {
    match reg {
      Register::A => self.accumulator,
      Register::X => self.x_index,
      Register::Y => self.y_index,
      Register::P => self.program_status,
      Register::SP => self.stack_pointer,
      Register::PC => panic!("Please use `get_pc` to retrieve the program counter."),
    }
  }

  pub fn set(&mut self, reg: &Register, val: u8) {
    match reg {
      Register::A => self.accumulator = val,
      Register::X => self.x_index = val,
      Register::Y => self.y_index = val,
      Register::P => self.program_status = val,
      Register::SP => self.stack_pointer = val,
      Register::PC => panic!("Please use `set_pc` to set the program counter."),
    }
  }

  pub fn get_pc(&self) -> u16 {
    self.program_counter
  }

  pub fn set_pc(&mut self, val: u16) {
    self.program_counter = val;
  }

  pub fn get_flag(&self, flag: &Flag) -> bool {
    match flag {
      Flag::Carry => self.program_status & 0b1 != 0,
      Flag::Zero => (self.program_status >> 1) & 0b1 != 0,
      Flag::InterruptDisable => (self.program_status >> 2) & 0b1 != 0,
      Flag::Decimal => (self.program_status >> 3) & 0b1 != 0,
      Flag::B1 => (self.program_status >> 4) & 0b1 != 0,
      Flag::B2 => (self.program_status >> 5) & 0b1 != 0,
      Flag::Overflow => (self.program_status >> 6) & 0b1 != 0,
      Flag::Negative => (self.program_status >> 7) & 0b1 != 0
    }
  }

  pub fn set_flag(&mut self, flag: &Flag, val: bool) {
    match flag {
      Flag::Carry =>
        self.program_status = if val { 1 | self.program_status } else { !(1) & self.program_status },
      Flag::Zero =>
        self.program_status = if val { (1 << 1) | self.program_status } else { !(1 << 1) & self.program_status },
      Flag::InterruptDisable =>
        self.program_status = if val { (1 << 2) | self.program_status } else { !(1 << 2) & self.program_status },
      Flag::Decimal =>
        self.program_status = if val { (1 << 3) | self.program_status } else { !(1 << 3) & self.program_status },
      Flag::B1 =>
        self.program_status = if val { (1 << 4) | self.program_status } else { !(1 << 4) & self.program_status },
      Flag::B2 =>
        self.program_status = if val { (1 << 5) | self.program_status } else { !(1 << 5) & self.program_status },
      Flag::Overflow =>
        self.program_status = if val { (1 << 6) | self.program_status } else { !(1 << 6) & self.program_status },
      Flag::Negative =>
        self.program_status = if val { (1 << 7) | self.program_status } else { !(1 << 7) & self.program_status },
    }
  }
}