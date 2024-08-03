use super::{envelope::Envelope, lengthcounter::LengthCounter};

pub struct Noise {
  enabled: bool,
  envelope: Envelope,
  length: LengthCounter,
  shift: Shift,
  timer: u16,
}

struct Shift {
  mode: ShiftMode,
  value: u16,
}

enum ShiftMode {
  One,
  Six,
}

impl Shift {
  fn new() -> Self {
    Shift {
      mode: ShiftMode::One,
      value: 0,
    }
  }
}

impl Noise {
  const FREQ_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
  ];

  pub fn new() -> Self {
    Noise {
      enabled: false,
      envelope: Envelope::new(),
      length: LengthCounter::new(),
      shift: Shift::new(),
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

  pub fn write_control(&mut self, val: u8) {
    self.length.halted = (val & 0x20) != 0;
    self.envelope.set(val);
  }

  pub fn write_timer(&mut self, val: u8) {
    self.timer = Noise::FREQ_TABLE[(val & 0xF) as usize];
    self.shift.mode = if val & 0x80 == 0x0 {
      ShiftMode::One
    } else {
      ShiftMode::Six
    };
  }

  pub fn write_length(&mut self, val: u8) {
    self.envelope.reset();
    if self.enabled {
      self.length.update(val >> 3);
    }

  }
}
