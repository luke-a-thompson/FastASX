use crate::enums::BuySellIndicator;
use crate::messageheader::MessageHeader;
use byteorder::{BigEndian, ByteOrder};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct AddOrder {
    header: MessageHeader,
    order_reference_number: u64,
    buy_sell_indicator: BuySellIndicator,
    shares: u32,
    stock: [u8; 8],
    price: u32,
}

impl AddOrder {
    pub fn parse(input: &[u8]) -> AddOrder {
        if input.len() != 35 {
            panic!("Invalid input length for AddOrderMessage");
        }

        AddOrder {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            buy_sell_indicator: {
                match input[18] {
                    b'B' => BuySellIndicator::Buy,
                    b'S' => BuySellIndicator::Sell,
                    _ => panic!("Invalid BuySellIndicator"),
                }
            },
            shares: BigEndian::read_u32(&input[19..23]),
            stock: input[23..31].try_into().unwrap(),
            price: BigEndian::read_u32(&input[31..35]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AddOrderMPID {
    add_order_message: AddOrder,
    mpid: [u8; 4],
}

impl AddOrderMPID {
    pub fn parse(input: &[u8]) -> AddOrderMPID {
        if input.len() != 39 {
            panic!("Invalid input length for AddOrderMPID");
        }

        AddOrderMPID {
            // header: MessageHeader::parse(&input[..10]),
            add_order_message: AddOrder::parse(&input[..35]),
            mpid: input[35..39].try_into().unwrap(),
        }
    }
}
