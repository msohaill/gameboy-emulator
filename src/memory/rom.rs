pub enum Mirroring {
  Vertical, Horizontal, FourScreen
}

pub struct ROM {
  pub prg: Vec<u8>,
  pub chr: Vec<u8>,
  pub mapper: u8,
  pub mirroring: Mirroring,
}

impl ROM {
  const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

  pub fn new(rom: &Vec<u8>) -> Result<ROM, &'static str> {
    if &rom[0..4] != ROM::NES_TAG {
      return Err("File not in iNES format.");
    } else if (rom[7] >> 2) & 0b11 != 0 {
      return Err("NES 2.0 format not supported.");
    }

    let prg_size = rom[4] as usize * 0x4000;
    let chr_size = rom[5] as usize * 0x2000;

    let prg_start = 16 + if rom[6] >> 2 != 0 { 512 } else { 0 };
    let chr_start = prg_start + prg_size;

    Ok(ROM {
      prg: rom[prg_start..(prg_start + prg_size)].to_vec(),
      chr: rom[chr_start..(chr_start + chr_size)].to_vec(),
      mapper: (rom[7] & 0b11110000) | (rom[6] >> 4),
      mirroring: match (rom[6] >> 3 & 1 != 0, rom[6] & 1 != 0) {
        (true, _) => Mirroring::FourScreen,
        (false, true) => Mirroring::Vertical,
        (false, false) => Mirroring::Horizontal,
      }
    })
  }
}