pub struct Decoder;

pub struct ParsedOutput {
    pub opcode: Opcode,
    pub direction: Destination,
    pub operation: Operation,
    pub mode: Mode,
    pub reg: u8,
}

impl Decoder {
    pub fn read_instruction_stream(input: &Vec<u8>) -> ParsedOutput {
        ParsedOutput {
            opcode: get_opcode(&input[0]).unwrap(),
            direction: Destination::FromReg,
            operation: Operation::Byte,
            mode: get_mode(&input[1]).unwrap(),
            reg: get_reg(&input[1])
        }
    }
}


pub enum Opcode {
    MOV,
}

pub enum Destination {
    FromReg,
    ToReg
}
pub enum Operation {
    Word,
    Byte
}

#[derive(Debug, PartialEq)]
pub enum Displacement {
    No,
    EightBit,
    SixteenBit
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Register,
    Memory(crate::instruction_decode::Displacement),
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Opcode::MOV => write!(f, "MOV")
        }
    }
}

pub fn get_opcode(byte: &u8) -> Option<Opcode> {
    match byte >> 2 {
        34 =>  Some(Opcode::MOV),
        _ => None
    }
}



pub fn get_mode(byte: &u8) -> Option<Mode> {
    match byte >> 6 {
        0 => Some(Mode::Memory(Displacement::No)),
        1 => Some(Mode::Memory(Displacement::EightBit)),
        2 => Some(Mode::Memory(Displacement::SixteenBit)),
        3 => Some(Mode::Register),
        _ => None
    }
}

pub fn get_reg(byte: &u8) -> u8 {
    (byte << 2) >> 5
}

