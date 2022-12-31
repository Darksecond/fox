use fox_vm::{VirtualMachine, Machine, DirectMemoryAccess};
use fox_vm::screen::Screen;
use fox_bytecode::memory::*;
use winit::event_loop::EventLoop;
use winit::event::{Event, StartCause};

struct ScreenMachine {
    screen: Screen,
    system_exit: Option<u32>,
}

impl ScreenMachine {
    pub fn new() -> Self {
        Self {
            screen: Screen::new(),
            system_exit: None,
        }
    }
}

const fn is_screen_addr(addr: u32) -> bool {
    use fox_bytecode::memory::*;
    (addr >= SCREEN_VECTOR && addr <= SCREEN_TOP) || (addr >= SCREEN_LAYER0 && addr <= SCREEN_LAYER1_TOP)
}

impl Machine for ScreenMachine {
    fn write_u32(&mut self, addr: u32, value: u32, _dma: DirectMemoryAccess<'_>) {
        match addr {
            SYSTEM_EXIT => {
                self.system_exit = Some(value);
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
            _ if is_screen_addr(addr) => {
                self.screen.write_u32(addr, value);
            },
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u32 {
        match addr {
            _ if is_screen_addr(addr) => {
                self.screen.read_u32(addr)
            },
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, value: u8, _dma: DirectMemoryAccess<'_>) {
        match addr {
            _ if is_screen_addr(addr) => {
                self.screen.write_u8(addr, value);
            },
            _ => unimplemented!(),
        }
    }

    fn read_u8(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u8 {
        match addr {
            _ if is_screen_addr(addr) => {
                self.screen.read_u8(addr)
            },
            _ => unimplemented!(),
        }
    }
}

pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Must have 1 argument");
        return;
    }

    let mut machine = ScreenMachine::new();
    let mut vm = VirtualMachine::new();

    {
        let data = std::fs::read(&args[1]).unwrap();
        vm.load(&data);
    }

    let event_loop = EventLoop::new();

    event_loop.run(move |event, event_loop, control_flow| {
        use winit::event::WindowEvent;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                control_flow.set_exit();
                return;
            },
            Event::NewEvents(StartCause::Init) => {
                machine.screen.init(event_loop);

                vm.run(&mut machine, RESET_VECTOR);
            },
            Event::RedrawRequested(_) => {
                let vector = machine.screen.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }

                machine.screen.render();
            },
            _ => (),
        }

        if let Some(code) = machine.system_exit {
            control_flow.set_exit_with_code(code as _);
        }
    });
}
