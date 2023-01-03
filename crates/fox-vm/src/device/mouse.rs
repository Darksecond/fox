use super::Device;
use fox_bytecode::memory::*;

pub struct MouseDevice {
    pub vector: u32,
    focus: bool,
    x: u32,
    y: u32,
    left: bool,
    right: bool,
    middle: bool,
}

impl MouseDevice {
    pub fn new() -> Self {
        Self {
            vector: 0,
            focus: false,
            x: 0,
            y: 0,
            left: false,
            right: false,
            middle: false,
        }
    }

    pub fn set_position(&mut self, xy: (u32, u32)) {
        self.x = xy.0;
        self.y = xy.1;
    }

    pub fn set_entered(&mut self, value: bool) {
        self.focus = value;
    }

    pub fn set_left(&mut self, pressed: bool) {
        self.left = pressed;
    }

    pub fn set_right(&mut self, pressed: bool) {
        self.right = pressed;
    }

    pub fn set_middle(&mut self, pressed: bool) {
        self.middle = pressed;
    }

    fn flags(&self) -> u32 {
        let mut flags = 0;

        if self.focus {
            flags |= MOUSE_FLAG_FOCUS;
        }

        flags
    }

    fn button(&self) -> u32 {
        let mut button = 0;


        if self.left {
            button |= MOUSE_BUTTON_LEFT;
        }

        if self.middle {
            button |= MOUSE_BUTTON_MIDDLE;
        }

        if self.right {
            button |= MOUSE_BUTTON_RIGHT;
        }

        button
    }
}

impl Device for MouseDevice {
    fn read_u8(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u8 {
        let addr = addr - MOUSE_BASE;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, _value: u8, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - MOUSE_BASE;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u32 {
        let addr = addr - MOUSE_BASE;

        match addr {
            MOUSE_VECTOR => self.vector,
            MOUSE_X => self.x,
            MOUSE_Y => self.y,
            MOUSE_FLAGS => self.flags(),
            MOUSE_BUTTON => self.button(),
            _ => unimplemented!(),
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - MOUSE_BASE;

        match addr {
            MOUSE_VECTOR => self.vector = value,
            _ => unimplemented!(),
        }
    }
}
