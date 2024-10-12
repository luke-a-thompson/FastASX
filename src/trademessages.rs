use crate::enums::CrossType;
use crate::messageheader::MessageHeader;
use crate::types::{BinaryMessageLength, MessageHeaderType, Parse, ParseError, Price4, PriceConversions, Stock};
use byteorder::{BigEndian, ByteOrder};

#[cfg(any(test, feature = "bench"))]
use crate::types::{EnumTestHelpers, GenerateExampleMessage};
#[cfg(any(test, feature = "bench"))]
use fastrand::Rng;

#[derive(Debug, PartialEq)]
pub struct NonCrossingTrade {
    header: MessageHeader,
    order_reference_number: u64,
    buy_sell_indicator: char,
    shares: u32,
    stock: Stock,
    price: Price4,
    match_number: u64,
}

impl Parse for NonCrossingTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(NonCrossingTrade {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            buy_sell_indicator: {
                match input[18] {
                    b'B' => 'B', // As of 2014, the only valid value is 'B'
                    _ => Err(ParseError::InvalidCrossType {
                        invalid_byte: input[18],
                    })?,
                }
            },
            shares: BigEndian::read_u32(&input[19..23]),
            stock: input[23..31].try_into().unwrap(),
            price: Price4::new(BigEndian::read_u32(&input[31..35])),
            match_number: BigEndian::read_u64(&input[35..43]),
        })
    }
}

impl BinaryMessageLength for NonCrossingTrade {
    const LENGTH: usize = 43;
}

impl MessageHeaderType for NonCrossingTrade {
    const MESSAGE_TYPE: u8 = b'P';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ Self::LENGTH }> for NonCrossingTrade {
    fn generate_binary_example() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_binary_example();
        let order_reference_number = rng.u64(..).to_be_bytes();
        let buy_sell_indicator = b'B';
        let shares = rng.u32(..).to_be_bytes();
        let stock = Stock::generate_binary_example();
        let price = rng.u32(..).to_be_bytes();
        let match_number = rng.u64(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&order_reference_number);
        message[18] = buy_sell_indicator;
        message[19..23].copy_from_slice(&shares);
        message[23..31].copy_from_slice(&stock);
        message[31..35].copy_from_slice(&price);
        message[35..43].copy_from_slice(&match_number);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct CrossingTrade {
    header: MessageHeader,
    shares: u64, // 64 for crossing trades, 32 for non-crossing trades
    stock: Stock,
    cross_price: Price4,
    match_number: u64,
    cross_type: CrossType,
}

impl Parse for CrossingTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(CrossingTrade {
            header: MessageHeader::parse(&input[..10]),
            shares: BigEndian::read_u64(&input[10..18]),
            stock: input[18..26].try_into().unwrap(),
            cross_price: Price4::new(BigEndian::read_u32(&input[26..30])),
            match_number: BigEndian::read_u64(&input[30..38]),
            cross_type: {
                match input[38] {
                    b'O' => CrossType::OpeningCross,
                    b'C' => CrossType::ClosingCross,
                    b'H' => CrossType::IPOCrossOrHaltedSecurity,
                    b'I' => CrossType::IntradayOrPostCloseCross,
                    _ => Err(ParseError::InvalidCrossType {
                        invalid_byte: input[38],
                    })?,
                }
            },
        })
    }
}

impl BinaryMessageLength for CrossingTrade {
    const LENGTH: usize = 39;
}

impl MessageHeaderType for CrossingTrade {
    const MESSAGE_TYPE: u8 = b'Q';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ Self::LENGTH }> for CrossingTrade {
    fn generate_binary_example() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_binary_example();
        let shares = rng.u64(..).to_be_bytes();
        let stock = Stock::generate_binary_example();
        let cross_price = rng.u32(..).to_be_bytes();
        let match_number = rng.u64(..).to_be_bytes();
        let cross_type = CrossType::generate_example_code();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&shares);
        message[18..26].copy_from_slice(&stock);
        message[26..30].copy_from_slice(&cross_price);
        message[30..38].copy_from_slice(&match_number);
        message[38] = cross_type;

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct BrokenTrade {
    header: MessageHeader,
    match_number: u64,
}

impl Parse for BrokenTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(BrokenTrade {
            header: MessageHeader::parse(&input[..10]),
            match_number: BigEndian::read_u64(&input[10..18]),
        })
    }
}

impl BinaryMessageLength for BrokenTrade {
    const LENGTH: usize = 18;
}

impl MessageHeaderType for BrokenTrade {
    const MESSAGE_TYPE: u8 = b'B';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ Self::LENGTH }> for BrokenTrade {
    fn generate_binary_example() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_binary_example();
        let match_number = rng.u64(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&match_number);

        message
    }
}
