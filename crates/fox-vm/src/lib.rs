pub mod device;

use fox_bytecode::*;

/// Public way of interfacing directly with VirtualMachine memory.
/// Use this through the `dma()` method, or by getting it as a parameter on read or write.
/// This will **not** forward read and write requests to the machine, instead only accessing directly
/// attached memory.
pub struct DirectMemoryAccess<'a> {
    vm: &'a mut VirtualMachine,
}

impl<'a> DirectMemoryAccess<'a> {
    pub fn read_u8(&self, addr: u32) -> u8 {
        self.vm.mem[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) {
        self.vm.mem[addr as usize] = value;
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        u32::from_le_bytes([
               self.read_u8(addr + 0),
               self.read_u8(addr + 1),
               self.read_u8(addr + 2),
               self.read_u8(addr + 3),
        ])
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) {
        let [a,b,c,d] = u32::to_le_bytes(value);

        self.write_u8(addr + 0, a);
        self.write_u8(addr + 1, b);
        self.write_u8(addr + 2, c);
        self.write_u8(addr + 3, d);
    }

    /// Read nul-terminated string.
    pub fn read_str(&self, addr: u32) -> String {
        let str = unsafe {
            let ptr = self.vm.mem.as_ptr().offset(addr as _);
            //TODO replace this with the `until` version once that stabilizes.
            std::ffi::CStr::from_ptr(ptr as _) 
        };

        let str = str.to_str().expect("String contains non UTF8 bytes");
        str.to_string()
    }

    /// Write nul-terminated string.
    pub fn write_str(&mut self, addr: u32, value: &str) {
        self.write(addr, value.as_bytes());
        self.write_u8(addr + value.len() as u32, 0);
    }

    pub fn read(&self, addr: u32, buf: &mut [u8]) {
        let start = addr as usize;
        let end = start + buf.len();
        let source = &self.vm.mem[start..end];
        buf.copy_from_slice(source);
    }

    pub fn write(&mut self, addr: u32, buf: &[u8]) {
        let start = addr as usize;
        let end = start + buf.len();
        let dest = &mut self.vm.mem[start..end];
        dest.copy_from_slice(buf);
    }
}

pub trait Machine {
    fn write_u32(&mut self, addr: u32, value: u32, dma: DirectMemoryAccess<'_>);
    fn read_u32(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u32; 

    fn write_u8(&mut self, addr: u32, value: u8, dma: DirectMemoryAccess<'_>) {
        self.write_u32(addr, value as _, dma)
    }
    fn read_u8(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u8 {
        (self.read_u32(addr, dma) & 0xFF) as _
    }
}

use fox_bytecode::memory::RESET_VECTOR;

const MEM_SIZE: usize = 16 * 1024 * 1024; // 16 Megabytes
const SP_SIZE: usize = 1024; // bytes
const RP_SIZE: usize = 1024; // bytes
const LOCAL_SIZE: usize = 1024; // bytes

const SP_OFFSET: usize = MEM_SIZE - SP_SIZE;
const RP_OFFSET: usize = MEM_SIZE - SP_SIZE - RP_SIZE;
const LOCAL_OFFSET: usize = MEM_SIZE - SP_SIZE - RP_SIZE - LOCAL_SIZE;

pub struct VirtualMachine {
    mem: Box<[u8]>,
    ip: *const u8,
    sp: *mut u32,
    rp: *mut u32,
    local: *mut u32,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut mem = vec![0; MEM_SIZE].into_boxed_slice();
        let ip = unsafe { mem.as_ptr().offset(RESET_VECTOR as _) };
        let sp = unsafe { mem.as_mut_ptr().offset(SP_OFFSET as _) };
        let rp = unsafe { mem.as_mut_ptr().offset(RP_OFFSET as _) };
        let local = unsafe { mem.as_mut_ptr().offset(LOCAL_OFFSET as _) };

        Self {
            mem,
            ip,
            sp: sp as _,
            rp: rp as _,
            local: local as _,
        }
    }

    pub fn dma(&mut self) -> DirectMemoryAccess<'_> {
        DirectMemoryAccess {
            vm: self,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = RESET_VECTOR as usize;
        let end = start + data.len();
        self.mem[start..end].copy_from_slice(data);
    }

    pub fn run(&mut self, machine: &mut dyn Machine, ip: u32) {
        if ip == 0 {
            return;
        }

        self.ip = unsafe { self.mem.as_ptr().offset(ip as _) };

        loop {
            match self.next_u8() {
                OP_HALT => {
                    break;
                },
                OP_DBG => {
                    self.dump();
                },

                OP_LITW => {
                    let number = self.next_u32();
                    self.push(number);
                },
                OP_DUP => {
                    let value = self.peek();
                    self.push(value);
                },
                OP_DROP => {
                    self.pop();
                }
                OP_SWAP => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(b);
                    self.push(a);
                },
                OP_OVER => {
                    let b = self.pop();
                    let a = self.peek();
                    self.push(b);
                    self.push(a);
                },
                OP_ROT => {
                    let c = self.pop();
                    let b = self.pop();
                    let a = self.pop();
                    self.push(b);
                    self.push(c);
                    self.push(a);
                },
                OP_LITB => {
                    let number = self.next_u8();
                    self.push(number as u32);
                },
                OP_PICK => {
                    let index = self.pop();
                    let value = self.peekn(index);
                    self.push(value);
                },

                OP_ADD => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_add(b));
                },
                OP_SUB => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_sub(b));
                },
                OP_MUL => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_mul(b));
                }
                OP_DIV => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_div(b));
                }
                OP_AND => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a & b);
                }
                OP_OR => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a | b);
                }
                OP_XOR => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a ^ b);
                }
                OP_SHL => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a << b);
                }
                OP_SHR => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a >> b);
                }
                OP_INC => {
                    let a = self.pop();
                    self.push(a.wrapping_add(1));
                }
                OP_DEC => {
                    let a = self.pop();
                    self.push(a.wrapping_sub(1));
                }
                OP_SAR => {
                    let b = self.pop() as i32;
                    let a = self.pop() as i32;
                    let value = a >> b;
                    self.push(value as u32);
                }
                OP_NOT => {
                    let a = self.pop();
                    self.push(!a);
                }

                OP_LW => {
                    let addr = self.pop();
                    let value = self.read_u32(addr, machine);
                    self.push(value as _);
                },
                OP_SW => {
                    let addr = self.pop();
                    let value = self.pop();
                    self.write_u32(addr, value, machine);
                },
                OP_SB => {
                    let addr = self.pop();
                    let value = self.pop();
                    self.write_u8(addr, (value & 0xFF) as u8, machine);
                }
                OP_LB => {
                    let addr = self.pop();
                    let value = self.read_u8(addr, machine);
                    self.push(value as _);
                },

                OP_EQU => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a == b { 1 } else { 0 };
                    self.push(out);
                },
                OP_GT => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a > b { 1 } else { 0 };
                    self.push(out);
                },
                OP_LT => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a < b { 1 } else { 0 };
                    self.push(out);
                },
                OP_GTE => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a >= b { 1 } else { 0 };
                    self.push(out);
                },
                OP_LTE => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a <= b { 1 } else { 0 };
                    self.push(out);
                },
                OP_NEQ => {
                    let b = self.pop();
                    let a = self.pop();
                    let out = if a != b { 1 } else { 0 };
                    self.push(out);
                },

                OP_JMP => {
                    let addr = self.pop();
                    unsafe {
                        self.ip = self.mem.as_mut_ptr().offset(addr as _);
                    }
                },
                OP_JZ => {
                    let addr = self.pop();
                    let cond = self.pop();
                    if cond == 0 {
                        unsafe {
                            self.ip = self.mem.as_mut_ptr().offset(addr as _);
                        }
                    }
                },
                OP_JNZ => {
                    let addr = self.pop();
                    let cond = self.pop();
                    if cond != 0 {
                        unsafe {
                            self.ip = self.mem.as_mut_ptr().offset(addr as _);
                        }
                    }
                },
                OP_CALL => {
                    let addr = self.pop();
                    let ip = unsafe { self.ip.offset_from(self.mem.as_ptr()) };
                    self.rpush(ip as _);
                    unsafe {
                        self.ip = self.mem.as_mut_ptr().offset(addr as _);
                    }
                }
                OP_RET => {
                    let addr = self.rpop();
                    unsafe {
                        self.ip = self.mem.as_mut_ptr().offset(addr as _);
                    }
                }
                OP_RPUSH => {
                    let value = self.pop();
                    self.rpush(value);
                }
                OP_RPOP => {
                    let value = self.rpop();
                    self.push(value);
                }
                OP_RPEEK => {
                    let value = self.rpeek();
                    self.push(value);
                }
                OP_RDROP => {
                    self.rpop();
                }
                OP_BEGIN => {
                    let length = self.pop();
                    self.lbegin(length);
                },
                OP_END => {
                    let length = self.pop();
                    self.lend(length);
                },
                OP_GET => {
                    let addr = self.pop();
                    self.push(self.lget(addr));
                },
                OP_SET => {
                    let addr = self.pop();
                    let value = self.pop();
                    self.lset(addr, value);
                },
                x => unimplemented!("0x{:02x}", x),
            }
        }
    }

    //TODO Add local variables?
    fn dump(&self) {
        unsafe {
            let ip = self.mem.as_ptr();
            let mut sp = self.mem.as_ptr().offset(SP_OFFSET as _) as *const u32;
            let mut rp = self.mem.as_ptr().offset(RP_OFFSET as _) as *const u32;

            eprintln!("IP: 0x{:08x}", self.ip.offset_from(ip));
            //eprintln!("INST: 0x{:02x}", *self.ip);

            eprint!("SP: ");
            while sp < self.sp as *const u32 {
                eprint!("0x{:08x} ", *sp);
                sp = sp.offset(1);
            }
            eprintln!();

            eprint!("RP: ");
            while rp < self.rp as *const u32 {
                eprint!("0x{:08x} ", *rp);
                rp = rp.offset(1);
            }
            eprintln!();

            eprintln!();
        }
    }
    fn read_u32(&mut self, addr: u32, machine: &mut dyn Machine) -> u32 {
        if addr < MEM_SIZE as _ {
            let addr = addr as usize;
            u32::from_le_bytes([
               self.mem[addr + 0],
               self.mem[addr + 1],
               self.mem[addr + 2],
               self.mem[addr + 3],
            ])
        } else {
            machine.read_u32(addr, self.dma())
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, machine: &mut dyn Machine) {
        if addr < MEM_SIZE as _ {
            let addr = addr as usize;
            let [a,b,c,d] = u32::to_le_bytes(value);
            self.mem[addr + 0] = a;
            self.mem[addr + 1] = b;
            self.mem[addr + 2] = c;
            self.mem[addr + 3] = d;
        } else {
            machine.write_u32(addr, value, self.dma());
        }
    }

    fn write_u8(&mut self, addr: u32, value: u8, machine: &mut dyn Machine) {
        if addr < MEM_SIZE as _ {
            let addr = addr as usize;
            self.mem[addr] = value;
        } else {
            machine.write_u8(addr, value, self.dma());
        }
    }

    fn read_u8(&mut self, addr: u32, machine: &mut dyn Machine) -> u8 {
        if addr < MEM_SIZE as _ {
            self.mem[addr as usize]
        } else {
            machine.read_u8(addr, self.dma())
        }
    }

    fn next_u32(&mut self) -> u32 {
        u32::from_le_bytes([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()])
    }

    fn next_u8(&mut self) -> u8 {
        unsafe {
            let value = *self.ip;
            self.ip = self.ip.offset(1);
            value
        }
    }

    fn push(&mut self, value: u32) {
        //TODO add overflow check
        unsafe {
            *self.sp = value;
            self.sp = self.sp.offset(1);
        }
    }

    fn pop(&mut self) -> u32 {
        //TODO Add underflow check
        unsafe {
            self.sp = self.sp.offset(-1);
            *self.sp
        }
    }

    fn peek(&self) -> u32 {
        //TODO Add underflow check
        unsafe {
            *self.sp.offset(-1)
        }
    }


    fn peekn(&self, n: u32) -> u32 {
        //TODO Add underflow/overflow check
        let offset = -1 - n as isize;
        dbg!(offset);
        unsafe {
            *self.sp.offset(offset)
        }
    }

    fn rpush(&mut self, value: u32) {
        //TODO add overflow check
        unsafe {
            *self.rp = value;
            self.rp = self.rp.offset(1);
        }
    }

    fn rpop(&mut self) -> u32 {
        //TODO Add underflow check
        unsafe {
            self.rp = self.rp.offset(-1);
            *self.rp
        }
    }

    fn rpeek(&self) -> u32 {
        //TODO Add underflow check
        unsafe {
            *self.rp.offset(-1)
        }
    }


    fn lbegin(&mut self, length: u32) {
        unsafe {
            self.local = self.local.offset(length as _);
        }
    }

    fn lend(&mut self, length: u32) {
        let offset = -(length as isize);
        unsafe {
            self.local = self.local.offset(offset);
        }
    }

    fn lget(&self, addr: u32) -> u32 {
        let offset = -1 - addr as isize;
        unsafe {
            *self.local.offset(offset)
        }
    }

    fn lset(&self, addr: u32, value: u32) {
        let offset = -1 - addr as isize;
        unsafe {
            *self.local.offset(offset) = value;
        }
    }
}
