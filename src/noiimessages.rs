use crate::enums::{CrossType, ImbalanceDirection};
use crate::messageheader::MessageHeader;
use crate::types::{BinaryMessageLength, MessageHeaderType, Parse, ParseError, Price4, PriceConversions, Stock};
use byteorder::{BigEndian, ByteOrder};

#[cfg(any(test, feature = "bench"))]
use crate::types::{EnumTestHelpers, GenerateExampleMessage};
#[cfg(any(test, feature = "bench"))]
use fastrand::Rng;

#[derive(Debug, PartialEq)]
pub struct NetOrderImbalanceIndicator {
    header: MessageHeader,
    paired_shares: u64,
    imbalance_shares: u64,
    imbalance_direction: ImbalanceDirection,
    stock: Stock,
    far_price: Price4,
    near_price: Price4,
    current_reference_price: Price4,
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
            far_price: Price4::new(BigEndian::read_u32(&input[35..39])),
            near_price: Price4::new(BigEndian::read_u32(&input[39..43])),
            current_reference_price: Price4::new(BigEndian::read_u32(&input[43..47])),
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
                    b => Err(ParseError::InvalidPriceVariationIndicator { invalid_byte: b })?,
                }
            },
        })
    }
}

impl BinaryMessageLength for NetOrderImbalanceIndicator {
    const LENGTH: usize = 49;
}

impl MessageHeaderType for NetOrderImbalanceIndicator {
    const MESSAGE_TYPE: u8 = b'I';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ Self::LENGTH }> for NetOrderImbalanceIndicator {
    fn generate_binary_example() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_binary_example();
        let paired_shares = rng.u64(..).to_be_bytes();
        let imbalance_shares = rng.u64(..).to_be_bytes();
        let imbalance_direction = ImbalanceDirection::generate_example_code();
        let stock = Stock::generate_binary_example();
        let far_price = rng.u32(..).to_be_bytes();
        let near_price = rng.u32(..).to_be_bytes();
        let current_reference_price = rng.u32(..).to_be_bytes();
        let cross_type = CrossType::generate_example_code();
        let price_variation_indicator = b'C';

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&paired_shares);
        message[18..26].copy_from_slice(&imbalance_shares);
        message[26] = imbalance_direction;
        message[27..35].copy_from_slice(&stock);
        message[35..39].copy_from_slice(&far_price);
        message[39..43].copy_from_slice(&near_price);
        message[43..47].copy_from_slice(&current_reference_price);
        message[47] = cross_type;
        message[48] = price_variation_indicator;

        message
    }
}

// Deprecated?
#[derive(Debug, PartialEq)]
pub struct RetailPriceImprovementIndicator {
    header: MessageHeader,
    stock: Stock,
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

impl BinaryMessageLength for RetailPriceImprovementIndicator {
    const LENGTH: usize = 19;
}

impl MessageHeaderType for RetailPriceImprovementIndicator {
    const MESSAGE_TYPE: u8 = b'N';
}
