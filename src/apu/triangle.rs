use super::lengthcounter::LengthCounter;

pub struct Triangle {
  enabled: bool,
  linear: LinearCounter,
  length: LengthCounter,
  timer: u16,
}

struct LinearCounter {
  reload: bool,
  control: bool,
  value: u8,
  buffer: u8,
}

impl LinearCounter {
  fn new() -> Self {
    LinearCounter {
      reload: false,
      control: false,
      value: 0,
      buffer: 0,
    }
  }

  fn set(&mut self, val: u8) {
    self.buffer = val;
  }

  fn load(&mut self) {
    self.value = self.buffer;
  }
}

impl Triangle {
  pub fn new() -> Self {
    Triangle {
      enabled: false,
      linear: LinearCounter::new(),
      length: LengthCounter::new(),
      timer: 0x0,
    }
  }

  pub fn length_counter(&self) -> u8 {
    self.length.counter
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = enabled;

    if !enabled {
      self.length.counter = 0;
    }
  }

  pub fn write_counter(&mut self, val: u8) {
    self.linear.control = val & 0x80 != 0x0;
    self.length.halted = val & 0x80 != 0x0;
    self.linear.set(val & 0x7F);
  }

  pub fn write_timer_lo(&mut self, val: u8) {
    self.timer = (self.timer & 0xFF00) | val as u16;
  }

  pub fn write_timer_hi(&mut self, val: u8) {
    self.timer = (self.timer & 0x00FF) | (((val & 0x7) as u16) << 8);
    self.linear.reload = true;

    if self.enabled {
      self.length.update(val >> 3);
    }
  }
}
