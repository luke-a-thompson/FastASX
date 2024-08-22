#![feature(test)]

extern crate test;
use test::{bench, Bencher};

use byteorder::{BigEndian, ByteOrder};
use ringbuf::{traits::*, HeapRb};
use std::fs::File;
use std::io::{self, Read};
use types::{Parse, ParseError};

#[cfg(test)]
mod tests;

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
pub mod benchmarks;

const FILE_BUFFER_SIZE: usize = 2048 * 1024; // Stack allocated
const RING_BUFFER_SIZE: usize = 4096 * 2048; // Heap allocated

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

pub fn main() -> Result<(), io::Error> {
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
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (19, b'B') => {
                    let event = parse_fixed_length_message::<18, trademessages::BrokenTrade, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (36, b'C') => {
                    let event = parse_fixed_length_message::<
                        35,
                        modifyordermessages::OrderExecutedWithPrice,
                        _,
                    >(&mut consumer);
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (19, b'D') => {
                    let event = parse_fixed_length_message::<18, modifyordermessages::OrderDelete, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (31, b'E') => {
                    let event =
                        parse_fixed_length_message::<30, modifyordermessages::OrderExecuted, _>(
                            &mut consumer,
                        );
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (40, b'F') => {
                    let event = parse_fixed_length_message::<39, addordermessages::AddOrderMPID, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (25, b'H') => {
                    let event =
                        parse_fixed_length_message::<24, stockmessages::StockTradingAction, _>(
                            &mut consumer,
                        );
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (50, b'I') => {
                    let event = parse_fixed_length_message::<
                        49,
                        noiimessages::NetOrderImbalanceIndicator,
                        _,
                    >(&mut consumer);
                    msg_ct += 1;
                    // println!("{:?}", event);
                }
                (28, b'K') => {
                    let event =
                        parse_fixed_length_message::<27, stockmessages::IPOQuotingPeriodUpdate, _>(
                            &mut consumer,
                        );
                    msg_ct += 1;
                }
                (26, b'L') => {
                    let event = parse_fixed_length_message::<
                        25,
                        stockmessages::MarketParticipantPosition,
                        _,
                    >(&mut consumer);
                    msg_ct += 1;
                }
                (20, b'N') => {
                    let event = parse_fixed_length_message::<
                        19,
                        noiimessages::RetailPriceImprovementIndicator,
                        _,
                    >(&mut consumer);
                    msg_ct += 1;
                }
                (44, b'P') => {
                    let event = parse_fixed_length_message::<43, trademessages::NonCrossingTrade, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (40, b'Q') => {
                    // was very wrong
                    let event = parse_fixed_length_message::<39, trademessages::CrossingTrade, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (39, b'R') => {
                    let event = parse_fixed_length_message::<38, stockmessages::StockDirectory, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (12, b'S') => {
                    let event =
                        parse_fixed_length_message::<11, SystemEventMessage, _>(&mut consumer);
                    msg_ct += 1;
                }
                (35, b'U') => {
                    let event =
                        parse_fixed_length_message::<34, modifyordermessages::OrderReplace, _>(
                            &mut consumer,
                        );
                    msg_ct += 1;
                }
                (35, b'V') => {
                    let event = parse_fixed_length_message::<34, stockmessages::MWCBDeclineLevel, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (12, b'W') => {
                    let event = parse_fixed_length_message::<11, stockmessages::MWCBStatus, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (23, b'X') => {
                    let event = parse_fixed_length_message::<22, modifyordermessages::OrderCancel, _>(
                        &mut consumer,
                    );
                    msg_ct += 1;
                }
                (20, b'Y') => {
                    let event = parse_fixed_length_message::<
                        19,
                        stockmessages::RegSHOShortSalePriceTestRestriction,
                        _,
                    >(&mut consumer);
                    msg_ct += 1;
                }
                _ => {
                    consumer.pop_slice(&mut consumer_slice_size); // Skip useless bytes
                }
            }
            if msg_ct % 10_000_000 == 1 {
                println!(
                    "Processed {}m messages contained in {:.2}gb",
                    msg_ct / 1_000_000,
                    total_bytes_read
                );
            }
        }
    }

    Ok(())
}

// #[bench]
// fn bench_parse_fixed_length_message(b: &mut Bencher) {
//     let mut file = File::open("/home/luke/fastasx/data/12302019.NASDAQ_ITCH50").expect("failed");

//     let rb = HeapRb::<u8>::new(RING_BUFFER_SIZE); // Ringbuffer
//     let (mut producer, mut consumer) = rb.split();

//     let mut file_buffer = [0u8; FILE_BUFFER_SIZE];
//     let mut consumer_slice_size = [0u8; 3];

//     let mut msg_ct: u64 = 0;
//     let mut total_bytes_read: f64 = 0.0;

//     b.iter(|| {
//         loop {
//             // Fill the buffer
//             let bytes_read = file.read(&mut file_buffer).expect("failed");
//             total_bytes_read += bytes_read as f64 / 1024.0 / 1024.0 / 1024.0;

//             if bytes_read == 0 {
//                 println!("End of file");
//                 break;
//             }

//             producer.push_slice(&file_buffer[..bytes_read]);

//             // Process data from the ring buffer
//             while consumer.vacant_len() < FILE_BUFFER_SIZE {
//                 consumer.pop_slice(&mut consumer_slice_size);
//                 match (
//                     BigEndian::read_u16(&consumer_slice_size[..2]),
//                     &consumer_slice_size[2],
//                 ) {
//                     (36, b'A') => {
//                         let event = parse_fixed_length_message::<35, addordermessages::AddOrder, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (19, b'B') => {
//                         let event = parse_fixed_length_message::<18, trademessages::BrokenTrade, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (36, b'C') => {
//                         let event = parse_fixed_length_message::<
//                             35,
//                             modifyordermessages::OrderExecutedWithPrice,
//                             _,
//                         >(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     (19, b'D') => {
//                         let event = parse_fixed_length_message::<18, modifyordermessages::OrderDelete, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (31, b'E') => {
//                         let event =
//                             parse_fixed_length_message::<30, modifyordermessages::OrderExecuted, _>(
//                                 &mut consumer,
//                             );
//                         msg_ct += 1;
//                     }
//                     (40, b'F') => {
//                         let event = parse_fixed_length_message::<39, addordermessages::AddOrderMPID, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (25, b'H') => {
//                         let event =
//                             parse_fixed_length_message::<24, stockmessages::StockTradingAction, _>(
//                                 &mut consumer,
//                             );
//                         msg_ct += 1;
//                     }
//                     (50, b'I') => {
//                         let event = parse_fixed_length_message::<
//                             49,
//                             noiimessages::NetOrderImbalanceIndicator,
//                             _,
//                         >(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     (28, b'K') => {
//                         let event =
//                             parse_fixed_length_message::<27, stockmessages::IPOQuotingPeriodUpdate, _>(
//                                 &mut consumer,
//                             );
//                         msg_ct += 1;
//                     }
//                     (26, b'L') => {
//                         let event = parse_fixed_length_message::<
//                             25,
//                             stockmessages::MarketParticipantPosition,
//                             _,
//                         >(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     (20, b'N') => {
//                         let event = parse_fixed_length_message::<
//                             19,
//                             noiimessages::RetailPriceImprovementIndicator,
//                             _,
//                         >(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     (44, b'P') => {
//                         let event = parse_fixed_length_message::<43, trademessages::NonCrossingTrade, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (40, b'Q') => {
//                         let event = parse_fixed_length_message::<39, trademessages::CrossingTrade, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (39, b'R') => {
//                         let event = parse_fixed_length_message::<38, stockmessages::StockDirectory, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (12, b'S') => {
//                         let event =
//                             parse_fixed_length_message::<11, SystemEventMessage, _>(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     (35, b'U') => {
//                         let event =
//                             parse_fixed_length_message::<34, modifyordermessages::OrderReplace, _>(
//                                 &mut consumer,
//                             );
//                         msg_ct += 1;
//                     }
//                     (35, b'V') => {
//                         let event = parse_fixed_length_message::<34, stockmessages::MWCBDeclineLevel, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (12, b'W') => {
//                         let event = parse_fixed_length_message::<11, stockmessages::MWCBStatus, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (23, b'X') => {
//                         let event = parse_fixed_length_message::<22, modifyordermessages::OrderCancel, _>(
//                             &mut consumer,
//                         );
//                         msg_ct += 1;
//                     }
//                     (20, b'Y') => {
//                         let event = parse_fixed_length_message::<
//                             19,
//                             stockmessages::RegSHOShortSalePriceTestRestriction,
//                             _,
//                         >(&mut consumer);
//                         msg_ct += 1;
//                     }
//                     _ => {
//                         consumer.pop_slice(&mut consumer_slice_size); // Skip useless bytes
//                     }
//                 }
//                 if msg_ct % 10_000_000 == 1 {
//                     println!(
//                         "Processed {}m messages contained in {:.2}gb",
//                         msg_ct / 1_000_000,
//                         total_bytes_read
//                     );
//                     if msg_ct == 20_000_000 {
//                         break;
//                     }
//                 }
//             }
//         }
//     });
// }
