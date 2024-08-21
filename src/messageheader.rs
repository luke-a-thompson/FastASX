use byteorder::{BigEndian, ByteOrder};

use crate::types::{BinaryMessageLength, GenerateBinaryExample};

#[cfg(test)]
use fastrand::Rng;

#[derive(Debug, PartialEq)]
pub struct MessageHeader {
    message_type: char,
    stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
}

impl MessageHeader {
    pub fn parse(input: &[u8]) -> MessageHeader {
        if input.len() != MessageHeader::LENGTH {
            panic!("Invalid input length for MessageHeader");
        }

        MessageHeader {
            message_type: b'_' as char,
            stock_locate: BigEndian::read_u16(&input[0..2]),
            tracking_number: BigEndian::read_u16(&input[2..4]),
            timestamp: BigEndian::read_u48(&input[4..10]),
        }
    }
}

impl BinaryMessageLength for MessageHeader {
    const LENGTH: usize = 10;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for MessageHeader {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let stock_locate = rng.u16(..).to_be_bytes();
        let tracking_number = rng.u16(..).to_be_bytes();
        let timestamp = rng.u64(..).to_be_bytes(); // Actually u48

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..2].copy_from_slice(&stock_locate);
        message[2..4].copy_from_slice(&tracking_number);
        message[4..10].copy_from_slice(&timestamp[..6]);

        message
    }
}
