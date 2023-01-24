crate::utils::bitflag!(ProgramStatus,
  Carry,
  Zero,
  InterruptDisable,
  Decimal,
  B1,
  B2,
  Overflow,
  Negative,
);

pub struct Registers {
  accumulator: u8,
  x_index: u8,
  y_index: u8,
  program_status: ProgramStatus,
  program_counter: u16,
  stack_pointer: u8,
}

#[derive(Clone, Copy)]
pub enum Register {
  A, X, Y, P, PC, SP,
}

impl Registers {
  pub fn new() -> Self {
    Registers {
      accumulator: 0,
      x_index: 0,
      y_index: 0,
      program_status: ProgramStatus::new(0b0010_0100),
      program_counter: 0,
      stack_pointer: 0xFD,
    }
  }

  pub fn get(&self, reg: Register) -> u8 {
    match reg {
      Register::A => self.accumulator,
      Register::X => self.x_index,
      Register::Y => self.y_index,
      Register::P => self.program_status.get(),
      Register::SP => self.stack_pointer,
      Register::PC => panic!("Please use `get_pc` to retrieve the program counter."),
    }
  }

  pub fn set(&mut self, reg: Register, val: u8) {
    match reg {
      Register::A => self.accumulator = val,
      Register::X => self.x_index = val,
      Register::Y => self.y_index = val,
      Register::P => self.program_status.set(val),
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

  pub fn get_flag(&self, flag: Flag) -> bool {
    self.program_status.get_flag(flag)
  }

  pub fn set_flag(&mut self, flag: Flag, val: bool) {
   self.program_status.set_flag(flag, val);
  }
}