use fox_vm::{VirtualMachine, Machine, DirectMemoryAccess};
use fox_bytecode::memory::*;

struct ConsoleMachine {
    console_vec: u32,
    console_read: u8,
}

impl ConsoleMachine {
    pub fn new() -> Self {
        Self {
            console_vec: 0,
            console_read: 0,
        }
    }
}

impl Machine for ConsoleMachine {
    fn write_u32(&mut self, addr: u32, value: u32, _dma: DirectMemoryAccess<'_>) {
        match addr {
            CONSOLE_VECTOR => {
                self.console_vec = value;
            },
            CONSOLE_WRITE => {
                use std::io::Write;

                print!("{}", value as u8 as char);
                std::io::stdout().flush().unwrap();
            },
            CONSOLE_ERROR => {
                use std::io::Write;

                eprint!("{}", value as u8 as char);
                std::io::stderr().flush().unwrap();
            },
            SYSTEM_EXIT => {
                std::process::exit(value as _);
            }
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u32 {
        match addr {
            CONSOLE_VECTOR => self.console_vec,
            CONSOLE_READ => self.console_read as _,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Must have 1 argument");
        return;
    }

    let mut stdin = std::io::stdin().lock();

    let mut machine = ConsoleMachine::new();

    let mut vm = VirtualMachine::new();

    {
        let data = std::fs::read(&args[1]).unwrap();
        vm.load(&data);
    }

    vm.run(&mut machine, RESET_VECTOR);

    loop {
        use std::io::Read;
        let mut buf = [0;1];
        if stdin.read(&mut buf).unwrap() > 0 {
            machine.console_read = buf[0];
            let vec = machine.console_vec;
            if vec != 0 {
                vm.run(&mut machine, vec);
            }
        }
    }
}
