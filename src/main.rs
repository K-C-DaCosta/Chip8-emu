#[derive(Copy, Clone)]
enum DecodedOp {
    CLS,
    RET,
    JMP { addr: u16 },
    CALL { addr: u16 },
    SkipNextInstruction { register: u8, byte: u8 },
    NOP,
}

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
#[derive(Copy, Clone)]
struct Opcode(u16);

impl Opcode {
    pub fn to_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Copy, Clone)]
struct OpcodeMask(u16);

#[derive(Copy, Clone)]
struct OpcodeExtraction(fn(Opcode) -> DecodedOp);

pub struct Cpu {
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    stack_pointer: u8,
    stack: [u8; 64],
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    frame_buffer: FrameBuffer,
}
impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            stack_pointer: 0,
            stack: [0; 64],
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200,
            frame_buffer: FrameBuffer::new(),
        }
    }
    pub fn execute(&mut self) {
        let opcode = self.get_opcode();

        self.program_counter += 1;
    }

    fn get_opcode(&self) -> Opcode {
        let current_program_counter = self.program_counter as usize;
        Opcode(
            self.memory[current_program_counter + 0] as u16
                | ((self.memory[current_program_counter + 1] as u16) << 8),
        )
    }

    fn decode_opcode(opcode: Opcode) -> DecodedOp {
        Self::opcode_decoder_table()
            .iter()
            .find_map(|&(OpcodeMask(mask), OpcodeExtraction(decoder))| {
                ((opcode.to_u16() & mask) != 0).then(|| decoder(opcode))
            })
            .unwrap_or(DecodedOp::NOP)
    }

    fn opcode_decoder_table() -> &'static [(OpcodeMask, OpcodeExtraction)] {
        &[
            (OpcodeMask(0x00E0), OpcodeExtraction(|_| DecodedOp::CLS)),
            (OpcodeMask(0x00EE), OpcodeExtraction(|_| DecodedOp::RET)),
            (
                OpcodeMask(0x1000),
                OpcodeExtraction(|Opcode(code)| DecodedOp::JMP {
                    addr: (code & 0x0FFF),
                }),
            ),
        ]
    }
}

fn main() {
    println!("Hello, world!");
}
