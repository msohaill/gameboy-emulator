pub(in crate::ppu) mod controller;
pub(in crate::ppu) mod mask;
pub(in crate::ppu) mod status;

use controller::{Controller, Flag as ControllerFlag};
use mask::Mask;
use status::{Status, Flag as StatusFlag};

use crate::system::System;

pub struct Registers {
  pub controller: Controller,
  pub status: Status,
  pub mask: Mask,
  pub oam_address: u8,
  v: u16,
  t: u16,
  x: u8,
  latch: bool
}

impl Registers {
  pub fn new() -> Self {
    Registers {
      controller: Controller::new(0x0),
      status: Status::new(0x0),
      mask: Mask::new(0x0),
      oam_address: 0x0,
      v: 0x0,
      t: 0x0,
      x: 0x0,
      latch: true,
    }
  }

  pub fn write_oam_addr(&mut self, data: u8) {
    self.oam_address = data;
  }

  pub fn increment_oam_addr(&mut self) {
    self.oam_address = self.oam_address.wrapping_add(1);
  }

  pub fn write_address(&mut self, data: u8) {
    if self.latch {
      // t: .CDEFGH ........ <- data: ..CDEFGH
      // t: Z...... ........ <- 0 (bit Z is cleared)
      self.t = (self.t & 0x80FF) | (((data as u16) & 0x3F) << 8);
    } else {
      // t: ....... ABCDEFGH <- d: ABCDEFGH
      // v: <...all bits...> <- t: <...all bits...>
      self.t = (self.t & 0xFF00) | (data as u16);
      self.v = self.t;
    }

    self.mirror_address();
    self.latch = !self.latch;
  }

  pub fn write_scroll(&mut self, data: u8) {
    if self.latch {
      // t: ....... ...ABCDE <- d: ABCDE...
      // x:              FGH <- d: .....FGH
      self.t = (self.t & 0xFFE0) | ((data as u16) >> 3);
      self.x = data & 0x07;
    } else {
      // t: FGH..AB CDE..... <- d: ABCDEFGH
      self.t = (self.t & 0x8FFF) | (((data as u16) & 0x07) << 12);
      self.t = (self.t & 0xFC1F) | (((data as u16) & 0xF8) << 2);
    }

    self.latch = !self.latch;
  }

  pub fn write_controller(&mut self, data: u8) -> bool {
    let nmi_gen = self.controller.get_flag(ControllerFlag::NMIGen);
    self.controller.set(data);

    // t: ...GH.. ........ <- d: ......GH
    self.t = (self.t & 0xF3FF) | (((data as u16) & 0x03) << 10);

    !nmi_gen
      && self.controller.get_flag(ControllerFlag::NMIGen)
      && self.status.get_flag(StatusFlag::VBLankStarted)
  }

  pub fn read_address(&self) -> u16 {
    self.v
  }

  pub fn x(&self) -> u8 {
    self.x
  }

  pub fn increment_address(&mut self, inc: u8) {
    self.v = self.v.wrapping_add(inc as u16);
    self.mirror_address();
  }

  fn mirror_address(&mut self) {
    if self.v > System::PPU_END {
      self.v &= System::PPU_END;
    }
  }

  pub fn reset_latch(&mut self) {
    self.latch = true;
  }

  pub fn transfer_v(&mut self) {
    // v: GHIA.BC DEF..... <- t: GHIA.BC DEF.....
    self.v = (self.v & 0x841F) | (self.t & 0x7BE0);
  }

  pub fn increment_x(&mut self) {
    if self.v & 0x1F == 0x1F {                  // if coarse x == 31
      self.v &= !0x1F;                          // coarse x = 0
      self.v ^= 0x0400;                         // switch horizontal nametable
    } else {
      self.v += 1;                              // coarse x++
    }
  }

  pub fn increment_y(&mut self) {
    if self.v & 0x7000 != 0x7000 {              // if fine y < 7
      self.v += 0x1000                          // fine y++
    } else {
      self.v &= !0x7000;                        // fine y = 0
      let mut y = (self.v & 0x03E0) >> 5;  // y = coarse y

      if y == 0x1D {
        y = 0;                                  // coarse y = 0
        self.v ^= 0x0800;                       // switch vertical nametable
      } else if y == 0x1F {
        y = 0                                   // coarse y = 0, no switch
      } else {
        y += 1;                                 // coarse y++
      }
      self.v = (self.v & !0x03E0) | (y << 5);   // restore coarse y
    }
  }

  pub fn transfer_h(&mut self) {
    // v: ....A.. ...BCDEF <- t: ....A.. ...BCDEF
    self.v = (self.v & 0xFBE0) | (self.t & 0x041F);
  }
}
