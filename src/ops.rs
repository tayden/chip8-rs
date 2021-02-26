use std::cmp::Ordering;

use crate::{Chip8, FONTSET_START_ADDRESS, VIDEO_HEIGHT, VIDEO_WIDTH};

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn it_returns_vx() {
        let mut chp8 = Chip8::new();
        chp8.opcode = 0x1234;
        assert_eq!(chp8.vx(), 0x02);
    }

    #[test]
    fn it_returns_vy() {
        let mut chp8 = Chip8::new();
        chp8.opcode = 0x1234;
        assert_eq!(chp8.vy(), 0x03);
    }

    #[test]
    fn it_returns_last_byte() {
        let mut chp8 = Chip8::new();
        chp8.opcode = 0x1234;
        assert_eq!(chp8.last_byte(), 0x34);
    }

    #[test]
    fn test_op_00e0() {
        let mut chp8 = Chip8::new();
        chp8.video.fill(1);
        chp8.video.iter().for_each(|b| assert_eq!(*b, 1));
        chp8.op_00e0();
        chp8.video.iter().for_each(|b| assert_eq!(*b, 0));
    }

    #[test]
    fn test_op_00ee() {
        let mut chp8 = Chip8::new();
        chp8.stack = (0x0000u16..0x0010u16).collect::<Vec<u16>>().try_into().unwrap();
        chp8.sp = 0x01;
        chp8.pc = chp8.stack[chp8.sp as usize];

        assert_eq!(chp8.pc, 0x0001);
        chp8.op_00ee();
        assert_eq!(chp8.pc, 0x0000);
        assert_eq!(chp8.sp, 0x00);
    }

    #[test]
    fn test_op_1nnn() {
        let mut chp8 = Chip8::new();
        chp8.pc = 0x0000;
        chp8.opcode = 0x1234;

        chp8.op_1nnn();
        assert_eq!(chp8.pc, 0x0234);
    }

    #[test]
    fn test_op_2nnn() {
        let mut chp8 = Chip8::new();
        chp8.sp = 0x01;
        chp8.pc = 0xABCD;
        chp8.opcode = 0x1234;

        chp8.op_2nnn();
        assert_eq!(chp8.stack[0x01], 0xABCD);
        assert_eq!(chp8.sp, 0x02);
        assert_eq!(chp8.pc, 0x0234);
    }

    #[test]
    fn test_op_3xnn() {
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.pc = 0x0000;

        chp8.opcode = 0x3100;
        chp8.op_3xnn();
        assert_eq!(chp8.pc, 0x0000);

        chp8.opcode = 0x3102;
        chp8.op_3xnn();
        assert_eq!(chp8.pc, 0x0000);

        chp8.opcode = 0x3101;
        chp8.op_3xnn();
        assert_eq!(chp8.pc, 0x0002);
    }

    #[test]
    fn test_op_4xnn() {
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.pc = 0x0000;

        chp8.opcode = 0x4101;
        chp8.op_4xnn();
        assert_eq!(chp8.pc, 0x0000);

        chp8.opcode = 0x4100;
        chp8.op_4xnn();
        assert_eq!(chp8.pc, 0x0002);

        chp8.opcode = 0x4102;
        chp8.op_4xnn();
        assert_eq!(chp8.pc, 0x0004);
    }

    #[test]
    fn test_op_5xy0() {
        // Skip next instruction if Vx = Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.pc = 0x0000;

        chp8.opcode = 0x5100;
        chp8.op_5xy0();
        assert_eq!(chp8.pc, 0x0000);

        chp8.opcode = 0x5110;
        chp8.op_5xy0();
        assert_eq!(chp8.pc, 0x0002);

        chp8.opcode = 0x5120;
        chp8.op_5xy0();
        assert_eq!(chp8.pc, 0x0002);
    }

    #[test]
    fn test_op_6xnn() {
        // Set Vx = kk.
        let mut chp8 = Chip8::new();
        chp8.registers.fill(0x00);
        chp8.registers.iter().for_each(|b| assert_eq!(*b, 0x00));

        chp8.opcode = 0x60FF;
        chp8.op_6xnn();
        assert_eq!(chp8.registers[0x00 as usize], 0xFF);
        chp8.registers[1..].iter().for_each(|b| assert_eq!(*b, 0x00));

        chp8.opcode = 0x61FF;
        chp8.op_6xnn();
        assert_eq!(chp8.registers[0x01 as usize], 0xFF);
        chp8.registers[2..].iter().for_each(|b| assert_eq!(*b, 0x00));
    }

    #[test]
    fn test_op_7xnn() {
        // Set Vx = Vx + kk.
        let mut chp8 = Chip8::new();
        chp8.registers.fill(0x01);
        chp8.registers.iter().for_each(|b| assert_eq!(*b, 0x01));

        chp8.opcode = 0x70F0;
        chp8.op_7xnn();
        assert_eq!(chp8.registers[0x00 as usize], 0xF1);
        chp8.registers[1..].iter().for_each(|b| assert_eq!(*b, 0x01));

        chp8.registers[0x01] = 0x02;
        chp8.opcode = 0x71F0;
        chp8.op_7xnn();
        assert_eq!(chp8.registers[0x01 as usize], 0xF2);
        chp8.registers[2..].iter().for_each(|b| assert_eq!(*b, 0x01));
    }

    #[test]
    fn test_op_8xy0() {
        // Set Vx = Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();

        chp8.opcode = 0x8030;
        chp8.op_8xy0();
        assert_eq!(chp8.registers[0x00], 0x03);

        chp8.opcode = 0x84F0;
        chp8.op_8xy0();
        assert_eq!(chp8.registers[0x04], 0x0F);
    }

    #[test]
    fn test_op_8xy1() {
        // Set Vx = Vx OR Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();

        chp8.opcode = 0x8031;
        chp8.op_8xy1();
        assert_eq!(chp8.registers[0x00], 0x00 | 0x03);

        chp8.opcode = 0x84F1;
        chp8.op_8xy1();
        assert_eq!(chp8.registers[0x04], 0x04 | 0x0F);
    }

    #[test]
    fn test_op_8xy2() {
        // Set Vx = Vx AND Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();

        chp8.opcode = 0x8032;
        chp8.op_8xy2();
        assert_eq!(chp8.registers[0x00], 0x00 & 0x03);

        chp8.opcode = 0x84F2;
        chp8.op_8xy2();
        assert_eq!(chp8.registers[0x04], 0x04 & 0x0F);
    }

    #[test]
    fn test_op_8xy3() {
        // Set Vx = Vx XOR Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();

        chp8.opcode = 0x8033;
        chp8.op_8xy3();
        assert_eq!(chp8.registers[0x00], 0x00 ^ 0x03);

        chp8.opcode = 0x84F3;
        chp8.op_8xy3();
        assert_eq!(chp8.registers[0x04], 0x04 ^ 0x0F);
    }

    #[test]
    fn test_op_8xy4() {
        // Set Vx = Vx + Vy, set VF = carry.
        // The values of Vx and Vy are added together.
        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        // Only the lowest 8 bits of the result are kept, and stored in Vx.
        let mut chp8 = Chip8::new();

        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8014;
        chp8.op_8xy4();
        assert_eq!(chp8.registers[0x00], 0x00 + 0x01);
        assert_eq!(chp8.registers[0x0F], 0);

        chp8.registers = (0xF0..=0xFF).collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8234;
        chp8.op_8xy4();
        assert_eq!(chp8.registers[0x02], ((0x00F2 + 0x00F3) & 0x00FF) as u8);
        assert_eq!(chp8.registers[0x0F], 1);
    }

    #[test]
    fn test_op_8xy5() {
        // Set Vx = Vx - Vy, set VF = NOT borrow.
        // If Vx > Vy, then VF is set to 1, otherwise 0.
        // Then Vy is subtracted from Vx, and the results stored in Vx.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8015;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x00);
        assert_eq!(chp8.registers[chp8.vx()], 0x00);
        assert_eq!(chp8.vy(), 0x01);
        assert_eq!(chp8.registers[chp8.vy()], 0x01);

        chp8.op_8xy5();
        assert_eq!(chp8.registers[0x00], 0x00u8.wrapping_sub(0x01u8));
        assert_eq!(chp8.registers[0x0F], 0);

        chp8.registers = (0xF0..=0xFF).rev().collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8235;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x02);
        assert_eq!(chp8.registers[chp8.vx()], 0xFD);
        assert_eq!(chp8.vy(), 0x03);
        assert_eq!(chp8.registers[chp8.vy()], 0xFC);

        chp8.op_8xy5();
        assert_eq!(chp8.registers[0x02], 0xFD - 0xFC);
        assert_eq!(chp8.registers[0xF], 1);
    }

    #[test]
    fn test_op_8xy6() {
        // Set Vx = Vx SHR 1.
        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
        // Then Vx is divided by 2.
        let mut chp8 = Chip8::new();
        chp8.registers.fill(0b00001001);
        chp8.opcode = 0x8016;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x00);
        assert_eq!(chp8.registers[chp8.vx()], 0b00001001);

        chp8.op_8xy6();
        assert_eq!(chp8.registers[0x00], 0b00000100);
        assert_eq!(chp8.registers[0xF], 0x1);
        chp8.registers[1..0xF].iter().for_each(|b| assert_eq!(*b, 0b00001001));
    }

    #[test]
    fn test_op_8xy7() {
        // Set Vx = Vy - Vx, set VF = NOT borrow.
        // If Vy > Vx, then VF is set to 1, otherwise 0.
        // Then Vx is subtracted from Vy, and the results stored in Vx.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8017;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x00);
        assert_eq!(chp8.registers[chp8.vx()], 0x00);
        assert_eq!(chp8.vy(), 0x01);
        assert_eq!(chp8.registers[chp8.vy()], 0x01);

        chp8.op_8xy7();
        assert_eq!(chp8.registers[0x00], 0x01u8.wrapping_sub(0x00u8));
        assert_eq!(chp8.registers[0x0F], 1);

        chp8.registers = (0xF0..=0xFF).rev().collect::<Vec<u8>>().try_into().unwrap();
        chp8.opcode = 0x8237;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x02);
        assert_eq!(chp8.registers[chp8.vx()], 0xFD);
        assert_eq!(chp8.vy(), 0x03);
        assert_eq!(chp8.registers[chp8.vy()], 0xFC);

        chp8.op_8xy7();
        assert_eq!(chp8.registers[0x02], 0xFCu8.wrapping_sub(0xFDu8));
        assert_eq!(chp8.registers[0xF], 0);
    }

    #[test]
    fn test_op_8xye() {
        // Set Vx = Vx SHL 1.
        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
        // Then Vx is multiplied by 2.
        let mut chp8 = Chip8::new();
        chp8.registers.fill(0b10001000);
        chp8.opcode = 0x801e;
        chp8.registers[0xF] = 0xF;

        assert_eq!(chp8.vx(), 0x00);
        assert_eq!(chp8.registers[chp8.vx()], 0b10001000);

        chp8.op_8xye();
        assert_eq!(chp8.registers[0x00], 0b00010000);
        assert_eq!(chp8.registers[0xF], 0x1);
        chp8.registers[1..0xF].iter().for_each(|b| assert_eq!(*b, 0b10001000));
    }

    #[test]
    fn test_op_9xy0() {
        // Skip next instruction if Vx != Vy.
        let mut chp8 = Chip8::new();
        chp8.registers = (0x00..0x10).collect::<Vec<u8>>().try_into().unwrap();
        chp8.pc = 0x0000;

        chp8.opcode = 0x9000;
        chp8.op_9xy0();
        assert_eq!(chp8.pc, 0x0000);

        chp8.opcode = 0x9010;
        chp8.op_9xy0();
        assert_eq!(chp8.pc, 0x0002);
    }

    #[test]
    fn test_op_annn() {
        // Set I = nnn.
        let mut chp8 = Chip8::new();
        chp8.opcode = 0xa123;

        assert_ne!(chp8.index, 0x0123);
        chp8.op_annn();
        assert_eq!(chp8.index, 0x0123);
    }

    #[test]
    fn test_op_bnnn() {
        // Jump to location nnn + V0.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0x0010;
        chp8.opcode = 0xb123;

        assert_ne!(chp8.pc, 0x0010 + 0x0123);
        chp8.op_bnnn();
        assert_eq!(chp8.pc, 0x0010 + 0x0123);
    }

    #[test]
    #[ignore] // TODO: How to test random number gen?
    fn test_op_cxkk() {
        // Set Vx = random byte AND kk.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0xFF;
        chp8.opcode = 0xc023;

        assert_eq!(chp8.vx(), 0x00 as usize);
        assert_ne!(chp8.registers[0x00], 0xFE);
        chp8.op_cxkk();
        assert_eq!(chp8.registers[0x00], 0xFE);
    }

    #[test]
    fn test_op_dxyn() {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        let mut chp8 = Chip8::new();

        // Check that we start with a blank screen
        chp8.video.iter().for_each(|p| assert_eq!(*p, 0));

        // Draw a simple dot
        chp8.index = 0;
        chp8.memory[0] = 0x80;
        chp8.opcode = 0xd002;

        chp8.op_dxyn();
        // chp8.draw();
        // println!("{:?}", chp8.video);

        // Top Left pixel is lit up
        assert_eq!(chp8.video[0], 0xFFFFFFFF);

        // No collisions on a blank canvas
        assert_eq!(chp8.registers[0xF], 0);

        // Redraw, check pixel goes off and collision detected
        chp8.op_dxyn();
        // chp8.draw();
        // println!("{:?}", chp8.video);
        assert_eq!(chp8.video[0], 0x00000000);
        assert_eq!(chp8.registers[0xF], 1);

        // Check drawing at arbitrary location ((1, 2) in this case)
        chp8.registers[1] = 1;
        chp8.registers[2] = 2;
        chp8.opcode = 0xd122;

        chp8.op_dxyn();
        assert_eq!(chp8.video[2 * VIDEO_WIDTH + 1], 0xFFFFFFFF);
    }

    #[test]
    fn test_op_ex9e() {
        // Skip next instruction if key with the value of Vx is pressed.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0x0;
        chp8.opcode = 0xE09E;
        chp8.pc = 0;

        // Key not pressed
        chp8.keypad[0] = 0;

        assert_eq!(chp8.pc, 0);
        chp8.op_ex9e();
        assert_eq!(chp8.pc, 0);

        // Key pressed
        chp8.keypad[0] = 1;

        assert_eq!(chp8.pc, 0);
        chp8.op_ex9e();
        assert_eq!(chp8.pc, 2);
    }

    #[test]
    fn test_op_exa1() {
        // Skip next instruction if key with the value of Vx is not pressed.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0x0;
        chp8.opcode = 0xE0A1;
        chp8.pc = 0;

        // Key pressed
        chp8.keypad[0] = 1;

        assert_eq!(chp8.pc, 0);
        chp8.op_exa1();
        assert_eq!(chp8.pc, 0);

        // Key not pressed
        chp8.keypad[0] = 0;

        assert_eq!(chp8.pc, 0);
        chp8.op_exa1();
        assert_eq!(chp8.pc, 2);
    }

    #[test]
    fn test_op_fx07() {
        // Set Vx = delay timer value.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0x0;
        chp8.opcode = 0xF007;
        chp8.delay_timer = 99;

        assert_ne!(chp8.registers[0x0], 99);
        chp8.op_fx07();
        assert_eq!(chp8.registers[0x0], 99);
    }

    #[test]
    fn test_op_fx0a() {
        // Wait for a key press, store the value of the key in Vx.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 0xF;
        chp8.opcode = 0xF00a;

        assert_eq!(chp8.vx(), 0x0);

        // keypad 0 pressed
        chp8.keypad[0] = 1;

        assert_ne!(chp8.registers[0x0], 0);
        chp8.op_fx0a();
        assert_eq!(chp8.registers[0x0], 0);

        // keypad 0xA pressed
        chp8.keypad[0] = 0;
        chp8.keypad[0xA] = 1;

        assert_ne!(chp8.registers[0x0], 0xA);
        chp8.op_fx0a();
        assert_eq!(chp8.registers[0x0], 0xA);
    }

    #[test]
    fn test_op_fx15() {
        // Set delay timer = Vx.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 99;
        chp8.opcode = 0xF015;
        chp8.delay_timer = 0x0;

        assert_ne!(chp8.delay_timer, 99);
        chp8.op_fx15();
        assert_eq!(chp8.delay_timer, 99);
    }

    #[test]
    fn test_op_fx18() {
        // Set sound timer = Vx.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 99;
        chp8.opcode = 0xF018;
        chp8.sound_timer = 0x0;

        assert_ne!(chp8.sound_timer, 99);
        chp8.op_fx18();
        assert_eq!(chp8.sound_timer, 99);
    }

    #[test]
    fn test_op_fx1e() {
        // Set I = I + Vx.
        let mut chp8 = Chip8::new();
        chp8.registers[0x0] = 99;
        chp8.opcode = 0xF01e;
        chp8.index = 1;

        assert_eq!(chp8.vx(), 0x0);

        assert_ne!(chp8.index, 100);
        chp8.op_fx1e();
        assert_eq!(chp8.index, 100);
    }

    // #[test]
    // fn test_op_fx29() {
    //     // Set I = location of sprite for digit Vx.
    //     self.index = FONTSET_START_ADDRESS + (5 * self.registers[self.vx() as usize]) as u16;
    // }

    #[test]
    fn test_op_fx33() {
        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
        // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
        // location in I, the tens digit at location I+1, and the ones digit at location I+2.
        let mut chp8 = Chip8::new();
        chp8.index = 0;
        chp8.memory[0] = 0;
        chp8.memory[1] = 0;
        chp8.memory[2] = 0;

        chp8.opcode = 0xF133;
        assert_eq!(chp8.vx(), 0x1);
        chp8.registers[0x1] = 123;

        chp8.op_fx33();
        assert_eq!(chp8.memory[0], 1);
        assert_eq!(chp8.memory[1], 2);
        assert_eq!(chp8.memory[2], 3);
    }

    #[test]
    fn test_op_fx55() {
        // Store registers V0 through Vx in memory starting at location I.
        // let vx = self.vx() as usize;
        // let s = self.index as usize;
        // self.memory[s..=s + vx].clone_from_slice(&self.registers[0..=vx])
        let mut chp8 = Chip8::new();
        chp8.registers[0] = 1;
        chp8.registers[1] = 2;
        chp8.registers[2] = 3;
        chp8.registers[3] = 4;

        chp8.opcode = 0xF355;
        assert_eq!(chp8.vx(), 0x3);

        chp8.index = 0x2;
        chp8.memory[..0x50].iter().for_each(|b| assert_eq!(*b, 0));

        chp8.op_fx55();
        assert_eq!(chp8.memory[0x2 - 1], 0);
        assert_eq!(chp8.memory[0x2 + 0], 1);
        assert_eq!(chp8.memory[0x2 + 1], 2);
        assert_eq!(chp8.memory[0x2 + 2], 3);
        assert_eq!(chp8.memory[0x2 + 3], 4);
        assert_eq!(chp8.memory[0x2 + 4], 0);
    }

    #[test]
    fn test_op_fx65() {
        // Read registers V0 through Vx from memory starting at location I.
        let mut chp8 = Chip8::new();
        chp8.opcode = 0xF365;
        assert_eq!(chp8.vx(), 0x3);

        chp8.memory[0x2 - 1] = 0;
        chp8.memory[0x2 + 0] = 1;
        chp8.memory[0x2 + 1] = 2;
        chp8.memory[0x2 + 2] = 3;
        chp8.memory[0x2 + 3] = 4;
        chp8.memory[0x2 + 4] = 0;
        chp8.index = 0x2;

        chp8.registers.iter().for_each(|b| assert_eq!(*b, 0));

        chp8.op_fx65();
        chp8.registers[0] = 1;
        chp8.registers[1] = 2;
        chp8.registers[2] = 3;
        chp8.registers[3] = 4;

    }
}

