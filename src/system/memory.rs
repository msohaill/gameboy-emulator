pub struct Memory {
  vram: [u8; 0x800],
}

impl Memory {
  pub fn new() -> Self {
    Memory {
      vram: [0; 0x800],
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    self.vram[(addr & 0x7FF) as usize]
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.vram[(addr & 0x7FF) as usize] = data
  }
}
