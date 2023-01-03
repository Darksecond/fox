use super::Device;
use fox_bytecode::memory::*;
use std::sync::mpsc::Receiver;

fn init_receiver() -> Receiver<u8> {
    use std::sync::mpsc::channel;

    let (sender, receiver) = channel();

    std::thread::spawn(move || {
        use std::io::Read;

        let mut stdin = std::io::stdin().lock();
        let mut buf = [0;1];

        loop {
            if stdin.read(&mut buf).unwrap() > 0 {
                sender.send(buf[0]).unwrap();
            }
        }
    });

    receiver
}

pub struct ConsoleDevice {
    pub vector: u32,
    read: u8,
    receiver: Receiver<u8>,
}

impl ConsoleDevice {
    pub fn new() -> Self {
        Self {
            vector: 0,
            read: 0,
            receiver: init_receiver(),
        }
    }

    pub fn read_block(&mut self) -> bool {
        let value = self.receiver.recv().unwrap();
        self.read = value;
        true
    }

    pub fn read_nonblock(&mut self) -> bool {
        use std::sync::mpsc::TryRecvError;

        match self.receiver.try_recv() {
            Ok(value) => {
                self.read = value;
                true
            },
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => panic!("Receiver Disconnected"),
        }
    }

    fn write_stdout(&mut self, value: u8) {
        use std::io::Write;

        let mut stream = std::io::stdout();

        stream.write(&[value as u8]).unwrap();
        stream.flush().unwrap();
    }

    fn write_stderr(&mut self, value: u8) {
        use std::io::Write;

        let mut stream = std::io::stderr();

        stream.write(&[value as u8]).unwrap();
        stream.flush().unwrap();
    }
}

impl Device for ConsoleDevice {
    fn read_u8(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u8 {
        let addr = addr - CONSOLE_BASE;

        match addr {
            CONSOLE_READ => self.read,
            _ => unimplemented!(),
        }
    }

    fn write_u8(&mut self, addr: u32, value: u8, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - CONSOLE_BASE;

        match addr {
            CONSOLE_WRITE => self.write_stdout(value),
            CONSOLE_ERROR => self.write_stderr(value),
            _ => unimplemented!(),
        }
    }

    fn read_u32(&mut self, addr: u32, _dma: crate::DirectMemoryAccess<'_>) -> u32 {
        let addr = addr - CONSOLE_BASE;

        match addr {
            CONSOLE_VECTOR => self.vector,
            CONSOLE_READ => self.read as _,
            _ => unimplemented!(),
        }
    }

    fn write_u32(&mut self, addr: u32, value: u32, _dma: crate::DirectMemoryAccess<'_>) {
        let addr = addr - CONSOLE_BASE;

        match addr {
            CONSOLE_VECTOR => self.vector = value,
            CONSOLE_WRITE => self.write_stdout(value as _),
            CONSOLE_ERROR => self.write_stderr(value as _),
            _ => unimplemented!(),
        }
    }
}
