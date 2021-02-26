use std::fs;
use std::io::Read;

use rand::{Rng, RngCore};

use crate::font::{FONTSET, FONTSET_SIZE};

mod font;
mod ops;
pub mod platform;

const START_ADDRESS: u16 = 0x200;
const FONTSET_START_ADDRESS: u16 = 0x50;

const PIXEL_ON_CHAR: char = '\u{25A0}';
const PIXEL_OFF_CHAR: char = ' ';

pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;

pub struct Chip8 {
    // TODO: Make these private as much as possible
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],
    pub opcode: u16,
    rand_gen: Box<dyn RngCore>,
}

impl Chip8 {
    pub fn new() -> Self {
        // Init memory pointer
        let pc = START_ADDRESS;

        // Load the font
        let mut memory = [0; 4096];
        let start = FONTSET_START_ADDRESS as usize;
        memory[start..start + FONTSET_SIZE].clone_from_slice(&FONTSET);

        // Initialize the random number gen
        let rand_gen = Box::new(rand::thread_rng());

        Chip8 {
            pc,
            memory,
            registers: [0; 16],
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            index: 0,
            opcode: 0,
            rand_gen,
        }
    }

    pub fn load_rom(&mut self, filename: &String) {
        let mut f = fs::File::open(&filename).expect("file not found");
        let metadata = fs::metadata(&filename).expect("unable to read file metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer.iter().enumerate()
            .for_each(|(i, b)| self.memory[START_ADDRESS as usize + i] = *b)
    }

    pub fn rand_byte(&mut self) -> u8 {
        // Initial rand_byte
        self.rand_gen.gen_range(0..255u8)
    }

    pub fn draw(&self) {
        self.video.iter().enumerate().for_each(|(i, pixel)| {
            if *pixel != 0 { print!("{}", PIXEL_ON_CHAR) } else { print!("{}", PIXEL_OFF_CHAR) }
            if (i + 1) % VIDEO_WIDTH == 0 { println!(); }
        })
    }

    pub fn cycle(&mut self) {
        // Fetch
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8) | (self.memory[self.pc as usize + 1] as u16);

        // Increment the PC before we execute anything
        self.pc += 2;

        // Decode and execute
        self.call_op();

        // Decrement the delay timer if it's been set
        if self.delay_timer > 0 { self.delay_timer -= 1; }

        // Decrement the sound timer if it's been set
        if self.sound_timer > 0 { self.sound_timer -= 1; }
    }
}