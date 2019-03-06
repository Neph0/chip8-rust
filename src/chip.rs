use std::io;
use std::io::prelude::*;
use std::fs::File;

use super::opcodes::*;

const MEMORY_SIZE: usize = 4096;
const NUMBER_OF_REGISTERS: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
const KEYBOARD_SIZE: usize = 16;

const APPLICATION_MEMORY_LOCATION: usize = 0x200;
const FONTSET_ELEMENT_SIZE: usize = 5;
const FONTSET_ELEMENT_NUMBERS: usize = 16;
const FONTSET: [u8; FONTSET_ELEMENT_SIZE * FONTSET_ELEMENT_NUMBERS] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
    key: [bool; KEYBOARD_SIZE],                         // Keyboard state
    pub draw_flag: u16,                                 // Draw flag
    pub exit_flag: u16,                                 // Exit flag
    pub clear_flag: u16,                                // Clear screen flag
}

impl Chip {
    pub fn new() -> Chip {
        let mut chip = Chip {
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
            draw_flag: 0,
            exit_flag: 0,
            clear_flag: 0,
        };

        for iterator in 0..80 {
            chip.memory[iterator] = FONTSET[iterator];
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
            FAMILY_MISCEALLENOUS => {
                match self.opcode & 0x0FFF {
                    OPCODE_CLEAR_SCREEN => {
                        // TODO: Clear the screen
                        self.clear_flag = 1;
                        self.pc += 2;
                    },
                    OPCODE_RETURN_FROM_SUBROUTINE => {
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
            OPCODE_JMP => {
                println!("JUMP TO {:0>4x?}", self.opcode & 0x0FFF);
                self.pc = (self.opcode & 0x0FFF).into();
            },
            // 0x2NNN: Call subroutine at NNN
            OPCODE_CALL_SUBROUTINE => {
                println!("CALL SUBROUTINE AT {:x?}", self.opcode & 0xFFF);
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = (self.opcode & 0x0FFF).into();
            },
            // 0x3XNN: Skip the next instruction if VX equals NN
            OPCODE_SKIP_IF_EQ_NN => {
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
            OPCODE_SKIP_IF_NEQ_NN => {
                let x  = (self.opcode as usize & 0x0F00) >> 8;
                let vx = self.v[x];
                let nn = self.opcode as u8 & 0x00FF;
                println!("SKIP NEXT INSTRUCTION IF V{} != {}", x, nn);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            // 0x5XY0: Skip the next instruction if VX equals VY
            OPCODE_SKIP_IF_EQ_XY => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x];
                let vy = self.v[y];
                println!("SKIP NEXT INSTRUCTION IF V{} == V{}", x, y);
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            // 0x6XNN: Set VX to NN
            OPCODE_SET_VX_TO_NN => {
                let nn = self.opcode as u8 & 0x00FF;
                println!("SET V{} TO {:x?}", (self.opcode as usize & 0x0F00) >> 8, nn);
                self.v[(self.opcode as usize & 0x0F00) >> 8] = nn;
                self.pc += 2;
            },
            // 0x7XNN: Add NN to VX (carry flag is not changed)
            OPCODE_ADD_NN_TO_VX => {
                let nn = self.opcode as u8 & 0x00FF;
                let x = (self.opcode as usize & 0x0F00) >> 8;
                // Ignore overflows
                println!("ADD {} TO V{} ({}) = {}", nn, x, self.v[x], self.v[x].wrapping_add(nn));
                self.v[x] = self.v[x].wrapping_add(nn);
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
            FAMILY_ARITHMETICS => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x];
                let vy = self.v[y];
                match self.opcode & 0xF00F {
                    OPCODE_SET_VX_TO_VY => {
                        println!("SET V{} to V{}", x, y);
                        self.v[x] = vy;
                    },
                    OPCODE_SET_VX_TO_VX_OR_VY => {
                        println!("SET V{} to V{} | V{}", x, x, y);
                        self.v[x] = vx | vy;
                    },
                    OPCODE_SET_VX_TO_VX_AND_VY => {
                        println!("SET V{} to V{} & V{}", x, x, y);
                        self.v[x] = vx & vy;
                    },
                    OPCODE_SET_VX_TO_VX_XOR_VY => {
                        println!("SET V{} to V{} ^ V{}", x, x, y);
                        self.v[x] = vx ^ vy;
                    },
                    OPCODE_SET_VX_TO_VX_PLUS_VY => {
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
                    OPCODE_SUBSTRACT_VY_FROM_VX => {
                        println!("SUBSTRACTION: V{} = V{} - V{} = {}",
                                 x, x, y, self.v[x] - vy);
                        if vy > vx { self.v[0xf] = 0; }
                        else { self.v[0xf] = 1; }
                        self.v[x] -= vy;
                    },
                    OPCODE_STORE_LSB_OF_VX_IN_VF_AND_RSHIFT_VX => {
                        panic!("TODO: 0x006");
                    },
                    OPCODE_SET_VX_TO_VY_MINUS_VX => {
                        panic!("TODO: 0x007");
                    },
                    OPCODE_STORE_MSB_OF_VX_IN_VF_AND_LSHIFT_VX => {
                        panic!("TODO: 0x00E");
                    },
                    _      => {
                        panic!("UNKNOWN OPCODE: {:0>4x?}", self.opcode);
                    }
                }

                self.pc += 2;
            },
            // 0x9XY0: Skip the next instruction if VX doesn't equal VY
            OPCODE_SKIP_IF_NEQ_XY => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x];
                let vy = self.v[y];
                println!("SKIP NEXT INSTRUCTION IF V{:x?} ({}) != V{:x?} ({})", x, vx, y, vy);
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            // 0xANNN: Set I to the address NNN
            OPCODE_SET_I_TO_NNN => {
                println!("SET I TO {:x?}", self.opcode & 0x0FFF);
                self.i = (self.opcode & 0x0FFF).into();
                self.pc += 2;
            },
            // 0xBNNN: Jump to the address NNN + V0
            OPCODE_JUMP_TO_NNN_PLUS_V0 =>  {
                panic!("TODO: 0xBNNN");
            },
            // 0xCXNN: Set VX to the result of NN & rand()[0..255]
            OPCODE_SET_VX_TO_NN_AND_RAND => {
                let r = rand::random::<u8>();
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let nn = self.opcode as u8 & 0x00FF;
                println!("SET V{} TO {} & {} = {}",
                         x, r, nn, r & nn);
                self.v[x] = r & nn;
                self.pc += 2;
            },
            // 0xDXYN: Read N bytes from memory, starting in I. Those bytes are then displayed
            //         as sprites on screen at coordinates (VX, VY). Sprites are XORed onto the
            //         existing screen. If this causes any pixels to be erased, VF is set to 1,
            //         otherwise it is set to 0.
            OPCODE_DRAW_SPRITE => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let y = (self.opcode as usize & 0x00F0) >> 4;
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;
                let n = (self.opcode as usize & 0x000F) >> 0;
                println!("DRAW SPRITE (V{} = {}, V{} = {}), HEIGHT {})", x, vx, y, vy, n);

                self.v[0xf] = 0;
                for i in 0..n {
                    for j in 0..8 {
                        let pos = (vy + i) * SCREEN_WIDTH + vx + j;
                        let bit = self.memory[self.i as usize + i * 8] >> j & 0x1;
                        if self.graphics[pos] ^ bit == 0 {
                            self.v[0xf] = 1;
                        }
                        self.graphics[pos] ^= bit;
                    }
                }

                self.draw_flag = 1;
                self.pc += 2;
            },
            // 0xE--- family: Input conditionals
            // - 0xEX9E: Skip the next instruction if the key stored in VX is pressed
            // - 0xEXA1: Skip the next instruction if the key stored in VX isn't pressed
            FAMILY_INPUT_CONDITIONALS => {
                let x = (self.opcode as usize & 0x0F00) >> 8;
                let vx = self.v[x] as usize;

                match self.opcode & 0xF0FF {
                    OPCODE_SKIP_IF_VX_IS_PRESSED => {
                        println!("SKIP NEXT INSTRUCTION IF KEY {} IS PRESSED", vx);
                        if self.key[vx] == true {
                            self.pc += 2;
                        }
                    },
                    OPCODE_SKIP_IF_VX_IS_NOT_PRESSED => {
                        println!("SKIP NEXT INSTRUCTION IF KEY {} IS NOT PRESSED", vx);
                        if self.key[vx] == false {
                            self.pc += 2;
                        }
                    },
                    _      => {
                        panic!("UNKNOWN OPCODE: {:0>4x?}", self.opcode);
                    }
                }

                self.pc += 2;
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
            FAMILY_TIMERS_INPUT_QUERY_ETC => {
                let x  = ((self.opcode & 0x0F00) >> 8) as usize;
                let vx = self.v[x];

                match self.opcode & 0xF0FF {
                    OPCODE_SET_VX_TO_DELAY_TIMER => {
                        println!("SET V{} = DELAY_TIMER", x);
                        self.v[x] = self.delay_timer as u8;
                    },
                    OPCODE_WAIT_FOR_INPUT_AND_SET_TO_VX => {
                        panic!("NOT IMPLEMENTED: {:0>4x?}", self.opcode);
                    },
                    OPCODE_SET_DELAY_TIMER_TO_VX => {
                        println!("SET DELAY_TIMER TO V{}", x);
                        self.delay_timer = vx.into();
                    },
                    OPCODE_SET_SOUND_TIMER_TO_VX => {
                        println!("SET SOUND_TIMER TO V{}", x);
                        self.sound_timer = vx.into();
                    },
                    OPCODE_ADD_VX_TO_I => {
                        println!("ADD V{} TO I", x);
                        self.i += vx as u32;
                    },
                    OPCODE_SET_I_TO_SPRITE_IN_VX => {
                        println!("SET I TO LOCATION OF SPRITE IN V{}", x);
                        self.i = (vx * FONTSET_ELEMENT_SIZE as u8).into();
                    },
                    OPCODE_STORE_VX_AS_DIGITS_AT_I => {
                        println!("STORE DECIMAL OF V{} AT {}", x, self.i);
                        self.memory[self.i as usize + 0] = vx / 100;
                        self.memory[self.i as usize + 1] = (vx % 100) / 10;
                        self.memory[self.i as usize + 2] = vx % 10;
                    },
                    OPCODE_STORE_REGISTERS_AT_I => {
                        println!("STORE REGISTERS AT {:x?}", self.i);
                        for i in 0x0..0xf {
                            self.memory[self.i as usize + i] = self.v[i];
                        }
                    },
                    OPCODE_RESTORE_REGISTERS_FROM_I => {
                        println!("RESTORE REGISTERS FROM {:x?}", self.i);
                        for i in 0x0..0xf {
                            self.v[i] = self.memory[self.i as usize + i];
                        }
                    },
                    _ =>      {
                        panic!("UNKNOWN OPCODE: {:0>4x?}", self.opcode);
                    }
                }

                self.pc += 2;
            }
            _ => {
                panic!("UNKNOWN OPCODE: {:0>4x?}", self.opcode);
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
