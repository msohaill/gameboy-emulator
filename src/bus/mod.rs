pub mod cartridge;
pub mod memory;

use crate::ppu::PPU;
use cartridge::Cartridge;
use memory::Memory;

pub struct Bus<'call> {
  callback: Box<dyn FnMut(&PPU) + 'call>,
  cycles: usize,
  memory: Memory,
  prg: Vec<u8>,
  ppu: PPU,
}

impl<'a> Bus<'a> {
  pub const RAM: u16 = 0x0000;
  pub const RAM_END: u16 = 0x1FFF;
  pub const PPU: u16 = 0x2000;
  pub const PPU_END: u16 = 0x3FFF;
  pub const ROM: u16 = 0x8000;
  pub const ROM_END: u16 = 0xFFFF;

  pub fn new<'call, F>(cartridge: Cartridge, callback: F) -> Bus<'call>
  where
    F: FnMut(&PPU) + 'call,
  {
    Bus {
      callback: Box::from(callback),
      cycles: 0,
      memory: Memory::new(),
      prg: cartridge.prg,
      ppu: PPU::new(cartridge.chr, cartridge.mirroring),
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      Bus::RAM..=Bus::RAM_END => self.memory.read(addr),
      Bus::PPU..=Bus::PPU_END => self.ppu.read(addr),
      Bus::ROM..=Bus::ROM_END => self.read_prg(addr),
      _ => {
        println!("Ignoring read: {:#0X}", addr);
        0
      }
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      Bus::RAM..=Bus::RAM_END => self.memory.write(addr, data),
      Bus::PPU..=Bus::PPU_END => self.ppu.write(addr, data),
      0x4014 => self.oam(data),
      Bus::ROM..=Bus::ROM_END => panic!("Attempting to write to cartridge ROM."),
      _ => println!("Ignoring write: {:#0X}", addr),
    }
  }

  pub fn readu16(&mut self, addr: u16) -> u16 {
    let lo = self.read(addr) as u16;
    let hi = self.read(addr.wrapping_add(1)) as u16;
    (hi << 8) | lo
  }

  pub fn writeu16(&mut self, addr: u16, data: u16) {
    let lo = (data & 0xFF) as u8;
    let hi = (data >> 8) as u8;
    self.write(addr, lo);
    self.write(addr.wrapping_add(1), hi);
  }

  pub fn tick(&mut self, cycles: u8) {
    self.cycles += cycles as usize;
    let new_frame = self.ppu.tick(3 * cycles);

    if new_frame {
      (self.callback)(&self.ppu)
    }
  }

  fn oam(&mut self, data: u8) {
    let mut buffer: [u8; 0x100] = [0; 0x100];

    let hi: u16 = (data as u16) << 8;
    for i in 0..0x100 {
      buffer[i as usize] = self.read(hi + i);
    }

    self.ppu.write_oam_dma(&buffer);
  }

  fn read_prg(&self, addr: u16) -> u8 {
    let mut index = addr - Bus::ROM;
    if self.prg.len() == 0x4000 && index >= 0x4000 {
      index = index % 0x4000
    }
    self.prg[index as usize]
  }

  pub fn poll_nmi(&self) -> bool {
    self.ppu.nmi_interrupt
  }

  pub fn clear_nmi(&mut self) {
    self.ppu.nmi_interrupt = false;
  }
}
