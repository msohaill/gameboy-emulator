use super::{envelope::Envelope, lengthcounter::LengthCounter};

pub struct Pulse {
  enabled: bool,
  sweep: Sweep,
  timer: u16,
  length: LengthCounter,
  envelope: Envelope,
  duty: Duty,
}

struct Sweep {
  enabled: bool,
  period: u8,
  negated: bool,
  shift: u8,
  reload: bool,
}

struct Duty {
  value: u8,
  cycle: u8,
}

impl Sweep {
  fn new() -> Self {
    Sweep {
      enabled: false,
      period: 0,
      negated: false,
      shift: 0,
      reload: false,
    }
  }

  fn update(&mut self, val: u8) {
    self.enabled = (val & 0x80) != 0;
    self.period = (val >> 4) & 0x07;
    self.negated = (val & 0x08) != 0;
    self.shift = val & 0x07;

    self.reload = true;
  }
}

impl Duty {
  const TABLE: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],  // 12.5%
    [0, 1, 1, 0, 0, 0, 0, 0],  // 25%
    [0, 1, 1, 1, 1, 0, 0, 0],  // 50%
    [1, 0, 0, 1, 1, 1, 1, 1],  // 75%
  ];

  fn new() -> Self {
    Duty {
      value: 0,
      cycle: 0,
    }
  }

  fn reset(&mut self) {
    self.value = 0;
  }
}

impl Pulse {
  pub fn new() -> Self {
    Pulse {
      enabled: false,
      sweep: Sweep::new(),
      timer: 0,
      length: LengthCounter::new(),
      envelope: Envelope::new(),
      duty: Duty::new(),
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

  pub fn write_control(&mut self, val: u8) {
    self.duty.cycle = (val >> 6) & 0x03;
    self.length.halted = (val & 0x20) != 0;
    self.envelope.set(val);
  }

  pub fn write_sweep(&mut self, val: u8) {
    self.sweep.update(val);
  }

  pub fn write_timer_lo(&mut self, val: u8) {
    self.timer = (self.timer & 0xFF00) | val as u16;
  }

  pub fn write_timer_hi(&mut self, val: u8) {
    self.timer = (self.timer & 0x00FF) | (((val & 0x7) as u16) << 8);
    self.length.update(val >> 3);

    self.duty.reset();
    self.envelope.reset();
  }
}
