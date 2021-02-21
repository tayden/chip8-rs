use chip8_rs::Chip8;

fn main() {
    let mut ch8 = Chip8::new();

    let filename = String::from("c8games/TICTAC");
    ch8.load_rom(&filename);
    // println!("{:?}", ch8.memory);

    // ch8.video[0] = 0xFFFFFFFF;
    // ch8.video[1] = 0xFFFFFFFF;
    // ch8.draw();
}
