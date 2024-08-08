use super::{timer::Timer, IRQ};

pub struct DMC {
  pub irq: IRQ,
  looped: bool,
  timer: Timer,
  sample: Sample,
  load: Sample,
  dma: DMA,
  buffer: Buffer,
}

#[derive(Clone)]
struct Sample {
  address: u16,
  length: u16
}

struct DMA {
  pending: bool,
  delay: u8,
}

struct Buffer {
  value: u8,
  bits: u8,
  shift: u8,
  silent: bool,
  reload: bool,
  sample: u8,
}

impl Sample {
  fn new() -> Self {
    Sample {
      address: 0x0,
      length: 0,
    }
  }
}

impl DMA {
  fn new() -> Self {
    DMA {
      pending: false,
      delay: 0,
    }
  }

  fn tick(&mut self) -> bool {
    if self.delay > 0 {
      self.delay -= 1;

      if self.delay == 0 {
        return true;
      }
    }
    false
  }
}

impl Buffer {
  fn new() -> Self {
    Buffer {
      value: 0x0,
      bits: 0,
      shift: 0x0,
      silent: true,
      reload: false,
      sample: 0x0,
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
      timer: Timer::new(),
      sample: Sample::new(),
      load: Sample::new(),
      dma: DMA::new(),
      buffer: Buffer::new(),
    }
  }

  pub fn length(&self) -> u16 {
    self.sample.length
  }

  pub fn set_enabled(&mut self, enabled: bool, cycle: usize) {
    self.irq.pending = false;

    if enabled {
      if self.sample.length == 0 {
        self.reset();
        self.dma.delay = if cycle & 0x01 == 0x01 { 3 } else { 2 };
      }
    } else {
      self.sample.length = 0;
    }
  }

  fn reset(&mut self) {
    self.sample = self.load.clone();
  }

  pub fn write_control(&mut self, val: u8) {
    self.irq.set_enabled(val & 0x80 != 0x0);
    self.looped = val & 0x40 != 0x0;
    self.timer.period = DMC::FREQ_TABLE[(val & 0xF) as usize];
  }

  pub fn write_output(&mut self, val: u8) {
    self.buffer.value = val & 0x7F;
  }

  pub fn write_address(&mut self, val: u8) {
    self.load.address = 0xC000 | ((val as u16) << 6);
  }

  pub fn write_length(&mut self, val: u8) {
    self.load.length = ((val as u16) << 4) + 1;
  }

  pub fn dma(&mut self) -> bool {
    let res = self.dma.pending;
    self.dma.pending = false;
    res
  }

  pub fn dma_addr(&self) -> u16 {
    self.sample.address
  }

  pub fn load(&mut self, val: u8) {
    if self.sample.length > 0 {
      self.buffer.sample = val;
      self.buffer.reload = false;

      self.sample.address = self.sample.address.checked_add(1).unwrap_or(0x8000);
      self.sample.length -= 1;

      if self.sample.length == 0 {
          if self.looped {
              self.reset();
          } else if self.irq.enabled {
              self.irq.pending = true;
          }
      }
  }
  }

  pub fn tick_dma(&mut self) {
    if self.dma.tick() {
      if self.buffer.reload && self.sample.length > 0 {
        self.dma.pending = true;
      }
    }
  }

  pub fn timer(&mut self) {
    if self.timer.tick_cycles(2) {
      if !self.buffer.silent {
        if self.buffer.shift & 0x01 == 0x01 {
          if self.buffer.value <= 0x7D {
            self.buffer.value += 0x02;
          }
        } else if self.buffer.value >= 0x02 {
          self.buffer.value -= 0x02;
        }

        self.buffer.shift >>= 1;
      }

      self.buffer.bits = self.buffer.bits.saturating_sub(1);

      if self.buffer.bits == 0 {
        self.buffer.bits = 8;

        if self.buffer.reload {
          self.buffer.silent = true;
        } else {
          self.buffer.silent = false;
          self.buffer.shift = self.buffer.sample;
          self.buffer.reload = true;

          if self.sample.length > 0 {
            self.dma.pending = true;
          }
        }
      }
    }
  }

  pub fn signal(&self) -> f32 {
    self.buffer.value as f32
  }
}
