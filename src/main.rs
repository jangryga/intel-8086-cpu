use std::fs;
use std::env;
use fake_cpu::instruction_decode::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut file_content = fs::read(file_name).expect("this should work");
    file_content.reverse();

    // 
    // let fake_instruction_stream: Vec<u8> = vec![184, 12, 240, 136, 229];

    // let mut p = Parser {
    //     memory: fake_instruction_stream.into()
    // };

    let mut p = Decoder::new();
    let _ = p.load(&file_name);
    p.dump_memory();
    p.decode();
    
}
