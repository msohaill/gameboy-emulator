pub struct Banks {
  size: usize,
  window: usize,
  banks: Vec<usize>,
  pages: usize,
  memory: Vec<u8>,
  writeable: bool,
}

impl Banks {
  pub fn new(start: usize, end: usize, window: usize, memory: Vec<u8>, writeable: bool) -> Self {
    let size = end - start;
    let mut banks = vec![0; (size + 1) / window];

    for i in 0 .. banks.len() {
      banks[i] = i * window;
    }

    Banks {
      size,
      window,
      banks,
      pages: std::cmp::max(1, memory.len() / window),
      memory,
      writeable,
    }
  }

  pub fn set_range(&mut self, start: usize, end: usize, bank: usize) {
    let mut addr = (bank % self.pages) * self.window;
    for slot in start ..= end {
      self.banks[slot] = addr;
      addr += self.window;
    }
  }

  pub fn set(&mut self, slot: usize, bank: usize) {
    self.set_range(slot, slot, bank);
  }

  pub const fn last(&self) -> usize {
    self.pages.saturating_sub(1)
  }

  const fn get_bank(&self, addr: u16) -> usize {
    ((addr as usize) & self.size) >> self.window.trailing_zeros()
  }

  pub fn translate(&self, addr: u16) -> usize {
    let page = self.banks[self.get_bank(addr)];
    page | ((addr as usize) % self.window)
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.memory[self.translate(addr)]
  }

  pub fn write(&mut self, addr: u16, val: u8) {
    if self.writeable {
      let index = self.translate(addr);
      self.memory[index] = val;
    }
  }

  pub fn capacity(&self) -> usize {
    self.memory.len()
  }
}
