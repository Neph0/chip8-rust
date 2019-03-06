use std::io;
use std::io::prelude::*;
use std::fs::File;

const MEMORY_SIZE: usize = 4096;
const NUMBER_OF_REGISTERS: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
const KEYBOARD_SIZE: usize = 16;

const APPLICATION_MEMORY_LOCATION: usize = 0x200;
const _FONTSET_MEMORY_LOCATION: usize = 0;
const _FONTSET_MEMORY_SIZE: usize = 0x50;

pub struct Chip {
    opcode: u16,                                        // Current opcode
    memory: [u8; MEMORY_SIZE],                          // Memory layout
    v: [u8; NUMBER_OF_REGISTERS],                       // Registers
    i: u32,                                             // Index register
    pc: usize,                                          // Program counter
    pub graphics: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],   // Screen display
    delay_timer: u32,                                   // Count down delay timer
    sound_timer: u32,                                   // Count down sound timer
    stack: [usize; STACK_SIZE],                         // Memory stack
    sp: usize,                                          // Stack pointer
    key: [bool; KEYBOARD_SIZE],                           // Keyboard state
    pub draw_flag: u16,                                 // Draw flag
    pub exit_flag: u16,                                 // Exit flag
}

impl Chip {
    pub fn new() -> Chip {
        let chip = Chip {
            opcode: 0,
            memory: [0; MEMORY_SIZE],
            v: [0; NUMBER_OF_REGISTERS],
            i: 0,
            pc: APPLICATION_MEMORY_LOCATION,
            graphics: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            key: [false; KEYBOARD_SIZE],
            draw_flag: 1, // FIXME: Should start at 0
            exit_flag: 0,
        };

        for _iterator in 0..80 {
            // TODO
            //chip.memory[_iterator] = chip8_fontset[_iterator];
        }

        return chip;
    }

