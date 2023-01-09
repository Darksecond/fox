use crate::DirectMemoryAccess;
use fox_bytecode::memory::{screen, screen::command};
use super::Device;

struct Sprite {
    data: [u8; 64],
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
        }
    }

    pub fn read_1bpp(&mut self, addr: u32, fg: u8, bg: u8, dma: &mut DirectMemoryAccess) {
        let mut data = [0; 8];
        dma.read(addr, &mut data);

        let src = data.iter().flat_map(|byte| {
            [
                (byte >> 7) & 0b1,
                (byte >> 6) & 0b1,
                (byte >> 5) & 0b1,
                (byte >> 4) & 0b1,
                (byte >> 3) & 0b1,
                (byte >> 2) & 0b1,
                (byte >> 1) & 0b1,
                (byte >> 0) & 0b1,
            ]
        });

        for (src, dst) in src.zip(self.data.iter_mut()) {
            *dst = if src > 0 { fg } else { bg };
        }
    }

    pub fn read_4bpp(&mut self, addr: u32, dma: &mut DirectMemoryAccess) {
        let mut data = [0; 32];
        dma.read(addr, &mut data);

        let src = data.iter().flat_map(|pp| {
            [pp >> 4, pp & 0x0F]
        });

        for (src, dst) in src.zip(self.data.iter_mut()) {
            *dst = src;
        }
    }

    pub fn draw<T: Display>(&self, layer: u8, x: u32, y: u32, screen: &mut ScreenDevice<T>) {
        for sy in 0..8u32 {
            for sx in 0..8u32 {
                let index = sy*8+sx;
                let pixel = self.data[index as usize];
                screen.set_pixel(layer, x+sx, y+sy, pixel);
            }
        }
    }
}

pub trait Display {
    /// Resize display.
    /// Returns a tuple of width, height, zoom
    fn resize(&mut self, width: u32, height: u32, zoom: u32) -> (u32, u32, u32);
    fn render(&mut self, buffer: &[u8], palette: &[u32; 16]);
}

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

pub struct ScreenDevice<D: Display> {
    pub display: D,
    pub vector: u32,
    width: u32,
    height: u32,
    composited_layer: Vec<u8>,
    layers: [Vec<u8>; 2],
    palette: [u32; 16],
    cmd_length: u32,
    zoom: u32,
}

