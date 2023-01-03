use std::fs::File;
use std::io::{Read, Write};
use super::Device;
use fox_bytecode::memory::*;

#[derive(PartialEq)]
enum Mode {
    Read,
    Write,
}

//TODO Consider combining Mode and File.
pub struct FileDevice {
    base: u32,
    file: Option<File>,
    filename: String,
    append: bool, //TODO Turn into Flags
    length: u32, //TODO drop field in favor of `self.buffer.len()`
    status: u32,
    buffer: Vec<u8>,
    mode: Mode,
}

impl FileDevice {
    pub fn new(base: u32) -> Self {
        Self {
            base,
            file: None,
            filename: String::new(),
            append: false,
            length: 0,
            status: 0,
            buffer: Vec::new(),
            mode: Mode::Read,
        }
    }

    fn write_from_buffer(&mut self) {
        use std::fs::OpenOptions;

        self.status = 0;

        if self.mode != Mode::Write || self.file.is_none() {
            self.file = None;
            self.mode = Mode::Write;

            let file = OpenOptions::new()
                .write(true)
                .append(self.append)
                .create(true)
                .open(&self.filename);

            let file = match file {
                Ok(file) => file,
                Err(_) => return,
            };

            self.file = Some(file);
        }

        let file = self.file.as_mut().unwrap();

        file.write_all(&self.buffer).unwrap();

        self.status = self.length;
    }

    fn read_into_buffer(&mut self) {
        use std::fs::OpenOptions;

        self.status = 0;

        if self.mode != Mode::Read || self.file.is_none() {
            self.file = None;
            self.mode = Mode::Read;

            let file = OpenOptions::new()
                .read(true)
                .open(&self.filename);

            let file = match file {
                Ok(file) => file,
                Err(_) => return,
            };

            self.file = Some(file);
        }

        let file = self.file.as_mut().unwrap();

        self.status = file.read(&mut self.buffer).unwrap() as _;
    }
}

impl Device for FileDevice {
    fn read_u8(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u8 {
        let addr = addr - self.base;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, _value: u8, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - self.base;

        match addr {
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u32 {
        let addr = addr - self.base;

        match addr {
            FILE_VECTOR => 0,
            FILE_FILENAME => 0,
            FILE_LENGTH => self.length,
            FILE_APPEND => if self.append { 1 } else { 0 },
            FILE_STATUS => self.status,
            FILE_READ => 0,
            FILE_WRITE => 0,
            _ => unimplemented!(),
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, mut dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - self.base;

        match addr {
            FILE_VECTOR => (),
            FILE_FILENAME => {
                self.file = None;
                self.filename = dma.read_str(value);
            },
            FILE_LENGTH => {
                self.length = value;
                self.buffer.resize(value as _, 0);
            },
            FILE_APPEND => {
                self.append = if value == 0x01 {
                    true
                } else {
                    false
                };
            }
            FILE_STATUS => (),
            FILE_READ => {
                self.read_into_buffer();
                dma.write(value, &self.buffer);
            },
            FILE_WRITE => {
                dma.read(value, &mut self.buffer);
                self.write_from_buffer();
            },
            _ => unimplemented!(),
        }
    }
}
