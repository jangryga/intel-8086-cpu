pub struct Decoder;

use lowercase_display_derive::{LowercaseDisplay};

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

    pub fn decode_input(partial_representation: &ParsedInput) {

        let (field1, field2) = Self::get_fields(
            partial_representation.w, 
            partial_representation.mode, 
            partial_representation.rm,
            partial_representation.reg
        );

        let out = DecodedStream {
            opcode: interpret_opcode(&partial_representation.opcode).unwrap(),
            field1: field1,
            field2: field2
        };

        println!("{} {}, {}", out.opcode, out.field1, out.field2);
    }

    fn get_fields(w: u8, mode: u8, rm: u8, reg: u8) -> (FieldEncoding, FieldEncoding) {
        if mode == 3 {
            let mut field1: Option<FieldEncoding> = None;
            let mut field2: Option<FieldEncoding> = None;
            if w == 0 {
                match reg {
                    0 => field1 = Some(FieldEncoding::AL),
                    1 => field1 = Some(FieldEncoding::CL),
                    2 => field1 = Some(FieldEncoding::DL),
                    3 => field1 = Some(FieldEncoding::BL),
                    4 => field1 = Some(FieldEncoding::AH),
                    5 => field1 = Some(FieldEncoding::CH),
                    6 => field1 = Some(FieldEncoding::DH),
                    7 => field1 = Some(FieldEncoding::BH),
                    _ => (),
                }

                match rm {
                    0 => field2 = Some(FieldEncoding::AL),
                    1 => field2 = Some(FieldEncoding::CL),
                    2 => field2 = Some(FieldEncoding::DL),
                    3 => field2 = Some(FieldEncoding::BL),
                    4 => field2 = Some(FieldEncoding::AH),
                    5 => field2 = Some(FieldEncoding::CH),
                    6 => field2 = Some(FieldEncoding::DH),
                    7 => field2 = Some(FieldEncoding::BH),
                    _ => (),
                }
            } else {
                match reg {
                    0 => field1 = Some(FieldEncoding::AX),
                    1 => field1 = Some(FieldEncoding::CX),
                    2 => field1 = Some(FieldEncoding::DX),
                    3 => field1 = Some(FieldEncoding::BX),
                    4 => field1 = Some(FieldEncoding::SP),
                    5 => field1 = Some(FieldEncoding::BP),
                    6 => field1 = Some(FieldEncoding::SI),
                    7 => field1 = Some(FieldEncoding::DI),
                    _ => (),
                }
                match rm {
                    0 => field2 = Some(FieldEncoding::AX),
                    1 => field2 = Some(FieldEncoding::CX),
                    2 => field2 = Some(FieldEncoding::DX),
                    3 => field2 = Some(FieldEncoding::BX),
                    4 => field2 = Some(FieldEncoding::SP),
                    5 => field2 = Some(FieldEncoding::BP),
                    6 => field2 = Some(FieldEncoding::SI),
                    7 => field2 = Some(FieldEncoding::DI),
                    _ => (),
                }
            }

            (field2.unwrap(), field1.unwrap())
        } else {
            todo!()
        }
        
    }
}



#[derive(LowercaseDisplay)]
pub enum FieldEncoding {
    AX,
    AL,
    AH,
    BX,
    BL,
    BH,
    CX,
    CL,
    CH,
    DX,
    DL,
    DH,
    DI,
    SI,
    SP,
    BP
}

pub struct DecodedStream {
    opcode: Opcode,
    field1: FieldEncoding,
    field2: FieldEncoding
}


#[derive(LowercaseDisplay)]
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



