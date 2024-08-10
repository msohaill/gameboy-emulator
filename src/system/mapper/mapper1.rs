use super::{Mirroring, Mapper};

const PRG_BANK_SIZE: usize = 0x4000;
const CHR_BANK_SIZE: usize = 0x1000;

pub struct Mapper1 {
  banks: u8,

  mirroring: Mirroring,
  chr_rom: Vec<u8>,
  chr_ram: Vec<u8>,
  prg_rom: Vec<u8>,
  prg_ram: [u8; 0x2000],

  // Registers
  control: u8,
  chr0: u8,
  chr1: u8,
  prg: u8,
  shift: u8,

  writes: u8,
}

impl Mapper for Mapper1 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
        0x0000 ..= 0x0FFF => self.chr0_read(addr),
        0x1000 ..= 0x1FFF => self.chr1_read(addr),
        0x6000 ..= 0x7FFF => self.prg_ram_read(addr),
        0x8000 ..= 0xBFFF => self.read_first(addr),
        0xC000 ..= 0xFFFF => self.read_last(addr),
        _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
        0x0000 ..= 0x0FFF => self.chr0_write(addr, val),
        0x1000 ..= 0x1FFF => self.chr1_write(addr, val),
        0x6000 ..= 0x7FFF => self.prg_ram_write(addr, val),
        0x8000 ..= 0xFFFF => self.load(addr, val),
        _ => { },
    }
  }
}

impl Mapper1 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    Mapper1 {
      banks: (prg_rom.len() / PRG_BANK_SIZE) as u8,
      mirroring,
      chr_rom,
      chr_ram: if chr { vec![] } else { vec![0; 0x2000] },
      prg_rom,
      prg_ram: [0; 0x2000],
      control: 0b11 << 2,
      chr0: 0,
      chr1: 0,
      prg: 0,
      shift: 0,
      writes: 0,
    }
  }

  fn prg_mode(&self) -> u8 {
    (self.control >> 2) & 0x03
  }

  fn chr_mode(&self) -> u8 {
    (self.control >> 4) & 0x01
  }

  fn chr0_read(&self, addr: u16) -> u8 {
    let bank = match self.chr_mode() == 0 {
      true => self.chr0 & 0x1E,
      false => self.chr0,
    } as usize;
    let index = (bank * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);

    let chr = if !self.chr_rom.is_empty() { &self.chr_rom } else { &self.chr_ram };
    chr[index]
  }

  fn chr1_read(&self, addr: u16) -> u8 {
    let bank = match self.chr_mode() == 0 {
      true => (self.chr0 & 0x1E) + 1,
      false => self.chr1,
    } as usize;
    let index = (bank * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);

    let chr = if !self.chr_rom.is_empty() { &self.chr_rom } else { &self.chr_ram };
    chr[index]
  }

  fn chr0_write(&mut self, addr: u16, val: u8) {
    if self.chr_ram.is_empty() {
      return;
    }

    let bank = match self.chr_mode() == 0 {
      true => self.chr0 & 0x1E,
      false => self.chr0,
    } as usize;
    let index = (bank * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);
    self.chr_ram[index] = val;
  }

  fn chr1_write(&mut self, addr: u16, val: u8) {
    if self.chr_ram.is_empty() {
      return;
    }

    let bank = match self.chr_mode() == 0 {
      true => (self.chr0 & 0x1E) + 1,
      false => self.chr1,
    } as usize;
    let index = (bank * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);
    self.chr_ram[index] = val;
  }

  fn prg_ram_read(&self, addr: u16) -> u8 {
    let index = addr as usize - 0x6000;
    self.prg_ram[index]
  }

  fn prg_ram_write(&mut self, addr: u16, val: u8) {
    let index = addr as usize - 0x6000;
    self.prg_ram[index] = val;
  }

  fn read_first(&self, addr: u16) -> u8 {
    let bank = match self.prg_mode() {
      0 | 1 => self.prg & 0xFE,
      2 => 0,
      3 => self.prg,
      _ => unreachable!("Impossible prg_mode: {}", self.prg_mode()),
    } as usize;
    let index = (bank * PRG_BANK_SIZE) + (addr as usize % PRG_BANK_SIZE);
    self.prg_rom[index]
  }

  fn read_last(&self, addr: u16) -> u8 {
    let bank = match self.prg_mode() {
      0 | 1 => (self.prg & 0xFE) + 1,
      2 => self.prg,
      3 => self.banks - 1,
      _ => unreachable!("Impossible prg_mode: {}", self.prg_mode()),
    } as usize;
    let index = (bank * PRG_BANK_SIZE) + (addr as usize % PRG_BANK_SIZE);
    self.prg_rom[index]
  }

  fn load(&mut self, addr: u16, val: u8) {
    if (val >> 7) == 0x01 {
      self.shift = 0x0;
      self.control = 0b11 << 2;
      self.writes = 0;
    } else {
      self.shift |= (val & 0x01) << self.writes;
      self.writes += 1;

      if self.writes == 5 {
        self.writes = 0;

        match addr {
          0x8000 ..= 0x9FFF => {
            self.control = self.shift;
            match self.control & 0x03 {
              0 => self.mirroring = Mirroring::Single0,
              1 => self.mirroring = Mirroring::Single1,
              2 => self.mirroring = Mirroring::Vertical,
              3 => self.mirroring = Mirroring::Horizontal,
              _ => { }
            }
          }
          0xA000 ..= 0xBFFF => self.chr0 = self.shift,
          0xC000 ..= 0xDFFF => self.chr1 = self.shift,
          0xE000 ..= 0xFFFF => self.prg = self.shift,
          _ => { } // Do nothing
        }

        self.shift = 0x0;
      }
    }
  }
}
