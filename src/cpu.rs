use super::*;

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
        let opcode = Self::decode_opcode(self.get_opcode());

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
            .find_map(|&(OpcodeMask(bits), OpcodeExtraction(decoder))| {
                ((opcode.to_u16() & bits) != 0).then(|| decoder(opcode))
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
            (
                OpcodeMask(0x2000),
                OpcodeExtraction(|Opcode(code)| DecodedOp::CALL {
                    addr: (code & 0x0FFF),
                }),
            ),
            (
                OpcodeMask(0x3000),
                OpcodeExtraction(|code| DecodedOp::SkipNxtInstIfEq {
                    register: code.get_nibble(2),
                    byte: code.get_byte(0) as u8,
                }),
            ),
            (
                OpcodeMask(0x4000),
                OpcodeExtraction(|code| DecodedOp::SkipNxtInstIfNe {
                    register: code.get_nibble(2),
                    byte: code.get_byte(0) as u8,
                }),
            ),
            (
                OpcodeMask(0x5000),
                OpcodeExtraction(|code| DecodedOp::SkipNxtInstIfEqReg {
                    reg_a: code.get_nibble(1) as usize,
                    reg_b: code.get_nibble(2) as usize,
                }),
            ),
            (
                OpcodeMask(0x6000),
                OpcodeExtraction(|code| DecodedOp::Load {
                    reg: code.get_nibble(2) as usize,
                    value: code.get_byte(0) as u8,
                }),
            ),
            (
                OpcodeMask(0x7000),
                OpcodeExtraction(|code| DecodedOp::ADDb {
                    reg_a: code.get_nibble(2) as usize,
                    byte: code.get_byte(0) as u8,
                }),
            ),
        ]
    }
}
