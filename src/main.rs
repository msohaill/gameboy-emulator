use neones::neones::NeoNES;
use neones::renderer::sdlrenderer::SDLRenderer;

fn main() {
  let path = std::env::args().nth(1).unwrap_or(String::from("dev/Super_Mario.nes"));
  let rom = std::fs::read(path).unwrap();

  let renderer = SDLRenderer::new();
  let mut nes = NeoNES::new(rom, Box::new(renderer));

  nes.start();
}
