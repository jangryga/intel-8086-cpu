use fake_cpu::sim86_memory::{load_memory_from_file, Memory, SegmentAccess};
use std::{env, path::{PathBuf}};
use fake_cpu::sim86::{dis_asm8086};

fn main() {
    let mut memory = Memory::new();

    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        eprintln!("USAGE: {} [8086 machine code file] ...", args[0]);
        return
    }
    for file_name in &args[1..] {
        let mut path_buf: PathBuf = file_name.into();
        let mut bytes_read = load_memory_from_file(&path_buf,&mut memory, 0);

        match bytes_read {
            Ok(bytes) => {
                println!("read {} bytes", bytes);
                dis_asm8086(&memory, bytes, SegmentAccess {segment_base: 0, segment_offset: 0})
            },
            Err(e) => println!("{}", e),
        }
    }
}
