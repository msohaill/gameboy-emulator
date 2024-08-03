pub struct LengthCounter {
  pub counter: u8,
  pub halted: bool,
}

impl LengthCounter {
  const TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6,
    160, 8, 60, 10, 14, 12, 26, 14,
    12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
  ];

  pub fn new() -> Self {
    LengthCounter {
      counter: 0,
      halted: true,
    }
  }

  pub fn update(&mut self, index: u8) {
    self.counter = LengthCounter::TABLE[index as usize];
  }
}
