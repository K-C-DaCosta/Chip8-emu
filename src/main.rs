/// The framebuffer of chip-8 only has a bit-depth of 1 (monochrome)
#[derive(Copy, Clone, Default)]
pub struct FrameBuffer {
    pixels: [u64; 32],
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self { pixels: [0; 32] }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|row| *row = 0);
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        //clear bit first
        self.pixels[y] &= !(1 << x);
        //set bit with value
        self.pixels[y] |= ((value & 1) as u64) << x;
    }
    
    pub fn get(&mut self, x: usize, y: usize) -> bool {
        (self.pixels[y] & (1 << x)) != 0
    }
}

pub struct Cpu {
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    stack_pointer: u8,
    stack: [u8; 64],
    delay_timer: u8,
    sound_timer: u8,
    frame_buffer: FrameBuffer,
}

fn main() {
    println!("Hello, world!");
}
