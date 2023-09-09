use static_assertions::const_assert;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::prelude::FileExt;
use std::path::Path;


pub const SIZE: usize = 1024 * 1024;
pub const MEMORY_ACCESS_MASK: usize = 0xfffff;

pub struct Memory {
    pub bytes: [u8; SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; SIZE]
        }
    }
}

pub struct SegmentAccess {
    pub segment_base: u16,
    pub segment_offset: u16,
}


pub fn get_absolute_address_of(segment_access: SegmentAccess, additional_offset: u16) -> u32 {
    let result = (((segment_access.segment_base as u32) << 4) + ((segment_access.segment_offset + additional_offset) as u32)) & MEMORY_ACCESS_MASK as u32;
    result
}

pub fn read_memory(memory: &Memory, absolute_address: u32) -> u8 {
    assert!(memory.bytes.len() as u32 > absolute_address);
    memory.bytes[absolute_address as usize]
}

pub fn load_memory_from_file(file_path: &Path, memory: &mut Memory, at_offset: usize) -> std::io::Result<u32> {
    match File::open(file_path) {
        Ok(file) => {
            let bytes_read = file.read_at(&mut memory.bytes, at_offset as u64)?;
            Ok(bytes_read as u32)
        },
        Err(e) => {
            writeln!(io::stderr(), "ERROR: Unable to open {:?}.", file_path).ok();
            Err(e)
        }
    }
}