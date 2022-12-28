use fox_bytecode::*;

pub trait Machine {
    fn write(&mut self, addr: u32, value: u32);
    fn read(&mut self, addr: u32) -> u32; 
}

pub const BASE: usize = 0x100; // Reset vector

const MEM_SIZE: usize = 16 * 1024 * 1024; // 16 Megabytes
const SP_SIZE: usize = 1024;
const RP_SIZE: usize = 1024;

const SP_OFFSET: usize = MEM_SIZE - SP_SIZE;
const RP_OFFSET: usize = MEM_SIZE - SP_SIZE - RP_SIZE;

pub struct VirtualMachine {
    mem: Box<[u8]>,
    ip: *const u8,
    sp: *mut u32,
    rp: *mut u32,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut mem = vec![0; MEM_SIZE].into_boxed_slice();
        let ip = unsafe { mem.as_ptr().offset(BASE as _) };
        let sp = unsafe { mem.as_mut_ptr().offset(SP_OFFSET as _) };
        let rp = unsafe { mem.as_mut_ptr().offset(RP_OFFSET as _) };

        Self {
            mem,
            ip,
            sp: sp as _,
            rp: rp as _,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.mem[BASE..BASE+data.len()].copy_from_slice(data);
    }

    pub fn run(&mut self, machine: &mut dyn Machine, ip: usize) {
        self.ip = unsafe { self.mem.as_ptr().offset(ip as _) };

        loop {
            //TODO Add flag to enable/disable
            //self.dump();

            match self.next_u8() {
                OP_HALT => {
                    break;
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
                OP_ADD => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a.wrapping_add(b));
                },
                OP_INC => {
                    let a = self.pop();
                    self.push(a.wrapping_add(1));
                }

                OP_SW => {
                    let addr = self.pop();
                    let value = self.pop();

                    if addr < MEM_SIZE as _ {
                        let addr = addr as usize;
                        let [a,b,c,d] = u32::to_le_bytes(value);
                        self.mem[addr + 0] = a;
                        self.mem[addr + 1] = b;
                        self.mem[addr + 2] = c;
                        self.mem[addr + 3] = d;
                    } else {
                        machine.write(addr, value);
                    }
                },
                OP_LB => {
                    let addr = self.pop();

                    let value = if addr < MEM_SIZE as _ {
                        self.mem[addr as usize] as _
                    } else {
                        machine.read(addr)
                    };

                    self.push(value);
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
                x => unimplemented!("0x{:x}", x),
            }
        }
    }

    fn dump(&self) {
        unsafe {
            let ip = self.mem.as_ptr();
            let mut sp = self.mem.as_ptr().offset(SP_OFFSET as _) as *const u32;
            let mut rp = self.mem.as_ptr().offset(RP_OFFSET as _) as *const u32;

            println!("IP: 0x{:08x}", self.ip.offset_from(ip));
            println!("INST: 0x{:02x}", *self.ip);

            print!("SP: ");
            while sp < self.sp as *const u32 {
                print!("0x{:08x} ", *sp);
                sp = sp.offset(1);
            }
            println!();

            print!("RP: ");
            while rp < self.rp as *const u32 {
                print!("0x{:08x} ", *rp);
                rp = rp.offset(1);
            }
            println!();

            println!();
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
        //TODO Add overflow check
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
}
