pub mod instruction_decode;

#[cfg(test)]
mod tests {
    use crate::instruction_decode::*;

    #[test]
    fn correct_mode() {
        let num1: u8 = 217; // 11..
        let num2: u8 = 129; // 10..
        let num3: u8 = 65; // 01..
        let num4: u8 = 12; // 00..

        assert_eq!(get_mode(&num1), Some(Mode::Register));
        assert_eq!(get_mode(&num2), Some(Mode::Memory(Displacement::SixteenBit)));
        assert_eq!(get_mode(&num3), Some(Mode::Memory(Displacement::EightBit)));
        assert_eq!(get_mode(&num4), Some(Mode::Memory(Displacement::No)));
    }

    #[test]
    fn correct_reg() {
        let num1: u8 = 217; //11011001 -> 011 (3)
        let num2: u8 = 129; // ..000.. -> 0
        let num3: u8 = 185; // ..111.. -> 7

        assert_eq!(get_reg(&num1), 3);
        assert_eq!(get_reg(&num2), 0);
        assert_eq!(get_reg(&num3), 7);
    }
}