use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub struct MessageHeader {
    message_type: char,
    stock_locate: u16,
    tracking_number: u16,
    timestamp: u64,
}

impl MessageHeader {
    pub fn parse(input: &[u8]) -> MessageHeader {
        if input.len() != 10 {
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
