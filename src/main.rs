use byteorder::{BigEndian, ByteOrder};
use ringbuf::{traits::*, HeapRb};
use std::fs::File;
use std::io::{self, Read, Seek, Write};

use systemmessages::*;

pub mod addordermessages;
pub mod enums;
pub mod helpers;
pub mod messageheader;
pub mod modifyordermessages;
pub mod noiimessages;
pub mod stockmessages;
pub mod systemmessages;
pub mod trademessages;
pub mod types;

const BUFFER_SIZE: usize = 1024 * 1024; // Heap allocated
const DISCARD_SIZE: usize = 512 * 1024;

fn parse_fixed_length_message<const N: usize, T, F, C>(consumer: &mut C, parser: F) -> Option<T>
where
    C: Consumer<Item = u8>,
    F: FnOnce(&[u8]) -> T,
{
    let mut buffer = [0u8; N];
    if consumer.pop_slice(&mut buffer) == N {
        Some(parser(&buffer))
    } else {
        None
    }
}

fn main() -> Result<(), io::Error> {
    let mut file = File::open("/home/luke/fastasx/data/12302019.NASDAQ_ITCH50")?;

    let rb = HeapRb::<u8>::new(BUFFER_SIZE); // Ringbuffer
    let (mut producer, mut consumer) = rb.split();

    let mut file_buffer = [0u8; BUFFER_SIZE];
    let mut consumer_slice_size = [0u8; 3];

    let mut msg_ct: u64 = 0;
    loop {
        // Fill the buffer
        let bytes_read = file.read(&mut file_buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }

        producer
            .write_all(&file_buffer[file.stream_position()? as usize..bytes_read])
            .expect("Ringbuffer write error");

        // Process data from the ring buffer
        println!("{:?}", consumer.vacant_len());
        while consumer.vacant_len() > 1024 {
            consumer.pop_slice(&mut consumer_slice_size);
            match (
                BigEndian::read_u16(&consumer_slice_size[..2]),
                &consumer_slice_size[2],
            ) {
                (36, b'A') => {
                    let event = parse_fixed_length_message::<35, _, _, _>(
                        &mut consumer,
                        addordermessages::AddOrder::parse,
                    )
                    .expect("1");
                }
                (19, b'B') => {
                    let event = parse_fixed_length_message::<18, _, _, _>(
                        &mut consumer,
                        trademessages::BrokenTrade::parse,
                    )
                    .expect("2");
                }
                (36, b'C') => {
                    let event = parse_fixed_length_message::<35, _, _, _>(
                        &mut consumer,
                        modifyordermessages::OrderExecutedWithPrice::parse,
                    )
                    .expect("3");
                }
                (20, b'D') => {
                    let event = parse_fixed_length_message::<19, _, _, _>(
                        &mut consumer,
                        modifyordermessages::OrderDelete::parse,
                    )
                    .expect("4");
                }
                (31, b'E') => {
                    let event = parse_fixed_length_message::<30, _, _, _>(
                        &mut consumer,
                        modifyordermessages::OrderExecuted::parse,
                    )
                    .expect("5");
                }
                (40, b'F') => {
                    let event = parse_fixed_length_message::<39, _, _, _>(
                        &mut consumer,
                        addordermessages::AddOrderMPID::parse,
                    )
                    .expect("6");
                }
                (25, b'H') => {
                    let event = parse_fixed_length_message::<24, _, _, _>(
                        &mut consumer,
                        stockmessages::StockTradingAction::parse,
                    )
                    .expect("7");
                }
                (51, b'I') => {
                    let event = parse_fixed_length_message::<50, _, _, _>(
                        &mut consumer,
                        noiimessages::NetOrderImbalanceIndicator::parse,
                    )
                    .expect("8");
                }
                (28, b'K') => {
                    let event = parse_fixed_length_message::<27, _, _, _>(
                        &mut consumer,
                        stockmessages::IPOQuotingPeriodUpdate::parse,
                    )
                    .expect("9");
                }
                (26, b'L') => {
                    let event = parse_fixed_length_message::<25, _, _, _>(
                        &mut consumer,
                        stockmessages::MarketParticipantPosition::parse,
                    )
                    .expect("10");
                }
                (20, b'N') => {
                    let event = parse_fixed_length_message::<19, _, _, _>(
                        &mut consumer,
                        noiimessages::RetainPriceImprovementIndicator::parse,
                    )
                    .expect("11");
                }
                (45, b'P') => {
                    let event = parse_fixed_length_message::<44, _, _, _>(
                        &mut consumer,
                        trademessages::NonCrossingTrade::parse,
                    )
                    .expect("12");
                }
                (46, b'Q') => {
                    let event = parse_fixed_length_message::<45, _, _, _>(
                        &mut consumer,
                        trademessages::CrossingTrade::parse,
                    )
                    .expect("13");
                }
                (39, b'R') => {
                    let event = parse_fixed_length_message::<38, _, _, _>(
                        &mut consumer,
                        stockmessages::StockDirectory::parse,
                    )
                    .expect("14");
                }
                (12, b'S') => {
                    let event = parse_fixed_length_message::<11, _, _, _>(
                        &mut consumer,
                        SystemEventMessage::parse,
                    )
                    .expect("15");
                }
                (36, b'U') => {
                    let event = parse_fixed_length_message::<35, _, _, _>(
                        &mut consumer,
                        modifyordermessages::OrderReplace::parse,
                    )
                    .expect("16");
                }
                (36, b'V') => {
                    let event = parse_fixed_length_message::<35, _, _, _>(
                        &mut consumer,
                        stockmessages::MWCBDeclineLevel::parse,
                    )
                    .expect("17");
                }
                (12, b'W') => {
                    let event = parse_fixed_length_message::<11, _, _, _>(
                        &mut consumer,
                        stockmessages::MWCBStatus::parse,
                    )
                    .expect("18");
                }
                (24, b'X') => {
                    let event = parse_fixed_length_message::<23, _, _, _>(
                        &mut consumer,
                        modifyordermessages::OrderCancel::parse,
                    )
                    .expect("19");
                }
                (20, b'Y') => {
                    let event = parse_fixed_length_message::<19, _, _, _>(
                        &mut consumer,
                        stockmessages::RegSHOShortSalePriceTestRestriction::parse,
                    )
                    .expect("20");
                }
                _ => {
                    // println!("Non-message type character: '{}'", message_type as char)
                } // _ => panic!("Unknown message type '{}'", message_type as char),
            }
            msg_ct += 1;
            if msg_ct % 1_000_000 == 1 {
                println!("Processed {}m messages", msg_ct / 1_000_000);
            }
        }
        let mut discard_buffer = [0u8; DISCARD_SIZE];
        consumer.pop_slice(&mut discard_buffer);
    }

    Ok(())
}
