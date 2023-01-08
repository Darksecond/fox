use fox_vm::{VirtualMachine, Machine, DirectMemoryAccess};
use fox_bytecode::memory::*;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::event::{Event, StartCause};
use fox_vm::device::{Device, match_device, FileDevice, SystemDevice, ConsoleDevice, ScreenDevice, MouseDevice, screen::Display};
use winit::window::{Window, WindowBuilder};
use winit::dpi::LogicalSize;
use softbuffer::GraphicsContext;
use fox_vm::device::keyboard::{Key, KeyboardDevice};
use winit::event::VirtualKeyCode;

struct PixelDisplay {
    width: u32,
    height: u32,
    zoom: u32,
    context: Option<GraphicsContext<Window>>,
}

impl PixelDisplay {
    pub fn new() -> Self {
        Self {
            context: None,
            width: 0,
            height: 0,
            zoom: 1,
        }
    }

    pub fn init(&mut self, event_loop: &EventLoopWindowTarget<()>, size: (u32, u32)) {
        let (width, height) = size;

        (self.width, self.height) = size;

        let size = LogicalSize::new(width as f64, height as f64);
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(size)
            .with_title("Fox")
            .build(event_loop)
            .expect("Could not construct window");


        let context = unsafe {
            GraphicsContext::new(window).unwrap()
        };

        self.context = Some(context);
    }

    pub fn position(&self, position: winit::dpi::PhysicalPosition<f64>) -> (u32, u32) {
        let context = self.context.as_ref().unwrap();
        let scale = context.window().scale_factor();
        let position = position.to_logical::<f64>(scale);
        let x = (position.x / self.zoom as f64) as u32;
        let y = (position.y / self.zoom as f64) as u32;

        (x, y)
    }
}

impl Display for PixelDisplay {
    fn resize(&mut self, width: u32, height: u32, zoom: u32) -> (u32, u32, u32) {
        let context = self.context.as_ref().unwrap();
        let width = u32::max(8, width);
        let height = u32::max(8, height);
        let zoom = u32::max(1, zoom);

        self.width = width;
        self.height = height;
        self.zoom = zoom;

        let window = context.window();

        let size = LogicalSize::new((width * zoom) as f64, (height * zoom) as f64);
        window.set_min_inner_size(Some(size));
        window.set_max_inner_size(Some(size));
        window.set_inner_size(size);

        (width, height, zoom)
    }

    fn render(&mut self, buffer: &[u8], palette: &[u32; 16]) {
        let context = self.context.as_mut().unwrap();
        let mut frame = Vec::new();

        let scale_factor = context.window().scale_factor(); // is 2.0 on my mac
        let window_size = context.window().inner_size();

        let scale = window_size.width / self.width / scale_factor as u32;

        let size = self.width * scale * self.height * scale;

        frame.resize(size as _, 0);

        let it = buffer.iter().flat_map(|pp| {
            [pp >> 4, pp & 0x0F]
        });

        let mut ay = 0;
        let mut ax = 0;

        for color in it {
            let color = palette[color as usize];

            let unscaled_x = ax;
            let unscaled_y = ay;

            ax += 1;
            if ax == self.width {
                ax = 0;
                ay += 1;
            }

            let x = unscaled_x * scale;
            let y = unscaled_y * scale;
            let width = self.width * scale;
            let index = y * width + x;

            for sy in 0..scale {
                for sx in 0..scale {
                    let local_index = sy*width+sx;
                    frame[index as usize + local_index as usize] = color;
                }
            }
        }

        context.set_buffer(&frame, (self.width * scale) as _, (self.height * scale) as _);
        context.window().request_redraw();
    }
}

fn map_key(keycode: VirtualKeyCode) -> Option<Key> {
    match keycode {
        VirtualKeyCode::Left => Some(Key::Left),
        VirtualKeyCode::Right => Some(Key::Right),
        VirtualKeyCode::Up => Some(Key::Up),
        VirtualKeyCode::Down => Some(Key::Down),
        _ => None,
    }
}

struct ScreenMachine {
    system: SystemDevice,
    console: ConsoleDevice,
    screen: ScreenDevice<PixelDisplay>,
    file0: FileDevice,
    file1: FileDevice,
    mouse: MouseDevice,
    keyboard: KeyboardDevice,
}

impl ScreenMachine {
    const DEVICES: [(u32, u32); 9] = [
        (SYSTEM_BASE  , DEVICE_LENGTH),       // 0
        (CONSOLE_BASE , DEVICE_LENGTH),       // 1
        (SCREEN_BASE  , DEVICE_LENGTH),       // 2
        (screen::LAYER0, SCREEN_LAYER_LENGTH), // 3
        (screen::LAYER1, SCREEN_LAYER_LENGTH), // 4
        (FILE0_BASE   , DEVICE_LENGTH),       // 5
        (FILE1_BASE   , DEVICE_LENGTH),       // 6
        (MOUSE_BASE   , DEVICE_LENGTH),       // 7
        (KEYBOARD_BASE, DEVICE_LENGTH),       // 8
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
            8 => &mut self.keyboard,
            _ => unimplemented!(),
        }
    }

    pub fn new() -> Self {
        Self {
            system: SystemDevice::new(),
            console: ConsoleDevice::new(),
            screen: ScreenDevice::new(PixelDisplay::new()),
            file0: FileDevice::new(FILE0_BASE),
            file1: FileDevice::new(FILE1_BASE),
            mouse: MouseDevice::new(),
            keyboard: KeyboardDevice::new(),
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
                machine.screen.display.init(event_loop, machine.screen.size());

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
                let position = machine.screen.display.position(position);
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
            Event::WindowEvent { event: WindowEvent::ReceivedCharacter(character), .. } => {
                machine.keyboard.on_char(character);

                let vector = machine.keyboard.vector;
                if vector != 0 {
                    vm.run(&mut machine, vector);
                }
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    if let Some(key) = map_key(keycode) {
                        let pressed = match input.state {
                            winit::event::ElementState::Pressed => true,
                            winit::event::ElementState::Released => false,
                        };
                        machine.keyboard.on_key(key, pressed);

                        let vector = machine.keyboard.vector;
                        if vector != 0 {
                            vm.run(&mut machine, vector);
                        }
                    }
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
