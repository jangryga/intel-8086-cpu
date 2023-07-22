#[allow(unused_assignments)]
pub mod instruction_decode;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::instruction_decode::*;

    #[test]
    fn immediate_to_memory() {
        // add byte [bx], 34
        // 10000000 00000111 00100010
        let fake_instruction_stream: Vec<u8> = vec![128, 7, 34];
        let expected: DecodedMemField = DecodedMemField {
            opcode: Opcode::ADD,
            field_one: FieldOrRawData::FieldEncoding(
                FieldEncoding::Reg(Register::BX),
                Some(ExplicitSize::Byte),
            ),
            field_two: FieldOrRawData::RawData(RawData::U8(34), None),
        };

        let mut p = Decoder {
            intermediate_repr: VecDeque::new().into(),
            memory: fake_instruction_stream.into(),
        };

        p.decode();

        assert_eq!(expected, *p.intermediate_repr.get(0).unwrap());
    }

    #[test]
    fn immediate_to_memory_with_displacement() {
        // add [bp + si + 1000], 29
        // 10000011 10000010 11101000 00000011 00011101
        let fake_instruction_stream: Vec<u8> = vec![131, 130, 232, 3, 29];
        let expected = DecodedMemField {
            opcode: Opcode::ADD,
            field_one: FieldOrRawData::FieldEncoding(
                FieldEncoding::Indexed(Register::BP, Some(Register::SI), Some(1000)),
                Some(ExplicitSize::Byte),
            ),
            field_two: FieldOrRawData::RawData(RawData::U8(29), None),
        };
        let mut p = Decoder {
            intermediate_repr: VecDeque::new().into(),
            memory: fake_instruction_stream.into(),
        };

        p.decode();

        assert_eq!(expected, *p.intermediate_repr.get(0).unwrap());
    }

    #[test]
    fn immediate_to_accumulator() {
        // add ax, 1000
        // 00000101 11101000 00000011
        let fake_instruction_stream: Vec<u8> = vec![5, 232, 3];
        let expected = DecodedMemField {
            opcode: Opcode::ADD,
            field_one: FieldOrRawData::FieldEncoding(
                FieldEncoding::Reg(Register::AX),
                None,
            ),
            field_two: FieldOrRawData::RawData(
                RawData::U16(1000),
                None,
            )
        };
        let mut p = Decoder {
            intermediate_repr: VecDeque::new().into(),
            memory: fake_instruction_stream.into(),
        };

        p.decode();

        assert_eq!(expected, *p.intermediate_repr.get(0).unwrap());
    }
}
