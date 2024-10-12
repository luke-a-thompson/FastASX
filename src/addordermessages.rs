use crate::enums::BuySellIndicator;
use crate::messageheader::MessageHeader;
use crate::types::{PriceConversions, MPID};
use crate::types::{
    AltBinaryMessageLength, AltMessageHeaderType, BinaryMessageLength, MessageHeaderType, Parse,
    ParseError, Price4, Stock,
};
use byteorder::{BigEndian, ByteOrder};

#[cfg(any(test, feature = "bench"))]
use crate::types::{EnumTestHelpers, GenerateExampleMessage};
#[cfg(any(test, feature = "bench"))]
use fastrand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct AddOrder {
    pub header: MessageHeader,
    pub order_reference_number: u64,
    pub buy_sell_indicator: BuySellIndicator,
    pub shares: u32,
    pub stock: Stock,
    pub price: Price4,
    pub mpid: Option<MPID>,
}

impl Parse for AddOrder {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH && input.len() != Self::LENGTH + 4 {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(AddOrder {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            buy_sell_indicator: BuySellIndicator::try_from(input[18])?,
            shares: BigEndian::read_u32(&input[19..23]),
            stock: input[23..31].try_into().unwrap(),
            price: Price4::new(BigEndian::read_u32(&input[31..35])),
            mpid: if input.len() == Self::LENGTH {
                None
            } else {
                Some(input[35..39].try_into().unwrap())
            },
        })
    }
}

impl BinaryMessageLength for AddOrder {
    const LENGTH: usize = 35;
}

impl MessageHeaderType for AddOrder {
    const MESSAGE_TYPE: u8 = b'A';
}

impl AltBinaryMessageLength for AddOrder {
    const ALT_LENGTH: usize = 39;
}

impl AltMessageHeaderType for AddOrder {
    const ALT_MESSAGE_TYPE: u8 = b'F';
}

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<{ Self::LENGTH }> for AddOrder {
    fn generate_binary_example() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_binary_example();
        let order_reference_number = rng.u64(..);
        let buy_sell_indicator = BuySellIndicator::generate_example_code();
        let shares = rng.u32(..);
        let stock = Stock::generate_binary_example();
        let price = rng.u32(..);

        let mut example = [0; Self::LENGTH];
        example[..10].copy_from_slice(&header);
        BigEndian::write_u64(&mut example[10..18], order_reference_number);
        example[18] = buy_sell_indicator.into();
        BigEndian::write_u32(&mut example[19..23], shares);
        example[23..31].copy_from_slice(&stock);
        BigEndian::write_u32(&mut example[31..35], price);

        example
    }
}
