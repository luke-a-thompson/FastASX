use crate::{helpers::byte_to_bool, messageheader::MessageHeader};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub struct OrderExecuted {
    header: MessageHeader,
    order_reference_number: u64,
    executed_shares: u32,
    match_number: u64, // matches trade break message
}

impl OrderExecuted {
    pub fn parse(input: &[u8]) -> OrderExecuted {
        if input.len() != 30 {
            panic!("Invalid input length for OrderExecutedMessage");
        }

        OrderExecuted {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            executed_shares: BigEndian::read_u32(&input[18..22]),
            match_number: BigEndian::read_u64(&input[22..30]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderExecutedWithPrice {
    order_executed_message: OrderExecuted,
    printable: bool,
    exec_price: u32,
}

impl OrderExecutedWithPrice {
    pub fn parse(input: &[u8]) -> OrderExecutedWithPrice {
        if input.len() != 35 {
            panic!("Invalid input length for OrderExecutedWithPriceMessage");
        }

        OrderExecutedWithPrice {
            order_executed_message: OrderExecuted::parse(&input[..30]),
            printable: byte_to_bool(input[30]),
            exec_price: BigEndian::read_u32(&input[31..35]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderCancel {
    header: MessageHeader,
    order_reference_number: u64,
    canceled_shares: u32,
}

// Byte layout:
// 0-9: Header (10 bytes)
// 10-17: Order Reference Number (8 bytes)
// 18-21: Canceled Shares (4 bytes)
// Total: 23 bytes
impl OrderCancel {
    pub fn parse(input: &[u8]) -> OrderCancel {
        if input.len() != 22 {
            panic!("Invalid input length for OrderCancelMessage");
        }

        OrderCancel {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            canceled_shares: BigEndian::read_u32(&input[18..22]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderDelete {
    header: MessageHeader,
    order_reference_number: u64,
}

impl OrderDelete {
    pub fn parse(input: &[u8]) -> OrderDelete {
        if input.len() != 19 {
            panic!("Invalid input length for OrderDeleteMessage");
        }

        OrderDelete {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderReplace {
    header: MessageHeader,
    original_order_reference_number: u64,
    new_order_reference_number: u64, // Assert old order dropped?
    shares: u32,
    price: u32,
}

impl OrderReplace {
    pub fn parse(input: &[u8]) -> OrderReplace {
        if input.len() != 35 {
            panic!("Invalid input length for OrderReplaceMessage");
        }

        OrderReplace {
            header: MessageHeader::parse(&input[..10]),
            original_order_reference_number: BigEndian::read_u64(&input[10..18]),
            new_order_reference_number: BigEndian::read_u64(&input[18..26]),
            shares: BigEndian::read_u32(&input[26..30]),
            price: BigEndian::read_u32(&input[30..34]),
        }
    }
}
