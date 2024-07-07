pub mod color;
pub mod frame;
pub mod palette;
pub mod viewport;

use crate::joypad::Flag as JoypadButton;
use crate::system::System;

use super::ppu::PPU;
use super::system::cartridge::Mirroring;
use frame::Frame;
use palette::PALETTE;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use viewport::Viewport;

pub struct Renderer {
  canvas: Canvas<Window>,
  texture_creator: TextureCreator<WindowContext>,
  event_pump: EventPump,
  frame: Frame,
}

impl Renderer {
  pub fn new() -> Self {
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
      .video()
      .unwrap()
      .window("NeoNES", (Frame::WIDTH * Frame::SCALE) as u32, (Frame::HEIGHT * Frame::SCALE) as u32)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    canvas
      .set_scale(Frame::SCALE as f32, Frame::SCALE as f32)
      .unwrap();
    let texture_creator = canvas.texture_creator();

    Renderer {
      canvas,
      texture_creator,
      frame: Frame::new(),
      event_pump,
    }
  }

  fn background_palette(ppu: &PPU, attribute_table: &[u8], column: usize, row: usize) -> [usize; 4] {
    let index = row / 4 * 8 + column / 4;
    let byte = attribute_table[index];

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

  fn render_name_table(&mut self, ppu: &mut PPU, name_table: &[u8], viewport: Viewport) {
    let bank = ppu.registers.controller.background_pattern_table();

    let attribute_table = &name_table[0x3C0..0x400];

    for i in 0..0x3C0 {
      let column = i % 32;
      let row = i / 32;
      let index = name_table[i] as u16;
      let tile = &ppu.chr[(bank + index * 16) as usize..=(bank + index * 16 + 15) as usize];
      let palette = Renderer::background_palette(ppu, attribute_table, column, row);

      for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        for x in (0..=7).rev() {
          let val = (1 & lower) << 1 | (1 & upper);
          upper >>= 1;
          lower >>= 1;

          let color = PALETTE[palette[val as usize]];

          let pixel_x = column * 8 + x;
          let pixel_y = row * 8 + y;

          if pixel_x >= viewport.0
            && pixel_x < viewport.2
            && pixel_y >= viewport.1
            && pixel_y < viewport.3
          {
            self.frame.set_pixel(
              (viewport.4 + pixel_x as isize) as usize,
              (viewport.5 + pixel_y as isize) as usize,
              color,
            );
          }
        }
      }
    }
  }

  fn update_canvas(system: &mut System) {
    let mut texture = system.renderer.texture_creator
      .create_texture_target(PixelFormatEnum::RGB24, Frame::WIDTH as u32, Frame::HEIGHT as u32)
      .unwrap();

    texture.update(None, &system.renderer.frame.data, Frame::WIDTH * Frame::SCALE).unwrap();
    system.renderer.canvas.copy(&texture, None, None).unwrap();

    system.renderer.canvas.present();
    for event in system.renderer.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => std::process::exit(0),
        Event::KeyDown {  keycode: Some(key), .. } => {
          match key {
            Keycode::Escape => std::process::exit(0),

            Keycode::W => system.joypad.push(JoypadButton::Up),
            Keycode::A => system.joypad.push(JoypadButton::Left),
            Keycode::S => system.joypad.push(JoypadButton::Down),
            Keycode::D => system.joypad.push(JoypadButton::Right),

            Keycode::N => system.joypad.push(JoypadButton::A),
            Keycode::M => system.joypad.push(JoypadButton::B),

            Keycode::Return => system.joypad.push(JoypadButton::Start),
            Keycode::Space => system.joypad.push(JoypadButton::Select),

            _ => {}
          };
        }
        Event::KeyUp {  keycode: Some(key), .. } => {
          match key {
            Keycode::W => system.joypad.release(JoypadButton::Up),
            Keycode::A => system.joypad.release(JoypadButton::Left),
            Keycode::S => system.joypad.release(JoypadButton::Down),
            Keycode::D => system.joypad.release(JoypadButton::Right),

            Keycode::N => system.joypad.release(JoypadButton::A),
            Keycode::M => system.joypad.release(JoypadButton::B),

            Keycode::Return => system.joypad.release(JoypadButton::Start),
            Keycode::Space => system.joypad.release(JoypadButton::Select),

            _ => {}
          };
        }
        _ => (),
      }
    }
  }

  pub fn render(system: &mut System) {
    let scrollx = system.ppu.registers.scroll.scrollx as usize;
    let scrolly = system.ppu.registers.scroll.scrolly as usize;
    let vram = system.ppu.vram.clone();

    let (main, secondary) = match (&system.ppu.mirroring, system.ppu.registers.controller.name_table()) {
      (Mirroring::Vertical, 0x2000)
      | (Mirroring::Vertical, 0x2800)
      | (Mirroring::Horizontal, 0x2000)
      | (Mirroring::Horizontal, 0x2400) => (&vram[0..0x400], &vram[0x400..0x800]),
      (Mirroring::Vertical, 0x2400)
      | (Mirroring::Vertical, 0x2C00)
      | (Mirroring::Horizontal, 0x2800)
      | (Mirroring::Horizontal, 0x2C00) => (&vram[0x400..0x800], &vram[0..0x400]),
      (_, _) => {
        panic!("Unsupported mirroring type.");
      }
    };

    system.renderer.render_name_table(
      &mut system.ppu,
      main,
      Viewport(
        scrollx,
        scrolly,
        Frame::WIDTH,
        Frame::HEIGHT,
        -(scrollx as isize),
        -(scrolly as isize),
      ),
    );

    if scrollx > 0 {
      system.renderer.render_name_table(
        &mut system.ppu,
        secondary,
        Viewport(0, 0, scrollx, Frame::HEIGHT, (Frame::WIDTH - scrollx) as isize, 0),
      );
    } else if scrolly > 0 {
      system.renderer.render_name_table(
        &mut system.ppu,
        secondary,
        Viewport(0, 0, Frame::WIDTH, scrolly, 0, (Frame::HEIGHT - scrolly) as isize),
      );
    }

    for i in (0..system.ppu.oam.len()).step_by(4).rev() {
      let tile = system.ppu.oam[i + 1] as u16;
      let column = system.ppu.oam[i + 3] as usize;
      let row = system.ppu.oam[i] as usize;

      let flip_horizontal = system.ppu.oam[i + 2] >> 6 & 1 == 1;
      let flip_vertical = system.ppu.oam[i + 2] >> 7 & 1 == 1;

      let palette_index = system.ppu.oam[i + 2] & 0b11;
      let sprite_palette = Renderer::sprite_palette(&system.ppu, palette_index);

      let bank = system.ppu.registers.controller.sprite_pattern_table();

      let tile = &system.ppu.chr[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];

      for y in 0..=7 {
        let mut upper = tile[y];
        let mut lower = tile[y + 8];

        'draw: for x in (0..=7).rev() {
          let val = (1 & lower) << 1 | (1 & upper);
          upper >>= 1;
          lower >>= 1;

          if val == 0 {
            continue 'draw;
          }

          let color = PALETTE[sprite_palette[val as usize]];

          match (flip_horizontal, flip_vertical) {
            (false, false) => system.renderer.frame.set_pixel(column + x, row + y, color),
            (true, false) => system.renderer.frame.set_pixel(column + 7 - x, row + y, color),
            (false, true) => system.renderer.frame.set_pixel(column + x, row + 7 - y, color),
            (true, true) => system.renderer.frame.set_pixel(column + 7 - x, row + 7 - y, color),
          };
        }
      }
    }

    Renderer::update_canvas(system)
  }
}
