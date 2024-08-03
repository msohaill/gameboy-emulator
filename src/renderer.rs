use crate::joypad::Flag as JoypadButton;
use crate::ppu::frame::Frame;
use crate::system::System;

use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

const ICON: &[u8] = include_bytes!("../assets/neones.bmp");

pub struct Renderer {
  canvas: Canvas<Window>,
  texture_creator: TextureCreator<WindowContext>,
  event_pump: EventPump,
  audio: AudioQueue<f32>,
}

impl Renderer {
  pub fn new() -> Self {
    let sdl_context = sdl2::init().unwrap();
    let mut window = sdl_context
      .video()
      .unwrap()
      .window("NeoNES", (Frame::WIDTH * Frame::SCALE) as u32, (Frame::HEIGHT * Frame::SCALE) as u32)
      .position_centered()
      .build()
      .unwrap();

    window.set_icon(Surface::load_bmp_rw(&mut RWops::from_bytes(ICON).unwrap()).unwrap());

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    canvas
      .set_scale(Frame::SCALE as f32, Frame::SCALE as f32)
      .unwrap();
    let texture_creator = canvas.texture_creator();

    let audio = sdl_context.audio().unwrap().open_queue(None, &AudioSpecDesired {
      freq: Some(44100),
      channels: Some(2),
      samples: Some(1024),
    }).unwrap();
    audio.resume();

    Renderer {
      canvas,
      texture_creator,
      event_pump,
      audio,
    }
  }

  fn gen_wave() -> Vec<f32> {
    let tone_volume = 0.1;
    let p = 5;
    let sample_count = 44100 * p;
    let mut result = Vec::new();

    for x in 0..sample_count {
      let x = x as f32;
      let m = x / 100.0;
      let p = p as f32;
      let a = 9.0 / p;
      let i = 0.5 * (a * p + 1.0) - (0.5 * (a * p - 1.0) - (a * x / 44100.0).floor() as f32).abs();
      result.push((i * m).sin() * tone_volume);
    }
    result
  }

  pub fn update_canvas(system: &mut System) {
    let mut texture = system.renderer.texture_creator
      .create_texture_target(PixelFormatEnum::RGB24, Frame::WIDTH as u32, Frame::HEIGHT as u32)
      .unwrap();

    texture.update(None, &system.ppu.frame.data, Frame::WIDTH * Frame::SCALE).unwrap();
    system.renderer.canvas.copy(&texture, None, None).unwrap();

    system.renderer.canvas.present();

    let wave = Renderer::gen_wave();
    system.renderer.audio.queue_audio(&wave).unwrap();

    for event in system.renderer.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => std::process::exit(0),
        Event::KeyDown {  keycode: Some(key), .. } => {
          match key {
            Keycode::Escape => std::process::exit(0),

            Keycode::W => system.joypads.0.push(JoypadButton::Up),
            Keycode::A => system.joypads.0.push(JoypadButton::Left),
            Keycode::S => system.joypads.0.push(JoypadButton::Down),
            Keycode::D => system.joypads.0.push(JoypadButton::Right),

            Keycode::N => system.joypads.0.push(JoypadButton::A),
            Keycode::M => system.joypads.0.push(JoypadButton::B),

            Keycode::Return => system.joypads.0.push(JoypadButton::Start),
            Keycode::Space => system.joypads.0.push(JoypadButton::Select),

            _ => {}
          };
        }
        Event::KeyUp {  keycode: Some(key), .. } => {
          match key {
            Keycode::W => system.joypads.0.release(JoypadButton::Up),
            Keycode::A => system.joypads.0.release(JoypadButton::Left),
            Keycode::S => system.joypads.0.release(JoypadButton::Down),
            Keycode::D => system.joypads.0.release(JoypadButton::Right),

            Keycode::N => system.joypads.0.release(JoypadButton::A),
            Keycode::M => system.joypads.0.release(JoypadButton::B),

            Keycode::Return => system.joypads.0.release(JoypadButton::Start),
            Keycode::Space => system.joypads.0.release(JoypadButton::Select),

            _ => {}
          };
        }
        _ => (),
      }
    }
  }
}
