use super::{envelope::Envelope, lengthcounter::LengthCounter, timer::Timer};

pub struct Noise {
  enabled: bool,
  envelope: Envelope,
  length: LengthCounter,
  shift: Shift,
  timer: Timer,
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
      value: 0x1,
    }
  }

  fn amount(&self) -> u8 {
    match self.mode {
      ShiftMode::One => 1,
      ShiftMode::Six => 6,
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
      timer: Timer::new(),
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
    self.timer.period = Noise::FREQ_TABLE[(val & 0xF) as usize];
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

  pub fn timer(&mut self) {
    if self.timer.tick() {
      let feedback = (self.shift.value & 0x01) ^ ((self.shift.value >> self.shift.amount()) & 0x01);
      self.shift.value = (self.shift.value & 0x7FFF) | (feedback << 14);
      self.shift.value >>= 1;
    }
  }

  pub fn quarter(&mut self) {
    self.envelope.tick();
  }

  pub fn half(&mut self) {
    self.length.tick();
  }

  pub fn signal(&self) -> f32 {
    if self.enabled && self.shift.value & 0x01 == 0x0 && self.length.counter != 0 {
      if self.envelope.enabled {
        self.envelope.volume as f32
      } else {
        self.envelope.rate as f32
      }
    } else {
      0.0
    }
  }
}
