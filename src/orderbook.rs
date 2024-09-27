use crate::addordermessages::AddOrder;
use crate::enums::BuySellIndicator;
use crate::modifyordermessages::{OrderCancel, OrderDelete, OrderExecuted};
use crate::stockdirectory::StockDirectoryManager;
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

type Price = u32;
type StockLocateCode = u16;
type OrderReferenceNumber = u64;

/// We maintain two maps:
/// - order_books: This is the stock locate code > limit order book.
/// - order_price_map: This is the order reference number > (stock locate code, price, buy sell indicator). This is needed as no orders (except buys) have a price.
pub struct OrderBookManager {
    order_books: HashMap<StockLocateCode, LimitOrderBook>,
    order_price_map: HashMap<OrderReferenceNumber, (StockLocateCode, Price, BuySellIndicator)>,
    stock_directory: StockDirectoryManager,
}

/// Manages the order books for all stocks.
impl OrderBookManager {
    pub fn new() -> Self {
        log::debug!("Initialising OrderBookManager");
        Self {
            order_books: HashMap::new(),
            order_price_map: HashMap::new(),
            stock_directory: StockDirectoryManager::new(),
        }
    }

    pub fn add_order(&mut self, order: AddOrder) {
        self.order_price_map.insert(
            order.order_reference_number,
            (
                order.header.stock_locate,
                order.price,
                order.buy_sell_indicator.clone(),
            ),
        );
        self.order_books
            .entry(order.header.stock_locate)
            .or_insert_with(|| LimitOrderBook::new()) //  or_insert with handles missing value: creates new book
            .add_order(order);
    }

    /// Cancel some shares of an order from the order book.
    /// # Arguments
    /// * `order` - The order to cancel some shares of.
    pub fn order_cancelled(&mut self, order: OrderCancel) {
        let (stock_locate, price, buy_sell_indicator) = self
            .order_price_map
            .get(&order.order_reference_number)
            .unwrap()
            .clone();
        self.order_books
            .entry(stock_locate)
            .or_insert_with(LimitOrderBook::new)
            .cancel_order(order, price, buy_sell_indicator);
    }

    /// Delete an order from the order book.
    /// # Arguments
    /// * `order` - The order to delete.
    /// We remove the order from the order map (getting stock locate, price and buy sell indicator), then use this data to delete it from the order book.
    pub fn delete_order(&mut self, order: OrderDelete) {
        let (stock_locate, price, buy_sell_indicator) = self
            .order_price_map
            .remove(&order.order_reference_number)
            .unwrap();
        self.order_books
            .entry(stock_locate)
            .or_insert_with(LimitOrderBook::new)
            .delete_order(order, price, buy_sell_indicator);
    }
}

/// Per stock limit order book.
pub struct LimitOrderBook {
    ask_book: BTreeMap<Price, PriceBucket>,
    bid_book: BTreeMap<Reverse<Price>, PriceBucket>,

    lowest_ask: u32,
    highest_bid: u32,
}

impl LimitOrderBook {
    pub fn new() -> Self {
        Self {
            ask_book: BTreeMap::new(),
            bid_book: BTreeMap::new(),
            lowest_ask: 0,
            highest_bid: 0,
        }
    }

    pub fn add_order(&mut self, order: AddOrder) {
        match order.buy_sell_indicator {
            BuySellIndicator::Buy => {
                self.bid_book
                    .entry(Reverse(order.price))
                    .or_insert_with(PriceBucket::new)
                    .add_order(order);
            }
            BuySellIndicator::Sell => {
                self.ask_book
                    .entry(order.price)
                    .or_insert_with(PriceBucket::new)
                    .add_order(order);
            }
        }
    }

    pub fn cancel_order(
        &mut self,
        order: OrderCancel,
        price: Price,
        buy_sell_indicator: BuySellIndicator,
    ) {
        match buy_sell_indicator {
            BuySellIndicator::Buy => {
                let price_bucket = self.bid_book.get_mut(&Reverse(price)).unwrap();
                price_bucket.cancel_order(order.order_reference_number, order.canceled_shares);
            }
            BuySellIndicator::Sell => {
                let price_bucket = self.ask_book.get_mut(&price).unwrap();
                price_bucket.cancel_order(order.order_reference_number, order.canceled_shares);
            }
        }
    }

    pub fn delete_order(
        &mut self,
        order: OrderDelete,
        price: Price,
        buy_sell_indicator: BuySellIndicator,
    ) {
        match buy_sell_indicator {
            BuySellIndicator::Buy => {
                let price_bucket = self.bid_book.get_mut(&Reverse(price)).unwrap();
                price_bucket.remove_order(order.order_reference_number);
            }
            BuySellIndicator::Sell => {
                let price_bucket = self.ask_book.get_mut(&price).unwrap();
                price_bucket.remove_order(order.order_reference_number);
            }
        }
    }
}

/// A price bucket is a collection of orders at a given price.
struct PriceBucket {
    pub quantity: u32,
    pub orders: HashMap<OrderReferenceNumber, AddOrder>, // By order reference number
}

impl PriceBucket {
    fn new() -> Self {
        Self {
            quantity: 0,
            orders: HashMap::new(),
        }
    }

    fn add_order(&mut self, order: AddOrder) {
        self.quantity += order.shares;
        self.orders.insert(order.order_reference_number, order);
    }

    fn cancel_order(
        &mut self,
        order_reference_number: OrderReferenceNumber,
        cancelled_shares: u32,
    ) {
        let order = self.orders.get_mut(&order_reference_number).unwrap();
        self.quantity -= cancelled_shares;
        order.shares -= cancelled_shares;
    }