    pub fn load_game(&mut self, path: &std::string::String) -> Result<(), io::Error> {
        // Copy the file at 'path' to the 0x200.. memory space
        println!("Loading game: {}", path);
        let mut buffer = vec![0; MEMORY_SIZE - APPLICATION_MEMORY_LOCATION as usize];
        let mut f = File::open(path)?;
        f.read(&mut buffer)?;
        self.memory[(APPLICATION_MEMORY_LOCATION as usize)..].clone_from_slice(&buffer);
        println!("Game {} properly loaded.", path);
        Ok(())
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        self.opcode = (self.memory[self.pc] as u16) << 8 | self.memory[self.pc + 1] as u16;
        print!("[{:0>4x?}] INSTRUCTION: {:0>4x?}: ", self.pc, self.opcode);
        
        // Decode opcode
        match self.opcode & 0xF000 {
            // 0x0--- family: Misceallenous
            // - 0x0NNN: Call RCA 1802 program at address NNN
            // - 0x00E0: Clear the screen
            // - 0x00EE: Return from subroutine
            0x0000 => {
                match self.opcode & 0x0FFF {
                    0x00E0 => {
                        // TODO: Clear the screen
                        println!("NOT_IMPLEMENTED: SCREEN_CLEAR");
                        self.pc += 2;
                    },
                    0x00EE => {
                        println!("RETURNING FROM SUBROUTINE TO {:0>4x?}", self.stack[self.sp - 1]);
                        self.stack[self.sp] = 0;
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    },
                    _      => {
                        println!("RCA 1802 calls are not supported");
                        self.pc += 2;
                    }
                }
            },
            // 0x1NNN: Jump to NNN
            0x1000 => {
                println!("JUMP TO {:0>4x?}", self.opcode & 0x0FFF);
                self.pc = (self.opcode & 0x0FFF).into();
            },
            // 0x2NNN: Call subroutine at NNN
            0x2000 => {
                println!("CALL SUBROUTINE AT {:x?}", self.opcode & 0xFFF);
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = (self.opcode & 0x0FFF).into();
            },
            // 0x3XNN: Skip the next instruction if VX equals NN
            0x3000 => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let vx = self.v[x];
                let nn = self.opcode as u8 & 0x00FF;
                println!("SKIP NEXT INSTRUCTION IF V{:x?} == {:x?}", x, nn);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            // 0x4XNN: Skip the next instruction if VX doesn't equal NN
            0x4000 => {
                let vx = self.v[(self.opcode as usize & 0x0F00) >> 8];
                let nn = self.opcode as u8 & 0x00FF;
                println!();
                if vx == nn {
                    self.pc += 4;
                }
            },
            // 0x5XY0: Skip the next instruction if VX equals VY
            0x5000 => {
                let vx = self.v[(self.opcode as usize & 0x0F00) >> 8];
                let vy = self.v[(self.opcode as usize & 0x00F0) >> 4];
                println!();
                if vx == vy {
                    self.pc += 4;
                }
            },
            // 0x6XNN: Set VX to NN
            0x6000 => {
                let nn = self.opcode as u8 & 0x00FF;
                println!("SET V{} TO {:x?}", (self.opcode as usize & 0x0F00) >> 8, nn);
                self.v[(self.opcode as usize & 0x0F00) >> 8] = nn;
                self.pc += 2;
            },
            // 0x7XNN: Add NN to VX (carry flag is not changed)
            0x7000 => {
                let nn = self.opcode as u8 & 0x00FF;
                let x = (self.opcode as usize & 0x0F00) >> 8;
                println!("ADD {:x?} TO V{:x?} = {:x?}",
                         nn, x, self.v[x] as u8 + nn);
                self.v[x] += nn;
                self.pc += 2;
            },
            // 0x8--- family: Arithmetics
            // - 0x8XY0: Set VX to the value of VY
            // - 0x8XY1: Set VX to VX | VY
            // - 0x8XY2: Set VX to VX & VY
            // - 0x8XY3: Set VX to VX ^ VY
            // - 0x8XY4: Add VY to VX (VF is set to 1 if there is a carry, else 0)
            // - 0x8XY5: Substract VY from VX (VF is set to 0 if there is a borrow, else 1)
            // - 0x8XY6: Store the LSB of VX in VF and then shift VX right by 1
            // - 0x8XY7: Set VX to VY minus VX (VF is set to 0 if there is a borrow, else 1)
            // - 0x8XYE: Store the MSB of VX in VF and then shift VX left by 1
            0x8000 => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x];
                let vy = self.v[y];
                match self.opcode & 0x000F {
                    0x0000 => {
                        // TODO
                        println!("TODO: 0x000");
                    },
                    0x0001 => {
                        // TODO
                        println!("TODO: 0x001");
                    },
                    0x0002 => {
                        // TODO
                        println!("TODO: 0x002");
                    },
                    0x0003 => {
                        // TODO
                        println!("TODO: 0x003");
                    },
                    0x0004 => {
                        println!("ADDITION: V{} = V{} + V{} = {}",
                                 x, x, y, self.v[x] + vy);
                        if vy > (0xFF - vx) {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[x] += vy;
                        self.pc += 2;
                    },
                    0x0005 => {
                        // TODO
                        println!("TODO: 0x005");
                    },
                    0x0006 => {
                        // TODO
                        println!("TODO: 0x006");
                    },
                    0x0007 => {
                        // TODO
                        println!("TODO: 0x007");
                    },
                    0x000E => {
                        // TODO
                        println!("TODO: 0x00E");
                    },
                    _      => {
                        // TODO
                        println!("TODO: 0x___");
                    }
                }

                self.pc += 2;
            },
            // 0x9XY0: Skip the next instruction if VX doesn't equal VY
            0x9000 => {
                // TODO
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x];
                let vy = self.v[y];
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            // 0xANNN: Set I to the address NNN
            0xA000 => {
                println!("SET I TO {:x?}", self.opcode & 0x0FFF);
                self.i = (self.opcode & 0x0FFF).into();
                self.pc += 2;
            },
            // 0xBNNN: Jump to the address NNN + V0
            0xB000 =>  {
                // TODO
            },
            // 0xCXNN: Set VC to the result of NN & rand()[0..255]
            0xC000 => {
                // TODO
            },
            // 0xDXYN: Draw a sprite at coordinate (VX, VY), that has a width of 8 pixels
            //   and a height of N pixels. Each row of 8 pixels is read as bit-coded starting
            //   from memory location I; I value doesn't change after the execution of this
            //   instruction. VF is set to 1 if any screen pixels are flipped from set to
            //   unset when the sprite is drawn, and to 0 if that doesn't happen
            0xD000 => {
                // TODO
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;
                let n = (self.opcode as usize & 0x000F) >> 0;
                println!("DRAW SPRITE (V{} = {}, V{} = {}), HEIGHT {})", x, vx, y, vy, n);
                for i in 0..n {
                    for j in 0..8 {
                        self.graphics[vx * (vy + i) + j] = 1;
                    }
                }
                self.draw_flag = 1;
                self.pc += 2;
            },
            // 0xE--- family: Input conditionals
            // - 0xEX9E: Skip the next instruction if the key stored in VX is pressed
            // - 0xEXA1: Skip the next instruction if the key stored in VX isn't pressed
            0xE000 => {
                // TODO
            },
            // 0xF--- family: Timers, input query, and others
            // - 0xFX07: Set VX to the value of the delay timer
            // - 0xFX0A: A key press is awaited, and then stored in VX (blocking)
            // - 0xFX15: Set the delay timer to VX
            // - 0xFX18: Set the sound timer to VX
            // - 0xFX1E: Add VX to I
            // - 0xFX29: Set I to the location of the sprite for the character in VX.
            //           Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            // - 0xFX33: Store the binary-coded decimal representation of VX, with the most
            //           significant of three digit at the address in I, the middle digit
            //           at I+1, and the least significant digit at I+2.
            //           -> Take the decimal representation of VX, place the hundreds digit
            //           at I, the tens digit at I+1, and the ones digit at I+2.
            // - 0xFX55: Store V0 to VX (including VX) in memory starting at address I.
            //           The offset from I is increased by 1 for each value written, but I
            //           itself is left unmodified.
            // - 0xFX65: Fill V0 to VX (including VX) with values from memory starting at
            //           address I. The offset from I is increased by 1 for each value written,
            //           but I itself is left unmodified.
            0xF000 => {
                // TODO
            },
            _      => {
                // Cannot happen but for some reason rustc is complaining
                std::panic!("Unknown opcode: 0x{}{}", self.opcode & 0xFF00, self.opcode & 0x00FF);
            }
        }
        
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // TODO: Actual beep handling.
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    // TODO
    pub fn set_key(&mut self, index: usize, state: bool) {
        println!("Setting key index {} to {}", index, state);
        self.key[index] = state;
    }
}
