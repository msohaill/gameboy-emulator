pub struct Timer {
  pub period: u16,
  current: u16,
}

impl Timer {
  pub fn new() -> Self {
    Timer {
      period: 0x0,
      current: 0x0,
    }
  }

  pub fn reset(&mut self) {
    self.current = self.period;
  }

  pub fn tick(&mut self) -> bool {
    self.tick_cycles(1)
  }

  pub fn tick_cycles(&mut self, cycles: u16) -> bool {
    if self.current > 0 {
      self.current -= cycles;
      false
    } else {
      self.reset();
      true
    }
  }
}
