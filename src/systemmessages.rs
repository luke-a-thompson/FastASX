use crate::enums::SystemEventCode;
use crate::messageheader::MessageHeader;

#[derive(Debug, PartialEq)]
pub struct SystemEventMessage {
    header: MessageHeader,
    event_code: SystemEventCode,
}

impl SystemEventMessage {
    pub fn parse(input: &[u8]) -> SystemEventMessage {
        if input.len() != 11 {
            panic!("Invalid input length for SystemEventMessage");
        }

        SystemEventMessage {
            header: MessageHeader::parse(&input[..10]),
            event_code: {
                match input[10] {
                    b'O' => SystemEventCode::StartOfMessages,
                    b'S' => SystemEventCode::StartOfSystemHours,
                    b'Q' => SystemEventCode::StartOfMarketHours,
                    b'M' => SystemEventCode::EndOfMarketHours,
                    b'E' => SystemEventCode::EndOfSystemHours,
                    b'C' => SystemEventCode::EndOfMessages,
                    _ => panic!("Invalid SystemEventCode"),
                }
            }, // We only read up to index 10, 1 less because of match. max spec offset-1
        }
    }
}
