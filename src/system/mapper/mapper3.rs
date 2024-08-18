use super::{banks::Banks, Mapper, Mirroring};

const CHR_BANK_SIZE: usize = 0x2000;

pub struct Mapper3 {
  mirroring: Mirroring,

  chr: Banks,
  prg_rom: Banks,
}

impl Mapper for Mapper3 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x1FFF => self.chr.read(addr),
      0x8000 ..= 0xFFFF => self.prg_rom.read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr.write(addr, val),
      0x8000 ..= 0xFFFF => self.chr.set(0, val as usize),
      _ => { },
    }
  }
}

impl Mapper3 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    Mapper3 {
      mirroring,
      chr: Banks::new(0x0000, 0x1FFF, CHR_BANK_SIZE, if chr { chr_rom } else { vec![0; 0x2000] }, !chr),
      prg_rom: Banks::new(0x8000, 0xFFFF, 0x4000, prg_rom, false),
    }
  }
}
