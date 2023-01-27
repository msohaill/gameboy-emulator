#[derive(Clone, Copy)]
pub enum Mirroring {
  Vertical, Horizontal, FourScreen
}

pub struct Cartridge {
  pub prg: Vec<u8>,
  pub chr: Vec<u8>,
  pub mapper: u8,
  pub mirroring: Mirroring,
}

impl Cartridge {
  const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

  pub fn new(raw: &Vec<u8>) -> Result<Cartridge, &'static str> {
    if &raw[0..4] != Cartridge::NES_TAG {
      return Err("File not in iNES format.");
    } else if (raw[7] >> 2) & 0b11 != 0 {
      return Err("NES 2.0 format not supported.");
    }

    let prg_size = raw[4] as usize * 0x4000;
    let chr_size = raw[5] as usize * 0x2000;

    let prg_start = 16 + if raw[6] >> 2 != 0 { 512 } else { 0 };
    let chr_start = prg_start + prg_size;

    Ok(Cartridge {
      prg: raw[prg_start..(prg_start + prg_size)].to_vec(),
      chr: raw[chr_start..(chr_start + chr_size)].to_vec(),
      mapper: (raw[7] & 0b11110000) | (raw[6] >> 4),
      mirroring: match (raw[6] >> 3 & 1 != 0, raw[6] & 1 != 0) {
        (true, _) => Mirroring::FourScreen,
        (false, true) => Mirroring::Vertical,
        (false, false) => Mirroring::Horizontal,
      }
    })
  }
}