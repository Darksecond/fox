use fox_vm::{VirtualMachine, Machine, DirectMemoryAccess};
use fox_bytecode::memory::*;
use fox_vm::device::{Device, match_device, FileDevice, SystemDevice, ConsoleDevice};

struct ConsoleMachine {
    console: ConsoleDevice,
    system: SystemDevice,
    file0: FileDevice,
    file1: FileDevice,
}

impl ConsoleMachine {
    const DEVICES: [(u32, u32); 4] = [
        (SYSTEM_BASE , DEVICE_LENGTH), // 0
        (CONSOLE_BASE, DEVICE_LENGTH), // 1
        (FILE0_BASE  , DEVICE_LENGTH), // 2
        (FILE1_BASE  , DEVICE_LENGTH), // 3
    ];

    fn device(&mut self, num: u32) -> &mut dyn Device {
        match num {
            0 => &mut self.system,
            1 => &mut self.console,
            2 => &mut self.file0,
            3 => &mut self.file1,
            _ => unimplemented!(),
        }
    }

    pub fn new() -> Self {
        Self {
            console: ConsoleDevice::new(),
            system: SystemDevice::new(),
            file0: FileDevice::new(FILE0_BASE),
            file1: FileDevice::new(FILE1_BASE),
        }
    }
}

impl Machine for ConsoleMachine {
    fn write_u32(&mut self, addr: u32, value: u32, dma: DirectMemoryAccess<'_>) {
        let device = self.device(match_device(Self::DEVICES, addr));
        device.write_u32(addr, value, dma)
    }

    fn read_u32(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u32 {
        let device = self.device(match_device(Self::DEVICES, addr));
        device.read_u32(addr, dma)
    }

    fn write_u8(&mut self, addr: u32, value: u8, dma: DirectMemoryAccess<'_>) {
        let device = self.device(match_device(Self::DEVICES, addr));
        device.write_u8(addr, value, dma)
    }

    fn read_u8(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u8 {
        let device = self.device(match_device(Self::DEVICES, addr));
        device.read_u8(addr, dma)
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Must have at least 1 argument");
        return;
    }

    let mut machine = ConsoleMachine::new();
    let mut vm = VirtualMachine::new();

    // Load rom
    {
        let data = std::fs::read(&args[1]).unwrap();
        vm.load(&data);
    }

    vm.run(&mut machine, RESET_VECTOR);

    loop {
        if let Some(exit) = machine.system.exit {
            std::process::exit(exit as _);
        }

        if machine.console.read_block() {
            let vector = machine.console.vector;
            if vector != 0 {
                vm.run(&mut machine, vector);
            }
        }
    }
}
