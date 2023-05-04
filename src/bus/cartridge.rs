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
    let header = &raw[0..16];
    let flags_6 = header[6];
    let flags_7 = header[7];

    if header[0..4] != Cartridge::NES_TAG {
      return Err("File not in iNES format.");
    } else if flags_7 & 0x0C == 0x08 {
      return Err("NES 2.0 format not supported (yet).");
    }

    let header_bytes = 16;
    let trainer_bytes = if flags_6 & 0x04 == 0x04 { 512 } else { 0 };
    let prg_bytes = header[4] as usize * 0x4000;
    let chr_bytes = header[5] as usize * 0x2000;

    let prg_start = header_bytes + trainer_bytes;
    let chr_start = header_bytes + trainer_bytes + prg_bytes;

    Ok(Cartridge {
      prg: raw[prg_start..(prg_start + prg_bytes)].to_vec(),
      chr: raw[chr_start..(chr_start + chr_bytes)].to_vec(),
      mapper: (flags_7 & 0xF0) | (flags_6 >> 4),
      mirroring: match (flags_6 & 0x08 == 0x08, flags_6 & 0x01 == 0x01) {
        (true, _) => Mirroring::FourScreen,
        (false, true) => Mirroring::Vertical,
        (false, false) => Mirroring::Horizontal,
      }
    })
  }
}
