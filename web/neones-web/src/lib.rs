mod utils;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use neones::{
  apu::mixer::NESAudioCallback,
  neones::NeoNES as InnerNES,
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
pub struct NeoNES {
  emulator: InnerNES,
  frame: Rc<RefCell<Vec<u8>>>,
  audio: NESAudioCallback,
}


#[wasm_bindgen]
impl NeoNES {
  #[wasm_bindgen(constructor)]
  pub fn new(rom: Vec<u8>) -> Self {
    utils::set_panic_hook();
    let frame = Rc::from(RefCell::from(vec![255; Frame::WIDTH * Frame::HEIGHT * 4]));
    let renderer = WebRenderer::new(frame.clone());
    let mut emulator = InnerNES::new(rom, Rc::from(RefCell::from(renderer)));
    let audio = emulator.audio();

    NeoNES {
      emulator,
      frame,
      audio
    }
  }

  pub fn frame(&self) -> *const u8 {
    self.frame.borrow().as_ptr()
  }

  pub fn step(&mut self) {
    self.emulator.step_frame();
  }

  pub fn push(&mut self, key: &str) {
    match key {
      "KeyW" => self.emulator.push(JoypadButton::Up),
      "KeyA" => self.emulator.push(JoypadButton::Left),
      "KeyS" => self.emulator.push(JoypadButton::Down),
      "KeyD" => self.emulator.push(JoypadButton::Right),

      "KeyN" => self.emulator.push(JoypadButton::A),
      "KeyM" => self.emulator.push(JoypadButton::B),

      "Enter" => self.emulator.push(JoypadButton::Start),
      "Space" => self.emulator.push(JoypadButton::Select),
      _ => ()
    }
  }

  pub fn release(&mut self, key: &str) {
    match key {
      "KeyW" => self.emulator.release(JoypadButton::Up),
      "KeyA" => self.emulator.release(JoypadButton::Left),
      "KeyS" => self.emulator.release(JoypadButton::Down),
      "KeyD" => self.emulator.release(JoypadButton::Right),

      "KeyN" => self.emulator.release(JoypadButton::A),
      "KeyM" => self.emulator.release(JoypadButton::B),

      "Enter" => self.emulator.release(JoypadButton::Start),
      "Space" => self.emulator.release(JoypadButton::Select),
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
