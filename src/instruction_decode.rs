use lowercase_display_derive::LowercaseDisplay;
use std::collections::VecDeque;
use std::fs;
use std::io::Result;

pub struct Decoder {
    pub instruction_queue: VecDeque<u8>,
    pub intermediate_repr: Vec<DecodedMemField>,
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            instruction_queue: VecDeque::new(),
            intermediate_repr: Vec::new(),
        }
    }

    pub fn load(&mut self, file_name: &str) -> Result<()> {
        let file_content = fs::read(file_name).expect("this should work");
        // file_content.reverse();

        self.instruction_queue.extend(file_content);
        Ok(())
    }

    pub fn dump_memory(&self) {
        for i in self.instruction_queue.iter() {
            print!("{:08b} ", i);
        }
        println!("")
    }

    pub fn decode(&mut self) {
        while let Some(byte) = self.instruction_queue.pop_front() {
            let mut w: Option<u8> = None;
            let mut d: Option<u8> = None;
            let mut s: Option<u8> = None;
            // Get instruction Opcode
            let (mut opcode, kind) = Self::match_opcode(&byte);

            if let Some(InstructionKind::ImmediateToAccumulator) = kind {
                w = Some((&byte << 7) >> 7);
                let third_byte = self.instruction_queue.pop_front().unwrap();

                if w == Some(1) {
                    let fourth_byte = self.instruction_queue.pop_front().unwrap();
                    self.intermediate_repr.push(DecodedMemField {
                        opcode: opcode.unwrap(),
                        field_one: FieldOrRawData::FieldEncoding(
                            FieldEncoding::Reg(Register::AX),
                            None,
                        ),
                        field_two: FieldOrRawData::RawData(
                            RawData::U16(u16::from_le_bytes([third_byte, fourth_byte])),
                            None,
                        ),
                    })
                } else {
                    self.intermediate_repr.push(DecodedMemField {
                        opcode: opcode.unwrap(),
                        field_one: FieldOrRawData::FieldEncoding(
                            FieldEncoding::Reg(Register::AL),
                            None,
                        ),
                        field_two: FieldOrRawData::RawData(RawData::I8(third_byte as i8), None),
                    })
                }
            } else if let Some(InstructionKind::FourBitOpcode) = kind {
                w = Some((&byte << 4) >> 7);
                let reg_numeric_value = (&byte << 5) >> 5;
                let reg_field = Decoder::get_reg_field(&reg_numeric_value, &w.unwrap());
                let data: Option<RawData> = match w {
                    Some(1) => {
                        let low = self.instruction_queue.pop_front().unwrap() as u16;
                        let high = self.instruction_queue.pop_front().unwrap() as u16;
                        Some(RawData::U16((high << 8) | low))
                    }
                    Some(0) => Some(RawData::U8(self.instruction_queue.pop_front().unwrap())),
                    _ => None,
                };
                self.append_intermediate_repr(
                    d.as_ref(),
                    opcode.unwrap(),
                    FieldOrRawData::RawData(data.unwrap(), None),
                    FieldOrRawData::FieldEncoding(reg_field, None),
                );
            } else if let Some(InstructionKind::SixBitOpcode) = kind {
                d = Some((&byte << 6) >> 7);
                w = Some((&byte << 7) >> 7);
                // finished with the first byte, I now know opcode, d, w
                // load second byte
                // now get the mod
                let second_byte: u8 = self.instruction_queue.pop_front().unwrap();
                let mode: u8 = second_byte >> 6;
                let reg = (second_byte << 2) >> 5;
                let rm = (second_byte << 5) >> 5;
                // WILDCARD handling immediate to register / memory - if memory, there could be a displacement
                // if first byte is 100000SW, opcode is encoded by reg field
                if let Some(Opcode::WILDCARD) = opcode {
                    opcode = Decoder::match_wildcard_opcode(&reg);
                    s = Some((&byte << 6) >> 7);
                    d = None;
                }
                // 00 - memory mode, no displacement (unless rm = 110 - direct address)
                // 01 - memory mode, 8 bit
                // 10 - memory mode, 16 bit
                // 11 - register mode
                match mode {
                    0 => {
                        let mut field1: Option<FieldOrRawData> = None;
                        let mut explicit_size: Option<ExplicitSize> = None;
                        let field2 = Decoder::get_rm_field(&rm, None);
                        if let None = d {
                            let third_byte = self.instruction_queue.pop_front().unwrap();
                            if w == Some(1) && s == Some(0) {
                                // if s:w = 01 ??
                                explicit_size = Some(ExplicitSize::Word);
                                let fourth_byte = self.instruction_queue.pop_front().unwrap();
                                field1 = Some(FieldOrRawData::RawData(
                                    RawData::U16(u16::from_le_bytes([third_byte, fourth_byte])),
                                    None,
                                ))
                            } else {
                                explicit_size = Some(ExplicitSize::Byte);
                                field1 =
                                    Some(FieldOrRawData::RawData(RawData::U8(third_byte), None));
                            }
                        } else {
                            field1 = Some(FieldOrRawData::FieldEncoding(
                                Decoder::get_reg_field(&reg, &w.unwrap()),
                                None,
                            ));
                        }

                        self.append_intermediate_repr(
                            d.as_ref(),
                            opcode.unwrap(),
                            field1.unwrap(),
                            FieldOrRawData::FieldEncoding(field2, explicit_size),
                        );
                    }
                    1 => {
                        let third_byte = self.instruction_queue.pop_front().unwrap() as i16;

                        let field1 = Decoder::get_reg_field(&reg, &w.unwrap());
                        let field2 = Decoder::get_rm_field(&rm, Some(third_byte));

                        self.append_intermediate_repr(
                            d.as_ref(),
                            opcode.unwrap(),
                            FieldOrRawData::FieldEncoding(field1, None), //Todo handle immediate to reg
                            FieldOrRawData::FieldEncoding(field2, None),
                        );
                    }
                    2 => {
                        let mut explicit_size: Option<ExplicitSize> = None;
                        let third_byte = self.instruction_queue.pop_front().unwrap();
                        let fourth_byte = self.instruction_queue.pop_front().unwrap();
                        let displacement = i16::from_le_bytes([third_byte, fourth_byte]);

                        let mut field1: Option<FieldOrRawData> = None;
                        let field2 = Decoder::get_rm_field(&rm, Some(displacement));

                        if let None = d {
                            let fifth_byte = self.instruction_queue.pop_front().unwrap();
                            if w == Some(1) && s == Some(0) {
                                explicit_size = Some(ExplicitSize::Word);
                                let sixth_byte = self.instruction_queue.pop_front().unwrap();
                                field1 = Some(FieldOrRawData::RawData(
                                    RawData::U16(u16::from_le_bytes([fifth_byte, sixth_byte])),
                                    None,
                                ))
                            } else {
                                explicit_size = Some(ExplicitSize::Byte);
                                field1 =
                                    Some(FieldOrRawData::RawData(RawData::U8(fifth_byte), None));
                            }
                        } else {
                            field1 = Some(FieldOrRawData::FieldEncoding(
                                Decoder::get_reg_field(&reg, &w.unwrap()),
                                None,
                            ));
                        }

                        self.append_intermediate_repr(
                            d.as_ref(),
                            opcode.unwrap(),
                            field1.unwrap(),
                            FieldOrRawData::FieldEncoding(field2, explicit_size),
                        );
                    }
                    3 => {
                        let mut field1: Option<FieldOrRawData> = None;
                        let field2 = FieldOrRawData::FieldEncoding(
                            Decoder::get_reg_field(&rm, &w.unwrap()),
                            None,
                        );
                        if let None = d {
                            let third_byte = self.instruction_queue.pop_front().unwrap();
                            if w == Some(1) && s == Some(0) {
                                // if s:w = 01 ??
                                let fourth_byte = self.instruction_queue.pop_front().unwrap();
                                field1 = Some(FieldOrRawData::RawData(
                                    RawData::U16(u16::from_le_bytes([third_byte, fourth_byte])),
                                    None,
                                ))
                            } else {
                                field1 =
                                    Some(FieldOrRawData::RawData(RawData::U8(third_byte), None));
                            }
                        } else {
                            field1 = Some(FieldOrRawData::FieldEncoding(
                                Decoder::get_reg_field(&reg, &w.unwrap()),
                                None,
                            ));
                        }
                        self.append_intermediate_repr(
                            d.as_ref(),
                            opcode.unwrap(),
                            field1.unwrap(),
                            field2,
                        );
                    }
                    _ => (),
                }
            } else {
                println!("{:08b}", byte);
                println!("Memory dump before panic");
                self.execute();
                panic!("Unrecognized instruction")
            }
        }
    }

    pub fn execute(&self) {
        for val in self.intermediate_repr.iter() {
            println!("{}", val)
        }
    }

    fn append_intermediate_repr(
        &mut self,
        d: Option<&u8>,
        opcode: Opcode,
        field_reg: FieldOrRawData,
        field_rm: FieldOrRawData,
    ) {
        match d {
            Some(1) => self.intermediate_repr.push(DecodedMemField {
                opcode,
                field_one: field_reg,
                field_two: field_rm,
            }),
            // if d is not specified, the rm field is the destination (immediate to reg/memory) so reg field will be RawData
            _ => self.intermediate_repr.push(DecodedMemField {
                opcode,
                field_one: field_rm,
                field_two: field_reg,
            }),
        }
    }

    fn match_wildcard_opcode(reg: &u8) -> Option<Opcode> {
        match reg {
            0 => Some(Opcode::ADD),
            2 => Some(Opcode::ADC),
            5 => Some(Opcode::SUB),
            _ => None,
        }
    }

    fn match_opcode(byte: &u8) -> (Option<Opcode>, Option<InstructionKind>) {
        match byte >> 1 {
            2 => (
                Some(Opcode::ADD),
                Some(InstructionKind::ImmediateToAccumulator),
            ),
            10 => (
                Some(Opcode::ADC),
                Some(InstructionKind::ImmediateToAccumulator),
            ),
            22 => (
                Some(Opcode::SUB),
                Some(InstructionKind::ImmediateToAccumulator),
            ),
            _ => match byte >> 2 {
                0 => (Some(Opcode::ADD), Some(InstructionKind::SixBitOpcode)),
                32 => (Some(Opcode::WILDCARD), Some(InstructionKind::SixBitOpcode)),
                34 => (Some(Opcode::MOV), Some(InstructionKind::SixBitOpcode)),
                _ => match byte >> 4 {
                    11 => (Some(Opcode::MOV), Some(InstructionKind::FourBitOpcode)),
                    _ => (None, None),
                },
            },
        }
    }

    fn get_rm_field(rm: &u8, disp: Option<i16>) -> FieldEncoding {
        let mut rm_field: Option<FieldEncoding> = None;

        rm_field = match disp {
            None => match rm {
                0 => Some(FieldEncoding::Indexed(
                    Register::BX,
                    Some(Register::SI),
                    None,
                )),
                1 => Some(FieldEncoding::Indexed(
                    Register::BX,
                    Some(Register::DI),
                    None,
                )),
                2 => Some(FieldEncoding::Indexed(
                    Register::BP,
                    Some(Register::SI),
                    None,
                )),
                3 => Some(FieldEncoding::Indexed(
                    Register::BP,
                    Some(Register::DI),
                    None,
                )),
                4 => Some(FieldEncoding::Reg(Register::SI)),
                5 => Some(FieldEncoding::Reg(Register::DI)),
                6 => Some(FieldEncoding::Reg(Register::BP)),
                7 => Some(FieldEncoding::Reg(Register::BX)),
                _ => panic!("R/M out of range"),
            },
            _ => match rm {
                0 => Some(FieldEncoding::Indexed(
                    Register::BX,
                    Some(Register::SI),
                    disp,
                )),
                1 => Some(FieldEncoding::Indexed(
                    Register::BX,
                    Some(Register::DI),
                    disp,
                )),
                2 => Some(FieldEncoding::Indexed(
                    Register::BP,
                    Some(Register::SI),
                    disp,
                )),
                3 => Some(FieldEncoding::Indexed(
                    Register::BP,
                    Some(Register::DI),
                    disp,
                )),
                4 => Some(FieldEncoding::Indexed(Register::SI, None, disp)),
                5 => Some(FieldEncoding::Indexed(Register::DI, None, disp)),
                6 => Some(FieldEncoding::Indexed(Register::BP, None, disp)),
                7 => Some(FieldEncoding::Indexed(Register::BX, None, disp)),
                _ => panic!("R/M out of range"),
            },
        };
        rm_field.unwrap()
    }

    fn get_reg_field(reg: &u8, w: &u8) -> FieldEncoding {
        let mut reg_field: Option<FieldEncoding> = None;
        match w {
            0 => {
                reg_field = match reg {
                    0 => Some(FieldEncoding::Reg(Register::AL)),
                    1 => Some(FieldEncoding::Reg(Register::CL)),
                    2 => Some(FieldEncoding::Reg(Register::DL)),
                    3 => Some(FieldEncoding::Reg(Register::BL)),
                    4 => Some(FieldEncoding::Reg(Register::AH)),
                    5 => Some(FieldEncoding::Reg(Register::CH)),
                    6 => Some(FieldEncoding::Reg(Register::DH)),
                    7 => Some(FieldEncoding::Reg(Register::BH)),
                    _ => None,
                }
            }
            1 => {
                reg_field = match reg {
                    0 => Some(FieldEncoding::Reg(Register::AX)),
                    1 => Some(FieldEncoding::Reg(Register::CX)),
                    2 => Some(FieldEncoding::Reg(Register::DX)),
                    3 => Some(FieldEncoding::Reg(Register::BX)),
                    4 => Some(FieldEncoding::Reg(Register::SP)),
                    5 => Some(FieldEncoding::Reg(Register::BP)),
                    6 => Some(FieldEncoding::Reg(Register::SI)),
                    7 => Some(FieldEncoding::Reg(Register::DI)),
                    _ => None,
                }
            }
            _ => (),
        }
        reg_field.unwrap()
    }
}

