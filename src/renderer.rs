#[cfg(not(target_arch = "wasm32"))]
pub mod sdlrenderer;

use crate::{ppu::frame::Frame, system::joypad::Joypad};

pub trait Renderer {
  fn render(&mut self, frame: &[u8; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE], joypad: &mut Joypad);
}
