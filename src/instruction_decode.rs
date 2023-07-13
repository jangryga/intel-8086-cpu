pub struct Decoder;

use lowercase_display_derive::{LowercaseDisplay};
use std::collections::VecDeque;
use std::fs;
use std::io::Result;

#[derive(Debug)]
pub struct ParsedInput {
    pub opcode: u8,
    pub d: u8,
    pub w: u8,
    pub mode: u8,
    pub reg: u8,
    pub rm: u8
}


pub struct Parser {
    pub memory: VecDeque<u8>
}

enum Data {
    U8(u8),
    U16(u16)
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::U16(x) => write!(f, "{}", x),
            Data::U8(x) => write!(f, "{}", x)
        }
    }
}


impl Parser {
    pub fn load(&mut self, file_name: &str) -> Result<()> {
        let mut file_content = fs::read(file_name).expect("this should work");
        file_content.reverse();

        self.memory.extend(file_content);
        Ok(())
    }

    pub fn decode(&mut self) {

        while let Some(byte) = self.memory.pop_front() {
            let mut w: Option<u8> = None;
            // Get instruction Opcode
            let (opcode,  kind) = Self::match_opcode(&byte);
            if let Some(InstructionKind::FourBitOpcode) = kind {
                w = Some((&byte << 4) >> 7);
                let reg_numeric_value = (&byte << 5) >> 5;
                let reg: FieldEncoding = get_reg_field_encoding(&reg_numeric_value, &w.unwrap());
                let data: Option<Data> = match w {
                    Some(1) => {
                        let low = self.memory.pop_front().unwrap() as u16;
                        let high = self.memory.pop_front().unwrap() as u16;
                        Some(Data::U16((high << 8) | low))
                    },
                    Some(0) => Some(Data::U8(self.memory.pop_front().unwrap())),
                    _ => None
                };
                
                println!("{} {}, {}", opcode.unwrap(), reg, data.unwrap());
            }
            else if let Some(InstructionKind::SixBitOpcode) = kind {}
            else {
                panic!("Unrecognized instruction")
            }

        }
    }

    fn match_opcode(byte: &u8) -> (Option<Opcode>, Option<InstructionKind>) {
        match byte >> 2 {
            34 => (Some(Opcode::MOV), Some(InstructionKind::SixBitOpcode)),
            _ =>  match byte >> 4 {
                11 => (Some(Opcode::MOV), Some(InstructionKind::FourBitOpcode)),
                _ => (None, None)
            }
        }
    }

 }

enum InstructionKind {
    FourBitOpcode,
    SixBitOpcode
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
        // no displacement
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
            // 00 - no displacement - unless rm is set 110 - direct address with 16 bit displacement
            // 01 - 8 bit displacement
            // 10 - 16 bit displacement
            unimplemented!()
        }
        
    }
}

fn get_reg_field_encoding(reg: &u8, w: &u8) -> FieldEncoding {
    let mut field: Option<FieldEncoding> = None;
    match w {
        0 => field = match reg {
            0 => Some(FieldEncoding::AL),
            1 => Some(FieldEncoding::CL),
            2 => Some(FieldEncoding::DL),
            3 => Some(FieldEncoding::BL),
            4 => Some(FieldEncoding::AH),
            5 => Some(FieldEncoding::CH),
            6 => Some(FieldEncoding::DH),
            7 => Some(FieldEncoding::BH),
            _ => None,
        },
        1 => field = match reg {
            0 => Some(FieldEncoding::AX),
            1 => Some(FieldEncoding::CX),
            2 => Some(FieldEncoding::DX),
            3 => Some(FieldEncoding::BX),
            4 => Some(FieldEncoding::SP),
            5 => Some(FieldEncoding::BP),
            6 => Some(FieldEncoding::SI),
            7 => Some(FieldEncoding::DI),
            _ => None,
        },
        _ => panic!()
    }
    field.unwrap()
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



