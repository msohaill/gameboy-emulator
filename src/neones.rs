use crate::{
  cpu::CPU,
  system::{cartridge::Cartridge, System},
};

pub struct NeoNES {
  cpu: CPU,
}

impl NeoNES {
  pub fn new(path: String) -> Self {
    NeoNES {
      cpu: CPU::new(System::new(Cartridge::new(path).unwrap())),
    }
  }

  pub fn start(&mut self) {
    self.cpu.start()
  }
}
