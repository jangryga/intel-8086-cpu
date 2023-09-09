use lazy_static::lazy_static;
use std::sync::Arc;

pub enum Mnemonic {
    MOV,
}

pub enum InstEncoding {
    BitsLiteral(u8),
    BitsD,
    BitsS,
    BitsW,
    BitsV,
    BitsZ,
    BitsMod,
    BitsReg,
    BitsRm,
    BitsRegImp(u8),
    BitsData,
    BitsDataIfW,

    BitsImpD(u8),
}

pub struct Inst {
    pub mnemonic: Mnemonic,
    pub encoding: Vec<InstEncoding>,
}

use crate::inst;
lazy_static! {
    pub static ref INSTRUCTION_SET: Arc<Vec<Inst>> = Arc::new(vec![
        inst!(
            MOV,
            BitsLiteral(0b100010),
            BitsD,
            BitsW,
            BitsMod,
            BitsReg,
            BitsRm
        ),
        inst!(
            MOV,
            BitsLiteral(0b1100011),
            BitsW,
            BitsMod,
            BitsRegImp(0b000),
            BitsRm,
            BitsData,
            BitsDataIfW,
            BitsImpD(0)
        ),
    ]);
}

#[macro_export]
macro_rules! inst {
    ($mnemonic:ident, $( $encoding:ident $(( $literal:expr ))* ),* ) => {
        {
            let mut encodings = vec![];
            $(
                encodings.push(InstEncoding::$encoding $(( $literal ))*);
            )*
            Inst {
                mnemonic: Mnemonic::$mnemonic,
                encoding: encodings,
            }
        }
    };
}
