use super::Device;
use crate::DirectMemoryAccess;
use fox_bytecode::memory::{KEYBOARD_BASE, keyboard::*};

pub enum Key {
    Left,
    Right,
    Up,
    Down,
}

impl Key {
    pub fn flag(&self) -> u32 {
        match self {
            Key::Left  => BUTTON_LEFT,
            Key::Right => BUTTON_RIGHT,
            Key::Up    => BUTTON_UP,
            Key::Down  => BUTTON_DOWN,
        }
    }
}

pub struct KeyboardDevice {
    pub vector: u32,
    buttons: u32,
    codepoint: u32,
}

impl KeyboardDevice {
    pub fn new() -> Self {
        Self {
            vector: 0,
            buttons: 0,
            codepoint: 0,
        }
    }

    pub fn on_key(&mut self, key: Key, pressed: bool) {
        let mask = !key.flag();

        self.buttons = if pressed {
            self.buttons | key.flag()
        } else {
            self.buttons & mask
        };
    }

    pub fn on_char(&mut self, value: char) {
        self.codepoint = u32::from(value);
    }
}

impl Device for KeyboardDevice {
    fn read_u8(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u8 {
        let addr = addr - KEYBOARD_BASE;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, _value: u8, _dma: DirectMemoryAccess<'_>) {
        let addr = addr - KEYBOARD_BASE;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: DirectMemoryAccess<'_>) -> u32 {
        let addr = addr - KEYBOARD_BASE;

        match addr {
            VECTOR => self.vector,
            CODEPOINT => self.codepoint,
            BUTTONS => self.buttons,
            _ => unimplemented!(),
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, _dma: DirectMemoryAccess<'_>) {
        let addr = addr - KEYBOARD_BASE;

        match addr {
            VECTOR => self.vector = value,
            CODEPOINT => (),
            BUTTONS => (),
            _ => unimplemented!(),
        }
    }
}
