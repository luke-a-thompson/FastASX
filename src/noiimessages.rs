use crate::enums::{CrossType, ImbalanceDirection};
use crate::messageheader::MessageHeader;
use crate::types::{Parse, ParseError};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub struct NetOrderImbalanceIndicator {
    header: MessageHeader,
    paired_shares: u64,
    imbalance_shares: u64,
    imbalance_direction: ImbalanceDirection,
    stock: [u8; 8],
    far_price: u32,
    near_price: u32,
    current_reference_price: u32,
    cross_type: CrossType,
    price_variation_indicator: char,
}

impl Parse for NetOrderImbalanceIndicator {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 50 {
            return Err(ParseError::IncompleteMessage { expected: 50 });
        }

        Ok(NetOrderImbalanceIndicator {
            header: MessageHeader::parse(&input[..10]),
            paired_shares: BigEndian::read_u64(&input[10..18]),
            imbalance_shares: BigEndian::read_u64(&input[18..26]),
            imbalance_direction: {
                match input[26] {
                    b'B' => ImbalanceDirection::BuyImbalance,
                    b'S' => ImbalanceDirection::SellImbalance,
                    b'N' => ImbalanceDirection::NoImbalance,
                    b'O' => ImbalanceDirection::InsufficnetOrdersToCalculate,
                    _ => panic!("Invalid ImbalanceDirection"),
                }
            },
            stock: input[27..35].try_into().unwrap(),
            far_price: BigEndian::read_u32(&input[35..39]),
            near_price: BigEndian::read_u32(&input[39..43]),
            current_reference_price: BigEndian::read_u32(&input[43..47]),
            cross_type: {
                match input[47] {
                    b'O' => CrossType::OpeningCross,
                    b'C' => CrossType::ClosingCross,
                    b'H' => CrossType::IPOCrossOrHaltedSecurity,
                    // b'i' => CrossType::IntradayOrPostCloseCross,  // This in invalid for NOII?
                    _ => panic!("Invalid CrossType"),
                }
            },
            price_variation_indicator: {
                match input[48] {
                    b'L' => 'L',
                    b'0' => '0',
                    b'1' => '1',
                    b'2' => '2',
                    b'3' => '3',
                    b'4' => '4',
                    b'5' => '5',
                    b'6' => '6',
                    b'7' => '7',
                    b'8' => '8',
                    b'9' => '9',
                    _ => panic!("Invalid price_variation_indicator"),
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct RetainPriceImprovementIndicator {
    header: MessageHeader,
    stock: [u8; 8],
    interest_flag: char,
}

impl Parse for RetainPriceImprovementIndicator {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 19 {
            return Err(ParseError::IncompleteMessage { expected: 19 });
        }

        Ok(RetainPriceImprovementIndicator {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(),
            interest_flag: input[18] as char,
        })
    }
}
