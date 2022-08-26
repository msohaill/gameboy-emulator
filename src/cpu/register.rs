pub struct Registers {
  accumulator: u8,
  x_index: u8,
  y_index: u8,
  program_status: u8,
  program_counter: u16,
  stack_pointer: u8,
}

pub enum RegisterType {
  A, X, Y, P, PC, SP,
}

pub struct Flags (bool, bool, bool, bool, bool, bool, bool, bool);
// (Carry, Zero, Interrupt Disable, Decimal, B, B, Overflow, Negative)

impl Registers {
  pub fn new() -> Self {
    Registers {
      accumulator: 0,
      x_index: 0,
      y_index: 0,
      program_status: 0,
      program_counter: 0,
      stack_pointer: 0,
    }
  }

  pub fn get(&self, reg: RegisterType) -> u8 {
    match reg {
      RegisterType::A => self.accumulator,
      RegisterType::X => self.x_index,
      RegisterType::Y => self.y_index,
      RegisterType::P => self.program_status,
      RegisterType::SP => self.stack_pointer,
      RegisterType::PC => panic!("Please use `get_pc` to retrieve the program counter."),
    }
  }

  pub fn set(&mut self, reg: RegisterType, val: u8) {
    match reg {
      RegisterType::A => self.accumulator = val,
      RegisterType::X => self.x_index = val,
      RegisterType::Y => self.y_index = val,
      RegisterType::P => self.program_status = val,
      RegisterType::SP => self.stack_pointer = val,
      RegisterType::PC => panic!("Please use `set_pc` to set the program counter."),
    }
  }

  pub fn get_pc(&self) -> u16 {
    self.program_counter
  }

  pub fn set_pc(&mut self, val: u16) {
    self.program_counter = val;
  }

  pub fn get_flags(&self) -> Flags {
    Flags(
      self.program_status & 0b1 != 0,
      (self.program_status >> 1) & 0b1 != 0,
      (self.program_status >> 2) & 0b1 != 0,
      (self.program_status >> 3) & 0b1 != 0,
      (self.program_status >> 4) & 0b1 != 0,
      (self.program_status >> 5) & 0b1 != 0,
      (self.program_status >> 6) & 0b1 != 0,
      (self.program_status >> 7) & 0b1 != 0,
    )
  }

  pub fn set_flag(&mut self, flag: u8, val: bool) {
    match flag {
      0 => self.program_status = if val { 1 | self.program_status } else { !(1) & self.program_status },
      1 => self.program_status = if val { (1 << 1) | self.program_status } else { !(1 << 1) & self.program_status },
      2 => self.program_status = if val { (1 << 2) | self.program_status } else { !(1 << 2) & self.program_status },
      3 => self.program_status = if val { (1 << 3) | self.program_status } else { !(1 << 3) & self.program_status },
      4 => self.program_status = if val { (1 << 4) | self.program_status } else { !(1 << 4) & self.program_status },
      5 => self.program_status = if val { (1 << 5) | self.program_status } else { !(1 << 5) & self.program_status },
      6 => self.program_status = if val { (1 << 6) | self.program_status } else { !(1 << 6) & self.program_status },
      7 => self.program_status = if val { (1 << 7) | self.program_status } else { !(1 << 7) & self.program_status },
      _ => panic!("Invalid flag position."),
    }
  }
}