pub mod bus;

use bus::Bus;

pub struct Memory {
  bus: Bus,
}

impl Memory {
  pub fn new() -> Self {
    Memory {
      bus: Bus::new(),
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

  pub fn load(&mut self, start: u16, program: Vec<u8>) {
    for i in 0..(program.len() as u16) {
      self.write(start + i, program[i as usize]);
    }
    // self.arr[start .. (start + program.len())].copy_from_slice(&program[..]);
  }
}