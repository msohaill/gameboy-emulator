use std::{cell::RefCell, rc::Rc};

use crate::{
  apu::mixer::NESAudioCallback, cpu::CPU, renderer::Renderer, system::{cartridge::Cartridge, joypad::Flag as JoypadButton, System}
};

pub struct NeoNES {
  cpu: CPU,
}

impl NeoNES {
  pub fn new(rom: Vec<u8>, renderer: Rc<RefCell<dyn Renderer>>) -> Self {
    NeoNES {
      cpu: CPU::new(System::new(Cartridge::new(rom).unwrap(), renderer)),
    }
  }

  pub fn start(&mut self) {
    self.cpu.start()
  }

  pub fn step_frame(&mut self) {
    self.cpu.step_frame();
  }

  pub fn audio(&mut self) -> NESAudioCallback {
    self.cpu.system.callback()
  }

  pub fn push(&mut self, button: JoypadButton) {
    self.cpu.system.joypads.0.push(button);
  }

  pub fn release(&mut self, button: JoypadButton) {
    self.cpu.system.joypads.0.release(button);
  }
}
