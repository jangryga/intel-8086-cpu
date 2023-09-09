use crate::sim86_instruction_table;
use crate::sim86_memory::Memory;
use crate::sim86_memory::SegmentAccess;
// use std::sync::Mutex;

pub struct Context {
    offset: u32
}

// static CONTEXT: Mutex<Context> = Mutex::new(Context {
//     offset: 0
// });

// fn decode_instruction

pub fn dis_asm8086(memory: &Memory, dis_asm_byte_count: u32, segmented_access: SegmentAccess) {}