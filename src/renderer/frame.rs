use super::color::Color;

pub struct Frame {
  pub data: Vec<u8>,
}

impl Frame {
  const WIDTH: usize = 256;
  const HEIGHT: usize = 240;

  pub fn new() -> Self {
    Frame {
      data: vec![0; Frame::WIDTH * Frame::HEIGHT * 3],
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    let loc = y * 3 * Frame::WIDTH + x * 3;
    if loc + 2 < self.data.len() {
      self.data[loc] = color.0;
      self.data[loc + 1] = color.1;
      self.data[loc + 2] = color.2;
    }
  }
}
