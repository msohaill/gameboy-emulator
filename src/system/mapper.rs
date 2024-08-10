mod mapper0;
mod mapper1;
mod mapper2;
mod mapper3;
mod mapper4;

use mapper0::Mapper0;
use mapper1::Mapper1;
use mapper2::Mapper2;
use mapper3::Mapper3;
use mapper4::Mapper4;


#[derive(Clone, Copy)]
pub enum Mirroring {
  Vertical, Horizontal, FourScreen, Single0, Single1,
}

pub enum MapperEvent {
  HBlank, VRAMAddressChanged(u16),
}

pub fn from(mapper: u8, chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Box<dyn Mapper> {
  match mapper {
    0 => Box::from(Mapper0::new(chr_rom, prg_rom, mirroring)),
    1 => Box::from(Mapper1::new(chr_rom, prg_rom, mirroring)),
    2 => Box::from(Mapper2::new(chr_rom, prg_rom, mirroring)),
    3 => Box::from(Mapper3::new(chr_rom, prg_rom, mirroring)),
    4 => Box::from(Mapper4::new(chr_rom, prg_rom, mirroring)),
    _ => panic!("Unsupported mapper: {}", mapper),
  }
}

pub trait Mapper {
  fn read(&self, addr: u16) -> u8;

  fn write(&mut self, addr: u16, val: u8);

  fn mirroring(&self) -> Mirroring;

  fn notify(&mut self, _: MapperEvent) { }

  fn poll(&self) -> bool { false }
}

impl Mirroring {
  pub fn coeff(&self) -> [u16; 4] {
    match self {
      Mirroring::Vertical   => [0, 1, 0, 1],
      Mirroring::Horizontal => [0, 0, 1, 1],
      Mirroring::FourScreen => [0, 1, 2, 3],
      Mirroring::Single0    => [0, 0, 0, 0],
      Mirroring::Single1    => [1, 1, 1, 1],
    }
  }
}
