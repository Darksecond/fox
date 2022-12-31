use winit::window::{Window, WindowBuilder};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;
use pixels::{SurfaceTexture, Pixels};

const PALETTE: [u32; 16] = [
    0x1a1c2c,
    0x5d275d,
    0xb13e53,
    0xef7d57,
    0xffcd75,
    0xa7f070,
    0x38b764,
    0x257179,
    0x29366f,
    0x3b5dc9,
    0x41a6f6,
    0x73eff7,
    0xf4f4f4,
    0x94b0c2,
    0x566c86,
    0x333c57,
];

pub struct Screen {
    pub vector: u32,
    width: u32,
    height: u32,
    layers: [Vec<u8>; 2],
    palette: [u32; 16],

    window: Option<Window>,
    pixels: Option<Pixels>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            vector: 0,
            width: 64*8,
            height: 40*8,
            layers: [
                Vec::new(),
                Vec::new(),
            ],
            palette: PALETTE,

            window: None,
            pixels: None,
        }
    }

    pub fn init(&mut self, event_loop: &EventLoopWindowTarget<()>) {
        // Configure layers
        {
            let size = (self.width * self.height) / 2;
            self.layers[0].resize(size as _, 0);
            self.layers[1].resize(size as _, 0);
        }

        let size = LogicalSize::new(self.width as f64, self.height as f64);
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(size)
            .with_title("Fox")
            .build(event_loop)
            .expect("Could not construct window");


        let window_size = window.inner_size();
        let texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(self.width, self.height, texture).expect("Could not pixels");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    pub fn render(&mut self) {
        let pixels = self.pixels.as_mut().unwrap();

        let frame = pixels.get_frame_mut();

        let a = self.layers[0].iter().flat_map(|pp| {
            [pp >> 4, pp & 0x0F]
        });

        let b = self.layers[1].iter().flat_map(|pp| {
            [pp >> 4, pp & 0x0F]
        });

        for (index, (bg, fg)) in a.zip(b).enumerate() {
            let color = if fg == 0 {
                self.palette[bg as usize]
            } else {
                self.palette[fg as usize]
            };
            let findex = index * 4;
            let [b,g,r,_] = u32::to_le_bytes(color);
            frame[findex + 0] = r;
            frame[findex + 1] = g;
            frame[findex + 2] = b;
            frame[findex + 3] = 0xFF;
        }

        pixels.render().unwrap();
        self.window.as_ref().unwrap().request_redraw();
    }

    fn resize(&mut self) {
        let window = self.window.as_ref().unwrap();
        let pixels = self.pixels.as_mut().unwrap();

        let size = LogicalSize::new(self.width as f64, self.height as f64);
        window.set_min_inner_size(Some(size));
        window.set_max_inner_size(Some(size));
        window.set_inner_size(size);

        let window_size = window.inner_size();
        pixels.resize_buffer(self.width, self.height).unwrap();
        pixels.resize_surface(window_size.width, window_size.height).unwrap();

        {
            let size = (self.width * self.height) / 2;
            self.layers[0].resize(size as _, 0);
            self.layers[1].resize(size as _, 0);
        }
    }
}

impl Screen {
    pub fn write_u32(&mut self, addr: u32, value: u32) {
        use fox_bytecode::memory::*;

        match addr {
            SCREEN_VECTOR => {
                self.vector = value;
            },
            SCREEN_WIDTH => {
                self.width = u32::max(value, 8);
                self.resize();
            },
            SCREEN_HEIGHT => {
                self.height = u32::max(value, 8);
                self.resize();
            },
            SCREEN_PALETTE0..=SCREEN_PALETTE15 => {
                let index = addr - SCREEN_PALETTE0;
                self.palette[index as usize] = value;
            },
            _ => unimplemented!(),
        }
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        use fox_bytecode::memory::*;

        match addr {
            SCREEN_VECTOR => {
                self.vector
            },
            SCREEN_WIDTH => {
                self.width
            },
            SCREEN_HEIGHT => {
                self.height
            },
            SCREEN_PALETTE0..=SCREEN_PALETTE15 => {
                let index = addr - SCREEN_PALETTE0;
                self.palette[index as usize]
            },
            _ => unimplemented!(),
        }
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) {
        use fox_bytecode::memory::*;

        match addr {
            SCREEN_LAYER0..=SCREEN_LAYER0_TOP => {
                let index = addr - SCREEN_LAYER0;
                self.layers[0][index as usize] = value;
            },
            SCREEN_LAYER1..=SCREEN_LAYER1_TOP => {
                let index = addr - SCREEN_LAYER1;
                self.layers[1][index as usize] = value;
            },
            _ => unimplemented!(),
        }
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        use fox_bytecode::memory::*;

        match addr {
            SCREEN_LAYER0..=SCREEN_LAYER0_TOP => {
                let index = addr - SCREEN_LAYER0;
                self.layers[0][index as usize]
            },
            SCREEN_LAYER1..=SCREEN_LAYER1_TOP => {
                let index = addr - SCREEN_LAYER1;
                self.layers[1][index as usize]
            },
            _ => unimplemented!(),
        }
    }
}
