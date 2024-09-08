use crate::enums::BoolOrUnavailable;
use crate::types::{Parse, ParseError};

pub fn byte_to_bool(byte: u8) -> Result<bool, ParseError> {
    match byte {
        b'Y' => Ok(true),
        b'N' => Ok(false),
        b => Err(ParseError::InvalidBooleanByte { invalid_byte: b }), // _ => panic!("Invalid input: expected 'Y' or 'N', got '{}'", byte as char),
    }
}

pub fn byte_to_bool_space(byte: u8) -> Result<BoolOrUnavailable, ParseError> {
    match byte {
        b'Y' => Ok(BoolOrUnavailable::Bool(true)),
        b'N' => Ok(BoolOrUnavailable::Bool(false)),
        b' ' => Ok(BoolOrUnavailable::Str("Not Available")),
        b => Err(ParseError::InvalidBooleanByte { invalid_byte: b }),
    }
}
// Was too slow
// pub fn u8s_to_ticker(input: &[u8]) -> String {
//     let mut ticker = String::new();
//     for byte in input {
//         if *byte == 0 {
//             break;
//         }
//         ticker.push(*byte as char);
//     }
//     ticker
// }
