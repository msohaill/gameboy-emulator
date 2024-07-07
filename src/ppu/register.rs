pub mod address;
pub mod controller;
pub mod mask;
pub mod scroll;
pub mod status;

use address::Address;
use controller::Controller;
use mask::Mask;
use scroll::Scroll;
use status::Status;

pub struct Registers {
  pub controller: Controller,
  pub status: Status,
  pub mask: Mask,
  pub oam_address: u8,
  pub scroll: Scroll,
  pub address: Address,
}

impl Registers {
  pub fn new() -> Self {
    Registers {
      controller: Controller::new(0x0),
      status: Status::new(0x0),
      mask: Mask::new(0x0),
      oam_address: 0x0,
      scroll: Scroll::new(),
      address: Address::new(),
    }
  }

  pub fn write_oam_addr(&mut self, data: u8) {
    self.oam_address = data;
  }
}
