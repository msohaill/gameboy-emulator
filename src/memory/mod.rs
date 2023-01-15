pub mod bus;
pub mod rom;

use bus::Bus;
use rom::ROM;

pub struct Memory {
  bus: Bus,
}

impl Memory {
  pub fn new(cartridge: ROM) -> Self {
    Memory {
      bus: Bus::new(cartridge),
    }
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.bus.read(addr)
  }

  pub fn readu16(&self, addr: u16) -> u16 {
    self.bus.readu16(addr)
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.bus.write(addr, data)
  }

  pub fn writeu16(&mut self, addr: u16, data: u16) {
    self.bus.writeu16(addr, data)
  }
}