pub mod apu;
pub mod cpu;
pub mod joypad;
pub mod neones;
pub mod ppu;
pub mod renderer;
pub mod system;
pub mod utils;

use neones::NeoNES;

fn main() {
  let path = std::env::args().nth(1).unwrap_or(String::from("dev/Super_Mario.nes"));
  let mut nes = NeoNES::new(path);
  nes.start();
}
