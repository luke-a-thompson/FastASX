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
        if input.len() != 49 {
            return Err(ParseError::IncompleteMessage { expected: 49 });
        }

        Ok(NetOrderImbalanceIndicator {
            header: MessageHeader::parse(&input[..10]),
            paired_shares: BigEndian::read_u64(&input[10..18]),
            imbalance_shares: BigEndian::read_u64(&input[18..26]),
            imbalance_direction: ImbalanceDirection::try_from(input[26])?,
            stock: input[27..35].try_into().unwrap(),
            far_price: BigEndian::read_u32(&input[35..39]),
            near_price: BigEndian::read_u32(&input[39..43]),
            current_reference_price: BigEndian::read_u32(&input[43..47]),
            cross_type: CrossType::try_from(input[47])?,
            price_variation_indicator: {
                match input[48] {
                    b'L' => 'L',
                    b'1' => '1',
                    b'2' => '2',
                    b'3' => '3',
                    b'4' => '4',
                    b'5' => '5',
                    b'6' => '6',
                    b'7' => '7',
                    b'8' => '8',
                    b'9' => '9',
                    b'A' => 'A',
                    b'B' => 'B',
                    b'C' => 'C',
                    b' ' => ' ',
                    _ => panic!("Invalid price_variation_indicator: {}", input[48] as char),
                }
            },
        })
    }
}

// Deprecated?
#[derive(Debug, PartialEq)]
pub struct RetailPriceImprovementIndicator {
    header: MessageHeader,
    stock: [u8; 8],
    interest_flag: char,
}

impl Parse for RetailPriceImprovementIndicator {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 19 {
            return Err(ParseError::IncompleteMessage { expected: 19 });
        }

        Ok(RetailPriceImprovementIndicator {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(),
            interest_flag: input[18] as char,
        })
    }
}
