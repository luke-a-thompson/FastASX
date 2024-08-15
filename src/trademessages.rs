use crate::messageheader::MessageHeader;
use byteorder::{BigEndian, ByteOrder};
use crate::enums::CrossType;

#[derive(Debug, PartialEq)]
pub struct NonCrossingTrade {
    header: MessageHeader,
    order_reference_number: u64,
    buy_sell_indicator: char,
    shares: u32,
    stock: [u8; 8],
    price: u32,
    match_number: u64,
}

impl NonCrossingTrade {
    pub fn parse(input: &[u8]) -> NonCrossingTrade {
        if input.len() != 44 {
            panic!("Invalid input length for NonCrossingTrade");
        }

        NonCrossingTrade {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            buy_sell_indicator: {
                match input[18] {
                    b'B' => 'B', // As of 2014, the only valid value is 'B'
                    _ => panic!("Invalid buy_sell_indicator"),
                }
            },
            shares: BigEndian::read_u32(&input[19..23]),
            stock: input[23..31].try_into().unwrap(),
            price: BigEndian::read_u32(&input[31..35]),
            match_number: BigEndian::read_u64(&input[35..43]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CrossingTrade {
    non_crossing_trade: NonCrossingTrade,
    cross_type: CrossType,
}

impl CrossingTrade {
    pub fn parse(input: &[u8]) -> CrossingTrade {
        if input.len() != 45 {
            panic!("Invalid input length for CrossingTrade");
        }

        CrossingTrade {
            non_crossing_trade: NonCrossingTrade::parse(&input[..44]),
            cross_type: {match input[44] {
                b'O' => CrossType::OpeningCross,
                b'C' => CrossType::ClosingCross,
                b'H' => CrossType::IPOCrossOrHaltedSecurity,
                b'i' => CrossType::IntradayOrPostCloseCross,
                _ => panic!("Invalid CrossType"),
            }},
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BrokenTrade {
    header: MessageHeader,
    match_number: u64,
}

impl BrokenTrade {
    pub fn parse(input: &[u8]) -> BrokenTrade {
        if input.len() != 18 {
            panic!("Invalid input length for BrokenTrade");
        }

        BrokenTrade {
            header: MessageHeader::parse(&input[..10]),
            match_number: BigEndian::read_u64(&input[10..18]),
        }
    }
}