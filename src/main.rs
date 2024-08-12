use std::{cell::RefCell, rc::Rc};

use neones::{renderer::sdlrenderer::SDLRenderer, neones::NeoNES};

fn main() {
  let path = std::env::args().nth(1).unwrap_or(String::from("dev/Super_Mario.nes"));
  let rom = std::fs::read(path).unwrap();

  let renderer = Rc::from(RefCell::from(SDLRenderer::new()));
  let mut nes = NeoNES::new(rom, renderer.clone());

  renderer.borrow_mut().use_callback(nes.audio());

  nes.start();
}
