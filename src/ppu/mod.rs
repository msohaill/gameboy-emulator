pub mod register;

use crate::memory::{cartridge::Mirroring, bus::Bus};
use register::Registers;

use register::status::Flag as StatusFlag;

pub struct PPU {
  pub chr: Vec<u8>,
  pub palette: [u8; 0x20],
  pub vram: [u8; 0x800],
  pub oam: [u8; 0x100],
  pub mirroring: Mirroring,
  pub registers: Registers,
  data_buffer: u8,
}

impl PPU {
  pub fn new(chr: Vec<u8>, mirroring: Mirroring) -> Self {
    PPU {
      chr,
      mirroring,
      palette: [0; 0x20],
      vram: [0; 0x800],
      oam: [0; 0x100],
      registers: Registers::new(),
      data_buffer: 0,
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 =>
        panic!("Illegal access at write-only PPU register: {:#0X}", addr),
      0x2002 => self.read_status(),
      0x2004 => self.read_oam_data(),
      0x2007 => self.read_data(),
      0x2008..=Bus::PPU_END => self.read(addr & 0x2007),
      _ => panic!("Illegal PPU read access: {:#0X}", addr),
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      0x2000 => self.registers.controller.set(data),
      0x2001 => self.registers.mask.set(data),
      0x2002 => panic!("Illegal write to PPU status register."),
      0x2003 => self.registers.write_oam_addr(data),
      0x2004 => self.write_oam_data(data),
      0x2005 => self.registers.scroll.write(data),
      0x2006 => self.registers.address.write(data),
      0x2007 => self.write_data(data),
      0x2008..=Bus::PPU_END => self.write(addr & 0x2007, data),
      _ => panic!("Illegal PPU write access: {:#0X}", addr),
    }
  }

  fn read_data(&mut self) -> u8 {
    let addr = self.registers.address.read();
    self.registers.address.increment(self.registers.controller.vram_increment());

    match addr {
      0x0     ..=   0x1FFF => self.chr_read(addr),
      0x2000  ..=   0x2FFF => self.vram_read(addr),
      0x3000  ..=   0x3EFF => panic!("Address space 0x3000..0x3EFF is not expected to be used. Requested = {:#0X} ", addr),
      0x3F00  ..=   0x3FFF => self.palette_read(addr),
      _                    => panic!("Unexpected access to mirrored space {}", addr),
    }
  }

  fn write_data(&mut self, data: u8) {
    let addr = self.registers.address.read();
    self.registers.address.increment(self.registers.controller.vram_increment());

    match addr {
      0x0     ..=   0x1FFF => panic!("Illegal write to CHR ROM: {:#0X}", addr),
      0x2000  ..=   0x2FFF => self.vram_write(addr, data),
      0x3000  ..=   0x3EFF => panic!("Address space 0x3000..0x3EFF is not expected to be used. Requested = {:#0X} ", addr),
      0x3F00  ..=   0x3FFF => self.palette_write(addr, data),
      _                    => panic!("Unexpected access to mirrored space {}", addr),
    };
  }

  fn chr_read(&mut self, addr: u16) -> u8 {
    let res = self.data_buffer;
    self.data_buffer = self.chr[addr as usize];
    res
  }

  fn vram_read(&mut self, addr: u16) -> u8 {
    let res = self.data_buffer;
    self.data_buffer = self.vram[self.mirror_vram(addr) as usize];
    res
  }

  fn vram_write(&mut self, addr: u16, data: u8) {
    self.vram[self.mirror_vram(addr) as usize] = data;
  }

  fn palette_read(&self, addr: u16) -> u8 {
    match addr {
      0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => self.palette[(addr - 0x10 - 0x3F00) as usize],
      0x3F00    ..=   0x3FFF            => self.palette[(addr - 0x3F00) as usize],
      _                                 => panic!("Illegal pallette table write: {:#0X}", addr),
    }
  }

  fn palette_write(&mut self, addr: u16, data: u8) {
    match addr {
      0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => self.palette[(addr - 0x10 - 0x3F00) as usize] = data,
      0x3F00    ..=   0x3FFF            => self.palette[(addr - 0x3F00) as usize] = data,
      _                                 => panic!("Illegal pallette table access: {:#0X}", addr),
    }
  }

  fn mirror_vram(&self, addr: u16) -> u16 {
    let index = (addr & 0x2FFF) - 0x2000;
    let table = index / 0x400;

    match (self.mirroring, table) {
      (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => index - 0x800,
      (Mirroring::Horizontal, 1) => index - 0x400,
      (Mirroring::Horizontal, 2) => index - 0x400,
      (Mirroring::Horizontal, 3) => index - 0x800,
      _ => index,
    }
  }

  fn read_oam_data(&self) -> u8 {
    self.oam[self.registers.oam_address as usize]
  }

  fn write_oam_data(&mut self, data: u8) {
    self.oam[self.registers.oam_address as usize] = data;
    self.registers.write_oam_addr(self.registers.oam_address.wrapping_add(1));
  }

  fn read_status(&mut self) -> u8 {
    let res = self.registers.status.get();

    self.registers.status.set_flag(StatusFlag::VBLankStarted, false);
    self.registers.address.reset();
    self.registers.scroll.reset();

    res
  }
}
