mod dmc;
mod envelope;
mod filter;
mod lengthcounter;
pub(crate) mod mixer;
mod noise;
mod pulse;
mod timer;
mod triangle;

use dmc::DMC;
use mixer::Mixer;
use noise::Noise;
use pulse::{Pulse, PulseChannel};
use triangle::Triangle;

use crate::cpu::CPU;

pub struct APU {
  pulse_one: Pulse,
  pulse_two: Pulse,
  triangle: Triangle,
  noise: Noise,
  dmc: DMC,

  mode: SequencerMode,
  step: u8,
  irq: IRQ,
  cycles: usize,

  samples: Vec<f32>,
  pub mixer: Mixer,
}

#[derive(PartialEq, Eq)]
enum SequencerMode {
  StepFour,
  StepFive
}

pub struct IRQ {
  enabled: bool,
  pending: bool,
}

impl SequencerMode {
  fn steps(&self) -> u8 {
    match *self {
      SequencerMode::StepFour => 4,
      SequencerMode::StepFive => 5,
    }
  }
}

impl IRQ {
  fn new() -> Self {
    IRQ {
      enabled: false,
      pending: false,
    }
  }

  fn set_enabled(&mut self, enabled: bool) {
    self.enabled = enabled;

    if !enabled {
      self.pending = false;
    }
  }
}

impl APU {
  const SEQUENCER_RATE: f32 = CPU::CLOCK_RATE / 240.0;

  pub fn new() -> Self {
    APU {
      pulse_one: Pulse::new(PulseChannel::One),
      pulse_two: Pulse::new(PulseChannel::Two),
      triangle: Triangle::new(),
      noise: Noise::new(),
      dmc: DMC::new(),

      mode: SequencerMode::StepFour,
      step: 0x0,
      irq: IRQ::new(),
      cycles: 0,

      samples: vec![],
      mixer: Mixer::new(),
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      0x4015 => self.read_status(),
      _ => unreachable!("Should not happen!"),
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      // Pulse 1
      0x4000 => self.pulse_one.write_control(data),
      0x4001 => self.pulse_one.write_sweep(data),
      0x4002 => self.pulse_one.write_timer_lo(data),
      0x4003 => self.pulse_one.write_timer_hi(data),

      // Pulse 2
      0x4004 => self.pulse_two.write_control(data),
      0x4005 => self.pulse_two.write_sweep(data),
      0x4006 => self.pulse_two.write_timer_lo(data),
      0x4007 => self.pulse_two.write_timer_hi(data),

      // Triangle
      0x4008 => self.triangle.write_counter(data),
      0x4009 => { } // Unused
      0x400A => self.triangle.write_timer_lo(data),
      0x400B => self.triangle.write_timer_hi(data),

      // Noise
      0x400C => self.noise.write_control(data),
      0x400D => { } // Unused
      0x400E => self.noise.write_timer(data),
      0x400F => self.noise.write_length(data),

      // DMC
      0x4010 => self.dmc.write_control(data),
      0x4011 => self.dmc.write_output(data),
      0x4012 => self.dmc.write_address(data),
      0x4013 => self.dmc.write_length(data),

      // Status
      0x4015 => self.write_status(data),

      // Frame Counter
      0x4017 => self.write_frame_counter(data),
      _ => unreachable!("Should not happen!"),
    }
  }

  pub fn tick(&mut self) {
    let prev = self.cycles as f32;
    self.cycles = self.cycles.wrapping_add(1);
    let post = self.cycles as f32;

    self.dmc.tick_dma();
    self.timers();

    if (prev / APU::SEQUENCER_RATE) as u32 != (post / APU::SEQUENCER_RATE) as u32 {
      self.frame();
    }

    self.signal();
  }

  pub fn poll(&mut self) -> bool {
    self.irq.pending || self.dmc.irq.pending
  }

  pub fn dma(&mut self) -> bool {
    self.dmc.dma()
  }

  pub fn dma_addr(&self) -> u16 {
    self.dmc.dma_addr()
  }

  pub fn dmcdma(&mut self, val: u8) {
    self.dmc.load(val);
  }

  fn timers(&mut self) {
    self.triangle.timer();

    if self.cycles % 2 == 0 {
      self.pulse_one.timer();
      self.pulse_two.timer();
      self.noise.timer();
      self.dmc.timer();
    }
  }

  fn quarter(&mut self) {
    self.pulse_one.quarter();
    self.pulse_two.quarter();
    self.triangle.quarter();
    self.noise.quarter();
  }

  fn half(&mut self) {
    self.pulse_one.half();
    self.pulse_two.half();
    self.triangle.half();
    self.noise.half();
  }

  fn frame(&mut self) {
    self.step = (self.step + 1) % self.mode.steps();

    match self.step {
      0 | 2 => self.quarter(),
      1 | 3 => {
        self.quarter();
        self.half();
      }
      _ => { }
    }

    if self.step == 3 && self.mode == SequencerMode::StepFour && self.irq.enabled {
      self.irq.pending = true;
    }
  }

  fn signal(&mut self) {
    let p1 = self.pulse_one.signal();
    let p2 = self.pulse_two.signal();
    let t = self.triangle.signal();
    let n = self.noise.signal();
    let d = self.dmc.signal();

    let pulse = (95.88) / ((8128.0 / (p1 + p2)) + 100.0);
    let tnd = (159.79) / ((1.0 / ((t / 8227.0) + (n / 12241.0) + (d / 22638.0))) + 100.0);

    self.samples.push(pulse + tnd);
  }

  pub fn mix(&mut self) {
    self.mixer.consume(&self.samples);
    self.samples.clear();
  }

  fn read_status(&mut self) -> u8 {
    let mut status = 0x0;

    if self.pulse_one.length_counter() > 0 {
      status |= 0x01;
    }

    if self.pulse_two.length_counter() > 0 {
      status |= 0x02;
    }

    if self.triangle.length_counter() > 0 {
      status |= 0x04;
    }

    if self.noise.length_counter() > 0 {
      status |= 0x08;
    }

    if self.dmc.length() > 0 {
      status |= 0x10;
    }

    if self.irq.pending {
      status |= 0x40;
    }

    if self.dmc.irq.pending {
      status |= 0x80;
    }

    self.irq.pending = false;

    return status;
  }

  fn write_status(&mut self, val: u8) {
    self.pulse_one.set_enabled(val & 0x01 != 0x0);
    self.pulse_two.set_enabled(val & 0x02 != 0x0);
    self.triangle.set_enabled(val & 0x04 != 0x0);
    self.noise.set_enabled(val & 0x08 != 0x0);
    self.dmc.set_enabled(val & 0x10 != 0x0, self.cycles);
  }

  fn write_frame_counter(&mut self, val: u8) {
    self.mode = if val & 0x80 == 0x0 {
      SequencerMode::StepFour
    } else {
      SequencerMode::StepFive
    };

    self.irq.set_enabled((val & 0x40) == 0x0);

    if self.mode == SequencerMode::StepFive {
      self.quarter();
      self.half();
    }
  }
}
