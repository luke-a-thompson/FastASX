#![feature(test)]
extern crate test;

use crate::tui::run;
use byteorder::{BigEndian, ByteOrder};
use orderbook::OrderBookManager;
use ringbuf::{traits::*, HeapRb};
use std::fs::File;
use std::io::{self, Read};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use types::{
    AltBinaryMessageLength, AltMessageHeaderType, BinaryMessageLength, MessageHeaderType, Parse,
    ParseError,
};

#[cfg(any(test, feature = "bench"))]
mod tests;

pub mod addordermessages;
pub mod enums;
pub mod helpers;
pub mod messageheader;
pub mod modifyordermessages;
pub mod noiimessages;
pub mod orderbook;
pub mod stockdirectory;
pub mod stockmessages;
pub mod systemmessages;
pub mod trademessages;
pub mod tui;
pub mod types;

const FILE_BUFFER_SIZE: usize = 2048 * 64; // Stack allocated
const RING_BUFFER_SIZE: usize = 4096 * 2048; // Heap allocated

/// Parses a message of fixed length N.
/// If the message is not complete, it will attempt to parse an incomplete message.
/// If the message is not parseable, it will return an error.
///
/// # Arguments
/// * `consumer` - A consumer of bytes - RingBuf crate
/// * `N` - The fixed length of the message. In the case of ITCH, this is present in the SoupBIN TCP header.
/// * `T` - The type of message to parse
/// * `C` - The type of consumer
///
/// # Returns
/// * `Result<T, ParseError>` - The parsed message, or an error if the message is not parseable
fn parse_fixed_length_message<const N: usize, T, C>(consumer: &mut C) -> Result<T, ParseError>
where
    C: Consumer<Item = u8>,
    T: Parse + BinaryMessageLength,
{
    let mut buffer = [0u8; N]; // N is the fixed length of the message
    let bytes_read = consumer.pop_slice(&mut buffer);

    // Only parse incomplete when the message buffer length is less than the message length
    if bytes_read < N {
        let result = parse_incomplete_message::<T, C, N>(consumer, &mut buffer, bytes_read)?;
        return Ok(result);
    }
    let result = T::parse(&buffer)?;
    Ok(result)
}

/// Parses an incomplete message of fixed length N.
/// If the message is not complete, it will attempt to parse an incomplete message.
/// If the message is not parseable, it will return an error.
///
/// # Arguments
/// * `consumer` - A consumer of bytes - RingBuf crate
/// * `N` - The fixed length of the message. This is used to determine the number of bytes required to parse the message.
/// * `T` - The type of message to parse
/// * `C` - The type of consumer
fn parse_incomplete_message<T, C, const N: usize>(
    consumer: &mut C,
    message_buffer: &mut [u8; N],
    mut message_buffer_len: usize,
) -> Result<T, ParseError>
where
    C: Consumer<Item = u8>,
    T: Parse,
{
    loop {
        let remaining_required_bytes = N - message_buffer_len;
        if remaining_required_bytes == 0 {
            // No more bytes required
            break;
        }

        let bytes_read = consumer.pop_slice(
            &mut message_buffer[message_buffer_len..remaining_required_bytes + message_buffer_len],
        );
        message_buffer_len += bytes_read;
    }
    T::parse(&message_buffer[..N])
}

