
pub struct State {
  pub buffer: u8,
  pub odd: bool,
  pub sprites: usize,
  pub tile: u64,

  // Cached bytes
  pub attrtable: u8,
  pub hitile: u8,
  pub lotile: u8,
  pub nametable: u8,
}

pub struct RenderState {
  pub line: u16,
  pub dot: usize,
}

pub struct SpriteState {
  pub count: usize,
  pub indices:    [usize; 0x08],
  pub patterns:   [u32; 0x08],
  pub positions:  [u8; 0x08],
  pub priorities: [u8; 0x08],
}

impl State {
  pub fn new() -> Self {
    State {
      buffer: 0x0,
      odd: false,
      sprites: 0,
      tile: 0x0,
      attrtable: 0x0,
      hitile: 0x0,
      lotile: 0x0,
      nametable: 0x0,
    }
  }
}

impl RenderState {
  pub fn new() -> Self {
    RenderState {
      line: 0,
      dot: 0,
    }
  }
}

impl SpriteState {
  pub fn new() -> Self {
    SpriteState {
      count: 0,
      indices:    [0; 0x08],
      patterns:   [0; 0x08],
      positions:  [0; 0x08],
      priorities: [0; 0x08],
    }
  }
}
