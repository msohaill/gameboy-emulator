crate::utils::bitflag!(pub JoypadStatus,
  A,
  B,
  Select,
  Start,
  Up,
  Down,
  Left,
  Right,
);

pub struct Joypad {
  strobe: bool,
  index: u8,
  pub buttons: JoypadStatus,
}

impl Joypad {
  pub fn new() -> Self {
    Joypad {
      strobe: false,
      index: 0,
      buttons: JoypadStatus::new(0),
    }
  }

  pub fn read(&mut self) -> u8 {
    if self.index > 7 {
      return 1;
    }

    let res = (self.buttons.get() & (1 << self.index)) >> self.index;
    self.index += !self.strobe as u8;

    res | 0x40
  }

  pub fn write(&mut self, data: u8) {
    self.strobe = data & 1 == 1;

    if self.strobe {
      self.index = 0;
    }
  }

  pub fn push(&mut self, button: Flag) {
    self.buttons.set_flag(button);
  }

  pub fn release(&mut self, button: Flag) {
    self.buttons.unset_flag(button);
  }
}
