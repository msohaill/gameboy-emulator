pub mod cpu;
pub mod joypad;
pub mod neones;
pub mod ppu;
pub mod renderer;
pub mod system;
pub mod utils;

use neones::NeoNES;

fn main() {
  let mut nes = NeoNES::new("dev/Super_Mario.nes");
  nes.start();
}