enum InstructionKind {
    FourBitOpcode,
    SixBitOpcode,
    ImmediateToAccumulator,
}

#[derive(LowercaseDisplay, Debug, PartialEq)]
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
    BP,
}

#[derive(Debug)]
pub struct DecodedMemField {
    pub opcode: Opcode,
    pub field_one: FieldOrRawData,
    pub field_two: FieldOrRawData,
}

impl std::fmt::Display for DecodedMemField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}, {}", self.opcode, self.field_one, self.field_two)
    }
}

impl PartialEq for DecodedMemField {
    fn eq(&self, other: &Self) -> bool {
        if self.opcode != other.opcode {
            return false;
        } else if self.field_one != other.field_one {
            return false;
        } else if self.field_two != other.field_two {
            return false;
        }
        true
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldOrRawData {
    FieldEncoding(FieldEncoding, Option<ExplicitSize>),
    RawData(RawData, Option<ExplicitSize>),
}

impl std::fmt::Display for FieldOrRawData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldOrRawData::FieldEncoding(val, size) => match size {
                Some(size) => write!(f, "{} {}", size, val),
                None => write!(f, "{}", val),
            },
            FieldOrRawData::RawData(val, size) => match size {
                Some(size) => write!(f, "{} {}", size, val),
                None => write!(f, "{}", val),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExplicitSize {
    Word,
    Byte,
}

impl std::fmt::Display for ExplicitSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplicitSize::Byte => write!(f, "byte"),
            ExplicitSize::Word => write!(f, "word"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldEncoding {
    Reg(Register),
    Indexed(Register, Option<Register>, Option<i16>),
}

impl std::fmt::Display for FieldEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FieldEncoding::Reg(reg) => write!(f, "{}", reg),
            FieldEncoding::Indexed(reg1, reg2, disp) => match disp {
                Some(disp) => match reg2 {
                    Some(reg2) => match disp {
                        disp if disp > &0 => write!(f, "[{} + {} + {}]", reg1, reg2, disp),
                        _ => write!(f, "[{} + {} - {}]", reg1, reg2, disp.abs()),
                    },
                    None => match disp {
                        disp if disp > &0 => write!(f, "[{} + {}]", reg1, disp),
                        disp if disp == &0 => write!(f, "[{}]", reg1),
                        _ => write!(f, "[{} - {}]", reg1, disp.abs()),
                    },
                },
                None => match reg2 {
                    Some(reg2) => write!(f, "[{} + {}]", reg1, reg2),
                    None => write!(f, "[{}]", reg1),
                },
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RawData {
    U8(u8),
    U16(u16),
    I8(i8),
}

impl std::fmt::Display for RawData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawData::U16(x) => write!(f, "{}", x),
            RawData::U8(x) => write!(f, "{}", x),
            RawData::I8(x) => write!(f, "{}", x),
        }
    }
}

#[derive(LowercaseDisplay, PartialEq, Debug)]
pub enum Opcode {
    ADD,
    ADC,
    CMP,
    SUB,
    MOV,
    WILDCARD,
}
