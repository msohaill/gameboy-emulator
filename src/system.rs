pub(crate) mod cartridge;
pub mod joypad;
pub(crate) mod mapper;
mod memory;

use std::cell::RefCell;
use std::rc::Rc;

use crate::apu::mixer::NESAudioCallback;
use crate::apu::APU;
use crate::ppu::PPU;
use crate::renderer::Renderer;
use cartridge::Cartridge;
use joypad::Joypad;
use memory::Memory;

pub struct System {
  pub apu: APU,
  pub ppu: PPU,
  pub joypads: (Joypad, Joypad),
  pub renderer: Rc<RefCell<dyn Renderer>>,
  cycles: usize,
  memory: Memory,
}

impl System {
  pub const RAM: u16 = 0x0000;
  pub const RAM_END: u16 = 0x1FFF;
  pub const PPU: u16 = 0x2000;
  pub const PPU_END: u16 = 0x3FFF;
  pub const EROM: u16 = 0x4020;
  pub const EROM_END: u16 = 0x5FFF;
  pub const SRAM: u16 = 0x6000;
  pub const SRAM_END: u16 = 0x7FFF;
  pub const ROM: u16 = 0x8000;
  pub const ROM_END: u16 = 0xFFFF;
  pub const OAM_REQ: u16 = 0x4014;
  pub const JOYPAD1: u16 = 0x4016;
  pub const JOYPAD2: u16 = 0x4017;

  pub fn new(cartridge: Cartridge, renderer: Rc<RefCell<dyn Renderer>>) -> Self {
    System {
      apu: APU::new(),
      ppu: PPU::new(cartridge.mapper),
      joypads: (Joypad::new(), Joypad::new()),
      renderer,
      cycles: 0,
      memory: Memory::new(),
    }
  }

  pub fn callback(&mut self) -> NESAudioCallback {
    NESAudioCallback::new(self.apu.mixer.consumer())
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      System::RAM..=System::RAM_END => self.memory.read(addr),
      System::PPU..=System::PPU_END => self.ppu.read(addr),
      System::EROM..=System::EROM_END => 0,
      System::SRAM..=System::SRAM_END => self.ppu.mapper.read(addr),
      System::ROM..=System::ROM_END => self.ppu.mapper.read(addr),
      System::JOYPAD1 => self.joypads.0.read(),
      System::JOYPAD2 => self.joypads.1.read(),
      0x4000..=0x4013 | 0x4015 => self.apu.read(addr),
      _ => {
        println!("Ignoring read: {:#0X}", addr);
        0
      }
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      System::RAM..=System::RAM_END => self.memory.write(addr, data),
      System::PPU..=System::PPU_END => self.ppu.write(addr, data),
      System::EROM..=System::EROM_END => { },
      System::SRAM..=System::SRAM_END => self.ppu.mapper.write(addr, data),
      System::ROM..=System::ROM_END => self.ppu.mapper.write(addr, data),
      System::OAM_REQ => self.oamdma(data),
      System::JOYPAD1 => {
        self.joypads.0.write(data);
        self.joypads.1.write(data);
      },
      0x4000..=0x4013 | 0x4015 | 0x4017 => self.apu.write(addr, data),
      _ => println!("Ignoring write: {:#0X}", addr),
    }
  }

  pub fn readu16(&mut self, addr: u16) -> u16 {
    let lo = self.read(addr) as u16;
    let hi = self.read(addr.wrapping_add(1)) as u16;
    (hi << 8) | lo
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles = self.cycles.wrapping_add(cycles as usize);

    let mut render = false;
    for _ in 0 .. 3 * cycles {

      if self.ppu.tick() {
        render = true
      }
    }

    for _ in 0 .. cycles {
      self.apu.tick();

      if self.apu.dma() {
        self.dmcdma();
      }
    }

    if render {
      self.apu.mix();
      self.renderer.borrow_mut().render(&self.ppu.frame.data, &mut self.joypads.0)
    }
  }

  fn oamdma(&mut self, data: u8) {
    let hi: u16 = (data as u16) << 8;
    for lo in 0x0..0x100 {
      let val = self.read(hi | lo);
      self.ppu.write(0x2004, val);
    }
    self.tick(if self.cycles % 2 == 0 { 513 } else { 514 })
  }

  fn dmcdma(&mut self) {
    let addr = self.apu.dma_addr();
    let val = self.read(addr);
    self.apu.dmcdma(val);
  }

  pub fn poll_nmi(&mut self) -> bool {
    self.ppu.poll()
  }

  pub fn poll_irq(&mut self) -> bool {
    self.apu.poll() || self.ppu.mapper.poll()
  }
}
