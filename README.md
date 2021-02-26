# CHIP-8 EMULATOR

A CHIP-8 emulator written in Rust to get familiar with the language features.

Based on the excellent [C++ CHIP-8 emulator tutorial](https://austinmorlan.com/posts/chip8_emulator/)  by Austin Morlan

<p float="left">
    <img alt="PONG2" src="https://media.giphy.com/media/lMDeLOlkVHOuj8oNij/giphy.gif" width="32%">
    <img alt="INVADERS" src="https://media.giphy.com/media/sXFdU3jKMrWbt7H5a1/giphy.gif" width="32%">
    <img alt="BRIX" src="https://media.giphy.com/media/02Dx8UDcHOcImahLRg/giphy.gif" width="32%">
</p>

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
You can download free CHIP-8 ROMs [here](https://github.com/loktar00/chip8/tree/master/roms)
and [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)

## Development
The codebase uses the usual cargo build tools. Run `cargo build --release` from the project root directory to build the executable.

---
Created by: Taylor Denouden (2021)