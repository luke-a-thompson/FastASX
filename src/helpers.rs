use crate::enums::BoolOrUnavailable;

pub fn byte_to_bool(byte: u8) -> bool {
    match byte {
        b'Y' => true,
        b'N' => false,
        _ => panic!("Invalid input: expected 'Y' or 'N', got '{}'", byte as char),
    }
}

pub fn byte_to_bool_space(byte: u8) -> BoolOrUnavailable {
    match byte {
        b'Y' => BoolOrUnavailable::Bool(true),
        b'N' => BoolOrUnavailable::Bool(false),
        b' ' => BoolOrUnavailable::Str("Not Available".to_string()),
        _ => panic!("Invalid input: expected 'Y' or 'N', got '{}'", byte as char),
    }
}

pub fn u8s_to_ticker(input: &[u8]) -> String {
    let mut ticker = String::new();
    for byte in input {
        if *byte == 0 {
            break;
        }
        ticker.push(*byte as char);
    }
    ticker
}
