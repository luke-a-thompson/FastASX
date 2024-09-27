use crate::enums::SystemEventCode;
use crate::messageheader::MessageHeader;
use crate::types::{BinaryMessageLength, MessageHeaderType, Parse, ParseError};

#[cfg(any(test, feature = "bench"))]
use crate::types::{EnumTestHelpers, GenerateExampleMessage};

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

impl MessageHeaderType for SystemEventMessage {
    const MESSAGE_TYPE: u8 = b'S';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ SystemEventMessage::LENGTH }> for SystemEventMessage {
    fn generate_binary_example() -> [u8; SystemEventMessage::LENGTH] {
        let header = MessageHeader::generate_binary_example();
        let event_code = SystemEventCode::generate_example_code();

        let mut message = [0u8; SystemEventMessage::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10] = event_code as u8;

        message
    }
}
