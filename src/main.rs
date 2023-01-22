pub mod memory;
pub mod cpu;
pub mod trace;

// use sdl2::event::Event;
// use sdl2::EventPump;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::render::TextureAccess;

// use rand::Rng;

use cpu::CPU;
use memory::rom::ROM;
use trace::trace;

fn main() {
    // let sdl_context = sdl2::init().unwrap();
    // let window = sdl_context.video().unwrap()
    //     .window("Snake Game", (32 * 10) as u32, (32 * 10) as u32)
    //     .position_centered()
    //     .build().unwrap();

    // let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    // let mut event_pump = sdl_context.event_pump().unwrap();

    // canvas.set_scale(10.0, 10.0).unwrap();

    // let creator = canvas.texture_creator();
    // let mut texture = creator
    //     .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static,32, 32).unwrap();

    // let mut frame = [0 as u8; 32 * 32 * 3];
    // let mut rng = rand::thread_rng();


    let rom = ROM::new(&std::fs::read("nestest.nes").unwrap()).unwrap();
    let mut cpu = CPU::new(rom);

    cpu.start(move |cpu| {
        // handle_input(cpu, &mut event_pump);
        // cpu.memory.write(0xFE, rng.gen_range(1..16));

        // if check_screen(cpu, &mut frame) {
        //     texture.update(None, &frame, 32 * 3).unwrap();
        //     canvas.copy(&texture, None, None).unwrap();
        //     canvas.present();
        // }

        // std::thread::sleep(std::time::Duration::new(0, 70000));
        println!("{}", trace(cpu));
    });

}

// fn handle_input(cpu: &mut CPU, event_pump: &mut EventPump) {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => std::process::exit(0),
//             Event::KeyDown { keycode: Some(Keycode::W), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. }
//                 => cpu.memory.write(0xFF, 0x77),
//             Event::KeyDown { keycode: Some(Keycode::S), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. }
//                 => cpu.memory.write(0xFF, 0x73),
//             Event::KeyDown { keycode: Some(Keycode::A), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. }
//                 => cpu.memory.write(0xFF, 0x61),
//             Event::KeyDown { keycode: Some(Keycode::D), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. }
//                 => cpu.memory.write(0xFF, 0x64),
//             _ => ()
//         }
//     }
// }

// fn color(byte: u8) -> Color {
//     match byte {
//         0 => Color::BLACK,
//         1 => Color::WHITE,
//         2 | 9 => Color::GREY,
//         3 | 10 => Color::RED,
//         4 | 11 => Color::GREEN,
//         5 | 12 => Color::BLUE,
//         6 | 13 => Color::MAGENTA,
//         7 | 14 => Color::YELLOW,
//         _ => Color::CYAN,
//     }
// }

// fn check_screen(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
//     let mut update = false;

//     for i in 0x200..0x600 {
//         let frame_i = 3 * (i - 0x200);
//         let color_idx = cpu.memory.read(i as u16);
//         let (r, g, b) = color(color_idx).rgb();

//         if frame[frame_i] != r || frame[frame_i + 1] != g || frame[frame_i + 2] != b {
//             frame[frame_i] = r;
//             frame[frame_i + 1] = g;
//             frame[frame_i + 2] = b;
//             update = true;
//         }
//     }

//     update
// }