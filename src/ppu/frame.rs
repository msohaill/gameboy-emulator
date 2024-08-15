use super::palette::Color;

pub struct Frame {
  pub data: [u8; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE],
  pub number: usize,
}

impl Frame {
  pub const WIDTH: usize = 256;
  pub const HEIGHT: usize = 240;
  pub const SCALE: usize = 3;

  pub fn new() -> Self {
    Frame {
      data: [0; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE],
      number: 0,
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
    let loc = y * Frame::WIDTH * Frame::SCALE + x * Frame::SCALE;
    if loc + 2 < self.data.len() {
      let Color(r, g, b) = color;
      self.data[loc + 0] = r;
      self.data[loc + 1] = g;
      self.data[loc + 2] = b;
    }
  }
}
