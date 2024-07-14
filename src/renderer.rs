use crate::joypad::Flag as JoypadButton;
use crate::ppu::frame::Frame;
use crate::system::System;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

pub struct Renderer {
  canvas: Canvas<Window>,
  texture_creator: TextureCreator<WindowContext>,
  event_pump: EventPump,
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
      event_pump,
    }
  }

  pub fn update_canvas(system: &mut System) {
    let mut texture = system.renderer.texture_creator
      .create_texture_target(PixelFormatEnum::RGB24, Frame::WIDTH as u32, Frame::HEIGHT as u32)
      .unwrap();

    texture.update(None, &system.ppu.frame.data, Frame::WIDTH * Frame::SCALE).unwrap();
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
}
