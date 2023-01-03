use fox_vm::{VirtualMachine, Machine, DirectMemoryAccess};
use fox_bytecode::memory::*;
use winit::event_loop::EventLoop;
use winit::event::{Event, StartCause};
use fox_vm::device::{Device, match_device, FileDevice, SystemDevice, ConsoleDevice, ScreenDevice, MouseDevice};

struct ScreenMachine {
    system: SystemDevice,
    console: ConsoleDevice,
    screen: ScreenDevice,
    file0: FileDevice,
    file1: FileDevice,
    mouse: MouseDevice,
}

impl ScreenMachine {
    const DEVICES: [(u32, u32); 8] = [
        (SYSTEM_BASE  , DEVICE_LENGTH),       // 0
        (CONSOLE_BASE , DEVICE_LENGTH),       // 1
        (SCREEN_BASE  , DEVICE_LENGTH),       // 2
        (SCREEN_LAYER0, SCREEN_LAYER_LENGTH), // 3
        (SCREEN_LAYER1, SCREEN_LAYER_LENGTH), // 4
        (FILE0_BASE   , DEVICE_LENGTH),       // 5
        (FILE1_BASE   , DEVICE_LENGTH),       // 6
        (MOUSE_BASE   , DEVICE_LENGTH),       // 7
    ];

    fn device(&mut self, num: u32) -> &mut dyn Device {
        match num {
            0 => &mut self.system,
            1 => &mut self.console,
            2 => &mut self.screen,
            3 => &mut self.screen,
            4 => &mut self.screen,
            5 => &mut self.file0,
            6 => &mut self.file1,
            7 => &mut self.mouse,
            _ => unimplemented!(),
        }
    }

    pub fn new() -> Self {
        Self {
            system: SystemDevice::new(),
            console: ConsoleDevice::new(),
            screen: ScreenDevice::new(),
            file0: FileDevice::new(FILE0_BASE),
            file1: FileDevice::new(FILE1_BASE),
            mouse: MouseDevice::new(),
        }
    }
}

impl Machine for ScreenMachine {
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

pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Must have at least 1 argument");
        return;
    }

    let mut machine = ScreenMachine::new();
    let mut vm = VirtualMachine::new();

    // Load rom
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
            Event::WindowEvent { event: WindowEvent::CursorLeft { .. }, ..  } => {
                machine.mouse.set_entered(false);

                let vector = machine.mouse.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }
            },
            Event::WindowEvent { event: WindowEvent::CursorEntered { .. }, ..  } => {
                machine.mouse.set_entered(true);

                let vector = machine.mouse.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }
            },
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, ..  } => {
                let position = machine.screen.position(position);
                machine.mouse.set_position(position);

                let vector = machine.mouse.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }
            },
            Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, ..  } => {
                let pressed = match state {
                    winit::event::ElementState::Pressed => true,
                    winit::event::ElementState::Released => false,
                };

                match button {
                    winit::event::MouseButton::Left => machine.mouse.set_left(pressed),
                    winit::event::MouseButton::Right => machine.mouse.set_right(pressed),
                    winit::event::MouseButton::Middle => machine.mouse.set_middle(pressed),
                    winit::event::MouseButton::Other(_) => (),
                }

                let vector = machine.mouse.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }
            },
            _ => (),
        }

        if machine.console.read_nonblock() {
            let vector = machine.console.vector;
            if vector != 0 {
                vm.run(&mut machine, vector);
            }
        }

        if let Some(code) = machine.system.exit {
            control_flow.set_exit_with_code(code as _);
        }
    });
}
