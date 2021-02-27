use std::env;
use std::process::exit;
use std::time::Instant;

use chip8_rs::{Chip8, VIDEO_HEIGHT, VIDEO_WIDTH};
use chip8_rs::platform::Platform;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <Scale> <Delay> <ROM>", args[0]);
        exit(1);
    }

    let video_scale: u32 = args[1].parse().expect("<Scale> must be an integer");
    let cycle_delay: u128 = args[2].parse().expect("<Delay> must be an integer");
    let rom_filename= &args[3];

    let (context, canvas, mut texture_creator) = Platform::create_window_canvas_texture_creator(
        "CHIP-8 Emulator", VIDEO_WIDTH as u32 * video_scale, VIDEO_HEIGHT as u32 * video_scale);
    let mut platform = Platform::new(context, canvas, &mut texture_creator, VIDEO_WIDTH as u32, VIDEO_HEIGHT as u32);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_filename);


    let video_pitch = std::mem::size_of::<u32>() * VIDEO_WIDTH;

    let mut last_cycle_time = Instant::now();
    let mut quit = false;

    while !quit {
        quit = platform.process_input(&mut chip8.keypad);

        let current_time = Instant::now();
        let dt = last_cycle_time.elapsed().as_millis();

        if dt > cycle_delay {
            last_cycle_time = current_time;
            chip8.cycle();
            platform.update(&chip8.video, video_pitch, &chip8.sound_state());
        }
    }
    exit(0);
}
