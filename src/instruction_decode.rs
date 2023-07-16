use lowercase_display_derive::LowercaseDisplay;
use std::collections::VecDeque;
use std::fs;
use std::io::Result;

pub struct Decoder {
    pub memory: VecDeque<u8>,
    pub intermediate_repr: Vec<DecodedMemField>
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            memory: VecDeque::new(),
            intermediate_repr: Vec::new()
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
            let mut w: Option<u8> = None;
            let mut d: Option<u8> = None;
            // Get instruction Opcode
            let (opcode,  kind) = Self::match_opcode(&byte);
            if let Some(InstructionKind::FourBitOpcode) = kind {
                w = Some((&byte << 4) >> 7);
                let reg_numeric_value = (&byte << 5) >> 5;
                let reg: Register = Decoder::get_register(&reg_numeric_value, &w.unwrap());
                let data: Option<RawData> = match w {
                    Some(1) => {
                        let low = self.memory.pop_front().unwrap() as u16;
                        let high = self.memory.pop_front().unwrap() as u16;
                        Some(RawData::U16((high << 8) | low))
                    },
                    Some(0) => Some(RawData::U8(self.memory.pop_front().unwrap())),
                    _ => None
                };
                self.append_intermidiate_repr(d.as_ref(), opcode.unwrap(), FieldOrRawData::RawData(data.unwrap()), FieldOrRawData::FieldEncoding(FieldEncoding::Reg(reg)));
            }
            else if let Some(InstructionKind::SixBitOpcode) = kind {
                d = Some((&byte << 6 ) >> 7); 
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
                        let field1 = FieldEncoding::Reg(Decoder::get_register(&reg, &w.unwrap()));
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
                        self.append_intermidiate_repr(d.as_ref(), opcode.unwrap(), FieldOrRawData::FieldEncoding(field1), FieldOrRawData::FieldEncoding(field2.unwrap()));
                    },
                    1 => {
                        let field1 = FieldEncoding::Reg(Decoder::get_register(&reg, &w.unwrap()));
                        let mut field2: Option<FieldEncoding> = None;
                        let third_byte = self.memory.pop_front().unwrap() as i8;
                        match rm {
                            0 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::SI), third_byte as i16)),
                            1 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, Some(Register::DI), third_byte as i16)),
                            2 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::SI), third_byte as i16)),
                            3 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BP, Some(Register::DI), third_byte as i16)),
                            4 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::SI, None, third_byte as i16)),
                            5 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::DI, None, third_byte as i16)),
                            6 => field2 = Some(FieldEncoding::Indexed(Register::BP, None)),
                            7 => field2 = Some(FieldEncoding::IndexedDisplaced(Register::BX, None, third_byte as i16)),
                            _ => todo!()
                        }
                        self.append_intermidiate_repr(d.as_ref(), opcode.unwrap(), FieldOrRawData::FieldEncoding(field1), FieldOrRawData::FieldEncoding(field2.unwrap()));
                    },
                    2 => {
                        let field1 = FieldEncoding::Reg(Decoder::get_register(&reg, &w.unwrap()));
                        let mut field2: Option<FieldEncoding> = None;
                        let third_byte = self.memory.pop_front().unwrap();
                        let fourth_bute = self.memory.pop_front().unwrap();
                        let displacement = i16::from_le_bytes([third_byte, fourth_bute]);
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
                        self.append_intermidiate_repr(d.as_ref(), opcode.unwrap(), FieldOrRawData::FieldEncoding(field1), FieldOrRawData::FieldEncoding(field2.unwrap()));
                    },
                    3 => {
                        let field1 = FieldEncoding::Reg(Decoder::get_register(&rm, &w.unwrap()));
                        let field2 = FieldEncoding::Reg(Decoder::get_register(&reg, &w.unwrap()));
                        self.append_intermidiate_repr(d.as_ref(), opcode.unwrap(), FieldOrRawData::FieldEncoding(field2), FieldOrRawData::FieldEncoding(field1));
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

    pub fn execute(&self) {
        for val in self.intermediate_repr.iter() {
            println!("{}", val)
        }
    }

    fn append_intermidiate_repr(&mut self, d: Option<&u8>, opcode: Opcode, field1: FieldOrRawData, field2: FieldOrRawData) {
        match d {
            Some(1) => self.intermediate_repr.push(DecodedMemField { opcode: opcode, field_one: field1, field_two: field2 }),
            _ => self.intermediate_repr.push(DecodedMemField { opcode: opcode, field_one: field2, field_two: field1 }),
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
 }

enum InstructionKind {
    FourBitOpcode,
    SixBitOpcode
}

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

impl std::fmt::Display for FieldEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FieldEncoding::Reg(reg) => write!(f, "{}", reg),
            FieldEncoding::Indexed(reg1, reg2) => match reg2 {
                Some(reg2) => write!(f, "[{} + {}]", reg1, reg2),
                None => write!(f, "[{}]", reg1)
            },
            FieldEncoding::IndexedDisplaced(reg1, reg2, disp) => match reg2 {
                Some(reg2) => match disp {
                    disp if disp > &0 =>  write!(f, "[{} + {} + {}]", reg1, reg2, disp),
                    _ => write!(f, "[{} + {} - {}]", reg1, reg2, disp.abs())
                },
                None => match disp {
                    disp if disp > &0 =>  write!(f, "[{} + {}]", reg1, disp),
                    _ => write!(f, "[{} - {}]", reg1, disp.abs())
                }
            }
        }
    }
}

pub struct DecodedMemField {
    pub opcode: Opcode,
    pub field_one: FieldOrRawData,
    pub field_two: FieldOrRawData
}

impl std::fmt::Display for DecodedMemField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}, {}", self.opcode, self.field_one, self.field_two)
    }
}

pub enum FieldOrRawData {
    FieldEncoding(FieldEncoding),
    RawData(RawData),
}

impl std::fmt::Display for FieldOrRawData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldOrRawData::FieldEncoding(val) => write!(f, "{}", val),
            FieldOrRawData::RawData(val) => write!(f, "{}", val)
        }
    }
}

pub enum FieldEncoding {
    Reg(Register),
    Indexed(Register, Option<Register>), 
    IndexedDisplaced(Register, Option<Register>, i16)
}

pub enum RawData {
    U8(u8),
    U16(u16)
}

impl std::fmt::Display for RawData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawData::U16(x) => write!(f, "{}", x),
            RawData::U8(x) => write!(f, "{}", x)
        }
    }
}

#[derive(LowercaseDisplay)]
pub enum Opcode {
    MOV,
}
