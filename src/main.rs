use fake_cpu::memory::{load_memory_from_file, Memory};
use std::{env, path::{PathBuf}};

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
            Ok(bytes) => println!("read {} bytes", bytes),
            Err(e) => println!("{}", e),
        }
        for byte in memory.bytes {
            if byte != 0 {
                print!("{}", byte as char);
            }
        }
    }
}
