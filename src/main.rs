use fake_cpu::instruction_decode::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut file_content = fs::read(file_name).expect("this should work");
    file_content.reverse();

    let mut p = Decoder::new();
    let _ = p.load(file_name);
    p.dump_memory();
    p.decode();
    p.execute();
}
