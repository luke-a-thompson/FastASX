use byteorder::{BigEndian, ByteOrder};
use ringbuf::{traits::*, HeapRb};
use std::fs::File;
use std::io::{self, Read, Seek, Write};
use types::{Parse, ParseError};

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

const FILE_BUFFER_SIZE: usize = 1024 * 1024; // Heap allocated
const RING_BUFFER_SIZE: usize = 4096 * 1024; // Heap allocated
const DISCARD_SIZE: usize = RING_BUFFER_SIZE / 4;

// fn parse_fixed_length_message<const N: usize, T, F, C>(
//     consumer: &mut C,
//     parser: F,
// ) -> Result<T, ParseError>
// where
//     C: Consumer<Item = u8>,
//     F: FnOnce(&[u8]) -> T,
// {
//     let mut buffer = [0u8; N];
//     if consumer.pop_slice(&mut buffer) == N {
//         Ok(parser(&buffer))
//     } else {
//         Err(ParseError::IncompleteMessage { expected: N })
//     }
// }

fn parse_fixed_length_message<const N: usize, T, C>(consumer: &mut C) -> Result<T, ParseError>
where
    C: Consumer<Item = u8>,
    T: Parse,
{
    let mut buffer = [0u8; N];
    if consumer.pop_slice(&mut buffer) == N {
        T::parse(&buffer)
    } else {
        Err(ParseError::IncompleteMessage { expected: N })
    }
}

fn main() -> Result<(), io::Error> {
    let mut file = File::open("/home/luke/fastasx/data/12302019.NASDAQ_ITCH50")?;

    let rb = HeapRb::<u8>::new(RING_BUFFER_SIZE); // Ringbuffer
    let (mut producer, mut consumer) = rb.split();

    let mut file_buffer = [0u8; FILE_BUFFER_SIZE];
    let mut consumer_slice_size = [0u8; 3];

    let mut msg_ct: u64 = 0;
    let mut total_bytes_read: f64 = 0.0;

    loop {
        // Fill the buffer
        let bytes_read = file.read(&mut file_buffer)?;
        total_bytes_read += bytes_read as f64 / 1024.0 / 1024.0 / 1024.0;

        if bytes_read == 0 {
            println!("End of file");
            break;
        }

        producer.push_slice(&file_buffer[..bytes_read]);
        // .expect("Ringbuffer write error");

        // Process data from the ring buffer
        while consumer.vacant_len() < FILE_BUFFER_SIZE {
            consumer.pop_slice(&mut consumer_slice_size);
            match (
                BigEndian::read_u16(&consumer_slice_size[..2]),
                &consumer_slice_size[2],
            ) {
                (36, b'A') => {
                    let event = parse_fixed_length_message::<35, addordermessages::AddOrder, _>(
                        &mut consumer,
                    );
                    // println!("{:?}", event);
                }
                (19, b'B') => {
                    let event = parse_fixed_length_message::<18, trademessages::BrokenTrade, _>(
                        &mut consumer,
                    );
                    // println!("{:?}", event);
                }
                (36, b'C') => {
                    let event = parse_fixed_length_message::<
                        35,
                        modifyordermessages::OrderExecutedWithPrice,
                        _,
                    >(&mut consumer);
                    // println!("{:?}", event);
                }
                (19, b'D') => {
                    let event = parse_fixed_length_message::<18, modifyordermessages::OrderDelete, _>(
                        &mut consumer,
                    );
                    // println!("{:?}", event);
                }
                (31, b'E') => {
                    let event =
                        parse_fixed_length_message::<30, modifyordermessages::OrderExecuted, _>(
                            &mut consumer,
                        );
                    // println!("{:?}", event);
                }
                (40, b'F') => {
                    let event = parse_fixed_length_message::<39, addordermessages::AddOrderMPID, _>(
                        &mut consumer,
                    );
                    // println!("{:?}", event);
                }
                (25, b'H') => {
                    let event =
                        parse_fixed_length_message::<24, stockmessages::StockTradingAction, _>(
                            &mut consumer,
                        );
                    // println!("{:?}", event);
                }
                (50, b'I') => {
                    let event = parse_fixed_length_message::<
                        49,
                        noiimessages::NetOrderImbalanceIndicator,
                        _,
                    >(&mut consumer);
                    // println!("{:?}", event);
                }
                (28, b'K') => {
                    let event =
                        parse_fixed_length_message::<27, stockmessages::IPOQuotingPeriodUpdate, _>(
                            &mut consumer,
                        );
                }
                (26, b'L') => {
                    let event = parse_fixed_length_message::<
                        25,
                        stockmessages::MarketParticipantPosition,
                        _,
                    >(&mut consumer);
                }
                (20, b'N') => {
                    let event = parse_fixed_length_message::<
                        19,
                        noiimessages::RetainPriceImprovementIndicator,
                        _,
                    >(&mut consumer);
                }
                (44, b'P') => {
                    let event = parse_fixed_length_message::<43, trademessages::NonCrossingTrade, _>(
                        &mut consumer,
                    );
                }
                (40, b'Q') => {
                    // was very wrong
                    let event = parse_fixed_length_message::<39, trademessages::CrossingTrade, _>(
                        &mut consumer,
                    );
                }
                (39, b'R') => {
                    let event = parse_fixed_length_message::<38, stockmessages::StockDirectory, _>(
                        &mut consumer,
                    );
                }
                (12, b'S') => {
                    let event =
                        parse_fixed_length_message::<11, SystemEventMessage, _>(&mut consumer);
                    // println!("{:?}", event);
                }
                (35, b'U') => {
                    let event =
                        parse_fixed_length_message::<34, modifyordermessages::OrderReplace, _>(
                            &mut consumer,
                        );
                }
                (35, b'V') => {
                    // was wrong
                    let event = parse_fixed_length_message::<34, stockmessages::MWCBDeclineLevel, _>(
                        &mut consumer,
                    );
                }
                (12, b'W') => {
                    let event = parse_fixed_length_message::<11, stockmessages::MWCBStatus, _>(
                        &mut consumer,
                    );
                }
                (23, b'X') => {
                    let event = parse_fixed_length_message::<22, modifyordermessages::OrderCancel, _>(
                        &mut consumer,
                    );
                }
                (20, b'Y') => {
                    let event = parse_fixed_length_message::<
                        19,
                        stockmessages::RegSHOShortSalePriceTestRestriction,
                        _,
                    >(&mut consumer);
                }
                _ => {
                    // println!("Non-message type character: {:?}", test);
                } // _ => panic!("Unknown message type '{}'", message_type as char),
            }
            msg_ct += 1;
            if msg_ct % 10_000_000 == 1 {
                println!(
                    "Processed {}m messages contained in {:.2}gb",
                    msg_ct / 1_000_000,
                    total_bytes_read
                );
            }
        }
        // let mut discard_buffer = [0u8; DISCARD_SIZE];
        // consumer.pop_slice(&mut discard_buffer);
        // println!("Discarded: {:?}", discard_buffer);
    }

    Ok(())
}
