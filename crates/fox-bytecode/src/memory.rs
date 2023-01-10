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
pub const KEYBOARD_BASE: u32 = 0x10060000;

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
pub mod screen {
    pub const VECTOR    : u32 = 0x10020000;
    pub const WIDTH     : u32 = 0x10020004;
    pub const HEIGHT    : u32 = 0x10020008;
    pub const CMD_LENGTH: u32 = 0x1002000C;
    pub const CMD_ADDR  : u32 = 0x10020010;
    pub const ZOOM      : u32 = 0x10020014;
    pub const PALETTE0  : u32 = 0x10020018;
    pub const PALETTE15 : u32 = 0x10020054;

    pub const LAYER0    : u32 = 0x20000000;
    pub const LAYER0_TOP: u32 = LAYER0 + super::SCREEN_LAYER_LENGTH-1;
    pub const LAYER1    : u32 = 0x20080000;
    pub const LAYER1_TOP: u32 = LAYER1 + super::SCREEN_LAYER_LENGTH-1;
    pub const LAYER2    : u32 = 0x20100000;
    pub const LAYER2_TOP: u32 = LAYER2 + super::SCREEN_LAYER_LENGTH-1;
    pub const LAYER3    : u32 = 0x20180000;
    pub const LAYER3_TOP: u32 = LAYER3 + super::SCREEN_LAYER_LENGTH-1;

    // -- SCREEN SPRITE COMMAND --
    pub mod command {
        pub const X      : u32 = 0x0;
        pub const Y      : u32 = 0x4;
        pub const SOURCE : u32 = 0x8;
        /// Command & Layer
        pub const COMMAND: u32 = 0xC;
        pub const FLAGS  : u32 = 0xD;
        /// Foreground & Background
        pub const COLOR  : u32 = 0xE;
        /// Width & Height
        pub const REPEAT : u32 = 0xF;

        /// Clear layer
        pub const COMMAND_CLEAR  : u32 = 0x00;
        /// 1bpp Sprite
        pub const COMMAND_SPRITE1: u32 = 0x01;
        /// 4bpp Sprite
        pub const COMMAND_SPRITE4: u32 = 0x02;

        pub const FLAGS_FLIP_X    : u8 = 0x01;
        pub const FLAGS_FLIP_Y    : u8 = 0x02;
        pub const FLAGS_FLIP_XY   : u8 = 0x04;
        pub const FLAGS_SKIP_CLEAR: u8 = 0x08;
    }
}

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

pub mod keyboard {
    pub const VECTOR   : u32 = 0x0000;
    pub const CODEPOINT: u32 = 0x0004;
    pub const BUTTONS  : u32 = 0x0008;

    // Buttons
    pub const BUTTON_LEFT : u32 = 0x01;
    pub const BUTTON_RIGHT: u32 = 0x02;
    pub const BUTTON_UP   : u32 = 0x04;
    pub const BUTTON_DOWN : u32 = 0x08;
}
