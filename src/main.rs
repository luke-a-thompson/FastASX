#![feature(test)]
extern crate test;

use byteorder::{BigEndian, ByteOrder};
use ringbuf::{traits::*, HeapRb};
use std::fs::File;
use std::io::{self, Read};
use std::sync::atomic::AtomicBool;
use types::{AltBinaryMessageLength, AltMessageHeaderType, BinaryMessageLength, MessageHeaderType, Parse, ParseError};

#[cfg(any(test, feature = "bench"))]
mod tests;

pub mod orderbook;
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

const FILE_BUFFER_SIZE: usize = 2048 * 64; // Stack allocated
const RING_BUFFER_SIZE: usize = 4096 * 2048; // Heap allocated

fn parse_fixed_length_message<const N: usize, T, C>(consumer: &mut C) -> Result<T, ParseError>
where
    C: Consumer<Item = u8>,
    T: Parse + BinaryMessageLength,
{
    let mut buffer = [0u8; N];
    let bytes_read = consumer.pop_slice(&mut buffer);

    if bytes_read < N {
        // Only parse incomplete when the message buffer length is less than the message length
        // println!("{:?}", buffer);
        let result = parse_incomplete_message::<T, C, N>(consumer, &mut buffer, bytes_read);
        if result.is_err() {
            // println!("{:?}", buffer);
            // println!("Failed to parse incomplete message");
        }
        return result;
    }
    let result = T::parse(&buffer);
    if result.is_err() {
        // println!("{:x?}", buffer);
        // println!("Failed to parse message");
    }
    result
}

fn parse_incomplete_message<T, C, const N: usize>(
    consumer: &mut C,
    message_buffer: &mut [u8; N],
    mut message_buffer_len: usize,
) -> Result<T, ParseError>
where
    T: Parse + BinaryMessageLength,
    C: Consumer<Item = u8>,
{
    loop {
        let remaining_required_bytes = T::LENGTH - message_buffer_len;
        if remaining_required_bytes == 0 {
            // No more bytes required
            break;
        }

        let bytes_read = consumer.pop_slice(
            &mut message_buffer[message_buffer_len..remaining_required_bytes + message_buffer_len],
        );
        message_buffer_len += bytes_read;
    }
    T::parse(&message_buffer[..T::LENGTH])
}

#[allow(unused_variables)]
pub fn main() -> Result<(), io::Error> {
    let mut file = File::open("/home/luke/fastasx/data/12302019.NASDAQ_ITCH50")?;

    let rb = HeapRb::<u8>::new(RING_BUFFER_SIZE); // Ringbuffer
    let (mut producer, mut consumer) = rb.split();

    let mut consumer_slice_size = [0u8; 3];

    let mut msg_ct: u64 = 0;

    let producer_done = AtomicBool::new(false);

    std::thread::scope(|s| {
        s.spawn(|| -> Result<(), io::Error> {
            let mut file_buffer = [0u8; FILE_BUFFER_SIZE];
            let mut total_bytes_read: f64 = 0.0;

            loop {
                if producer.vacant_len() < (RING_BUFFER_SIZE as f64 * 0.1) as usize {
                    continue;
                }
                let bytes_read = file.read(&mut file_buffer)?;
                total_bytes_read += bytes_read as f64 / 1024.0 / 1024.0 / 1024.0;
                // println!("Read {} bytes", bytes_read);

                if bytes_read == 0 {
                    println!("End of file");
                    break;
                }
                producer.push_slice(&file_buffer[..bytes_read]);
            }
            println!("Producer done: {total_bytes_read:.2}gb");
            producer_done.store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(())
        });
        s.spawn(|| -> Result<(), io::Error> {
            while producer_done.load(std::sync::atomic::Ordering::Relaxed) == false {
                // This prevents the consumption of only the message header (len + type) if the producer is too slow to push a whole message
                if consumer.occupied_len() < consumer_slice_size.len() {
                    continue;
                }

                consumer.pop_slice(&mut consumer_slice_size);
                let length = BigEndian::read_u16(&consumer_slice_size[0..2]);
                if length > 50 {
                    println!("Message length too long: {:?}", length);
                    // consumer.pop_slice(&mut vec![0u8; length as usize]);
                    continue;
                };
                match &consumer_slice_size[2] {
                    &addordermessages::AddOrder::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { addordermessages::AddOrder::LENGTH },
                            addordermessages::AddOrder,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &trademessages::BrokenTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::BrokenTrade::LENGTH },
                            trademessages::BrokenTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &modifyordermessages::OrderExecutedWithPrice::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderExecutedWithPrice::LENGTH },
                            modifyordermessages::OrderExecutedWithPrice,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &modifyordermessages::OrderDelete::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderDelete::LENGTH },
                            modifyordermessages::OrderDelete,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &modifyordermessages::OrderExecuted::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderExecuted::LENGTH },
                            modifyordermessages::OrderExecuted,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &addordermessages::AddOrder::ALT_MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { addordermessages::AddOrder::ALT_LENGTH },
                            addordermessages::AddOrder,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::StockTradingAction::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::StockTradingAction::LENGTH },
                            stockmessages::StockTradingAction,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &noiimessages::NetOrderImbalanceIndicator::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { noiimessages::NetOrderImbalanceIndicator::LENGTH },
                            noiimessages::NetOrderImbalanceIndicator,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::IPOQuotingPeriodUpdate::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::IPOQuotingPeriodUpdate::LENGTH },
                            stockmessages::IPOQuotingPeriodUpdate,
                            _,
                        >(&mut consumer)
                        .expect(&format!("{consumer_slice_size:x?} msg parse failed"));
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::MarketParticipantPosition::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MarketParticipantPosition::LENGTH },
                            stockmessages::MarketParticipantPosition,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &noiimessages::RetailPriceImprovementIndicator::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { noiimessages::RetailPriceImprovementIndicator::LENGTH },
                            noiimessages::RetailPriceImprovementIndicator,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &trademessages::NonCrossingTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::NonCrossingTrade::LENGTH },
                            trademessages::NonCrossingTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &trademessages::CrossingTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::CrossingTrade::LENGTH },
                            trademessages::CrossingTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::StockDirectory::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::StockDirectory::LENGTH },
                            stockmessages::StockDirectory,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &systemmessages::SystemEventMessage::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { systemmessages::SystemEventMessage::LENGTH },
                            systemmessages::SystemEventMessage,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &modifyordermessages::OrderReplace::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderReplace::LENGTH },
                            modifyordermessages::OrderReplace,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::MWCBDeclineLevel::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MWCBDeclineLevel::LENGTH },
                            stockmessages::MWCBDeclineLevel,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::MWCBStatus::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MWCBStatus::LENGTH },
                            stockmessages::MWCBStatus,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &modifyordermessages::OrderCancel::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderCancel::LENGTH },
                            modifyordermessages::OrderCancel,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    &stockmessages::RegSHOShortSalePriceTestRestriction::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::RegSHOShortSalePriceTestRestriction::LENGTH },
                            stockmessages::RegSHOShortSalePriceTestRestriction,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        // println!("{:?}", event);
                    }
                    _ => {
                        consumer.pop_slice(&mut consumer_slice_size); // Skip useless bytes
                    }
                }
                if msg_ct % 1_000_000 == 1 {
                    println!("Processed {}m messages.", msg_ct / 1_000_000,);
                }
                consumer_slice_size = [0u8; 3];
            }
            Ok(())
        });
    });
    Ok(())
}

// #[allow(unused_variables)]
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
