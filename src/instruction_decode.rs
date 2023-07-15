use lowercase_display_derive::LowercaseDisplay;
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
        let file_content = fs::read(file_name).expect("this should work");
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
            println!("");
            println!("Trying to match byte: {:08b}", byte);
            let mut w: Option<u8> = None;
            // let mut d: Option<u8> = None;
            // Get instruction Opcode
            let (opcode,  kind) = Self::match_opcode(&byte);
            if let Some(InstructionKind::FourBitOpcode) = kind {
                w = Some((&byte << 4) >> 7);
                let reg_numeric_value = (&byte << 5) >> 5;
                let reg: Register = get_register(&reg_numeric_value, &w.unwrap());
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
                // d = Some((&byte << 6 ) >> 7); // TODO: not used for now
                w = Some((&byte << 7) >> 7);
                // finished with the first byte, I now know opcode, d, w
                // load second byte
                // now get the mod 
                let second_byte: u8 = self.memory.pop_front().unwrap();
                let mode: u8 = second_byte >> 6;
                let reg = (second_byte << 2) >> 5;
                let rm = (second_byte << 5) >> 5;
                // 00 - memory mode, no displacement (unless rm = 110 - direct address)
                // 01 - memory mode, 8 bit 
                // 10 - memory mode, 16 bit
                // 11 - register mode
                match mode {
                    0 => {
                        let field1 = FieldEncoding::Reg(get_register(&reg, &w.unwrap()));
                        let mut field2: Option<FieldEncoding> = None;
                        match rm {
                            0 => field2 = Some(FieldEncoding::Indexed(Register::BX, Some(Register::SI))),
                            1 => field2 = Some(FieldEncoding::Indexed(Register::BX, Some(Register::DI))),
                            2 => field2 = Some(FieldEncoding::Indexed(Register::BP, Some(Register::SI))),
                            3 => field2 = Some(FieldEncoding::Indexed(Register::BP, Some(Register::DI))),
                            4 => field2 = Some(FieldEncoding::Indexed(Register::SI, None)),
                            5 => field2 = Some(FieldEncoding::Indexed(Register::DI, None)),
                            6 => field2 = Some(FieldEncoding::Indexed(Register::BP, None)), // direct address
                            7 => field2 = Some(FieldEncoding::Indexed(Register::BX, None)),
                            _ => unimplemented!()
                        };
                        println!("{} {}, {}", opcode.unwrap(), field1, field2.unwrap());
                    },
                    1 => {
                        let field1 = FieldEncoding::Reg(get_register(&reg, &w.unwrap()));
                        let mut field2: Option<FieldEncoding> = None;
                        let third_byte = self.memory.pop_front().unwrap();
                        match rm {
                            0 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::SI), third_byte as u16)),
                            1 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::DI), third_byte as u16)),
                            2 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::SI), third_byte as u16)),
                            3 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::DI), third_byte as u16)),
                            4 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::SI, None, third_byte as u16)),
                            5 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::DI, None, third_byte as u16)),
                            6 => field2 = Some(FieldEncoding::Indexed(Register::BP, None)),
                            7 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, None, third_byte as u16)),
                            _ => todo!()
                        }
                        println!("{} {}, {}", opcode.unwrap(), field1, field2.unwrap());

                    },
                    2 => {
                        let field1 = FieldEncoding::Reg(get_register(&reg, &w.unwrap()));
                        let mut field2: Option<FieldEncoding> = None;
                        let third_byte = self.memory.pop_front().unwrap();
                        let fourth_bute = self.memory.pop_front().unwrap();
                        let displacement = u16::from_le_bytes([third_byte, fourth_bute]);
                        match rm {
                            0 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::SI), displacement)),
                            1 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::DI), displacement)),
                            2 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::SI), displacement)),
                            3 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::DI), displacement)),
                            4 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::SI, None, displacement)),
                            5 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::DI, None, displacement)),
                            6 => field2 = Some(FieldEncoding::Indexed(Register::BP, None)),
                            7 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, None, displacement)),
                            _ => todo!()
                        }
                        println!("{} {}, {}", opcode.unwrap(), field1, field2.unwrap());
                    },
                    3 => {
                        let field1 = FieldEncoding::Reg(get_register(&rm, &w.unwrap()));
                        let field2 = FieldEncoding::Reg(get_register(&reg, &w.unwrap()));
                        println!("{} {}, {}", opcode.unwrap(), field1, field2);
                    }
                    _ => ()
                }
            }
            else {
                println!("{:08b}", byte);
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


fn get_register(reg: &u8, w: &u8) -> Register {
    let mut field: Option<Register> = None;
    match w {
        0 => field = match reg {
            0 => Some(Register::AL),
            1 => Some(Register::CL),
            2 => Some(Register::DL),
            3 => Some(Register::BL),
            4 => Some(Register::AH),
            5 => Some(Register::CH),
            6 => Some(Register::DH),
            7 => Some(Register::BH),
            _ => None,
        },
        1 => field = match reg {
            0 => Some(Register::AX),
            1 => Some(Register::CX),
            2 => Some(Register::DX),
            3 => Some(Register::BX),
            4 => Some(Register::SP),
            5 => Some(Register::BP),
            6 => Some(Register::SI),
            7 => Some(Register::DI),
            _ => None,
        },
        _ => ()
    }
    field.unwrap()
}

// fn get_rm_field_encoding(rm: &u8, w: &u8) -> FieldEncoding {
//     let mut field: Option<Register> = None;
//     match w {
//         0 => field = match rm {
//                 0 => Some(Register::AL),
//                 1 => Some(Register::CL),
//                 2 => Some(Register::DL),
//                 3 => Some(Register::BL),
//                 4 => Some(Register::AH),
//                 5 => Some(Register::CH),
//                 6 => Some(Register::DH),
//                 7 => Some(Register::BH),
//                 _ => None,
//             },
//         1 => field = match rm {
//             0 => Some(Register::AX),
//             1 => Some(Register::CX),
//             2 => Some(Register::DX),
//             3 => Some(Register::BX),
//             4 => Some(Register::SP),
//             5 => Some(Register::BP),
//             6 => Some(Register::SI),
//             7 => Some(Register::DI),
//             _ => None,
//         },
//         _ => ()
//     }

//     field.unwrap()
// }

#[derive(LowercaseDisplay)]
pub enum Register {
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

pub enum FieldEncoding {
    Reg(Register),
    Indexed(Register, Option<Register>), // First register is base, second is index
    IndexedDisplaced(Register, Option<Register>, u16)
}

impl std::fmt::Display for FieldEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FieldEncoding::Reg(reg) => write!(f, "{}", reg),
            FieldEncoding::Indexed(reg1, reg2) => match reg2 {
                Some(reg2) => write!(f, "[{} + {}]", reg1, reg2),
                None => write!(f, "[{}]", reg1)
            } ,
            FieldEncoding::IndexedDisplaced(reg1, reg2, disp) => match reg2 {
                Some(reg2) =>  write!(f, "[{} + {} + {}]", reg1, reg2, disp),
                None => write!(f, "[{} + {}]", reg1, disp)
            }
        }
    }
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



