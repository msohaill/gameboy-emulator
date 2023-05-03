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
      32u8
    } else {
      1u8
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
}