impl<D: Display> ScreenDevice<D> {
    pub fn new(display: D) -> Self {
        let mut screen = Self {
            display,
            vector: 0,
            width: 64 * 8,
            height: 40 * 8,
            composited_layer: Vec::new(),
            layers: [
                Vec::new(),
                Vec::new(),
            ],
            palette: PALETTE,
            cmd_length: 0,
            zoom: 1,
        };

        // Configure layers
        {
            let size = (screen.width * screen.height) / 2;
            screen.layers[0].resize(size as _, 0);
            screen.layers[1].resize(size as _, 0);
            screen.composited_layer.resize(size as _, 0);
        }

        screen
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn render(&mut self) {
        // Bg
        self.composited_layer.copy_from_slice(&self.layers[0]);

        // Fg
        for layer in &self.layers[1..] {
            for (dst, src) in self.composited_layer.iter_mut().zip(layer.iter().copied()) {
                if src > 0 {
                    *dst = src;
                }
            }
        }

        self.display.render(&self.composited_layer, &self.palette);
    }

    fn resize(&mut self) {
        (self.width, self.height, self.zoom) = self.display.resize(self.width, self.height, self.zoom);

        {
            let size = (self.width * self.height) / 2;
            self.layers[0].resize(size as _, 0);
            self.layers[1].resize(size as _, 0);
            self.composited_layer.resize(size as _, 0);
        }
    }

    fn set_pixel(&mut self, layer: u8, x: u32, y: u32, color: u8) {
        if x >= self.width || y >= self.height { return; }

        let color = color & 0x0F;
        let index = y * (self.width/2) + (x/2);
        let mut byte = self.layers[layer as usize][index as usize];
        if x & 1 == 1 {
            byte &= 0xF0;
            byte |= color;
        } else {
            byte &= 0x0F;
            byte |= color << 4;
        }
        self.layers[layer as usize][index as usize] = byte;
    }

    fn cmd_clear(&mut self, addr: u32, dma: &mut DirectMemoryAccess<'_>) {
        let layer = dma.read_u32(addr + command::COMMAND) & 0xF;
        self.layers[layer as usize].fill(0);
    }

    fn cmd_sprite1(&mut self, addr: u32, dma: &mut DirectMemoryAccess<'_>) {
        let mut x = dma.read_u32(addr + command::X);
        let mut y = dma.read_u32(addr + command::Y);
        let mut source = dma.read_u32(addr + command::SOURCE);

        let layer = (dma.read_u8(addr + command::COMMAND) & 0xF) as u8;
        let _flags = dma.read_u8(addr + command::FLAGS);
        let color = dma.read_u8(addr + command::COLOR);
        let repeat = dma.read_u8(addr + command::REPEAT);

        let fg = (color >> 4) as u8;
        let bg = (color & 0xF) as u8;
        let width = 1 + (repeat >> 4);
        let height = 1 + (repeat & 0xF);

        for _ in 0..height {
            for _ in 0..width {
                let mut sprite = Sprite::new();
                sprite.read_1bpp(source, fg, bg, dma);
                sprite.draw(layer, x, y, self);

                x += 8;
                source += 8;
            }

            y += 8;
        }
    }

    fn cmd_sprite4(&mut self, addr: u32, dma: &mut DirectMemoryAccess<'_>) {
        let mut x = dma.read_u32(addr + command::X);
        let mut y = dma.read_u32(addr + command::Y);
        let mut source = dma.read_u32(addr + command::SOURCE);

        let layer = (dma.read_u8(addr + command::COMMAND) & 0xF) as u8;
        let _flags = dma.read_u8(addr + command::FLAGS);
        let repeat = dma.read_u8(addr + command::REPEAT);

        let width = 1 + (repeat >> 4);
        let height = 1 + (repeat & 0xF);

        for _ in 0..height {
            for _ in 0..width {
                let mut sprite = Sprite::new();
                sprite.read_4bpp(source, dma);
                sprite.draw(layer, x, y, self);

                x += 8;
                source += 32;
            }

            y += 8;
        }
    }

    fn process_command(&mut self, addr: u32, dma: &mut DirectMemoryAccess<'_>) {
        let command = dma.read_u32(addr + command::COMMAND) >> 4;
        match command {
            0x00 => self.cmd_clear(addr, dma),
            0x01 => self.cmd_sprite1(addr, dma),
            0x02 => self.cmd_sprite4(addr, dma),
            _ => unimplemented!(),
        }
    }
}

impl<D: Display> Device for ScreenDevice<D> {
    fn write_u32(&mut self, addr: u32, value: u32, mut dma: DirectMemoryAccess<'_>) {
        match addr {
            screen::VECTOR => {
                self.vector = value;
            },
            screen::WIDTH => {
                self.width = value;
                self.resize();
            },
            screen::HEIGHT => {
                self.height = value;
                self.resize();
            },
            screen::CMD_LENGTH => {
                self.cmd_length = value;
            },
            screen::CMD_ADDR => {
                for index in 0..self.cmd_length {
                    self.process_command(value + index*16, &mut dma);
                }
            },
            screen::ZOOM => {
                self.zoom = value;
                self.resize();
            }
            screen::PALETTE0..=screen::PALETTE15 => {
                let index = addr - screen::PALETTE0;
                self.palette[index as usize] = value;
            },
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u32 {
        match addr {
            screen::VECTOR => {
                self.vector
            },
            screen::WIDTH => {
                self.width
            },
            screen::HEIGHT => {
                self.height
            },
            screen::CMD_LENGTH => {
                self.cmd_length
            },
            screen::ZOOM => {
                self.zoom
            },
            screen::PALETTE0..=screen::PALETTE15 => {
                let index = addr - screen::PALETTE0;
                self.palette[index as usize]
            },
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, value: u8, _dma: DirectMemoryAccess<'_>) {
        match addr {
            screen::LAYER0..=screen::LAYER0_TOP => {
                let index = addr - screen::LAYER0;
                self.layers[0][index as usize] = value;
            },
            screen::LAYER1..=screen::LAYER1_TOP => {
                let index = addr - screen::LAYER1;
                self.layers[1][index as usize] = value;
            },
            _ => unimplemented!(),
        }
    }

    fn read_u8(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u8 {
        match addr {
            screen::LAYER0..=screen::LAYER0_TOP => {
                let index = addr - screen::LAYER0;
                self.layers[0][index as usize]
            },
            screen::LAYER1..=screen::LAYER1_TOP => {
                let index = addr - screen::LAYER1;
                self.layers[1][index as usize]
            },
            _ => unimplemented!(),
        }
    }
}
