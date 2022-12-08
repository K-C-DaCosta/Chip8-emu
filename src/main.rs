pub struct CompactFrameBuffer{
    
}

pub struct Cpu{
    registers:[u8;16],
    index_register:u16,
    stack:[u8;64],
    delay_timer:u8,
    sound_timer:u8,
    frame_buffer:[u64;32]
}



fn main() {
    println!("Hello, world!");
}
