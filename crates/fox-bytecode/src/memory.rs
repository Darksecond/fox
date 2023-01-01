pub const RESET_VECTOR  : u32 = 0x00000100;

pub const CONSOLE_VECTOR: u32 = 0x10000000;
pub const CONSOLE_WRITE : u32 = 0x10000004;
pub const CONSOLE_READ  : u32 = 0x10000008;
pub const CONSOLE_ERROR : u32 = 0x1000000C;

pub const SYSTEM_VECTOR : u32 = 0x10010000;
pub const SYSTEM_EXIT   : u32 = 0x10010004;

pub const SCREEN_VECTOR    : u32 = 0x10020000;
pub const SCREEN_WIDTH     : u32 = 0x10020004;
pub const SCREEN_HEIGHT    : u32 = 0x10020008;
pub const SCREEN_CMD_LENGTH: u32 = 0x1002000C;
pub const SCREEN_CMD_ADDR  : u32 = 0x10020010;
pub const SCREEN_PALETTE0  : u32 = 0x10020014;
pub const SCREEN_PALETTE15 : u32 = 0x10020050;

pub const SCREEN_LAYER0    : u32 = 0x20000000;
pub const SCREEN_LAYER0_TOP: u32 = 0x2007FFFF;
pub const SCREEN_LAYER1    : u32 = 0x20080000;
pub const SCREEN_LAYER1_TOP: u32 = 0x2008FFFF;

pub const SCREEN_TOP      : u32 = 0x1002FFFF;


pub const SCREEN_CMD_X     : u32 = 0x0;
pub const SCREEN_CMD_Y     : u32 = 0x4;
pub const SCREEN_CMD_SOURCE: u32 = 0x8;
pub const SCREEN_CMD_FLAGS : u32 = 0xC;

pub const SCREEN_CMD_FLAGS_LAYER  : u32 = 0b000011; //2 bits
pub const SCREEN_CMD_FLAGS_FLIP_X : u32 = 0b000100;
pub const SCREEN_CMD_FLAGS_FLIP_Y : u32 = 0b001000;
pub const SCREEN_CMD_FLAGS_FLIP_XY: u32 = 0b010000;
/// Clear layer instead of drawing sprite
pub const SCREEN_CMD_FLAGS_CLEAR  : u32 = 0b100000;
