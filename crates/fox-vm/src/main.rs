use fox_vm::{VirtualMachine, BASE, Machine};

const CONSOLE_WRITE: u32 = 0x10000004;

struct ConsoleMachine;

impl Machine for ConsoleMachine {
    fn write(&mut self, addr: u32, value: u32) {
        if addr == CONSOLE_WRITE {
            print!("{}", value as u8 as char);
        } else {
            todo!()
        }
    }

    fn read(&mut self, _addr: u32) -> u32 {
        todo!()
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Must have 1 argument");
        return;
    }

    let mut machine = ConsoleMachine;

    let mut vm = VirtualMachine::new();

    {
        let data = std::fs::read(&args[1]).unwrap();
        vm.load(&data);
    }

    vm.run(&mut machine, BASE);
}
