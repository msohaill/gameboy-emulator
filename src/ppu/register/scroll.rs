pub struct Scroll {
  pub scrollx: u8,
  pub scrolly: u8,
  latch: bool,
}

impl Scroll {
  pub fn new() -> Self {
    Scroll {
      scrollx: 0x0,
      scrolly: 0x0,
      latch: true,
    }
  }

  pub fn write(&mut self, data: u8) {
    if self.latch {
      self.scrollx = data;
    } else {
      self.scrolly = data;
    }

    self.latch = !self.latch;
  }

  pub fn reset(&mut self) {
    self.latch = true;
  }
}
