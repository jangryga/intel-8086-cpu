pub mod instruction_decode;

#[cfg(test)]
mod tests {
    use crate::instruction_decode::{Decoder, interpret_mode, Mode, Displacement};

    fn fake_stream(x: u8, y: u8) -> Vec<u8> {
        vec![x, y]
    }

    #[test]
    fn correct_mode() {    
        let in1 = Decoder::read_instruction_stream(&fake_stream(139, 217));
        let in2 = Decoder::read_instruction_stream(&fake_stream(139, 129));
        let in3 = Decoder::read_instruction_stream(&fake_stream(139, 65));
        let in4 = Decoder::read_instruction_stream(&fake_stream(139, 12));
        
        assert_eq!(interpret_mode(&in1.mode), Some(Mode::Register));
        assert_eq!(interpret_mode(&in2.mode), Some(Mode::Memory(Displacement::SixteenBit)));
        assert_eq!(interpret_mode(&in3.mode), Some(Mode::Memory(Displacement::EightBit)));
        assert_eq!(interpret_mode(&in4.mode), Some(Mode::Memory(Displacement::No)));
    }
}