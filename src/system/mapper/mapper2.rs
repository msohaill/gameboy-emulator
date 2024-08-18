use super::{banks::Banks, Mapper, Mirroring};

const PRG_ROM_BANK_SIZE: usize = 0x4000;

pub struct Mapper2 {
  mirroring: Mirroring,

  chr: Banks,
  prg_ram: Banks,
  prg_rom: Banks,
}

impl Mapper for Mapper2 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x1FFF => self.chr.read(addr),
      0x6000 ..= 0x7FFF => self.prg_ram.read(addr),
      0x8000 ..= 0xFFFF => self.prg_rom.read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr.write(addr, val),
      0x6000 ..= 0x7FFF => self.prg_ram.write(addr, val),
      0x8000 ..= 0xFFFF => self.prg_rom.set(0, val as usize & 0x1F),
      _ => { },
    }
  }
}

impl Mapper2 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    let mut mapper = Mapper2 {
      mirroring,
      chr: Banks::new(0x0000, 0x1FFF, 0x2000, if chr { chr_rom } else { vec![0; 0x2000] }, !chr),
      prg_ram: Banks::new(0x6000, 0x7FFF, 0x2000, vec![0; 0x2000], true),
      prg_rom: Banks::new(0x8000, 0xFFFF, PRG_ROM_BANK_SIZE, prg_rom, false),
    };

    mapper.prg_rom.set(1, mapper.prg_rom.last());
    mapper
  }
}
