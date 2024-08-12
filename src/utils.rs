macro_rules! bitflag {
  ($mod:vis $name:ident,
    $zero:ident, $one:ident, $two:ident, $three:ident,
    $four:ident, $five:ident, $six:ident, $seven:ident$(,)?
  ) => {
    $mod struct $name {
      value: u8,
    }

    impl $name {
      pub fn new(value: u8) -> Self {
        $name {
          value,
        }
      }

      #[allow(unused)]
      pub fn get(&self) -> u8 {
        self.value
      }

      #[allow(unused)]
      pub fn set(&mut self, data: u8) {
        self.value = data;
      }

      #[allow(unused)]
      pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
          Flag::$zero => self.value & 0b1 != 0,
          Flag::$one => (self.value >> 1) & 0b1 != 0,
          Flag::$two => (self.value >> 2) & 0b1 != 0,
          Flag::$three => (self.value >> 3) & 0b1 != 0,
          Flag::$four => (self.value >> 4) & 0b1 != 0,
          Flag::$five => (self.value >> 5) & 0b1 != 0,
          Flag::$six => (self.value >> 6) & 0b1 != 0,
          Flag::$seven => (self.value >> 7) & 0b1 != 0
          }
      }

      #[allow(unused)]
      pub fn change_flag(&mut self, flag: Flag, val: bool) {
        match flag {
          Flag::$zero =>
            self.value = if val { 1 | self.value } else { !(1) & self.value },
          Flag::$one =>
            self.value = if val { (1 << 1) | self.value } else { !(1 << 1) & self.value },
          Flag::$two =>
            self.value = if val { (1 << 2) | self.value } else { !(1 << 2) & self.value },
          Flag::$three =>
            self.value = if val { (1 << 3) | self.value } else { !(1 << 3) & self.value },
          Flag::$four =>
            self.value = if val { (1 << 4) | self.value } else { !(1 << 4) & self.value },
          Flag::$five =>
            self.value = if val { (1 << 5) | self.value } else { !(1 << 5) & self.value },
          Flag::$six =>
            self.value = if val { (1 << 6) | self.value } else { !(1 << 6) & self.value },
          Flag::$seven =>
            self.value = if val { (1 << 7) | self.value } else { !(1 << 7) & self.value },
        }
      }

      #[allow(unused)]
      pub fn set_flag(&mut self, flag: Flag) {
        self.change_flag(flag, true);
      }

      #[allow(unused)]
      pub fn unset_flag(&mut self, flag: Flag) {
        self.change_flag(flag, false);
      }
    }

    #[derive(Copy, Clone)]
    pub enum Flag {
      #[allow(unused)]
      $zero,
      #[allow(unused)]
      $one,
      #[allow(unused)]
      $two,
      #[allow(unused)]
      $three,
      #[allow(unused)]
      $four,
      #[allow(unused)]
      $five,
      #[allow(unused)]
      $six,
      #[allow(unused)]
      $seven,
    }
  };
}

pub(crate) use bitflag;
