use crate::bus::Bus;

pub struct Address {
  value: u16,
  latch: bool,
}

impl Address {
  pub fn new() -> Self {
    Address {
      value: 0x0,
      latch: true,
    }
  }

  pub fn write(&mut self, data: u8) {
    if self.latch {
      self.value &= 0xFF;
      self.value |= (data as u16) << 8;
    } else {
      self.value &= 0xFF00;
      self.value |= data as u16;
    }

    self.mirror();
    self.latch = !self.latch;
  }

  pub fn read(&mut self) -> u16 {
    self.value
  }

  pub fn increment(&mut self, inc: u8) {
    self.value = self.value.wrapping_add(inc as u16);
    self.mirror();
  }

  fn mirror(&mut self) {
    if self.value > Bus::PPU_END {
      self.value &= Bus::PPU_END;
    }
  }

  pub fn reset(&mut self) {
    self.latch = true;
  }
}
