pub mod cpu;
use cpu::{CPU, register::{Register}};

fn main() {
    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x05, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0x05);
    println!("{}", cpu.registers.get(&Register::P) & 0b0000_0010 == 0);
    println!("{}", cpu.registers.get(&Register::P) & 0b1000_0000 == 0);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x00, 0x00]);
    println!("{}", cpu.registers.get(&Register::P) & 0b0000_0010 == 0b10);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 10 as u8, 0xaa, 0x00]);
    println!("{}", cpu.registers.get(&Register::X) == 10);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    println!("{}", cpu.registers.get(&Register::X) == 0xc1);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
    println!("{}", cpu.registers.get(&Register::X) == 1);

    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x55);
    cpu.start(vec![0xa5, 0x10, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0x55);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x01, 0xaa, 0xa9, 0xaa, 0x95, 0xa0, 0xe8, 0x95, 0xa0, 0x00]);
    println!("{}", cpu.memory.read(0xa1) == cpu.registers.get(&Register::A));
    println!("{}", cpu.memory.read(0xa2) == cpu.registers.get(&Register::A));

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0x05, 0xaa, 0x95, 0xff, 0x00]);
    println!("{}", cpu.memory.read(0x04) == cpu.registers.get(&Register::A));

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0b1010_0011, 0x29, 0b0010_0101, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b0010_0001);

    let mut cpu = CPU::new();
    cpu.memory.write(0x09, 0b1010_0101);
    cpu.start(vec![0xa9, 0b1010_0011, 0x25, 0x09, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b1010_0001);
    println!("{}", (cpu.registers.get(&Register::P) >> 7) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0b1010_0011, 0x09, 0b0010_0101, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b1010_0111);
    println!("{}", (cpu.registers.get(&Register::P) >> 7) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.memory.write(0x09, 0b1010_0101);
    cpu.start(vec![0xa9, 0b1010_0011, 0x05, 0x09, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b1010_0111);
    println!("{}", (cpu.registers.get(&Register::P) >> 7) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.start(vec![0xa9, 0b1010_0011, 0x49, 0b0010_0101, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b1000_0110);
    println!("{}", (cpu.registers.get(&Register::P) >> 7) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.memory.write(0x09, 0b1010_0101);
    cpu.start(vec![0xa9, 0b1010_0011, 0x45, 0x09, 0x00]);
    println!("{}", cpu.registers.get(&Register::A) == 0b0000_0110);

    let mut cpu = CPU::new();
    cpu.memory.write(0x18f3, 0b1010_1110);
    cpu.start(vec![0x4e, 0xf3, 0x18, 0x00]);
    println!("{}", cpu.registers.get(&Register::P) & 0b1 == 0);
    println!("{}", cpu.memory.read(0x18f3) == 0b0101_0111);

    let mut cpu = CPU::new();
    cpu.memory.write(0x18f3, 0b1010_1110);
    cpu.start(vec![0x0e, 0xf3, 0x18, 0x00]);
    println!("{}", cpu.registers.get(&Register::P) & 0b1 == 1);
    println!("{}", cpu.memory.read(0x18f3) == 0b0101_1100);

    let mut cpu = CPU::new();
    cpu.memory.write(0x02, 0b1111_1111);
    cpu.start(vec![0xe6, 0x02, 0x00]);
    println!("{}", cpu.memory.read(0x02) == 0);
    println!("{}", (cpu.registers.get(&Register::P) >> 1) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.memory.write(0x02, 1);
    cpu.start(vec![0xc6, 0x02, 0x00]);
    println!("{}", cpu.memory.read(0x02) == 0);
    println!("{}", (cpu.registers.get(&Register::P) >> 1) & 0b1 != 0);

    let mut cpu = CPU::new();
    cpu.memory.write(0x02, 0);
    cpu.start(vec![0xc6, 0x02, 0x00]);
    println!("{}", cpu.memory.read(0x02) == 0b1111_1111);
    println!("{}", (cpu.registers.get(&Register::P) >> 7) & 0b1 != 0);
}
