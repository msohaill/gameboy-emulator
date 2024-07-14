pub mod mapper0;

use std::{collections::HashSet, ops::RangeInclusive};

use mapper0::Mapper0;


#[derive(Clone, Copy)]
pub enum Mirroring {
  Vertical, Horizontal, FourScreen, Single0, Single1,
}

pub fn from(mapper: u8, chr: Vec<u8>, prg_rom: Vec<u8>, mirrorring: Mirroring) -> Box<dyn Mapper> {
  match mapper {
    0 => Box::from(Mapper0::new(chr, prg_rom, mirrorring)),
    _ => panic!("Unsupported mapper: {}", mapper),
  }
}

pub trait Mapper {
  fn read(&self, addr: u16) -> u8;

  fn write(&mut self, addr: u16, val: u8);

  fn mirroring(&self) -> Mirroring;

  fn ranges(&self) -> &HashSet<RangeInclusive<u16>>;
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
