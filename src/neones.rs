use crate::{
  cpu::CPU, renderer::Renderer, system::{cartridge::Cartridge, System}
};

pub struct NeoNES {
  cpu: CPU,
}

impl NeoNES {
  pub fn new(rom: Vec<u8>, renderer: Box<dyn Renderer>) -> Self {
    NeoNES {
      cpu: CPU::new(System::new(Cartridge::new(rom).unwrap(), renderer)),
    }
  }

  pub fn start(&mut self) {
    self.cpu.start()
  }
}