#[allow(unused_variables)]
pub fn main() -> Result<(), io::Error> {
    env_logger::init();

    let mut file = File::open("/home/luke/fastasx/data/12302019.NASDAQ_ITCH50")?;

    let rb = HeapRb::<u8>::new(RING_BUFFER_SIZE); // Ringbuffer
    let (mut producer, mut consumer) = rb.split();

    let mut consumer_slice_size = [0u8; 3];

    let mut msg_ct: u64 = 0;
    let mut last_million_time = std::time::Instant::now();

    let producer_done = AtomicBool::new(false);

    let order_book_manager = Arc::new(RwLock::new(OrderBookManager::new()));
    let order_book_manager_clone = Arc::clone(&order_book_manager);

    let stock_directory_manager = Arc::new(RwLock::new(stockdirectory::StockDirectoryManager::new()));
    let stock_directory_manager_clone = Arc::clone(&stock_directory_manager);

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

                if bytes_read == 0 {
                    log::info!("End of file");
                    break;
                }
                producer.push_slice(&file_buffer[..bytes_read]);
            }
            producer_done.store(true, std::sync::atomic::Ordering::Relaxed);
            println!("EOF, Producer done: {total_bytes_read:.2}gb");
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
                    log::warn!("Message length too long: {:?}", length);
                    continue;
                };
                match &consumer_slice_size[2] {
                    &addordermessages::AddOrder::MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { addordermessages::AddOrder::LENGTH },
                            addordermessages::AddOrder,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .add_order(order)
                            .expect("order book add failed");
                        msg_ct += 1;
                        log::trace!("Parsed AddOrder");
                    }
                    &addordermessages::AddOrder::ALT_MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { addordermessages::AddOrder::ALT_LENGTH },
                            addordermessages::AddOrder,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .add_order(order)
                            .expect("order book add failed");
                        msg_ct += 1;
                        log::trace!("Parsed AddOrder");
                    }
                    &trademessages::BrokenTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::BrokenTrade::LENGTH },
                            trademessages::BrokenTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed BrokenTrade");
                    }
                    &modifyordermessages::OrderExecuted::MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { modifyordermessages::OrderExecuted::LENGTH },
                            modifyordermessages::OrderExecuted,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .execute_order(order)
                            .expect("order book execute failed");
                        msg_ct += 1;
                        log::trace!("Parsed OrderExecuted");
                    }
                    &modifyordermessages::OrderExecutedWithPrice::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { modifyordermessages::OrderExecutedWithPrice::LENGTH },
                            modifyordermessages::OrderExecutedWithPrice,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed OrderExecutedWithPrice");
                    }
                    &modifyordermessages::OrderDelete::MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { modifyordermessages::OrderDelete::LENGTH },
                            modifyordermessages::OrderDelete,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .delete_order(order)
                            .expect("order book delete failed");
                        msg_ct += 1;
                        log::trace!("Parsed OrderDelete");
                    }
                    &stockmessages::StockTradingAction::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::StockTradingAction::LENGTH },
                            stockmessages::StockTradingAction,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed StockTradingAction");
                    }
                    &noiimessages::NetOrderImbalanceIndicator::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { noiimessages::NetOrderImbalanceIndicator::LENGTH },
                            noiimessages::NetOrderImbalanceIndicator,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed NetOrderImbalanceIndicator");
                    }
                    &stockmessages::IPOQuotingPeriodUpdate::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::IPOQuotingPeriodUpdate::LENGTH },
                            stockmessages::IPOQuotingPeriodUpdate,
                            _,
                        >(&mut consumer)
                        .expect(&format!("{consumer_slice_size:x?} msg parse failed"));
                        msg_ct += 1;
                        log::trace!("Parsed IPOQuotingPeriodUpdate");
                    }
                    &stockmessages::MarketParticipantPosition::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MarketParticipantPosition::LENGTH },
                            stockmessages::MarketParticipantPosition,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed MarketParticipantPosition");
                    }
                    &noiimessages::RetailPriceImprovementIndicator::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { noiimessages::RetailPriceImprovementIndicator::LENGTH },
                            noiimessages::RetailPriceImprovementIndicator,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed RetailPriceImprovementIndicator");
                    }
                    &trademessages::NonCrossingTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::NonCrossingTrade::LENGTH },
                            trademessages::NonCrossingTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed NonCrossingTrade");
                    }
                    &trademessages::CrossingTrade::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { trademessages::CrossingTrade::LENGTH },
                            trademessages::CrossingTrade,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed CrossingTrade");
                    }
                    &stockmessages::StockDirectory::MESSAGE_TYPE => {
                        let message = parse_fixed_length_message::<
                            { stockmessages::StockDirectory::LENGTH },
                            stockmessages::StockDirectory,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");

                        stock_directory_manager.write().unwrap().add_stock(message);
                        msg_ct += 1;
                        log::trace!("Parsed StockDirectory");
                    }
                    &systemmessages::SystemEventMessage::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { systemmessages::SystemEventMessage::LENGTH },
                            systemmessages::SystemEventMessage,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed SystemEventMessage");
                    }
                    &modifyordermessages::OrderReplace::MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { modifyordermessages::OrderReplace::LENGTH },
                            modifyordermessages::OrderReplace,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .replace_order(order)
                            .expect("order book replace failed");
                        msg_ct += 1;
                        log::trace!("Parsed OrderReplace");
                    }
                    &stockmessages::MWCBDeclineLevel::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MWCBDeclineLevel::LENGTH },
                            stockmessages::MWCBDeclineLevel,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed MWCBDeclineLevel");
                    }
                    &stockmessages::MWCBStatus::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::MWCBStatus::LENGTH },
                            stockmessages::MWCBStatus,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed MWCBStatus");
                    }
                    &modifyordermessages::OrderCancel::MESSAGE_TYPE => {
                        let order = parse_fixed_length_message::<
                            { modifyordermessages::OrderCancel::LENGTH },
                            modifyordermessages::OrderCancel,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        order_book_manager
                            .write()
                            .unwrap()
                            .cancel_order(order)
                            .expect("order book cancel failed");
                        msg_ct += 1;
                        log::trace!("Parsed OrderCancel");
                    }
                    &stockmessages::RegSHOShortSalePriceTestRestriction::MESSAGE_TYPE => {
                        parse_fixed_length_message::<
                            { stockmessages::RegSHOShortSalePriceTestRestriction::LENGTH },
                            stockmessages::RegSHOShortSalePriceTestRestriction,
                            _,
                        >(&mut consumer)
                        .expect("msg parse failed");
                        msg_ct += 1;
                        log::trace!("Parsed RegSHOShortSalePriceTestRestriction");
                    }
                    _ => {
                        consumer.pop_slice(&mut consumer_slice_size); // Skip useless bytes
                    }
                }
                if msg_ct % 1_000_000 == 0 {
                    let elapsed = last_million_time.elapsed();
                    log::debug!(
                        "Processed {}m messages in {:.2?} ({:.2}m messages/second)",
                        msg_ct / 1_000_000,
                        elapsed,
                        1_000_000.0 / elapsed.as_secs_f64() / 1_000_000.0
                    );
                    last_million_time = std::time::Instant::now();
                }
                consumer_slice_size = [0u8; 3];
            }
            Ok(())
        });
        s.spawn(move || -> Result<(), io::Error> {
            let mut terminal = ratatui::init();
            terminal.clear()?;
            let app_result = run(
                terminal,
                order_book_manager_clone,
                stock_directory_manager_clone,
            );
            ratatui::restore();
            app_result
        });
    });
    Ok(())
}
