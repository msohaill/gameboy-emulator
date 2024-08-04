pub mod color;
pub mod frame;
pub mod palette;
pub mod register;
pub mod state;

use crate::system::mapper::Mapper;
use crate::system::System;

use frame::Frame;
use palette::PALETTE;
use register::Registers;
use register::controller::Flag as ControllerFlag;
use register::mask::Flag as MaskFlag;
use register::status::Flag as StatusFlag;
use state::{RenderState, SpriteState, State};

pub struct PPU {
  pub palette: [u8; 0x20],
  pub vram: [u8; 0x800],
  pub oam: [u8; 0x100],
  pub mapper: Box<dyn Mapper>,
  pub frame: Frame,
  pub registers: Registers,
  nmi: NMI,
  state: State,
  scan: RenderState,
  sprites: SpriteState,
}

pub struct NMI {
  pending: bool,
  delay: u8,
  prev: bool,
}

impl NMI {
  fn new() -> Self {
    NMI {
      pending: false,
      delay: 0,
      prev: false,
    }
  }

  // @fogleman hack
  fn change(&mut self, occurred: bool, enabled: bool) {
    let curr = enabled && occurred;
    if curr && !self.prev {
      self.delay = 15;
    }
    self.prev = curr;
  }

  pub fn poll(&mut self) -> bool {
    let res = self.pending;
    self.pending = false;
    res
  }

  fn tick(&mut self) {
    if self.delay > 0 {
      self.delay -= 1;

      if self.delay == 0 {
          self.pending = true;
      }
  }
  }
}

impl PPU {
  const TOTAL_SCANLINES: u16 = 262;
  const VISIBLE_SCANLINES: u16 = 241;
  const SCANLINE_DURATION: usize = 341;

