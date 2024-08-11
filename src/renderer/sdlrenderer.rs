use crate::apu::mixer::{Consumer, Mixer};
use crate::ppu::frame::Frame;
use crate::renderer::Renderer;
use crate::system::joypad::{Flag as JoypadButton, Joypad};

use ringbuf::traits::{Consumer as _, Observer as _};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use sdl2::{AudioSubsystem, EventPump};

const ICON: &[u8] = include_bytes!("../../assets/neones.bmp");

pub struct SDLRenderer {
  canvas: Canvas<Window>,
  texture_creator: TextureCreator<WindowContext>,
  event_pump: EventPump,
  audio: AudioSubsystem,
}

pub struct Callback {
  initialized: bool,
  buffer: Consumer,
}

impl Callback {
  fn new(buffer: Consumer) -> Self {
    Callback {
      initialized: false,
      buffer,
    }
  }
}

impl AudioCallback for Callback {
  type Channel = f32;

  fn callback(&mut self, out: &mut [f32]) {
    if !self.initialized && self.buffer.occupied_len() < out.len() {
      out.fill(0.0);
      return;
    }
    self.initialized = true;

    for x in out {
      if let Some(sample) = self.buffer.try_pop() {
        *x = sample;
      } else {
        *x = 0.0;
      }
    }
  }
}

impl Renderer for SDLRenderer {
  fn render(&mut self, frame: &[u8; Frame::WIDTH * Frame::HEIGHT * Frame::SCALE], joypad: &mut Joypad) {
    let mut texture = self.texture_creator
      .create_texture_target(PixelFormatEnum::RGB24, Frame::WIDTH as u32, Frame::HEIGHT as u32)
      .unwrap();

    texture.update(None, frame, Frame::WIDTH * Frame::SCALE).unwrap();
    self.canvas.copy(&texture, None, None).unwrap();
    self.canvas.present();

    for event in self.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => std::process::exit(0),
        Event::KeyDown {  keycode: Some(key), .. } => {
          match key {
            Keycode::Escape => std::process::exit(0),

            Keycode::W => joypad.push(JoypadButton::Up),
            Keycode::A => joypad.push(JoypadButton::Left),
            Keycode::S => joypad.push(JoypadButton::Down),
            Keycode::D => joypad.push(JoypadButton::Right),

            Keycode::N => joypad.push(JoypadButton::A),
            Keycode::M => joypad.push(JoypadButton::B),

            Keycode::Return => joypad.push(JoypadButton::Start),
            Keycode::Space => joypad.push(JoypadButton::Select),

            _ => {}
          };
        }
        Event::KeyUp {  keycode: Some(key), .. } => {
          match key {
            Keycode::W => joypad.release(JoypadButton::Up),
            Keycode::A => joypad.release(JoypadButton::Left),
            Keycode::S => joypad.release(JoypadButton::Down),
            Keycode::D => joypad.release(JoypadButton::Right),

            Keycode::N => joypad.release(JoypadButton::A),
            Keycode::M => joypad.release(JoypadButton::B),

            Keycode::Return => joypad.release(JoypadButton::Start),
            Keycode::Space => joypad.release(JoypadButton::Select),

            _ => {}
          };
        }
        _ => (),
      }
    }
  }

  fn use_consumer(&mut self, consumer: Consumer) {
    let audio_callback = Callback::new(consumer);

    let audio = self.audio.open_playback(None, &AudioSpecDesired {
      freq: Some(Mixer::OUTPUT_FREQ as i32),
      channels: Some(1),
      samples: Some(Mixer::BUFFER_SIZE as u16 / 2),
    }, |_| audio_callback).unwrap();

    audio.resume();
    std::mem::forget(audio); // To ensure audio is not dropped
  }
}

impl SDLRenderer {
  pub fn new() -> Self {
    let sdl_context = sdl2::init().unwrap();
    let mut window = sdl_context
      .video()
      .unwrap()
      .window("NeoNES", (Frame::WIDTH * Frame::SCALE) as u32, (Frame::HEIGHT * Frame::SCALE) as u32)
      .borderless()
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

    let audio = sdl_context.audio().unwrap();

    SDLRenderer {
      canvas,
      texture_creator,
      event_pump,
      audio,
    }
  }
}
