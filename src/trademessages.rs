use crate::enums::CrossType;
use crate::messageheader::MessageHeader;
use crate::types::{Parse, ParseError, Stock};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub struct NonCrossingTrade {
    header: MessageHeader,
    order_reference_number: u64,
    buy_sell_indicator: char,
    shares: u32,
    stock: Stock,
    price: u32,
    match_number: u64,
}

impl Parse for NonCrossingTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 44 {
            return Err(ParseError::IncompleteMessage { expected: 44 });
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
            price: BigEndian::read_u32(&input[31..35]),
            match_number: BigEndian::read_u64(&input[35..43]),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct CrossingTrade {
    non_crossing_trade: NonCrossingTrade,
    cross_type: CrossType,
}

impl Parse for CrossingTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 45 {
            return Err(ParseError::IncompleteMessage { expected: 45 });
        }

        Ok(CrossingTrade {
            non_crossing_trade: NonCrossingTrade::parse(&input[..44]).unwrap(),
            cross_type: {
                match input[44] {
                    b'O' => CrossType::OpeningCross,
                    b'C' => CrossType::ClosingCross,
                    b'H' => CrossType::IPOCrossOrHaltedSecurity,
                    b'I' => CrossType::IntradayOrPostCloseCross,
                    _ => Err(ParseError::InvalidCrossType {
                        invalid_byte: input[44],
                    })?,
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct BrokenTrade {
    header: MessageHeader,
    match_number: u64,
}

impl Parse for BrokenTrade {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 18 {
            return Err(ParseError::IncompleteMessage { expected: 18 });
        }

        Ok(BrokenTrade {
            header: MessageHeader::parse(&input[..10]),
            match_number: BigEndian::read_u64(&input[10..18]),
        })
    }
}
