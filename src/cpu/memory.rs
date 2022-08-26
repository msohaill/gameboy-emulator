pub struct Memory {
  arr: [u8; 0xFFFF],
}

impl Memory {
  pub fn new() -> Self {
    Memory {
      arr: [0; 0xFFFF],
    }
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.arr[addr as usize]
  }

  pub fn readu16(&self, addr: u16) -> u16 {
    let lo = self.read(addr) as u16;
    let hi = self.read(addr.wrapping_add(1)) as u16;
    (hi << 8) | lo
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.arr[addr as usize] = data;
  }

  pub fn writeu16(&mut self, addr: u16, data: u16) {
    let lo = (data & 0xFF) as u8;
    let hi = (data >> 8) as u8;
    self.write(addr, lo);
    self.write(addr.wrapping_add(1), hi);
  }

  pub fn load(&mut self, start: usize, program: Vec<u8>) {
    self.arr[start .. (start + program.len())].copy_from_slice(&program[..]);
  }
}