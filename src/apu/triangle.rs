use super::{lengthcounter::LengthCounter, timer::Timer};

pub struct Triangle {
  enabled: bool,
  linear: LinearCounter,
  length: LengthCounter,
  timer: Timer,
  step: u8,
}

struct LinearCounter {
  reload: bool,
  control: bool,
  counter: u8,
  period: u8,
}

impl LinearCounter {
  fn new() -> Self {
    LinearCounter {
      reload: false,
      control: false,
      counter: 0,
      period: 0,
    }
  }

  fn set(&mut self, val: u8) {
    self.period = val;
  }

  fn load(&mut self) {
    self.counter = self.period;
  }
}

impl Triangle {
  pub fn new() -> Self {
    Triangle {
      enabled: false,
      linear: LinearCounter::new(),
      length: LengthCounter::new(),
      timer: Timer::new(),
      step: 0x0,
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
    self.timer.period = (self.timer.period & 0xFF00) | val as u16;
  }

  pub fn write_timer_hi(&mut self, val: u8) {
    self.timer.period = (self.timer.period & 0x00FF) | (((val & 0x7) as u16) << 8);
    self.timer.reset();
    self.linear.reload = true;

    if self.enabled {
      self.length.update(val >> 3);
    }
  }

  pub fn timer(&mut self) {
    if self.timer.tick() {
      if self.length.counter > 0 && self.linear.counter > 0 {
        self.step = (self.step + 1) % 0x20;
      }
    }
  }

  pub fn quarter(&mut self) {
    if self.linear.reload {
      self.linear.load();
    } else if self.linear.counter > 0 {
      self.linear.counter -= 1;
    }

    if !self.linear.control {
      self.linear.reload = false;
    }
  }

  pub fn half(&mut self) {
    self.length.tick();
  }

  pub fn signal(&self) -> f32 {
    if self.enabled && self.length.counter != 0 && self.linear.counter != 0 {
      if self.step & 0x10 != 0x10 {
        (self.step ^ 0xF) as f32
      } else {
        (self.step - 0x10) as f32
      }
    } else {
      0.0
    }
  }
}
