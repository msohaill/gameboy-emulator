use super::{banks::Banks, Mapper, MapperEvent, Mirroring};

const PRG_BANK_SIZE: usize = 0x2000;
const CHR_BANK_SIZE: usize = 0x400;

pub struct Mapper4 {
  mirroring: Mirroring,

  chr: Banks,
  prg_ram: Banks,
  prg_rom: Banks,

  select: u8,
  registers: [u8; 0x08],

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
      0x0000 ..= 0x1FFF => self.chr.read(addr),
      0x6000 ..= 0x7FFF => self.prg_ram.read(addr),
      0x8000 ..= 0xFFFF => self.prg_rom.read(addr),
      _ => 0,
    }
  }

  fn write(&mut self, addr: u16, val: u8) {
    match addr {
      0x0000 ..= 0x1FFF => self.chr.write(addr, val),
      0x6000 ..= 0x7FFF => self.prg_ram.write(addr, val),
      0x8000 ..= 0xFFFF => self.write_registers(addr, val),
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

    let mut mapper = Mapper4 {
      mirroring,
      chr: Banks::new(0x0000, 0x1FFFF, CHR_BANK_SIZE, if chr { chr_rom } else { vec![0; 0x2000] }, !chr),
      prg_ram: Banks::new(0x6000, 0x7FFF, PRG_BANK_SIZE, vec![0; 0x2000], true),
      prg_rom: Banks::new(0x8000, 0xFFFF, PRG_BANK_SIZE, prg_rom, false),

      select: 0x0,
      registers: [0; 0x08],

      irq: IRQ::new(),
      last: false,
    };

    mapper.prg_rom.set(2, mapper.prg_rom.last() - 1);
    mapper.prg_rom.set(3, mapper.prg_rom.last());
    mapper
  }

  fn update_banks(&mut self) {
    if self.select & 0x40 == 0x0 {
      self.prg_rom.set(0, self.registers[6] as usize);
      self.prg_rom.set(2, self.prg_rom.last() - 1);
    } else {
      self.prg_rom.set(0, self.prg_rom.last() - 1);
      self.prg_rom.set(2, self.registers[6] as usize);
    }
    self.prg_rom.set(1, self.registers[7] as usize);
    self.prg_rom.set(3, self.prg_rom.last());

    if self.select & 0x80 == 0x0 {
      self.chr.set_range(0, 1, self.registers[0] as usize & 0xFE);
      self.chr.set_range(2, 3, self.registers[1] as usize & 0xFE);
      self.chr.set(4, self.registers[2] as usize);
      self.chr.set(5, self.registers[3] as usize);
      self.chr.set(6, self.registers[4] as usize);
      self.chr.set(7, self.registers[5] as usize);
    } else {
      self.chr.set(0, self.registers[2] as usize);
      self.chr.set(1, self.registers[3] as usize);
      self.chr.set(2, self.registers[4] as usize);
      self.chr.set(3, self.registers[5] as usize);
      self.chr.set_range(4, 5, self.registers[0] as usize & 0xFE);
      self.chr.set_range(6, 7, self.registers[1] as usize & 0xFE);
    }
  }

  fn write_registers(&mut self, addr: u16, val: u8) {
    match addr {
      0x8000 ..= 0x9FFF if addr % 2 == 0 => self.bank_select(val),
      0x8000 ..= 0x9FFF if addr % 2 != 0 => self.bank_data(val),
      0xA000 ..= 0xBFFF if addr % 2 == 0 => self.set_mirroring(val),
      0xA000 ..= 0xBFFF if addr % 2 != 0 => { /* PRG RAM protect ?! */ },
      0xC000 ..= 0xDFFF if addr % 2 == 0 => self.irq_latch(val),
      0xC000 ..= 0xDFFF if addr % 2 != 0 => self.irq_reload(),
      0xE000 ..= 0xFFFF if addr % 2 == 0 => self.irq_disable(),
      0xE000 ..= 0xFFFF if addr % 2 != 0 => self.irq_enable(),
      _ => unreachable!("Should not happen!"),
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

  fn bank_select(&mut self, val: u8) {
    self.select = val;
    self.update_banks();
  }

  fn bank_data(&mut self, val: u8) {
    self.registers[self.select as usize & 0x07] = val;
    self.update_banks();
  }

  fn set_mirroring(&mut self, val: u8) {
    match self.mirroring {
      Mirroring::FourScreen =>  { }
      _ => {
        self.mirroring = match val & 0x01 == 0x01  {
          false => Mirroring::Vertical,
          true => Mirroring::Horizontal,
        };
        self.update_banks();
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
