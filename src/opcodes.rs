// CHIP-8 opcodes table
pub const FAMILY_MISCEALLENOUS: u16                       = 0x0000;
pub const FAMILY_ARITHMETICS: u16                         = 0x8000;
pub const FAMILY_INPUT_CONDITIONALS: u16                  = 0xE000;
pub const FAMILY_TIMERS_INPUT_QUERY_ETC: u16              = 0xF000;

pub const OPCODE_CLEAR_SCREEN: u16                        = 0x00E0;
pub const OPCODE_RETURN_FROM_SUBROUTINE: u16              = 0x00EE;
pub const OPCODE_JMP: u16                                 = 0x1000;
pub const OPCODE_CALL_SUBROUTINE: u16                     = 0x2000;
pub const OPCODE_SKIP_IF_EQ_NN: u16                       = 0x3000;
pub const OPCODE_SKIP_IF_NEQ_NN: u16                      = 0x4000;
pub const OPCODE_SKIP_IF_EQ_XY: u16                       = 0x5000;
pub const OPCODE_SET_VX_TO_NN: u16                        = 0x6000;
pub const OPCODE_ADD_NN_TO_VX: u16                        = 0x7000;
pub const OPCODE_SET_VX_TO_VY: u16                        = 0x8000;
pub const OPCODE_SET_VX_TO_VX_OR_VY: u16                  = 0x8001;
pub const OPCODE_SET_VX_TO_VX_AND_VY: u16                 = 0x8002;
pub const OPCODE_SET_VX_TO_VX_XOR_VY: u16                 = 0x8003;
pub const OPCODE_SET_VX_TO_VX_PLUS_VY: u16                = 0x8004;
pub const OPCODE_SUBSTRACT_VY_FROM_VX: u16                = 0x8005;
pub const OPCODE_STORE_LSB_OF_VX_IN_VF_AND_RSHIFT_VX: u16 = 0x8006;
pub const OPCODE_SET_VX_TO_VY_MINUS_VX: u16               = 0x8007;
pub const OPCODE_STORE_MSB_OF_VX_IN_VF_AND_LSHIFT_VX: u16 = 0x800E;
pub const OPCODE_SKIP_IF_NEQ_XY: u16                      = 0x9000;
pub const OPCODE_SET_I_TO_NNN: u16                        = 0xA000;
pub const OPCODE_JUMP_TO_NNN_PLUS_V0: u16                 = 0xB000;
pub const OPCODE_SET_VX_TO_NN_AND_RAND: u16               = 0xC000;
pub const OPCODE_DRAW_SPRITE: u16                         = 0xD000;
pub const OPCODE_SKIP_IF_VX_IS_PRESSED: u16               = 0xE09E;
pub const OPCODE_SKIP_IF_VX_IS_NOT_PRESSED: u16           = 0xE0A1;
pub const OPCODE_SET_VX_TO_DELAY_TIMER: u16               = 0xF007;
pub const OPCODE_WAIT_FOR_INPUT_AND_SET_TO_VX: u16        = 0xF00A;
pub const OPCODE_SET_DELAY_TIMER_TO_VX: u16               = 0xF015;
pub const OPCODE_SET_SOUND_TIMER_TO_VX: u16               = 0xF018;
pub const OPCODE_ADD_VX_TO_I: u16                         = 0xF01E;
pub const OPCODE_SET_I_TO_SPRITE_IN_VX: u16               = 0xF029;
pub const OPCODE_STORE_VX_AS_DIGITS_AT_I: u16             = 0xF033;
pub const OPCODE_STORE_REGISTERS_AT_I: u16                = 0xF055;
pub const OPCODE_RESTORE_REGISTERS_FROM_I: u16            = 0xF065;
