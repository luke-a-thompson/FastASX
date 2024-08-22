use crate::enums::SystemEventCode;
use crate::messageheader::MessageHeader;
use crate::types::{BinaryMessageLength, Parse, ParseError};

#[cfg(test)]
use crate::types::{EnumTestHelpers, GenerateBinaryExample};

#[derive(Debug, PartialEq)]
pub struct SystemEventMessage {
    header: MessageHeader,
    event_code: SystemEventCode,
}

impl Parse for SystemEventMessage {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 11 {
            return Err(ParseError::IncompleteMessage { expected: 11 });
        }

        Ok(SystemEventMessage {
            header: MessageHeader::parse(&input[..10]),
            event_code: SystemEventCode::try_from(input[10])?,
        })
    }
}

impl BinaryMessageLength for SystemEventMessage {
    const LENGTH: usize = 11;
}

#[cfg(test)]
impl GenerateBinaryExample<{ SystemEventMessage::LENGTH }> for SystemEventMessage {
    fn generate_example_message() -> [u8; SystemEventMessage::LENGTH] {
        let header = MessageHeader::generate_example_message();
        let event_code = SystemEventCode::generate_example_code();

        let mut message = [0u8; SystemEventMessage::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10] = event_code as u8;

        message
    }
}
