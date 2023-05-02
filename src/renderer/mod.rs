pub mod color;
pub mod frame;
pub mod palette;

use super::ppu::{register::controller::Flag as ControllerFlag, PPU};
use frame::Frame;
use palette::PALETTE;

fn background_palette(ppu: &PPU, column: usize, row: usize) -> [usize; 4] {
  let index = row / 4 * 8 + column / 4;
  let byte = ppu.vram[0x3C0 + index];

  let palette_index = match (column % 4 / 2, row % 4 / 2) {
    (0, 0) => byte & 0b11,
    (1, 0) => (byte >> 2) & 0b11,
    (0, 1) => (byte >> 4) & 0b11,
    (1, 1) => (byte >> 6) & 0b11,
    _ => panic!("Should not happen!"),
  };

  let palette_start = 1 + (palette_index as usize) * 4;
  [
    ppu.palette[0] as usize,
    ppu.palette[palette_start] as usize,
    ppu.palette[palette_start + 1] as usize,
    ppu.palette[palette_start + 2] as usize,
  ]
}

fn sprite_palette(ppu: &PPU, palette_index: u8) -> [usize; 4] {
  let start = 0x11 + (palette_index * 4) as usize;
  [
    0,
    ppu.palette[start] as usize,
    ppu.palette[start + 1] as usize,
    ppu.palette[start + 2] as usize,
  ]
}

pub fn render(ppu: &PPU, frame: &mut Frame) {
  let bank = 0x1000
    * (ppu
      .registers
      .controller
      .get_flag(ControllerFlag::BackgroundAddr) as u16);

  for i in 0..0x03C0 {
    let tile = ppu.vram[i] as u16;
    let column = i % 32;
    let row = i / 32;

    let tile = &ppu.chr[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
    let palette = background_palette(ppu, column, row);

    for y in 0..=7 {
      let mut upper = tile[y];
      let mut lower = tile[y + 8];

      for x in (0..=7).rev() {
        let val = (1 & lower) << 1 | (1 & upper);
        upper = upper >> 1;
        lower = lower >> 1;

        let color = PALETTE[palette[val as usize]];

        frame.set_pixel(column * 8 + x, row * 8 + y, color)
      }
    }
  }

  for i in (0..ppu.oam.len()).step_by(4).rev() {
    let tile = ppu.oam[i + 1] as u16;
    let column = ppu.oam[i + 3] as usize;
    let row = ppu.oam[i] as usize;

    let flip_horizontal = ppu.oam[i + 2] >> 6 & 1 == 1;
    let flip_vertical = ppu.oam[i + 2] >> 7 & 1 == 1;

    let palette_index = ppu.oam[i + 2] & 0b11;
    let sprite_palette = sprite_palette(ppu, palette_index);

    let bank = 0x1000
      * (ppu
        .registers
        .controller
        .get_flag(ControllerFlag::SpriteAddr) as u16);

    let tile = &ppu.chr[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];

    for y in 0..=7 {
      let mut upper = tile[y];
      let mut lower = tile[y + 8];

      'draw: for x in (0..=7).rev() {
        let val = (1 & lower) << 1 | (1 & upper);
        upper = upper >> 1;
        lower = lower >> 1;

        if val == 0 {
          continue 'draw;
        }

        let color = PALETTE[sprite_palette[val as usize]];

        match (flip_horizontal, flip_vertical) {
          (false, false) => frame.set_pixel(column + x, row + y, color),
          (true, false) => frame.set_pixel(column + 7 - x, row + y, color),
          (false, true) => frame.set_pixel(column + x, row + 7 - y, color),
          (true, true) => frame.set_pixel(column + 7 - x, row + 7 - y, color),
        };
      }
    }
  }
}
