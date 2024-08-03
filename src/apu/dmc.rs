use super::IRQ;

pub struct DMC {
  pub irq: IRQ,
  looped: bool,
  output: u8,
  timer: u16,
  length: u16,
  sample: Sample,
  load: Sample,
}

#[derive(Clone)]
struct Sample {
  address: u16,
  length: u16
}

impl Sample {
  fn new() -> Self {
    Sample {
      address: 0x0,
      length: 0,
    }
  }
}

impl DMC {
  const FREQ_TABLE: [u16; 16] = [
    0x1AC, 0x17C, 0x154, 0x140, 0x11E, 0x0FE, 0x0E2, 0x0D6,
    0x0BE, 0x0A0, 0x08E, 0x080, 0x06A, 0x054, 0x048, 0x036,
  ];

  pub fn new() -> Self {
    DMC {
      irq: IRQ::new(),
      looped: false,
      output: 0,
      timer: 0,
      length: 0,
      sample: Sample::new(),
      load: Sample::new(),
    }
  }

  pub fn length(&self) -> u16 {
    self.length
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.irq.pending = false;

    if enabled {
      if self.length == 0 {
        self.reset();
      }
    } else {
      self.length = 0;
    }
  }

  fn reset(&mut self) {
    self.sample = self.load.clone();
  }

  pub fn write_control(&mut self, val: u8) {
    self.irq.set_enabled(val & 0x80 != 0x0);
    self.looped = val & 0x40 != 0x0;
    self.timer = DMC::FREQ_TABLE[(val & 0xF) as usize];
  }

  pub fn write_output(&mut self, val: u8) {
    self.output = val & 0x7F;
  }

  pub fn write_address(&mut self, val: u8) {
    self.load.address = 0xC000 | ((val as u16) << 6);
  }

  pub fn write_length(&mut self, val: u8) {
    self.load.length = ((val as u16) << 4) + 1;
  }
}