  pub fn new(mapper: Box<dyn Mapper>) -> Self {
    PPU {
      mapper,
      palette: [0; 0x20],
      vram: [0; 0x800],
      oam: [0; 0x100],
      frame: Frame::new(),
      registers: Registers::new(),
      nmi: NMI::new(),
      state: State::new(),
      scan: RenderState::new(),
      sprites: SpriteState::new(),
    }
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
        panic!("Illegal access at write-only PPU register: {:#0X}", addr)
      }
      0x2002 => self.read_status(),
      0x2004 => self.read_oam_data(),
      0x2007 => self.read_data(),
      0x2008..=System::PPU_END => self.read(addr & 0x2007),
      _ => panic!("Illegal PPU read access: {:#0X}", addr),
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      0x2000 => {
        let occurred = self.registers.write_controller(data);
        self.nmi(occurred);
      }
      0x2001 => self.registers.mask.set(data),
      0x2002 => panic!("Illegal write to PPU status register."),
      0x2003 => self.registers.write_oam_addr(data),
      0x2004 => self.write_oam_data(data),
      0x2005 => self.registers.write_scroll(data),
      0x2006 => self.registers.write_address(data),
      0x2007 => self.write_data(data),
      0x2008..=System::PPU_END => self.write(addr & 0x2007, data),
      _ => panic!("Illegal PPU write access: {:#0X}", addr),
    }
  }

  fn clock_tick(&mut self) {
    self.nmi.tick();

    if self.rendering_enabled()
      && self.state.odd && self.scan.line == PPU::TOTAL_SCANLINES - 1
      && self.scan.dot == PPU::SCANLINE_DURATION - 2 {
        self.scan.line = 0;
        self.scan.dot = 0;
        self.state.odd = false;
        return;
    }

    self.scan.dot += 1;

    if self.scan.dot == PPU::SCANLINE_DURATION {
      self.scan.dot = 0;
      self.scan.line += 1;

      if self.scan.line == PPU::TOTAL_SCANLINES {
        self.scan.line = 0;
        self.state.odd = !self.state.odd;
      }
    }
  }

  pub fn tick(&mut self) -> bool {
    self.clock_tick();

    let prerender = self.scan.line == PPU::TOTAL_SCANLINES - 1;
    let visible   = self.scan.line < PPU::VISIBLE_SCANLINES - 1;
    let render    = prerender || visible;

    let prefetch  = self.scan.dot >= 321 && self.scan.dot <= 336;
    let render_cycle    = self.scan.dot > 0 && self.scan.dot <= Frame::WIDTH;
    let fetch           = prefetch || render_cycle;

    if self.rendering_enabled() {
      if visible && render_cycle {
        self.render_pixel()
      }

      if render && fetch {
        self.state.tile <<= 4;

        match self.scan.dot % 8 {
          0 => self.store_tile_state(),
          1 => self.state.nametable = self.fetch_nametable(),
          3 => self.state.attrtable = self.fetch_attrtable(),
          5 => self.state.lotile = self.fetch_lotile(),
          7 => self.state.hitile = self.fetch_hitile(),
          _ => { } // Nothing to do on even/other cycles
        }
      }

      if prerender && self.scan.dot >= 280 && self.scan.dot <= 304 {
        self.registers.transfer_v();
      }

      if render {
        if fetch && self.scan.dot % 8 == 0 {
          self.registers.increment_x();
        }

        if self.scan.dot == Frame::WIDTH {
          self.registers.increment_y();
        }

        if self.scan.dot == Frame::WIDTH + 1 {
          self.registers.transfer_h();
        }
      }
    }

    if self.rendering_enabled() && self.scan.dot == Frame::WIDTH + 1 {
      if visible {
        self.evaluate_sprites();
      } else {
        self.sprites.count = 0;
      }
    }

    if self.scan.line == PPU::VISIBLE_SCANLINES && self.scan.dot == 1 {
      self.registers.status.set_flag(StatusFlag::VBLankStarted);

        self.nmi(true);
      return true;
    }

    if prerender && self.scan.dot == 1 {
      self.nmi(false);
      self.registers.status.unset_flag(StatusFlag::SpriteZeroHit);
      self.registers.status.unset_flag(StatusFlag::SpriteOverflow);
      self.registers.status.unset_flag(StatusFlag::VBLankStarted);
    }

    return false;
  }

  fn read_data(&mut self) -> u8 {
    let addr = self.registers.read_address();
    self
      .registers
      .increment_address(self.registers.controller.vram_increment());

    match addr {
      0x0000..=0x1FFF => self.mapper_read(addr),
      0x2000..=0x3EFF => self.vram_read(addr),
      0x3F00..=0x3FFF => self.palette_read(addr),
      _ => panic!("Unexpected read at {:#0x}", addr),
    }
  }

  fn write_data(&mut self, data: u8) {
    let addr = self.registers.read_address();
    self
      .registers
      .increment_address(self.registers.controller.vram_increment());

    match addr {
      0x0000..=0x1FFF => self.mapper.write(addr, data),
      0x2000..=0x3EFF => self.vram_write(addr, data),
      0x3F00..=0x3FFF => self.palette_write(addr, data),
      _ => panic!("Unexpected write at {:#0x}", addr),
    };
  }

  fn mapper_read(&mut self, addr: u16) -> u8 {
    let res = self.state.buffer;
    self.state.buffer = self.mapper.read(addr);
    res
  }

  fn vram_read(&mut self, addr: u16) -> u8 {
    let res = self.state.buffer;
    self.state.buffer = self.vram[self.mirror_vram(addr) as usize];
    res
  }

  fn vram_write(&mut self, addr: u16, data: u8) {
    self.vram[self.mirror_vram(addr) as usize] = data;
  }

  fn palette_read(&self, addr: u16) -> u8 {
    let addr = addr % 0x20;
    let idx = match addr {
      0x10 | 0x14 | 0x18 | 0x1C => addr as usize - 0x10,
      _ => addr as usize,
    };
    self.palette[idx]
  }

  fn palette_write(&mut self, addr: u16, data: u8) {
    let addr = addr % 0x20;
    let idx = match addr {
      0x10 | 0x14 | 0x18 | 0x1C => addr as usize - 0x10,
      _ => addr as usize,
    };
    self.palette[idx] = data;
  }

  fn mirror_vram(&self, addr: u16) -> u16 {
    let addr = (addr - 0x2000) % 0x1000;
    let table = addr / 0x400;
    let offset = addr % 0x400;

    let index = 0x2000
      + self.mapper.as_ref().mirroring().coeff()[table as usize] * 0x400
      + offset;

    index % 0x800
  }

  fn read_oam_data(&self) -> u8 {
    self.oam[self.registers.oam_address as usize]
  }

  fn write_oam_data(&mut self, data: u8) {
    self.oam[self.registers.oam_address as usize] = match self.registers.oam_address % 0x04 {
      0x02 => data & 0xE3, // Byte 2, unimplemented bits
      _ => data,
    };
    self.registers.increment_oam_addr();
  }

  fn read_status(&mut self) -> u8 {
    let res = self.registers.status.get();
    self.nmi(false);
    self.registers.status.unset_flag(StatusFlag::VBLankStarted);
    self.registers.reset_latch();

    res
  }

  fn rendering_enabled(&self) -> bool {
    self.registers.mask.get_flag(MaskFlag::ShowBackground) || self.registers.mask.get_flag(MaskFlag::ShowSprites)
  }

  fn render_pixel(&mut self) {
    let x = self.scan.dot - 1;
    let y = self.scan.line as usize;

    let mut background = self.background_pixel();
    let (sprite_idx, mut sprite) = self.sprite_pixel();

    if x < 8 {
      (!self.registers.mask.get_flag(MaskFlag::ShowLeftBackground)).then(|| background = 0);
      (!self.registers.mask.get_flag(MaskFlag::ShowLeftSprites)).then(|| sprite = 0);
    }

    let b = background % 4 != 0;
    let s = sprite % 4 != 0;

    let lo = match (b, s) {
      (false, false) => 0,
      (false, true) => (sprite as u16) | 0x10,
      (true, false) => background as u16,
      (true, true) => {
        if self.sprites.indices[sprite_idx] == 0 && x < Frame::WIDTH - 1 {
          self.registers.status.set_flag(StatusFlag::SpriteZeroHit);
        }

        if self.sprites.priorities[sprite_idx] == 0 {
          (sprite as u16) | 0x10
        } else {
          background as u16
        }
      }
    };

    let address = 0x3F00 | lo;
    let color_idx = self.palette_read(address) % 0x40;
    let color = PALETTE[color_idx as usize];
    self.frame.set_pixel(x, y, color);
  }

  fn background_pixel(&self) -> u8 {
    self.registers.mask.get_flag(MaskFlag::ShowBackground).then(|| {
      let tile = ((self.state.tile >> 32) as u32) >> ((7 - self.registers.x()) * 4);
      let color = (tile & 0x0F) as u8;
      color
    }).unwrap_or(0)
  }

  fn sprite_pixel(&self) -> (usize, u8) {
    self.registers.mask.get_flag(MaskFlag::ShowSprites).then(|| {
      for i in 0..self.sprites.count {
        let mut offset = (self.scan.dot as i16 - 1) - self.sprites.positions[i] as i16;

        if offset < 0 || offset > 7 {
          continue;
        }

        offset = 7 - offset;

        let color = ((self.sprites.patterns[i] >> (offset * 4)) & 0x0F) as u8;

        if color % 4 == 0 {
          continue;
        }

        return (i, color);
      }
      return (0, 0);
    }).unwrap_or((0, 0))
  }

  fn store_tile_state(&mut self) {
    let data: u32 = (0 .. 8).fold(0, |acc, _| {
        let p1 = (self.state.lotile & 0x80) >> 7;
        let p2 = (self.state.hitile & 0x80) >> 6;

        self.state.lotile <<= 1;
        self.state.hitile <<= 1;

        let a = self.state.attrtable;
        let b = (a | p1 | p2) as u32;

        (acc << 4) | b
    } );

    self.state.tile |= data as u64;
  }

  fn fetch_nametable(&self) -> u8 {
    let v = self.registers.read_address();
    let address = 0x2000 | (v & 0x0FFF);
    self.vram[self.mirror_vram(address) as usize]
  }

  fn fetch_attrtable(&self) -> u8 {
    let v = self.registers.read_address();
    let address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x0038) | ((v >> 2) & 0x0007);
    let byte = self.vram[self.mirror_vram(address) as usize];
    let shift = ((v >> 4) & 0x04) | (v & 0x02);
    ((byte >> shift) & 0x03) << 2
  }

  fn fetch_lotile(&self) -> u8 {
    let y = (self.registers.read_address() >> 12) & 0x07;
    let tile = self.state.nametable as u16;
    let address = self.registers.controller.background_pattern_table() + y + (16 * tile);
    self.mapper.read(address)
  }

  fn fetch_hitile(&self) -> u8 {
    let y = (self.registers.read_address() >> 12) & 0x07;
    let tile = self.state.nametable as u16;
    let address = self.registers.controller.background_pattern_table() + y + (16 * tile) + 8;
    self.mapper.read(address)
  }

  fn evaluate_sprites(&mut self) {
    let size = self.registers.controller.sprite_size();
    let mut count = 0;

    for sprite_idx in 0 .. 64 {
      let y = self.oam[(sprite_idx * 4 + 0) as usize % 0x100];
      let a = self.oam[(sprite_idx * 4 + 2) as usize % 0x100];
      let x = self.oam[(sprite_idx * 4 + 3) as usize % 0x100];

      let row = (self.scan.line as i16) - (y as i16);

      if row < 0 || row >= (size as i16) {
        continue;
      }

      if count < 8 {
        self.sprites.patterns[count]    = self.fetch_sprite_patterns(sprite_idx, row);
        self.sprites.positions[count]   = x;
        self.sprites.priorities[count]  = (a >> 5) & 0x01;
        self.sprites.indices[count]     = sprite_idx as usize;
      }

      count += 1;
    }

    if count > 8 {
      count = 8;
      self.registers.status.set_flag(StatusFlag::SpriteOverflow);
    }

    self.sprites.count = count;
  }

  fn fetch_sprite_patterns(&mut self, i: u16, row: i16) -> u32 {
    let mut tile = self.oam[(i * 4 + 1) as usize % 0x100] as u16;
    let attrs = self.oam[(i * 4 + 2) as usize % 0x100];

    let mut row = row;
    let address = if self.registers.controller.sprite_size() == 8 {
      if attrs & 0x80 == 0x80 {
        row = 7 - row;
      }

      self.registers.controller.sprite_pattern_table() + (tile * 16) + row as u16
    } else {
      if attrs & 0x80 == 0x80 {
        row = 15 - row;
      }

      let table = tile & 1;
      tile &= 0xFE;

      if row > 7 {
        tile += 1;
        row -= 8;
      }

      0x1000 * table + (tile * 16) + row as u16
    };

    let a = ((attrs & 0x03) << 2) as u32;
    let mut lo = self.mapper.read(address) as u32;
    let mut hi = self.mapper.read(address + 8) as u32;

    (0 .. 8).fold(0, |acc, _| {
      let p1;
      let p2;

      // Flip the sprite vertically, so we read from the other end of the
      // low and high tile bytes
      if attrs & 0x40 == 0x40 {
        p1 = (lo & 1) << 0;
        p2 = (hi & 1) << 1;
        lo >>= 1;
        hi >>= 1;
      } else {
        p1 = (lo & 0x80) >> 7;
        p2 = (hi & 0x80) >> 6;
        lo <<= 1;
        hi <<= 1;
      }

      (acc << 4) | (a | p1 | p2)
    })
  }

  fn nmi(&mut self, occurred: bool) {
    self.nmi.change(occurred, self.registers.controller.get_flag(ControllerFlag::NMIGen));
  }

  pub fn poll(&mut self) -> bool {
    self.nmi.poll()
  }
}
