use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::Sdl;
use sdl2::video::WindowContext;
use crate::SoundState;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Platform<'tex> {
    context: Sdl,
    canvas: WindowCanvas,
    texture: Texture<'tex>,
    audio_device: AudioDevice<SquareWave>,
}

impl<'tex> Platform<'tex> {
    pub fn create_window_canvas_texture_creator(title: &str, window_width: u32, window_height: u32) -> (Sdl, WindowCanvas, TextureCreator<WindowContext>) {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem.window(title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        return (context, canvas, texture_creator);
    }

    pub fn new(context: Sdl, canvas: WindowCanvas, texture_creator: &'tex TextureCreator<WindowContext>, texture_width: u32, texture_height: u32) -> Self {
        let texture = texture_creator.create_texture_streaming(
            PixelFormatEnum::RGBA8888, texture_width, texture_height).unwrap();

        let audio_subsystem = context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None,       // default sample size
        };

        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        Platform { context, canvas, texture, audio_device }
    }

    pub fn update(&mut self, buffer: &[u32], pitch: usize, sound_state: &SoundState) {
        self.texture.update(None, unsafe { &buffer.align_to::<u8>().1 }, pitch).unwrap();
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();

        match sound_state {
            SoundState::On => self.audio_device.resume(),
            SoundState::Off => self.audio_device.pause(),
        }
    }

    fn get_keycode(keycode: &Option<Keycode>) -> Option<usize> {
        match keycode {
            Some(Keycode::X) => Some(0),
            Some(Keycode::Num1) => Some(1),
            Some(Keycode::Num2) => Some(2),
            Some(Keycode::Num3) => Some(3),
            Some(Keycode::Q) => Some(4),
            Some(Keycode::W) => Some(5),
            Some(Keycode::E) => Some(6),
            Some(Keycode::A) => Some(7),
            Some(Keycode::S) => Some(8),
            Some(Keycode::D) => Some(9),
            Some(Keycode::Z) => Some(0xA),
            Some(Keycode::C) => Some(0xB),
            Some(Keycode::Num4) => Some(0xC),
            Some(Keycode::R) => Some(0xD),
            Some(Keycode::F) => Some(0xE),
            Some(Keycode::V) => Some(0xF),
            _ => None,
        }
    }

    pub fn process_input(&self, keys: &mut [u8]) -> bool {
        let mut event_pump = self.context.event_pump().unwrap();
        let mut quit = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    quit = true;
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(k) = Self::get_keycode(&keycode) {
                        keys[k] = 1;
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(k) = Self::get_keycode(&keycode) {
                        keys[k] = 0;
                    }
                }
                _ => {}
            }
        };

        quit
    }
}