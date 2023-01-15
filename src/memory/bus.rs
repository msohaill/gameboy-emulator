use super::rom::ROM;

pub struct Bus {
  vram: [u8; 2048],
  rom: ROM,
}

impl Bus {
  const RAM: u16 = 0x0000;
  const RAM_END: u16 = 0x1FFF;
  const PPU: u16 = 0x2000;
  const PPU_END: u16 = 0x3FFF;
  const ROM: u16 = 0x8000;
  const ROM_END: u16 = 0xFFFF;

  pub fn new(rom: ROM) -> Self {
    Bus {
      vram: [0; 2048],
      rom,
    }
  }

  pub fn read(&self, mut addr: u16) -> u8 {
    match addr {
      Bus::RAM ..= Bus::RAM_END => self.vram[(addr & ((1 << 11) - 1)) as usize],
      Bus::PPU ..= Bus::PPU_END => todo!("PPU Later"),
      Bus::ROM..=Bus::ROM_END => {
        addr -= Bus::ROM;
        if self.rom.prg.len() == 0x4000 && addr >= 0x4000 { addr = addr % 0x4000 }
        self.rom.prg[addr as usize]
      }
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
      Bus::ROM..=Bus::ROM_END => panic!("Attempting to write to ROM."),
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