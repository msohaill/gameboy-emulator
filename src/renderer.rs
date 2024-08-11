pub mod sdlrenderer;

use crate::{apu::mixer::Consumer, ppu::frame::Frame, system::joypad::Joypad};

pub trait Renderer {
  fn render(&mut self, frame: &[u8; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE], joypad: &mut Joypad);

  fn use_consumer(&mut self, consumer: Consumer);
}
