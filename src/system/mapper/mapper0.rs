use super::{Mirroring, Mapper};


pub struct Mapper0 {
  mirroring: Mirroring,
  chr_rom: Vec<u8>,
  chr_ram: Vec<u8>,
  prg_rom: Vec<u8>,
  prg_ram: [u8; 0x2000],
}

impl Mapper for Mapper0 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
        0x0000 ..= 0x1FFF => self.chr_read(addr),
        0x6000 ..= 0x7FFF => self.prg_ram_read(addr),
        0x8000 ..= 0xFFFF => self.prg_rom_read(addr),
        _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
        0x0000 ..= 0x1FFF => self.chr_write(addr, val),
        0x6000 ..= 0x7FFF => self.prg_ram_write(addr, val),
        _ => { },
    }
  }
}

impl Mapper0 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    Mapper0 {
      mirroring,
      chr_rom,
      chr_ram: if chr { vec![] } else { vec![0; 0x2000] },
      prg_rom,
      prg_ram: [0; 0x2000],
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

  fn prg_rom_read(&self, addr: u16) -> u8 {
    let mut index = addr - 0x8000;
    index %= self.prg_rom.len() as u16;
    self.prg_rom[index as usize]
  }
}
