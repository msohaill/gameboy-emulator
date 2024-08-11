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
    self.get_flag(Flag::VRAMInc).then_some(0x20).unwrap_or(0x01)
  }

  #[allow(unused)]
  pub fn name_table(&self) -> u16 {
    let offset = ((self.get_flag(Flag::NameHi) as u16) << 1) + self.get_flag(Flag::NameLo) as u16;
    0x2000 + 0x400 * offset
  }

  pub fn background_pattern_table(&self) -> u16 {
    self.get_flag(Flag::BackgroundAddr).then_some(0x1000).unwrap_or(0x0000)
  }

  pub fn sprite_pattern_table(&self) -> u16 {
    self.get_flag(Flag::SpriteAddr).then_some(0x1000).unwrap_or(0x0000)
  }

  pub fn sprite_size(&self) -> usize {
    self.get_flag(Flag::SpriteSize).then_some(16).unwrap_or(8)
  }
}
