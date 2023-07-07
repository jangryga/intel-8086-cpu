use std::fs;
use std::env;
use fake_cpu::instruction_decode::*;

fn main() {

    // Each register - 16 bits 
    // moving memory then...
    // first - instruction decode
    // in instruction stream 
    // move - start by moving 
    //    des, src
    // MOV Ax, Bx        - X means copy the whole thing (so 16 bits) - L (only low bits) - H (only high bits)
    // 
    // 8 bit       8bit
    // 100010DW   mod reg rm
    // 6 bit      2   3   3  
    // 
    // mod - memory or register operation (eg 11 register to register)
    // reg - register
    // D - d == 0 ? reg is not destination : reg is destination
    // W (wide) - w == 0 ? 8 bits : 16 bits


    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let file_content = fs::read(file_name).expect("this should work");

    let first_val = file_content[0];

    let second_val = file_content[1];
    // let num: u8 = 137;
    println!("{:08b}", first_val);
    println!("{:08b}", second_val);


    // for byte in file_content {
    //     print!("{:08b} ", byte);
    // }
    // println!("");


    // let opcode = get_opcode(&first_val);

    // match opcode {
    //     Some(x) => println!("{}", x),
    //     None => println!("Opcode not recognized")
    // }

    // println!("{}", get_instruction(first_val).map(|x| x.to_string()).unwrap_or(String::from("Opcode not recognized")))

    
}
