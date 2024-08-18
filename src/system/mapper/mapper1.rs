use super::{banks::Banks, Mapper, Mirroring};

const PRG_RAM_BANK_SIZE: usize = 0x2000;
const PRG_ROM_BANK_SIZE: usize = 0x4000;
const CHR_BANK_SIZE: usize = 0x1000;

pub struct Mapper1 {
  mirroring: Mirroring,

  chr: Banks,
  prg_ram: Banks,
  prg_rom: Banks,

  // Registers
  control: u8,
  chr0: u8,
  chr1: u8,
  prg: u8,
  shift: u8,
}

impl Mapper for Mapper1 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
        0x0000 ..= 0x1FFF => {
          let val = self.chr.read(addr);
          // println!("Reading {val:#0x} from {addr:#0x} (actually {:#0x})", self.chr.translate(addr));
          val
        }
        0x6000 ..= 0x7FFF if self.prg & 0x10 == 0x0 => self.prg_ram.read(addr),
        0x8000 ..= 0xFFFF => self.prg_rom.read(addr),
        _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
        0x0000 ..= 0x1FFF => {
          // println!("Writing {val:#0x} to {addr:#0x} (actually {:#0x})", self.chr.translate(addr));
          self.chr.write(addr, val)
        }
        0x6000 ..= 0x7FFF if self.prg & 0x10 == 0x0 => self.prg_ram.write(addr, val),
        0x8000 ..= 0xFFFF => self.load(addr, val),
        _ => { },
    }
  }
}

impl Mapper1 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    let mut mapper = Mapper1 {
      mirroring,

      chr: Banks::new(0x0000, 0x1FFF, CHR_BANK_SIZE, if chr { chr_rom } else { vec![0; 0x2000] }, !chr),
      prg_ram: Banks::new(0x6000, 0x7FFF, PRG_RAM_BANK_SIZE, vec![0; 0x8000], true),
      prg_rom: Banks::new(0x8000, 0xFFFF, PRG_ROM_BANK_SIZE, prg_rom, false),

      control: 0xC,
      chr0: 0,
      chr1: 0,
      prg: 0,
      shift: 0x10,
    };

    mapper.update_banks(0x0000);
    mapper
  }

  fn update_banks(&mut self, addr: u16) {
    self.mirroring = match self.control & 0x03 {
      0 => Mirroring::Single0,
      1 => Mirroring::Single1,
      2 => Mirroring::Vertical,
      3 => Mirroring::Horizontal,
      mode => unreachable!("Impossible mirroring mode: {mode}"),
    };

    match self.chr_mode() == 0x00 {
      true => self.chr.set_range(0, 1, (self.chr0 & 0x1E) as usize),
      false => {
        self.chr.set(0, self.chr0 as usize);
        self.chr.set(1, self.chr1 as usize);
      }
    }

    let extra = match addr {
      0xC000 ..= 0xDFFF if self.chr_mode() != 0x0 => self.chr1,
      _ => self.chr0,
    } as usize;

    let selector = if self.prg_rom.capacity() == 0x80000 { extra & 0x10 } else { 0x0 };

    let prg = self.prg as usize & 0xF;

    match self.prg_mode() {
      0 | 1 => self.prg_rom.set_range(0, 1, selector | (prg & 0xFE)),
      2 => {
        self.prg_rom.set(0, selector);
        self.prg_rom.set(1, selector | prg);
      }
      3 => {
        self.prg_rom.set(0, selector | prg);
        self.prg_rom.set(1, selector | self.prg_rom.last());
      }
      mode => unreachable!("Impossible prg_mode: {mode}"),
    }
  }

  fn prg_mode(&self) -> u8 {
    (self.control >> 2) & 0x03
  }

  fn chr_mode(&self) -> u8 {
    (self.control >> 4) & 0x01
  }

  fn load(&mut self, addr: u16, val: u8) {
    if (val >> 7) == 0x01 {
      self.shift = 0x10;
      self.control |= 0xC;
    } else {
      let write = self.shift & 0x01 == 0x01;

      self.shift >>= 1;
      self.shift |= (val & 0x01) << 4;

      if write {
        match addr {
          0x8000 ..= 0x9FFF => self.control = self.shift,
          0xA000 ..= 0xBFFF => self.chr0 = self.shift & 0x1F,
          0xC000 ..= 0xDFFF => self.chr1 = self.shift & 0x1F,
          0xE000 ..= 0xFFFF => self.prg = self.shift & 0x1F,
          _ => { } // Do nothing
        }

        self.shift = 0x10;
        self.update_banks(addr);
      }
    }
  }
}
