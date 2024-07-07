use super::color::Color;

pub struct Frame {
  pub data: Vec<u8>,
}

impl Frame {
  pub const WIDTH: usize = 256;
  pub const HEIGHT: usize = 240;
  pub const SCALE: usize = 3;

  pub fn new() -> Self {
    Frame {
      data: vec![0; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE],
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    let loc = y * Frame::WIDTH * Frame::SCALE + x * Frame::SCALE;
    if loc + 2 < self.data.len() {
      self.data[loc] = color.0;
      self.data[loc + 1] = color.1;
      self.data[loc + 2] = color.2;
    }
  }
}
