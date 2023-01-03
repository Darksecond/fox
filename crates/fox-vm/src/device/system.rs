use super::Device;
use fox_bytecode::memory::*;
use std::vec::IntoIter;

fn args() -> IntoIter<u8> {
    let mut args: Vec<u8> = Vec::new();

    // Skip first 2 arguments since they're the name of the vm and the rom
    for arg in std::env::args().skip(2) {
        args.extend(arg.as_bytes());
        args.push(0);
    }

    args.into_iter()
}

pub struct SystemDevice {
    pub exit: Option<u32>,
    args: IntoIter<u8>,
}

impl SystemDevice {
    pub fn new() -> Self {
        Self {
            args: args(),
            exit: None,
        }
    }
}

impl Device for SystemDevice {
    fn read_u8(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u8 {
        let addr = addr - SYSTEM_BASE;

        match addr {
            SYSTEM_READ => self.args.next().unwrap_or(0),
            _ => unimplemented!("0x{:08x}", addr),
        }
    }

    fn write_u8(&mut self, addr: u32, value: u8, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - SYSTEM_BASE;

        match addr {
            _ => unimplemented!("0x{:08x}=0x{:02x}", addr, value),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u32 {
        let addr = addr - SYSTEM_BASE;

        match addr {
            SYSTEM_READ => self.args.next().unwrap_or(0) as _,
            _ => unimplemented!("0x{:08x}", addr),
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - SYSTEM_BASE;

        match addr {
            SYSTEM_EXIT => self.exit = Some(value),
            _ => unimplemented!("0x{:08x}=0x{:08x}", addr, value),
        }
    }
}
