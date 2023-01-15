pub struct Bus {
  vram: [u8; 2048],
}

impl Bus {
  const RAM: u16 = 0x0000;
  const RAM_END: u16 = 0x1FFF;
  const PPU: u16 = 0x2000;
  const PPU_END: u16 = 0x3FFF;


  pub fn new() -> Self {
    Bus {
      vram: [0; 2048],
    }
  }

  pub fn read(&self, addr: u16) -> u8 {
    match addr {
      Bus::RAM ..= Bus::RAM_END => self.vram[(addr & ((1 << 11) - 1)) as usize],
      Bus::PPU ..= Bus::PPU_END => todo!("PPU Later"),
      _ => {
        println!("Ignoring read: {:#0X}", addr);
        0
      },
    }
  }

  pub fn readu16(&self, addr: u16) -> u16 {
    let lo = self.read(addr) as u16;
    let hi = self.read(addr.wrapping_add(1)) as u16;
    (hi << 8) | lo
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      Bus::RAM ..= Bus::RAM_END => self.vram[(addr & ((1 << 11) - 1)) as usize] = data,
      Bus::PPU ..= Bus::PPU_END => todo!("PPU Later"),
      _ => {
        println!("Ignoring write: {:#0X}", addr);
        ()
      },
    }
  }

  pub fn writeu16(&mut self, addr: u16, data: u16) {
    let lo = (data & 0xFF) as u8;
    let hi = (data >> 8) as u8;
    self.write(addr, lo);
    self.write(addr.wrapping_add(1), hi);
  }
}