mod cpu;
mod framebuffer;

pub use cpu::*;
pub use framebuffer::*;

#[derive(Copy, Clone)]
struct Opcode(u16);
impl Opcode {
    pub fn get_nibble(&self,idx:u32)->u16{
        (self.0 >> (idx*4))*0xF
    }
    pub fn get_byte(&self,idx:u32)->u16{
        (self.0 >> (idx*8))*0xFF
    }
}

impl Opcode {
    pub fn to_u16(&self) -> u16 {
        self.0
    }
}

#[derive(Copy, Clone)]
struct OpcodeMask(u16);

#[derive(Copy, Clone)]
struct OpcodeExtraction(fn(Opcode) -> DecodedOp);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArithKind {
 
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DecodedOp {
    CLS,
    RET,
    JMP { addr: u16 },
    CALL { addr: u16 },
    SkipNxtInstIfEq { register: u16, byte: u8 },
    SkipNxtInstIfNe { register: u16, byte: u8 },
    SkipNxtInstIfEqReg { reg_a: usize, reg_b: usize },
    Load { reg: usize, value: u8 },
    ADDb { reg_a: usize, byte: u8 },
    OR { reg_a: usize, reg_b: usize },
    AND { reg_a: usize, reg_b: usize },
    XOR { reg_a: usize, reg_b: usize },
    ADD { reg_a: usize, reg_b: usize },
    SUB { reg_a: usize, reg_b: usize },
    SHR { reg_a: usize, reg_b: usize },
    SUBN { reg_a: usize, reg_b: usize },
    SHL { reg_a: usize, reg_b: usize },
    SNE { reg_a: usize, reg_b: usize },
    NOP,
}
