#[derive(PartialEq, Eq)]
pub struct Interrupt {
  pub read_address: u16,
  pub mask: u8,
  pub cycles: u8,
}

impl Interrupt {
  pub const NMI: Interrupt = Interrupt {
    read_address: 0xFFFA,
    mask: 0b00100000,
    cycles: 7,
  };

  pub const BRK: Interrupt = Interrupt {
    read_address: 0xFFFE,
    mask: 0b00110000,
    cycles: 0,
  };

  pub const IRQ: Interrupt = Interrupt {
    read_address: 0xFFFE,
    mask: 0b00100000,
    cycles: 7,
  };
}
