use super::{envelope::Envelope, lengthcounter::LengthCounter, timer::Timer};

pub struct Pulse {
  channel: PulseChannel,
  enabled: bool,
  sweep: Sweep,
  timer: Timer,
  length: LengthCounter,
  envelope: Envelope,
  duty: Duty,
}

pub enum PulseChannel {
  One,
  Two,
}

struct Sweep {
  enabled: bool,
  period: u8,
  counter: u8,
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
      counter: 0,
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
  pub fn new(channel: PulseChannel) -> Self {
    Pulse {
      channel,
      enabled: false,
      sweep: Sweep::new(),
      timer: Timer::new(),
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
    self.timer.period = (self.timer.period & 0xFF00) | val as u16;
  }

  pub fn write_timer_hi(&mut self, val: u8) {
    self.timer.period = (self.timer.period & 0x00FF) | (((val & 0x7) as u16) << 8);
    self.timer.reset();

    self.duty.reset();
    self.envelope.reset();

    if self.enabled {
      self.length.update(val >> 3);
    }
  }

  pub fn timer(&mut self) {
    if self.timer.tick() {
      self.duty.value = (self.duty.value + 1) % 0x08;
    }
  }

  pub fn quarter(&mut self) {
    self.envelope.tick();
  }

  pub fn half(&mut self) {
    self.sweep_tick();
    self.length.tick();
  }

  fn sweep_tick(&mut self) {
    if self.sweep.reload {
      self.sweep.counter = self.sweep.period;
      self.sweep.reload = false;
    } else if self.sweep.counter > 0 {
      self.sweep.counter -= 1;
    } else {
      self.sweep.counter = self.sweep.period;

      if self.sweep.enabled && !self.sweep_silent() {
        let delta = self.timer.period >> self.sweep.shift;

        if self.sweep.negated {
          self.timer.period += delta + 1;

          match self.channel {
            PulseChannel::One => self.timer.period += 1,
            PulseChannel::Two => { }
          }
        } else {
          self.timer.period += delta;
        }
      }
    }
  }

  fn sweep_silent(&self) -> bool {
    let next = self.timer.period + (self.timer.period >> self.sweep.shift);
    self.timer.period < 8 || (!self.sweep.negated && next > 0x7FF)
  }

  pub fn signal(&self) -> f32 {
    if Duty::TABLE[self.duty.cycle as usize][self.duty.value as usize] != 0
      && self.length.counter != 0
      && !self.sweep_silent() {
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
