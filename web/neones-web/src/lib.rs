mod utils;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use neones::{
  apu::mixer::NESAudioCallback,
  neones::NeoNES,
  ppu::frame::Frame,
  renderer::Renderer,
  system::joypad::{Flag as JoypadButton, Joypad}
};

struct WebRenderer {
  frame: Rc<RefCell<Vec<u8>>>,
}

impl Renderer for WebRenderer {
  fn render(&mut self, frame: &[u8; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE], _: &mut Joypad) {
    for i in 0 .. (Frame::WIDTH * Frame::HEIGHT) {
      self.frame.borrow_mut()[4 * i + 0] = frame[3 * i + 0];
      self.frame.borrow_mut()[4 * i + 1] = frame[3 * i + 1];
      self.frame.borrow_mut()[4 * i + 2] = frame[3 * i + 2];
    }
  }
}

impl WebRenderer {
  fn new(frame: Rc<RefCell<Vec<u8>>>) -> Self {
    WebRenderer {
      frame,
    }
  }
}

#[wasm_bindgen]
pub struct NES {
  neo: NeoNES,
  frame: Rc<RefCell<Vec<u8>>>,
  audio: NESAudioCallback,
}


#[wasm_bindgen]
impl NES {
  #[wasm_bindgen(constructor)]
  pub fn new(rom: Vec<u8>) -> Self {
    utils::set_panic_hook();
    let frame = Rc::from(RefCell::from(vec![69; Frame::WIDTH * Frame::HEIGHT * 4]));
    let renderer = WebRenderer::new(frame.clone());
    let mut neo = NeoNES::new(rom, Rc::from(RefCell::from(renderer)));
    let audio = neo.audio();

    NES {
      neo,
      frame,
      audio
    }
  }

  pub fn frame(&self) -> *const u8 {
    self.frame.borrow().as_ptr()
  }

  pub fn step(&mut self) {
    self.neo.step_frame();
  }

  pub fn push(&mut self, key: &str) {
    match key {
      "KeyW" => self.neo.push(JoypadButton::Up),
      "KeyA" => self.neo.push(JoypadButton::Left),
      "KeyS" => self.neo.push(JoypadButton::Down),
      "KeyD" => self.neo.push(JoypadButton::Right),

      "KeyN" => self.neo.push(JoypadButton::A),
      "KeyM" => self.neo.push(JoypadButton::B),

      "Enter" => self.neo.push(JoypadButton::Start),
      "Space" => self.neo.push(JoypadButton::Select),
      _ => ()
    }
  }

  pub fn release(&mut self, key: &str) {
    match key {
      "KeyW" => self.neo.release(JoypadButton::Up),
      "KeyA" => self.neo.release(JoypadButton::Left),
      "KeyS" => self.neo.release(JoypadButton::Down),
      "KeyD" => self.neo.release(JoypadButton::Right),

      "KeyN" => self.neo.release(JoypadButton::A),
      "KeyM" => self.neo.release(JoypadButton::B),

      "Enter" => self.neo.release(JoypadButton::Start),
      "Space" => self.neo.release(JoypadButton::Select),
      _ => ()
    }
  }

  pub fn signal(&mut self, out: &mut [f32]) {
    self.audio.signal(out);
  }
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}
