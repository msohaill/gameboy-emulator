use std::{collections::HashSet, ops::RangeInclusive};

use super::{Mirroring, Mapper};

const PRG_BANK_SIZE: usize = 16384;

pub struct Mapper2 {
  mirroring: Mirroring,
  chr_rom: Vec<u8>,
  chr_ram: Vec<u8>,
  prg_rom: Vec<u8>,
  prg_ram: [u8; 0x2000],
  ranges: HashSet<RangeInclusive<u16>>,

  first: usize,
  last: usize,
}

impl Mapper for Mapper2 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn ranges(&self) -> &HashSet<RangeInclusive<u16>> {
    &self.ranges
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_read(addr),
      0x6000 ..= 0x7FFF => self.prg_ram_read(addr),
      0x8000 ..= 0xBFFF => self.first_read(addr),
      0xC000 ..= 0xFFFF => self.last_read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_write(addr, val),
      0x6000 ..= 0x7FFF => self.prg_ram_write(addr, val),
      0x8000 ..= 0xFFFF => self.update_first(val),
      _ => { },
    }
  }
}

impl Mapper2 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let last = (prg_rom.len() / PRG_BANK_SIZE) - 1;
    let chr = !chr_rom.is_empty();

    Mapper2 {
      mirroring,
      chr_rom,
      chr_ram: if chr { vec![] } else { vec![0; 0x2000] },
      prg_rom,
      prg_ram: [0; 0x2000],
      ranges: HashSet::new(),
      first: 0,
      last,
    }
  }

  fn chr_read(&self, addr: u16) -> u8 {
    if self.chr_rom.is_empty() {
      self.chr_ram[addr as usize % self.chr_ram.len()]
    } else {
      self.chr_rom[addr as usize % self.chr_rom.len()]

    }
  }

  fn chr_write(&mut self, addr: u16, val: u8) {
    if !self.chr_ram.is_empty() {
      let index = addr as usize % self.chr_ram.len();
      self.chr_ram[index] = val;
    }
  }

  fn prg_ram_read(&self, addr: u16) -> u8 {
    let index = addr as usize - 0x6000;
    self.prg_ram[index]
  }

  fn prg_ram_write(&mut self, addr: u16, val: u8) {
    let index = addr as usize - 0x6000;
    self.prg_ram[index] = val;
  }

  fn first_read(&self, addr: u16) -> u8 {
    let index = (self.first as usize * PRG_BANK_SIZE) + (addr as usize % PRG_BANK_SIZE);
    self.prg_rom[index]
  }

  fn last_read(&self, addr: u16) -> u8 {
    let index = (self.last as usize * PRG_BANK_SIZE) + (addr as usize % PRG_BANK_SIZE);
    self.prg_rom[index]
  }

  fn update_first(&mut self, val: u8) {
    self.first = val as usize & 0x1F;
  }
}
