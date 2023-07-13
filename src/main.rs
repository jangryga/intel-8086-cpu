use std::fs;
use std::env;
use fake_cpu::instruction_decode::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let mut file_content = fs::read(file_name).expect("this should work");
    file_content.reverse();

    // while let (Some(y), Some(x)) = (file_content.pop(), file_content.pop()) {
    //     let temp = vec![y, x];
    //     let intermediate =  Decoder::read_instruction_stream(&temp);
    //     Decoder::decode_input(&intermediate);
    // }


    let instructions: Vec<u8> = vec![184, 12, 240];

    let mut p = Parser {
        memory: instructions.into()
    };

    p.decode()
    
}
