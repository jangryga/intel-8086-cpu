pub struct Decoder;

#[derive(Debug)]
pub struct ParsedInput {
    pub opcode: u8,
    pub d: u8,
    pub w: u8,
    pub mode: u8,
    pub reg: u8,
    pub rm: u8
}

impl Decoder {
    pub fn read_instruction_stream(input: &Vec<u8>) -> ParsedInput {
        ParsedInput {
            opcode: &input[0] >> 2,
            d: (&input[0] << 6) >> 7,
            w: (&input[0] << 7) >> 7,
            mode: &input[1] >> 6,
            reg: (&input[1] << 2) >> 5,
            rm: (&input[1] << 5 ) >> 5
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

pub fn interpret_opcode(opcode: &u8) -> Option<Opcode> {
    match opcode {
        34 =>  Some(Opcode::MOV),
        _ => None
    }
}

pub fn interpret_mode(mode: &u8) -> Option<Mode> {
    match mode {
        0 => Some(Mode::Memory(Displacement::No)),
        1 => Some(Mode::Memory(Displacement::EightBit)),
        2 => Some(Mode::Memory(Displacement::SixteenBit)),
        3 => Some(Mode::Register),
        _ => None
    }
}



