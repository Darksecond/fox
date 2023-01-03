pub const DEVICE_LENGTH      : u32 = 0x10000; // 64 Kilobytes
pub const SCREEN_LAYER_LENGTH: u32 = 0x80000; // 512 Kilobytes

pub const RESET_VECTOR  : u32 = 0x00000100;

// --- Devices ---
pub const CONSOLE_BASE : u32 = 0x10000000;
pub const SYSTEM_BASE  : u32 = 0x10010000;
pub const SCREEN_BASE  : u32 = 0x10020000;
pub const FILE0_BASE   : u32 = 0x10030000;
pub const FILE1_BASE   : u32 = 0x10040000;
pub const MOUSE_BASE   : u32 = 0x10050000;

// -- CONSOLE DEVICE --
// All of these are relative.
pub const CONSOLE_VECTOR: u32 = 0x0000;
pub const CONSOLE_WRITE : u32 = 0x0004;
pub const CONSOLE_READ  : u32 = 0x0008;
pub const CONSOLE_ERROR : u32 = 0x000C;

// -- SYSTEM DEVICE --
// All of these are relative.
/// Currently unused.
pub const SYSTEM_VECTOR : u32 = 0x0000;
/// Exit some time in the future.
pub const SYSTEM_EXIT   : u32 = 0x0004;
/// Read command line arguents as nul-terminated strings.
/// Each argument is seperated by a single nul.
/// If there are no more arguments this will return '0' continuously.
pub const SYSTEM_READ   : u32 = 0x0008;

// -- SCREEN DEVICE --
pub const SCREEN_VECTOR    : u32 = 0x10020000;
pub const SCREEN_WIDTH     : u32 = 0x10020004;
pub const SCREEN_HEIGHT    : u32 = 0x10020008;
pub const SCREEN_CMD_LENGTH: u32 = 0x1002000C;
pub const SCREEN_CMD_ADDR  : u32 = 0x10020010;
pub const SCREEN_ZOOM      : u32 = 0x10020014;
pub const SCREEN_PALETTE0  : u32 = 0x10020018;
pub const SCREEN_PALETTE15 : u32 = 0x10020054;

pub const SCREEN_LAYER0    : u32 = 0x20000000;
pub const SCREEN_LAYER0_TOP: u32 = SCREEN_LAYER0+SCREEN_LAYER_LENGTH-1;
pub const SCREEN_LAYER1    : u32 = 0x20080000;
pub const SCREEN_LAYER1_TOP: u32 = SCREEN_LAYER1+SCREEN_LAYER_LENGTH-1;

// -- SCREEN SPRITE COMMAND --
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
//TODO Flag for "don't touch transparent pixels (treat 0 as skip kinda thing)"


// -- FILE DEVICE --
// All of these are relative.
pub const FILE_VECTOR  : u32 = 0x0000;
pub const FILE_FILENAME: u32 = 0x0004;
pub const FILE_LENGTH  : u32 = 0x0008;
pub const FILE_APPEND  : u32 = 0x000C; //TODO Turn into FLAGS with append being 0x01
pub const FILE_STATUS  : u32 = 0x0010;
pub const FILE_READ    : u32 = 0x0014;
pub const FILE_WRITE   : u32 = 0x0018;

// -- MOUSE DEVICE --
// All of these are relative.
pub const MOUSE_VECTOR: u32 = 0x0000;
pub const MOUSE_X     : u32 = 0x0004;
pub const MOUSE_Y     : u32 = 0x0008;
pub const MOUSE_FLAGS : u32 = 0x000C;
pub const MOUSE_BUTTON: u32 = 0x0010;

pub const MOUSE_BUTTON_LEFT  : u32 = 0b0000_0001;
pub const MOUSE_BUTTON_MIDDLE: u32 = 0b0000_0010;
pub const MOUSE_BUTTON_RIGHT : u32 = 0b0000_0100;

pub const MOUSE_FLAG_FOCUS   : u32 = 0b0000_0001;
