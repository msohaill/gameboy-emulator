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
      1u8
    } else {
      32u8
    }
  }
}
