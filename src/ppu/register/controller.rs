crate::utils::bitflag!(pub Controller,
  NameLo,
  NameHi,
  VRAMInc,
  SpriteAddr,
  BackgroundAddr,
  SpriteSize,
  MasterSlave,
  NMIGen,
);


impl Controller {
  pub fn vram_increment(&self) -> u8 {
    if self.get_flag(Flag::VRAMInc) {
      0x20
    } else {
      0x01
    }
  }

  pub fn name_table(&self) -> u16 {
    match (self.get_flag(Flag::NameHi), self.get_flag(Flag::NameLo)) {
      (false, false) => 0x2000,
      (false, true) => 0x2400,
      (true, false) => 0x2800,
      (true, true) => 0x2C00,
    }
  }

  pub fn background_pattern_table(&self) -> u16 {
    if self.get_flag(Flag::BackgroundAddr) {
      0x1000
    } else {
      0x0000
    }
  }

  pub fn sprite_pattern_table(&self) -> u16 {
    if self.get_flag(Flag::SpriteAddr) {
      0x1000
    } else {
      0x0000
    }
  }
}
