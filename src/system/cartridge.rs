use super::mapper::{Mapper, Mirroring, from};


pub struct Cartridge {
  pub mapper: Box<dyn Mapper>,
}

impl Cartridge {
  const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

  pub fn new(rom: Vec<u8>) -> Result<Cartridge, &'static str> {
    let header = &rom[0..16];
    let flags_6 = header[6];
    let flags_7 = header[7];

    if header[0..4] != Cartridge::NES_TAG {
      return Err("File not in iNES format.");
    } else if flags_7 & 0x0C == 0x08 {
      println!("NES 2.0 format not supported (yet).");
    }

    let header_bytes = 16;
    let trainer_bytes = if flags_6 & 0x04 == 0x04 { 512 } else { 0 };
    let prg_bytes = header[4] as usize * 0x4000;
    let chr_bytes = header[5] as usize * 0x2000;

    let prg_start = header_bytes + trainer_bytes;
    let chr_start = header_bytes + trainer_bytes + prg_bytes;

    Ok(Cartridge {
      mapper: from(
        (flags_7 & 0xF0) | (flags_6 >> 4),
        rom[chr_start..(chr_start + chr_bytes)].to_vec(),
        rom[prg_start..(prg_start + prg_bytes)].to_vec(),
        match (flags_6 & 0x08 == 0x08, flags_6 & 0x01 == 0x01) {
          (true, _) => Mirroring::FourScreen,
          (false, true) => Mirroring::Vertical,
          (false, false) => Mirroring::Horizontal,
        }
      ),
    })
  }
}