    fn remove_order(&mut self, order_reference_number: u64) -> Option<AddOrder> {
        let order = self.orders.remove(&order_reference_number)?;
        self.quantity -= order.shares;
        Some(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addordermessages::AddOrder;
    use crate::enums::BuySellIndicator;
    use crate::messageheader::MessageHeader;
    use crate::types::GenerateExampleMessage;

    #[test]
    fn test_add_order_to_book() {
        let mut book_manager = OrderBookManager::new();
        let header = MessageHeader::parse(&MessageHeader::generate_binary_example()); // Use same header for both orders

        let buy_order = AddOrder {
            header: header.clone(),
            order_reference_number: 1,
            buy_sell_indicator: BuySellIndicator::Buy,
            shares: 100,
            stock: *b"AAPL    ", // Stocks are left justified in the spec
            price: 15000,
            mpid: None,
        };
        let sell_order = AddOrder {
            header: header.clone(),
            order_reference_number: 2,
            buy_sell_indicator: BuySellIndicator::Sell,
            shares: 50,
            stock: *b"AAPL    ",
            price: 15100,
            mpid: Some(*b"JPMC"),
        };

        let stock_locate = header.stock_locate;
        book_manager.add_order(buy_order);
        book_manager.add_order(sell_order);

        assert_eq!(
            book_manager
                .order_books
                .get(&stock_locate)
                .unwrap()
                .bid_book
                .len(),
            1
        );
        assert_eq!(
            book_manager
                .order_books
                .get(&stock_locate)
                .unwrap()
                .ask_book
                .get(&15100)
                .unwrap()
                .orders
                .get(&2) // Order reference number
                .unwrap()
                .mpid,
            Some(*b"JPMC")
        );
        assert_eq!(
            book_manager
                .order_books
                .get(&stock_locate)
                .unwrap()
                .ask_book
                .get(&15100) // Price bucket
                .unwrap()
                .quantity,
            50
        );
    }

    #[test]
    fn test_delete_order_from_book() {
        use crate::addordermessages::AddOrder;
        use crate::enums::BuySellIndicator;
        use crate::messageheader::MessageHeader;
        use crate::modifyordermessages::OrderDelete;

        // Create a new OrderBookManager
        let mut book_manager = OrderBookManager::new();

        // Create an AddOrder to add to the book
        let add_order = AddOrder {
            header: MessageHeader::parse(&MessageHeader::generate_binary_example()),
            order_reference_number: 1,
            buy_sell_indicator: BuySellIndicator::Buy,
            shares: 100,
            stock: *b"AAPL    ", // Stocks are left justified in the spec
            price: 15000,
            mpid: Some(*b"JPMC"),
        };

        // Add the order to the book
        book_manager.add_order(add_order.clone());

        // Create an OrderDelete message to delete the order
        let delete_order = OrderDelete {
            header: MessageHeader {
                stock_locate: add_order.header.stock_locate,
                // Other necessary fields can be filled with dummy data or copied from add_order.header
                ..MessageHeader::parse(&MessageHeader::generate_binary_example())
            },
            order_reference_number: add_order.order_reference_number,
        };

        // Delete the order from the book
        book_manager.delete_order(delete_order);

        // Verify that the order is no longer in the book
        let order_book = book_manager
            .order_books
            .get(&add_order.header.stock_locate)
            .unwrap();
        let price_bucket = order_book.bid_book.get(&Reverse(add_order.price));

        if let Some(price_bucket) = price_bucket {
            assert!(
                !price_bucket
                    .orders
                    .contains_key(&add_order.order_reference_number),
                "Order was not deleted from the price bucket"
            );
        } else {
            // If the price bucket was removed because it's empty, the test passes
            assert!(true, "Price bucket was removed as expected");
        }

        // Verify that the order_price_map no longer contains the order
        assert!(
            !book_manager
                .order_price_map
                .contains_key(&add_order.order_reference_number),
            "Order was not removed from order_price_map"
        );
    }

    #[test]
    fn test_order_cancel() {
        let mut book_manager = OrderBookManager::new();

        // Create an AddOrder message
        let add_order = AddOrder {
            header: MessageHeader {
                stock_locate: 1234,
                // Other necessary fields can be filled with dummy data
                ..MessageHeader::parse(&MessageHeader::generate_binary_example())
            },
            order_reference_number: 987654321,
            buy_sell_indicator: BuySellIndicator::Buy,
            shares: 100,
            stock: *b"AAPL    ", // Stocks are left justified in the spec
            price: 15000,
            mpid: Some(*b"JPMC"),
        };

        // Add the order to the book
        book_manager.add_order(add_order.clone());

        // Create an OrderCancel message to cancel 30 shares
        let cancel_order = OrderCancel {
            header: MessageHeader {
                stock_locate: add_order.header.stock_locate,
                // Other necessary fields can be filled with dummy data or copied from add_order.header
                ..MessageHeader::parse(&MessageHeader::generate_binary_example())
            },
            order_reference_number: add_order.order_reference_number,
            canceled_shares: 30,
        };

        // Cancel 30 shares of the order
        book_manager.order_cancelled(cancel_order);

        // Verify that the order's shares have been reduced
        let order_book = book_manager
            .order_books
            .get(&add_order.header.stock_locate)
            .unwrap();
        let price_bucket = order_book.bid_book.get(&Reverse(add_order.price)).unwrap();
        let updated_order = price_bucket.orders.get(&add_order.order_reference_number).unwrap();

        assert_eq!(
            updated_order.shares,
            70,
            "Order shares were not correctly reduced after cancellation"
        );

        // Verify that the order is still in the order_price_map
        assert!(
            book_manager
                .order_price_map
                .contains_key(&add_order.order_reference_number),
            "Order was incorrectly removed from order_price_map"
        );
    }
}
