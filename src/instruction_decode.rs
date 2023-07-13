use lowercase_display_derive::{LowercaseDisplay};
use std::collections::VecDeque;
use std::fs;
use std::io::Result;

pub struct Decoder {
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


impl Decoder {
    pub fn new() -> Self {
        Decoder {
            memory: VecDeque::new()
        }
    }

    pub fn load(&mut self, file_name: &str) -> Result<()> {
        let mut file_content = fs::read(file_name).expect("this should work");
        // file_content.reverse();

        self.memory.extend(file_content);
        Ok(())
    }

    pub fn dump_memory(&self) {
        for i in  self.memory.iter() {
            print!("{:08b} ", i);
        }
        println!("")
    }

    pub fn decode(&mut self) {
        while let Some(byte) = self.memory.pop_front() {
            let mut w: Option<u8> = None;
            let mut d: Option<u8> = None;
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
            else if let Some(InstructionKind::SixBitOpcode) = kind {
                d = Some((&byte << 6 ) >> 7);
                w = Some((&byte << 7) >> 7);
                // finished with the first byte, I now know opcode, d, w
                // load second byte
                // now get the mod 
                let second_byte: u8 = self.memory.pop_front().unwrap();
                let mode: u8 = second_byte >> 6;

                // 00 - memory mode, no displacement (unless rm = 110 - direct address)
                // 01 - memory mode, 8 bit 
                // 10 - memory mode, 16 bit
                // 11 - register mode
                match mode {
                    0 => {},
                    1 => {},
                    2 => {},
                    3 => {
                        // TODO: can these be abstracted?
                        let reg = (second_byte << 2) >> 5;
                        let rm = (second_byte << 5) >> 5;
                        let field1 = get_rm_field_encoding(&rm, &w.unwrap());
                        let field2 = get_reg_field_encoding(&reg, &w.unwrap());
                        println!("{} {}, {}", opcode.unwrap(), field1, field2);
                    }
                    _ => ()
                }
            }
            else {
                println!("{}", byte);
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
        _ => ()
    }
    field.unwrap()
}

fn get_rm_field_encoding(rm: &u8, w: &u8) -> FieldEncoding {
    let mut field: Option<FieldEncoding> = None;
    match w {
        0 => field = match rm {
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
        1 => field = match rm {
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
        _ => ()
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



#[derive(LowercaseDisplay)]
pub enum Opcode {
    MOV,
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
#[deprecated]
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



