use super::{Mirroring, Mapper};

const CHR_BANK_SIZE: usize = 0x2000;

pub struct Mapper3 {
  mirroring: Mirroring,
  chr_rom: Vec<u8>,
  chr_ram: Vec<u8>,
  prg_rom: Vec<u8>,
  bank: u8,
}

impl Mapper for Mapper3 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_read(addr),
      0x8000 ..= 0xFFFF => self.prg_rom_read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_write(addr, val),
      0x8000 ..= 0xFFFF => self.bank_select(val),
      _ => { },
    }
  }
}

impl Mapper3 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    Mapper3 {
      mirroring,
      chr_rom,
      chr_ram: if chr { vec![] } else { vec![0; 0x2000] },
      prg_rom,
      bank: 0,
    }
  }

  fn chr_read(&self, addr: u16) -> u8 {
    let index = (self.bank as usize * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);

    if self.chr_rom.is_empty() {
      self.chr_ram[index]
    } else {
      self.chr_rom[index]

    }
  }

  fn chr_write(&mut self, addr: u16, val: u8) {
    if !self.chr_ram.is_empty() {
      let index = (self.bank as usize * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);
      self.chr_ram[index] = val;
    }
  }

  fn prg_rom_read(&self, addr: u16) -> u8 {
    let index = addr as usize - 0x8000;
    self.prg_rom[index]
  }

  fn bank_select(&mut self, bank: u8) {
    self.bank = bank & 0x03;
  }
}
