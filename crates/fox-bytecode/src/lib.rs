pub mod memory;

pub const OP_HALT: u8 = 0x00;
pub const OP_DBG : u8 = 0x01;

pub const OP_LITW: u8 = 0x10;
pub const OP_DUP : u8 = 0x11;
pub const OP_DROP: u8 = 0x12;
pub const OP_SWAP: u8 = 0x13;
pub const OP_OVER: u8 = 0x14;
pub const OP_ROT : u8 = 0x15;
pub const OP_LITB: u8 = 0x16;
pub const OP_PICK: u8 = 0x17;

pub const OP_ADD: u8 = 0x20;
pub const OP_SUB: u8 = 0x21;
pub const OP_MUL: u8 = 0x22;
pub const OP_DIV: u8 = 0x23;
pub const OP_AND: u8 = 0x24;
pub const OP_OR : u8 = 0x25;
pub const OP_XOR: u8 = 0x26;
pub const OP_SHL: u8 = 0x27;
pub const OP_SHR: u8 = 0x28;
pub const OP_INC: u8 = 0x29;
pub const OP_DEC: u8 = 0x2A;
pub const OP_SAR: u8 = 0x2B;
pub const OP_NOT: u8 = 0x2C;

pub const OP_SW: u8 = 0x30;
pub const OP_LW: u8 = 0x31;
pub const OP_SB: u8 = 0x32;
pub const OP_LB: u8 = 0x33;

pub const OP_EQU: u8 = 0x40;
pub const OP_NEQ: u8 = 0x41;
pub const OP_LT : u8 = 0x42;
pub const OP_GT : u8 = 0x43;
pub const OP_LTE: u8 = 0x44;
pub const OP_GTE: u8 = 0x45;

pub const OP_JMP : u8 = 0x50;
pub const OP_JZ  : u8 = 0x51;
pub const OP_CALL: u8 = 0x52;
pub const OP_RET : u8 = 0x53;
pub const OP_JNZ : u8 = 0x54;

pub const OP_RPUSH: u8 = 0x60;
pub const OP_RPOP : u8 = 0x61;
pub const OP_RPEEK: u8 = 0x62;
pub const OP_RDROP: u8 = 0x63;

pub const OP_BEGIN: u8 = 0x70;
pub const OP_END  : u8 = 0x71;
pub const OP_GET  : u8 = 0x72;
pub const OP_SET  : u8 = 0x73;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Halt = OP_HALT,
    Dbg = OP_DBG,

    LitW = OP_LITW,
    Dup = OP_DUP,
    Drop = OP_DROP,
    Swap = OP_SWAP,
    Over = OP_OVER,
    Rot = OP_ROT,
    LitB = OP_LITB,
    Pick = OP_PICK,

    Add = OP_ADD,
    Sub = OP_SUB,
    Mul = OP_MUL,
    Div = OP_DIV,
    And = OP_AND,
    Or = OP_OR,
    Xor = OP_XOR,
    Shl = OP_SHL,
    Shr = OP_SHR,
    Inc = OP_INC,
    Dec = OP_DEC,
    Sar = OP_SAR,
    Not = OP_NOT,

    Sw = OP_SW,
    Lw = OP_LW,
    Sb = OP_SB,
    Lb = OP_LB,

    Equ = OP_EQU,
    Neq = OP_NEQ,
    Lt = OP_LT,
    Gt = OP_GT,
    Lte = OP_LTE,
    Gte = OP_GTE,

    Jmp = OP_JMP,
    Jz = OP_JZ,
    Call = OP_CALL,
    Ret = OP_RET,
    Jnz = OP_JNZ,

    Rpush = OP_RPUSH,
    Rpop = OP_RPOP,
    Rpeek = OP_RPEEK,
    Rdrop = OP_RDROP,

    Begin = OP_BEGIN,
    End = OP_END,
    Get = OP_GET,
    Set = OP_SET,
}

impl std::str::FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "halt" => Ok(Opcode::Halt),
            "dbg" => Ok(Opcode::Dbg),

            "litw" => Ok(Opcode::LitW),
            "dup" => Ok(Opcode::Dup),
            "drop" => Ok(Opcode::Drop),
            "swap" => Ok(Opcode::Swap),
            "over" => Ok(Opcode::Over),
            "rot" => Ok(Opcode::Rot),
            "litb" => Ok(Opcode::LitB),
            "pick" => Ok(Opcode::Pick),

            "add" => Ok(Opcode::Add),
            "sub" => Ok(Opcode::Sub),
            "mul" => Ok(Opcode::Mul),
            "div" => Ok(Opcode::Div),
            "and" => Ok(Opcode::And),
            "or" => Ok(Opcode::Or),
            "xor" => Ok(Opcode::Xor),
            "shl" => Ok(Opcode::Shl),
            "shr" => Ok(Opcode::Shr),
            "inc" => Ok(Opcode::Inc),
            "dec" => Ok(Opcode::Dec),
            "sar" => Ok(Opcode::Sar),
            "not" => Ok(Opcode::Not),

            "sw" => Ok(Opcode::Sw),
            "lw" => Ok(Opcode::Lw),
            "sb" => Ok(Opcode::Sb),
            "lb" => Ok(Opcode::Lb),

            "equ" => Ok(Opcode::Equ),
            "neq" => Ok(Opcode::Neq),
            "lt" => Ok(Opcode::Lt),
            "gt" => Ok(Opcode::Gt),
            "lte" => Ok(Opcode::Lte),
            "gte" => Ok(Opcode::Gte),

            "jmp" => Ok(Opcode::Jmp),
            "jz" => Ok(Opcode::Jz),
            "call" => Ok(Opcode::Call),
            "ret" => Ok(Opcode::Ret),
            "jnz" => Ok(Opcode::Jnz),

            "rpush" => Ok(Opcode::Rpush),
            "rpop" => Ok(Opcode::Rpop),
            "rpeek" => Ok(Opcode::Rpeek),
            "rdrop" => Ok(Opcode::Rdrop),

            "begin" => Ok(Opcode::Begin),
            "end" => Ok(Opcode::End),
            "get" => Ok(Opcode::Get),
            "set" => Ok(Opcode::Set),

            _ => Err(()),
        }
    }
}
