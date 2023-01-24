use crate::cpu::CPU;
use crate::cpu::instruction::{Addressing, Instruction};
use crate::cpu::register::Register;

pub fn trace(cpu: &mut CPU) -> String {
  let start = cpu.registers.get_pc();
  let code = cpu.memory.read(start);

  let instruction = Instruction::get(code);

  let mut hex_dump = vec![];
  hex_dump.push(code);

  let (memory_addr, stored_value) = match instruction.mode {
    Addressing::Immediate | Addressing::Implied => (0, 0),
    _ => {
      cpu.registers.set_pc(cpu.registers.get_pc().wrapping_add(1));
      let addr = cpu.get_operand_addr(instruction.mode);
      cpu.registers.set_pc(cpu.registers.get_pc().wrapping_sub(1));
      (addr, cpu.memory.read(addr))
    },
  };

  let temp = match instruction.len {
    1 => match code {
      0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
      _ => String::from(""),
    },
    2 => {
      let addr: u8 = cpu.memory.read(start + 1);
      hex_dump.push(addr);

      match instruction.mode {
        Addressing::Immediate => match code {
          0xD0 | 0x70 | 0x50 | 0x30 | 0xF0 | 0xB0 | 0x90 | 0x10 => {
            let address: usize = (start as usize + 2).wrapping_add((addr as i8) as usize);
            format!("${:04x}", address)
          }
         _ => format!("#${:02x}", addr)
        }
        Addressing::ZeroPage => format!("${:02x} = {:02x}", memory_addr, stored_value),
        Addressing::ZeroPageX => format!("${:02x},X @ {:02x} = {:02x}", addr, memory_addr, stored_value),
        Addressing::ZeroPageY => format!("${:02x},Y @ {:02x} = {:02x}", addr, memory_addr, stored_value),
        Addressing::IndirectX => format!("(${:02x},X) @ {:02x} = {:04x} = {:02x}", addr, (addr.wrapping_add(cpu.registers.get(Register::X))), memory_addr, stored_value),
        Addressing::IndirectY => format!("(${:02x}),Y = {:04x} @ {:04x} = {:02x}", addr, (memory_addr.wrapping_sub(cpu.registers.get(Register::Y) as u16)), memory_addr, stored_value),
        _ => panic!("unexpected addressing mode"),
      }
    }
    3 => {
      let address_lo = cpu.memory.read(start + 1);
      let address_hi = cpu.memory.read(start + 2);
      hex_dump.push(address_lo);
      hex_dump.push(address_hi);

      let address = cpu.memory.readu16(start + 1);

      match instruction.mode {
        Addressing::Absolute => {
          if code == 0x4C || code == 0x20 {
            format!("${:04x}", address)
          } else {
          format!("${:04x} = {:02x}", memory_addr, stored_value)
          }
      }
        Addressing::AbsoluteX => format!("${:04x},X @ {:04x} = {:02x}", address, memory_addr, stored_value),
        Addressing::AbsoluteY => format!("${:04x},Y @ {:04x} = {:02x}", address, memory_addr, stored_value),
        Addressing::AbsoluteIndirect => {
          let jmp_addr = if address & 0x00FF == 0x00FF {
            let lo = cpu.memory.read(address);
            let hi = cpu.memory.read(address & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
          } else {
            cpu.memory.readu16(address)
          };
          format!("(${:04x}) = {:04x}", address, jmp_addr)
        }
        _ => panic!("unexpected addressing mode."),
      }
    },
    _ => String::from(""),
  };

  let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8} {: >4} {}", start, hex_str, instruction.opcode.to_string().replace("_", "*").as_str(), temp)
        .trim()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.registers.get(Register::A), cpu.registers.get(Register::X), cpu.registers.get(Register::Y), cpu.registers.get(Register::P), cpu.registers.get(Register::SP),
    )
    .to_ascii_uppercase()
}