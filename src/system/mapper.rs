use std::{collections::HashSet, ops::RangeInclusive};

use super::{cartridge::Mirroring, System};


pub struct Mapper {
  pub mirroring: Mirroring,
  pub chr: Vec<u8>,
  prg_rom: Vec<u8>,
  prg_ram: [u8; 0x2000],
  ranges: HashSet<RangeInclusive<u16>>
}

impl Mapper {
  pub fn new(chr: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    Mapper {
      mirroring,
      chr,
      prg_rom,
      prg_ram: [0; 0x2000],
      ranges: HashSet::new(),
    }
  }

  pub fn ranges(&self) -> &HashSet<RangeInclusive<u16>> {
    &self.ranges
  }

  pub fn read(&self, addr: u16) -> u8 {
    match addr {
        0x0000 ..= 0x1FFF => self.chr_read(addr),
        0x6000 ..= 0x7FFF => self.prg_ram_read(addr),
        System::ROM ..= System::ROM_END => self.prg_rom_read(addr),
        _ => 0,
    }
  }

  pub fn write(&mut self, addr: u16, val: u8) {
    match addr {
        0x0000 ..= 0x1FFF => self.chr_write(addr, val),
        0x6000 ..= 0x7FFF => self.prg_ram_write(addr, val),
        _ => { },
    }
  }

  fn chr_read(&self, addr: u16) -> u8 {
    let index = addr as usize % self.chr.len();
    self.chr[index]
  }

  fn chr_write(&mut self, addr: u16, val: u8) {
    let index = addr as usize % self.chr.len();
    self.chr[index] = val;
  }

  fn prg_ram_read(&self, addr: u16) -> u8 {
    let index = addr as usize - 0x6000;
    self.prg_ram[index]
  }

  fn prg_ram_write(&mut self, addr: u16, val: u8) {
    let index = addr as usize - 0x6000;
    self.prg_ram[index] = val
  }

  fn prg_rom_read(&self, addr: u16) -> u8 {
    let mut index = addr - System::ROM;
    index %= self.prg_rom.len() as u16;
    self.prg_rom[index as usize]
  }
}
