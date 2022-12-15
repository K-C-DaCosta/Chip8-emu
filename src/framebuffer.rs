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