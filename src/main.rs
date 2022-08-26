pub mod cpu;
use cpu::{CPU, register::{RegisterType}};

fn main() {
    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x05, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::A) == 0x05);
    println!("{}", cpu.registers.get(RegisterType::P) & 0b0000_0010 == 0);
    println!("{}", cpu.registers.get(RegisterType::P) & 0b1000_0000 == 0);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x00, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::P) & 0b0000_0010 == 0b10);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 10 as u8, 0xaa, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::X) == 10);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::X) == 0xc1);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::X) == 1);

    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x55);
    cpu.start(vec![0xa5, 0x10, 0x00]);
    println!("{}", cpu.registers.get(RegisterType::A) == 0x55);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x01, 0xaa, 0xa9, 0xaa, 0x95, 0xa0, 0xe8, 0x95, 0xa0, 0x00]);
    println!("{}", cpu.memory.read(0xa1) == cpu.registers.get(RegisterType::A));
    println!("{}", cpu.memory.read(0xa2) == cpu.registers.get(RegisterType::A));

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x05, 0xaa, 0x95, 0xff, 0x00]);
    println!("{}", cpu.memory.read(0x04) == cpu.registers.get(RegisterType::A));
}
