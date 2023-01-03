pub mod file;
pub mod system;
pub mod console;
pub mod screen;
pub mod mouse;

pub use file::FileDevice;
pub use system::SystemDevice;
pub use console::ConsoleDevice;
pub use screen::ScreenDevice;
pub use mouse::MouseDevice;

use crate::DirectMemoryAccess;

pub trait Device {
    fn read_u8(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u8;
    fn write_u8(&mut self, addr: u32, value: u8, dma: DirectMemoryAccess<'_>);

    fn read_u32(&mut self, addr: u32, dma: DirectMemoryAccess<'_>) -> u32;
    fn write_u32(&mut self, addr: u32, value: u32, dma: DirectMemoryAccess<'_>);
}

pub const fn match_device<const T: usize>(ranges: [(u32, u32); T], addr: u32) -> u32 {
    let mut i = 0;
    while i < T {
        let start = ranges[i].0;
        let end = start + ranges[i].1;

        if addr >= start && addr < end {
            return i as _;
        }

        i += 1;
    }

    u32::MAX
}
