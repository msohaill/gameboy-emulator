use super::{Mapper, MapperEvent, Mirroring};

const PRG_BANK_SIZE: usize = 0x2000;
const CHR_BANK_SIZE: usize = 0x400;

pub struct Mapper4 {
  mirroring: Mirroring,
  chr_rom: Vec<u8>,
  chr_ram: Vec<u8>,
  prg_rom: Vec<u8>,
  prg_ram: [u8; 0x2000],

  registers: [u8; 0x08],
  index: u8,

  chr: bool,
  prg: bool,

  irq: IRQ,

  last: bool,
}

struct IRQ {
  enabled: bool,
  pending: bool,
  reload: bool,
  latch: u8,
  counter: u8,
}

impl IRQ {
  fn new() -> Self {
    IRQ {
      enabled: false,
      pending: false,
      reload: false,
      latch: 0x0,
      counter: 0x0,
    }
  }
}

impl Mapper for Mapper4 {
  fn mirroring(&self) -> Mirroring {
    self.mirroring
  }

  fn read(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_read(addr),
      0x6000 ..= 0x7FFF => self.prg_ram_read(addr),
      0x8000 ..= 0xFFFF => self.prg_rom_read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr_write(addr, val),
      0x6000 ..= 0x7FFF => self.prg_ram_write(addr, val),
      0x8000 ..= 0x9FFF if addr % 2 == 0 => self.bank_select(val),
      0x8000 ..= 0x9FFF if addr % 2 != 0 => self.bank_data(val),
      0xA000 ..= 0xBFFF if addr % 2 == 0 => self.set_mirroring(val),
      0xA000 ..= 0xBFFF if addr % 2 != 0 => { /* PRG RAM protect ?! */ },
      0xC000 ..= 0xDFFF if addr % 2 == 0 => self.irq_latch(val),
      0xC000 ..= 0xDFFF if addr % 2 != 0 => self.irq_reload(),
      0xE000 ..= 0xFFFF if addr % 2 == 0 => self.irq_disable(),
      0xE000 ..= 0xFFFF if addr % 2 != 0 => self.irq_enable(),
      _ => { },
    }
  }

  fn notify(&mut self, event: MapperEvent) {
    match event {
      MapperEvent::HBlank => self.irq_tick(),
      MapperEvent::VRAMAddressChanged(addr) => {
        let next = (addr >> 12) & 0x01 != 0x0;

        if !self.last && next {
          self.irq_tick();
        }

        self.last = next;
      }
    }
  }

  fn poll(&self) -> bool {
      self.irq.pending
  }
}

impl Mapper4 {
  pub fn new(chr_rom: Vec<u8>, prg_rom: Vec<u8>, mirroring: Mirroring) -> Self {
    let chr = !chr_rom.is_empty();

    Mapper4 {
      mirroring,
      chr_rom,
      chr_ram: if chr { vec![] } else { vec![0; 0x2000] },
      prg_rom,
      prg_ram: [0; 0x2000],
      registers: [0; 0x08],
      index: 0x0,
      chr: false,
      prg: false,
      irq: IRQ::new(),
      last: false,
    }
  }

  fn irq_tick(&mut self) {
    if self.irq.counter == 0 || self.irq.reload {
      self.irq.counter = self.irq.latch;
    } else {
      self.irq.counter -= 1;
    }

    if self.irq.counter == 0 && self.irq.enabled {
      self.irq.pending = true;
    }

    self.irq.reload = false;
  }

  fn chr_read(&self, addr: u16) -> u8 {

    let bank = match addr {
      0x0000 ..= 0x03FF if self.chr => self.registers[2] as usize,
      0x0000 ..= 0x03FF if !self.chr => self.registers[0] as usize & 0xFE,

      0x0400 ..= 0x07FF if self.chr => self.registers[3] as usize,
      0x0400 ..= 0x07FF if !self.chr => self.registers[0] as usize | 0x01,

      0x0800 ..= 0x0BFF if self.chr => self.registers[4] as usize,
      0x0800 ..= 0x0BFF if !self.chr => self.registers[1] as usize & 0xFE,

      0x0C00 ..= 0x0FFF if self.chr => self.registers[5] as usize,
      0x0C00 ..= 0x0FFF if !self.chr => self.registers[1] as usize | 0x01,

      0x1000 ..= 0x13FF if self.chr => self.registers[0] as usize & 0xFE,
      0x1000 ..= 0x13FF if !self.chr => self.registers[2] as usize,

      0x1400 ..= 0x17FF if self.chr => self.registers[0] as usize | 0x01,
      0x1400 ..= 0x17FF if !self.chr => self.registers[3] as usize,

      0x1800 ..= 0x1BFF if self.chr => self.registers[1] as usize & 0xFE,
      0x1800 ..= 0x1BFF if !self.chr => self.registers[4] as usize,

      0x1C00 ..= 0x1FFF if self.chr => self.registers[1] as usize | 0x01,
      0x1C00 ..= 0x1FFF if !self.chr => self.registers[5] as usize,

      _ => unreachable!("Should never happen!"),
    };

    let index = (bank * CHR_BANK_SIZE) + (addr as usize % CHR_BANK_SIZE);
    let chr = if self.chr_rom.is_empty() { &self.chr_ram } else { &self.chr_rom };
    chr[index]
  }

  fn prg_ram_read(&self, addr: u16) -> u8 {
    let index = addr as usize - 0x6000;
    self.prg_ram[index]
  }

  fn prg_rom_read(&self, addr: u16) -> u8 {
    let bank = match addr {
      0x8000 ..= 0x9FFF if self.prg => (self.prg_rom.len() / PRG_BANK_SIZE) - 2,
      0x8000 ..= 0x9FFF if !self.prg => self.registers[6] as usize,
      0xA000 ..= 0xBFFF => self.registers[7] as usize,
      0xC000 ..= 0xDFFF if self.prg => self.registers[6] as usize,
      0xC000 ..= 0xDFFF if !self.prg => (self.prg_rom.len() / PRG_BANK_SIZE) - 2,
      0xE000 ..= 0xFFFF => (self.prg_rom.len() / PRG_BANK_SIZE) - 1,
      _ => unreachable!("Should never happen!"),
    };

    let index = (bank * PRG_BANK_SIZE) + (addr as usize % PRG_BANK_SIZE);
    self.prg_rom[index]
  }

  fn chr_write(&mut self, addr: u16, val: u8) {
    if !self.chr_ram.is_empty() {
      let index = addr as usize % self.chr_ram.len();
      self.chr_ram[index] = val;
    }
  }

  fn prg_ram_write(&mut self, addr: u16, val: u8) {
    let index = addr as usize - 0x6000;
    self.prg_ram[index] = val;
  }

  fn bank_select(&mut self, val: u8) {
    self.index = val & 0x07;
    self.prg = val & 0x40 != 0x0;
    self.chr = val & 0x80 != 0x0;
  }

  fn bank_data(&mut self, val: u8) {
    self.registers[self.index as usize] = val;
  }

  fn set_mirroring(&mut self, val: u8) {
    match self.mirroring {
      Mirroring::FourScreen =>  { }
      _ => self.mirroring = match val & 0x01 == 0x01  {
        false => Mirroring::Vertical,
        true => Mirroring::Horizontal,
      }
    }
  }

  fn irq_latch(&mut self, val: u8) {
    self.irq.latch = val;
  }

  fn irq_reload(&mut self) {
    self.irq.reload = true;
  }

  fn irq_disable(&mut self) {
    self.irq.pending = false;
    self.irq.enabled = false;
  }

  fn irq_enable(&mut self) {
    self.irq.enabled = true;
  }
}
