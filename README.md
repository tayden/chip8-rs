# CHIP-8 EMULATOR

Written in Rust to get familiar with the language.

Code based on and modified from the [CHIP-8 emulator tutorial in C++ by Austin Morlan](https://austinmorlan.com/posts/chip8_emulator/) 

## Usage:

```shell
    ./chip8-rs <SCALE> <DELAY> <ROM>
```

### Example

```shell
    ./chip8-rs 20 4 /path/to/PONG2.ch8
```

## Controls
The keyboard is used as the controller, with the following keys mapped: 

```text
1 2 3 4
Q W E R
A S D F
Z X C V
```

The controls can be changed by editing the `platform::get_keycode` static method.

## ROMs
There are many free ROMs available for CHIP-8, including [here](https://github.com/loktar00/chip8/tree/master/roms)
and [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)

## Development
The codebase uses the usual cargo build tools. Run `cargo build --release` from the project root directory to build the executable.

---
Created by: Taylor Denouden (2021)