impl Chip8 {
    fn vx(&self) -> usize {
        ((&self.opcode & 0x0F00u16) >> 8) as usize
    }

    fn vy(&self) -> usize {
        ((&self.opcode & 0x00F0u16) >> 4) as usize
    }

    fn last_byte(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    fn skip_next_inst(&mut self) {
        self.pc += 2;
    }

    fn repeat_last_inst(&mut self) {
        self.pc -= 2;
    }

    fn op_00e0(&mut self) {
        // Clear the display.
        self.video.fill(0);
    }

    fn op_00ee(&mut self) {
        // Return from a subroutine.
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn op_1nnn(&mut self) {
        // Jump to location nnn.
        self.pc = &self.opcode & 0x0FFFu16;
    }

    fn op_2nnn(&mut self) {
        // Call subroutine at nnn.
        let address = &self.opcode & 0x0FFFu16;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn op_3xnn(&mut self) {
        // Skip next instruction if Vx = kk.
        if self.registers[self.vx()] == self.last_byte() {
            self.skip_next_inst()
        }
    }

    fn op_4xnn(&mut self) {
        // Skip next instruction if Vx != kk.
        if self.registers[self.vx()] != self.last_byte() {
            self.skip_next_inst()
        }
    }

    fn op_5xy0(&mut self) {
        // Skip next instruction if Vx = Vy.
        if self.registers[self.vx()] == self.registers[self.vy()] {
            self.skip_next_inst();
        }
    }

    fn op_6xnn(&mut self) {
        // Set Vx = kk.
        let val = self.last_byte();
        let vx = self.vx();
        self.registers[vx] = val;
    }

    fn op_7xnn(&mut self) {
        // Set Vx = Vx + kk.
        let val = self.last_byte();
        let vx = self.vx();
        self.registers[vx] = self.registers[vx].wrapping_add(val);
    }

    fn op_8xy0(&mut self) {
        // Set Vx = Vy.
        self.registers[self.vx()] = self.registers[self.vy()];
    }

    fn op_8xy1(&mut self) {
        // Set Vx = Vx OR Vy.
        self.registers[self.vx()] |= self.registers[self.vy()];
    }

    fn op_8xy2(&mut self) {
        // Set Vx = Vx AND Vy.
        self.registers[self.vx()] &= self.registers[self.vy()];
    }

    fn op_8xy3(&mut self) {
        // Set Vx = Vx XOR Vy.
        self.registers[self.vx()] ^= self.registers[self.vy()];
    }

    fn op_8xy4(&mut self) {
        // Set Vx = Vx + Vy, set VF = carry.
        // The values of Vx and Vy are added together.
        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        // Only the lowest 8 bits of the result are kept, and stored in Vx.
        let sum = self.registers[self.vx()] as u16 + self.registers[self.vy()] as u16;
        self.registers[0xF] = match sum.cmp(&255u16) {
            Ordering::Greater => 1,
            _ => 0
        };

        self.registers[self.vx()] = (sum & 0x00FFu16) as u8
    }

    fn op_8xy5(&mut self) {
        // Set Vx = Vx - Vy, set VF = NOT borrow.
        // If Vx > Vy, then VF is set to 1, otherwise 0.
        // Then Vy is subtracted from Vx, and the results stored in Vx.
        self.registers[0xF] = match self.registers[self.vx()].cmp(&self.registers[self.vy()]) {
            Ordering::Greater => 1,
            _ => 0
        };

        self.registers[self.vx()] = self.registers[self.vx()].wrapping_sub(self.registers[self.vy()]);
    }

    fn op_8xy6(&mut self) {
        // Set Vx = Vx SHR 1.
        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
        // Then Vx is divided by 2.
        self.registers[0xF] = self.registers[self.vx()] & 0x1;
        self.registers[self.vx()] >>= 1;
    }

    fn op_8xy7(&mut self) {
        // Set Vx = Vy - Vx, set VF = NOT borrow.
        // If Vy > Vx, then VF is set to 1, otherwise 0.
        // Then Vx is subtracted from Vy, and the results stored in Vx.
        self.registers[0xF] = match self.registers[self.vy()].cmp(&self.registers[self.vx()]) {
            Ordering::Greater => 1,
            _ => 0
        };

        self.registers[self.vx()] = self.registers[self.vy()].wrapping_sub(self.registers[self.vx()]);
    }

    fn op_8xye(&mut self) {
        // Set Vx = Vx SHL 1.
        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
        // Then Vx is multiplied by 2.
        self.registers[0xF] = (self.registers[self.vx()] & 0x80u8) >> 7;
        self.registers[self.vx()] <<= 1;
    }

    fn op_9xy0(&mut self) {
        // Skip next instruction if Vx != Vy.
        if self.registers[self.vx()] != self.registers[self.vy()] {
            self.skip_next_inst();
        }
    }

    fn op_annn(&mut self) {
        // Set I = nnn.
        self.index = self.opcode & 0x0FFF;
    }

    fn op_bnnn(&mut self) {
        // Jump to location nnn + V0.
        let address = self.opcode & 0x0FFF;
        self.pc = self.registers[0] as u16 + address;
    }

    fn op_cxkk(&mut self) {
        // Set Vx = random byte AND kk.
        self.registers[self.vx()] = self.rand_byte() & self.last_byte();
    }

    fn op_dxyn(&mut self) {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        let vx = self.vx();
        let vy = self.vy();
        let height = self.opcode & 0x000F;

        let x_pos = self.registers[vx];
        let y_pos = self.registers[vy];

        self.registers[0xF] = 0;

        (0..height).collect::<Vec<u16>>().iter().for_each(|row| {
            let sprite_byte = self.memory[(self.index + row) as usize];

            (0..8).collect::<Vec<u16>>().iter().for_each(|col| {
                // Wrap if going beyond screen boundaries
                let x_pos = (x_pos as u16 + col) % VIDEO_WIDTH as u16;
                let y_pos = (y_pos as u16 + row) % VIDEO_HEIGHT as u16;

                let sprite_pixel: u16 = sprite_byte as u16 & (0x80 >> col);
                let screen_pixel: &mut u32 = &mut self.video[(y_pos * VIDEO_WIDTH as u16 + x_pos) as usize];

                // Sprite pixel is on
                if sprite_pixel != 0 {

                    // Screen pixel also on - collision
                    if *screen_pixel != 0 {
                        self.registers[0xF] = 1;
                    }

                    // Effectively XOR with the sprite pixel
                    *screen_pixel ^= 0xFFFFFFFF;
                }
            })
        })
    }

    fn op_ex9e(&mut self) {
        // Skip next instruction if key with the value of Vx is pressed.
        let key = self.registers[self.vx()] as usize;
        if self.keypad[key] != 0 {
            self.skip_next_inst();
        }
    }

    fn op_exa1(&mut self) {
        // Skip next instruction if key with the value of Vx is not pressed.
        let key = self.registers[self.vx()] as usize;
        if self.keypad[key] == 0 {
            self.skip_next_inst();
        }
    }

    fn op_fx07(&mut self) {
        // Set Vx = delay timer value.
        self.registers[self.vx()] = self.delay_timer;
    }

    fn op_fx0a(&mut self) {
        // Wait for a key press, store the value of the key in Vx.
        match self.keypad.iter().position(|k| *k > 0) {
            Some(i) => self.registers[self.vx()] = i as u8,
            None => self.repeat_last_inst()
        };
    }

    fn op_fx15(&mut self) {
        // Set delay timer = Vx.
        self.delay_timer = self.registers[self.vx()];
    }

    fn op_fx18(&mut self) {
        // Set sound timer = Vx.
        self.sound_timer = self.registers[self.vx()];
    }

    fn op_fx1e(&mut self) {
        // Set I = I + Vx.
        self.index += self.registers[self.vx() as usize] as u16;
    }

    fn op_fx29(&mut self) {
        // Set I = location of sprite for digit Vx.
        self.index = FONTSET_START_ADDRESS + (5 * self.registers[self.vx() as usize]) as u16;
    }

    fn op_fx33(&mut self) {
        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
        // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
        // location in I, the tens digit at location I+1, and the ones digit at location I+2.
        let vx = self.vx();
        let mut value = self.registers[vx];

        // Ones-place
        self.memory[(self.index + 2) as usize] = value % 10;
        value /= 10;

        // Tens-place
        self.memory[(self.index + 1) as usize] = value % 10;
        value /= 10;

        // Hundreds-place
        self.memory[self.index as usize] = value % 10;
    }

    fn op_fx55(&mut self) {
        // Store registers V0 through Vx in memory starting at location I.
        let vx = self.vx() as usize;
        let s = self.index as usize;
        self.memory[s..=s + vx].clone_from_slice(&self.registers[0..=vx])
    }

    fn op_fx65(&mut self) {
        // Read registers V0 through Vx from memory starting at location I.
        let vx = self.vx() as usize;
        let s = self.index as usize;
        self.registers[0..=vx].clone_from_slice(&self.memory[s..=s + vx])
    }

    pub fn call_op(&mut self) {
        let n1 = self.opcode >> 4 * 3;
        let n34 = self.opcode & 0x00FF;
        let n4 = self.opcode & 0x000F;

        match n1 {
            0x0 => match n4 {
                0x0 => self.op_00e0(),
                0xE => self.op_00ee(),
                _ => {}
            },
            0x1 => self.op_1nnn(),
            0x2 => self.op_2nnn(),
            0x3 => self.op_3xnn(),
            0x4 => self.op_4xnn(),
            0x5 => self.op_5xy0(),
            0x6 => self.op_6xnn(),
            0x7 => self.op_7xnn(),
            0x8 => match n4 {
                0x0 => self.op_8xy0(),
                0x1 => self.op_8xy1(),
                0x2 => self.op_8xy2(),
                0x3 => self.op_8xy3(),
                0x4 => self.op_8xy4(),
                0x5 => self.op_8xy5(),
                0x6 => self.op_8xy6(),
                0x7 => self.op_8xy7(),
                0xE => self.op_8xye(),
                _ => {}
            },
            0x9 => self.op_9xy0(),
            0xA => self.op_annn(),
            0xB => self.op_bnnn(),
            0xC => self.op_cxkk(),
            0xD => self.op_dxyn(),
            0xE => match n34 {
                0x9E => self.op_ex9e(),
                0xA1 => self.op_exa1(),
                _ => {}
            },
            0xF => match n34 {
                0x07 => self.op_fx07(),
                0x0A => self.op_fx0a(),
                0x15 => self.op_fx15(),
                0x18 => self.op_fx18(),
                0x1E => self.op_fx1e(),
                0x29 => self.op_fx29(),
                0x33 => self.op_fx33(),
                0x55 => self.op_fx55(),
                0x65 => self.op_fx65(),
                _ => {}
            },
            _ => {}
        }
    }
}