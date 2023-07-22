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
            field_one: FieldOrRawData::FieldEncoding(FieldEncoding::Reg(Register::BX), Some(ExplicitSize::Byte)),
            field_two: FieldOrRawData::RawData(RawData::U8(34), None)
        };

        let mut p = Decoder {
            intermediate_repr: VecDeque::new().into(),
            memory: fake_instruction_stream.into(),
        };

        p.decode();
        p.execute();

        assert_eq!(*p.intermediate_repr.get(0).unwrap(), expected);
    }
}
