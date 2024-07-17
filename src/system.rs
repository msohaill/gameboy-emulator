pub mod cartridge;
pub mod mapper;
pub mod memory;

use crate::joypad::Joypad;
use crate::ppu::PPU;
use crate::renderer::Renderer;
use cartridge::Cartridge;
use memory::Memory;

pub struct System {
  pub ppu: PPU,
  pub joypad: Joypad,
  pub renderer: Renderer,
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

  pub fn new(cartridge: Cartridge) -> Self {
    System {
      ppu: PPU::new(cartridge.mapper),
      joypad: Joypad::new(),
      renderer: Renderer::new(),
      cycles: 0,
      memory: Memory::new(),
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      System::RAM..=System::RAM_END => self.memory.read(addr),
      System::PPU..=System::PPU_END => self.ppu.read(addr),
      System::EROM..=System::EROM_END => 0,
      System::SRAM..=System::SRAM_END => self.ppu.mapper.read(addr),
      System::ROM..=System::ROM_END => self.ppu.mapper.read(addr),
      System::JOYPAD1 => self.joypad.read(),
      System::JOYPAD2 => 0,          // Ignoring for now
      0x4000..=0x4013 | 0x4015 => 0, // Ignoring APU for now
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
      System::JOYPAD1 => self.joypad.write(data),
      System::JOYPAD2 => (),          // Ignoring for now
      0x4000..=0x4013 | 0x4015 => (), // Ignoring APU for now
      _ => println!("Ignoring write: {:#0X}", addr),
    }
  }

  pub fn readu16(&mut self, addr: u16) -> u16 {
    let lo = self.read(addr) as u16;
    let hi = self.read(addr.wrapping_add(1)) as u16;
    (hi << 8) | lo
  }

  pub fn writeu16(&mut self, addr: u16, data: u16) {
    let lo = (data & 0xFF) as u8;
    let hi = (data >> 8) as u8;
    self.write(addr, lo);
    self.write(addr.wrapping_add(1), hi);
  }

  pub fn tick(&mut self, cycles: u16) {
    self.cycles += cycles as usize;

    let mut render = false;
    for _ in 0 .. 3 * cycles {
      let nmi_status = self.ppu.nmi_interrupt;
      self.ppu.tick();
      if !nmi_status && self.ppu.nmi_interrupt {
        render = true
      }
    }

    if render {
      Renderer::update_canvas(self);
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

  pub fn poll_nmi(&mut self) -> bool {
    let res = self.ppu.nmi_interrupt;
    self.ppu.nmi_interrupt = false;
    res
  }
}
