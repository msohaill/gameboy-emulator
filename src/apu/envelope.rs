pub struct Envelope {
  pub looped: bool,
  pub enabled: bool,
  pub rate: u8,
  pub volume: u8,
  reset: bool,
  counter: u8,
}

impl Envelope {
  pub fn new() -> Self {
    Envelope {
      looped: false,
      enabled: false,
      rate: 0,
      volume: 0,
      reset: false,
      counter: 0,
    }
  }

  pub fn set(&mut self, val: u8) {
    self.looped = (val & 0x20) != 0;
    self.enabled = (val & 0x10) == 0;
    self.rate = val & 0xF;
  }

  pub fn reset(&mut self) {
    self.reset = true;
  }
}
