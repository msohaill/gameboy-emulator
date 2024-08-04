mod dmc;
mod envelope;
mod lengthcounter;
mod noise;
mod pulse;
mod triangle;

use dmc::DMC;
use noise::Noise;
use pulse::Pulse;
use triangle::Triangle;

pub struct APU {
  pulse_one: Pulse,
  pulse_two: Pulse,
  triangle: Triangle,
  noise: Noise,
  dmc: DMC,

  mode: SequencerMode,
  irq: IRQ,
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
  pub fn new() -> Self {
    APU {
      pulse_one: Pulse::new(),
      pulse_two: Pulse::new(),
      triangle: Triangle::new(),
      noise: Noise::new(),
      dmc: DMC::new(),

      mode: SequencerMode::StepFour,
      irq: IRQ::new(),
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

  }

  pub fn poll(&mut self) -> bool {
    self.irq.pending || self.dmc.irq.pending
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
    self.pulse_one.set_enabled(val & 0x02 != 0x0);
    self.triangle.set_enabled(val & 0x04 != 0x0);
    self.noise.set_enabled(val & 0x08 != 0x0);
    self.dmc.set_enabled(val & 0x10 != 0x0);
  }

  fn write_frame_counter(&mut self, val: u8) {
    self.mode = if val & 0x80 == 0x0 {
      SequencerMode::StepFour
    } else {
      SequencerMode::StepFive
    };

    self.irq.set_enabled((val & 0x40) == 0x0);

    if self.mode == SequencerMode::StepFive {
      // step ...
    }
  }
}
