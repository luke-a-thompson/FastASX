use crate::addordermessages::AddOrder;
use crate::enums::BuySellIndicator;
use crate::modifyordermessages::{
    OrderCancel, OrderDelete, OrderExecuted, OrderExecutedWithPrice, OrderReplace,
};
use crate::types::{OrderBookError, Price4};
use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

pub type StockLocateCode = u16;
type OrderReferenceNumber = u64;

/// Manages the order books for all stocks.
///
/// # Fields
/// - order_books: This is the stock locate code > limit order book.
/// - order_price_map: This is the order reference number > (stock locate code, price, buy sell indicator). This is needed as no orders (except buys) have a price.
pub struct OrderBookManager {
    pub order_books: HashMap<StockLocateCode, LimitOrderBook>,
    order_price_map: HashMap<OrderReferenceNumber, (StockLocateCode, Price4, BuySellIndicator)>,
}

impl OrderBookManager {
    pub fn new() -> Self {
        log::debug!("Initialising OrderBookManager");
        Self {
            order_books: HashMap::new(),
            order_price_map: HashMap::new(),
        }
    }

    /// Add an order to the order book.
    ///
    /// # Arguments
    /// * `order` - The order to add.
    ///
    /// # Returns
    /// * `Ok(())` - If the order was added successfully.
    /// * `Err(OrderBookError)` - If the order could not be added.
    pub fn add_order(&mut self, order: AddOrder) -> Result<(), OrderBookError> {
        self.order_price_map.insert(
            order.order_reference_number,
            (
                order.header.stock_locate,
                order.price,
                order.buy_sell_indicator,
            ),
        );
        self.order_books
            .entry(order.header.stock_locate)
            .or_insert_with(|| LimitOrderBook::new()) //  or_insert with handles missing value: creates new book
            .add_order(order)?;
        Ok(())
    }

    /// Execute an order in the order book.
    ///
    /// # Arguments
    /// * `order` - The order to execute.
    ///
    /// # Returns
    /// * `Ok(u64)` - The match number of the executed order.
    /// * `Err(OrderBookError)` - If the order does not exist.
    pub fn execute_order(&mut self, order: OrderExecuted) -> Result<u64, OrderBookError> {
        match self.order_price_map.get(&order.order_reference_number) {
            Some((stock_locate, price, buy_sell_indicator)) => {
                self.order_books
                    .entry(*stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .cancel_order(
                        order.order_reference_number,
                        order.executed_shares,
                        *price, // Must deref as we do .get, which returns a borrow, not a value (like .remove)
                        *buy_sell_indicator,
                        false,
                    )?;
                Ok(order.match_number)
            }
            None => Err(OrderBookError::NonExistentOrder),
        }
    }

    /// Execute an order in the order book with a price.
    ///
    /// # Arguments
    /// * `order` - The order to execute.
    ///
    /// # Returns
    /// * `Ok((u64, Price, bool))` - The match number, execution price, and whether the order is printable.
    /// * `Err(OrderBookError)` - If the order does not exist.
    pub fn execute_order_with_price(
        &mut self,
        order: OrderExecutedWithPrice,
    ) -> Result<(u64, Price4, bool), OrderBookError> {
        match self
            .order_price_map
            .get(&order.order_executed_message.order_reference_number)
        {
            Some((stock_locate, price, buy_sell_indicator)) => {
                self.order_books
                    .entry(*stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .cancel_order(
                        order.order_executed_message.order_reference_number,
                        order.order_executed_message.executed_shares,
                        *price, // Must deref as we do .get, which returns a borrow, not a value (like .remove)
                        *buy_sell_indicator,
                        false,
                    )?;
                Ok((
                    order.order_executed_message.match_number,
                    order.exec_price,
                    order.printable,
                ))
            }
            None => Err(OrderBookError::NonExistentOrder),
        }
    }

    /// Replace an order in the order book. Changes the order reference number, shares, price.
    ///
    /// # Arguments
    /// * `order` - The order to replace.
    ///
    /// # Returns
    /// * `Ok(())` - If the order was replaced successfully.
    /// * `Err(OrderBookError)` - If the order could not be replaced.
    pub fn replace_order(&mut self, order: OrderReplace) -> Result<(), OrderBookError> {
        match self
            .order_price_map
            .remove(&order.original_order_reference_number)
        {
            Some((stock_locate, price, old_buy_sell_indicator)) => {
                let mut old_order = self
                    .order_books
                    .entry(stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .delete_order(
                        order.original_order_reference_number,
                        price,
                        old_buy_sell_indicator,
                    )?;

                // Insert the new order into the order price map.
                self.order_price_map.insert(
                    order.new_order_reference_number,
                    (stock_locate, order.price, old_buy_sell_indicator),
                );

                // Update the old order with the new order's details.
                old_order.order_reference_number = order.new_order_reference_number;
                old_order.shares = order.shares;
                old_order.price = order.price;
                // Insert the new order into the order book.
                self.order_books
                    .entry(stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .add_order(old_order)?;
                Ok(())
            }
            None => {
                log::warn!("Attempted to replace non-existent order: {:?}", order);
                Ok(())
            }
        }
    }

    /// Cancel some shares of an order from the order book.
    ///
    /// # Arguments
    /// * `order` - The order to cancel some shares of.
    ///
    /// # Returns
    /// * `Ok(())` - If the order was cancelled successfully.
    /// * `Err(OrderBookError)` - If the order could not be cancelled.
    pub fn cancel_order(&mut self, order: OrderCancel) -> Result<(), OrderBookError> {
        match self.order_price_map.get(&order.order_reference_number) {
            Some((stock_locate, price, buy_sell_indicator)) => {
                self.order_books
                    .entry(*stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .cancel_order(
                        order.order_reference_number,
                        order.canceled_shares,
                        *price, // Must deref as we do .get, which returns a borrow, not a value (like .remove)
                        *buy_sell_indicator,
                        true,
                    )?;
                Ok(())
            }
            None => {
                log::warn!("Attempted to cancel non-existent order: {:?}", order);
                Ok(())
            }
        }
    }

    /// Delete an order from the order book.
    /// # Arguments
    /// * `order` - The order to delete.
    ///
    /// # Returns
    /// * `Ok(())` - If the order was deleted successfully.
    /// * `Err(OrderBookError)` - If the order could not be deleted.
    /// We remove the order from the order map (getting stock locate, price and buy sell indicator), then use this data to delete it from the order book.
    pub fn delete_order(&mut self, order: OrderDelete) -> Result<(), OrderBookError> {
        match self.order_price_map.remove(&order.order_reference_number) {
            Some((stock_locate, price, buy_sell_indicator)) => {
                self.order_books
                    .entry(stock_locate)
                    .or_insert_with(LimitOrderBook::new)
                    .delete_order(order.order_reference_number, price, buy_sell_indicator)?;
                Ok(())
            }
            None => {
                log::warn!("Attempted to delete non-existent order: {:?}", order);
                Ok(())
            }
        }
    }
}

/// Per stock limit order book.
///
/// # Fields
/// * `ask_book` - The ask book, which is a BTreeMap of prices to PriceBucket.
/// * `bid_book` - The bid book, which is a BTreeMap of reverse prices to PriceBucket.
/// * `highest_bid` - The highest bid price.
/// * `lowest_ask` - The lowest ask price.
pub struct LimitOrderBook {
    pub ask_book: BTreeMap<Price4, PriceBucket>,
    pub bid_book: BTreeMap<Reverse<Price4>, PriceBucket>,
    pub highest_bid: u32,
    pub lowest_ask: u32,
}

impl LimitOrderBook {
    pub fn new() -> Self {
        log::debug!("Creating new limit order book");
        Self {
            ask_book: BTreeMap::new(),
            bid_book: BTreeMap::new(),
            highest_bid: 0,
            lowest_ask: 0,
        }
    }

    /// Update the lowest ask and highest bid prices.
    ///
    /// # Returns
    /// * `Ok(())` - If the order was added successfully.
    /// * `Err(OrderBookError)` - If the order could not be added.
    fn update_best_prices(&mut self) {
        self.lowest_ask = self.ask_book.keys().next().map(|&p| p.value).unwrap_or(u32::MAX);
        self.highest_bid = self.bid_book.keys().next().map(|r| r.0.value).unwrap_or(0);
    }

    /// Get the current spread (difference between lowest ask and highest bid).
    ///
    /// # Returns
    /// * `u32` - The spread between the lowest ask and highest bid.
    pub fn get_spread(&self) -> u32 {
        self.lowest_ask.saturating_sub(self.highest_bid)
    }

    /// Get the current best ask price.
    ///
    /// # Returns
    /// * `Option<u32>` - The current best ask price.
    pub fn get_best_ask(&self) -> Option<u32> {
        if self.lowest_ask == u32::MAX {
            None
        } else {
            Some(self.lowest_ask)
        }
    }

    /// Get the current best bid price.
    ///
    /// # Returns
    /// * `Option<u32>` - The current best bid price.
    pub fn get_best_bid(&self) -> Option<u32> {
        if self.highest_bid == 0 {
            None
        } else {
            Some(self.highest_bid)
        }
    }

    pub fn add_order(&mut self, order: AddOrder) -> Result<(), OrderBookError> {
        match order.buy_sell_indicator {
            BuySellIndicator::Buy => {
                self.bid_book
                    .entry(Reverse(order.price))
                    .or_insert_with(PriceBucket::new)
                    .add_order(order)?;
                self.update_best_prices();
            }
            BuySellIndicator::Sell => {
                self.ask_book
                    .entry(order.price)
                    .or_insert_with(PriceBucket::new)
                    .add_order(order)?;
                self.update_best_prices();
            }
        }
        Ok(())
    }

    pub fn cancel_order(
        &mut self,
        order_reference_number: OrderReferenceNumber,
        cancelled_shares: u32,
        price: Price4,
        buy_sell_indicator: BuySellIndicator,
        order_cancellation: bool, // If true, the order is being cancelled, not executed
    ) -> Result<(), OrderBookError> {
        match buy_sell_indicator {
            BuySellIndicator::Buy => {
                let price_bucket = self.bid_book.get_mut(&Reverse(price)).unwrap();
                price_bucket.cancel_order(
                    order_reference_number,
                    cancelled_shares,
                    order_cancellation,
                )?;
                self.update_best_prices();
            }
            BuySellIndicator::Sell => {
                let price_bucket = self.ask_book.get_mut(&price).unwrap();
                price_bucket.cancel_order(
                    order_reference_number,
                    cancelled_shares,
                    order_cancellation,
                )?;
                self.update_best_prices();
            }
        }
        Ok(())
    }

    pub fn delete_order(
        &mut self,
        order_reference_number: OrderReferenceNumber,
        price: Price4,
        buy_sell_indicator: BuySellIndicator,
    ) -> Result<AddOrder, OrderBookError> {
        match buy_sell_indicator {
            BuySellIndicator::Buy => {
                let price_bucket = self.bid_book.get_mut(&Reverse(price)).unwrap();
                let order = price_bucket.delete_order(order_reference_number)?;

                if price_bucket.share_quantity == 0 {
                    log::trace!("Buy side price bucket at {} is empty, removing it", price);
                    self.bid_book.remove(&Reverse(price));
                }

                self.update_best_prices();
                Ok(order)
            }
            BuySellIndicator::Sell => {
                let price_bucket = self.ask_book.get_mut(&price).unwrap();
                let order = price_bucket.delete_order(order_reference_number)?;

                if price_bucket.share_quantity == 0 {
                    log::trace!("Sell side price bucket at {} is empty, removing it", price);
                    self.ask_book.remove(&price);
                }

                self.update_best_prices();
                Ok(order)
            }
        }
    }
}

/// A price bucket is a collection of orders at a given price.
pub struct PriceBucket {
    pub share_quantity: u32,
    pub orders: HashMap<OrderReferenceNumber, AddOrder>, // By order reference number
}

impl PriceBucket {
    fn new() -> Self {
        log::trace!("Creating new price bucket");
        Self {
            share_quantity: 0,
            orders: HashMap::new(),
        }
    }

    fn add_order(&mut self, order: AddOrder) -> Result<(), OrderBookError> {
        self.share_quantity += order.shares;
        match self.orders.insert(order.order_reference_number, order) {
            Some(_) => Err(OrderBookError::DuplicateOrder),
            None => Ok(()),
        }
    }

    /// Cancel shares off an order in order book.
    /// # Arguments
    /// * `order_reference_number` - The order reference number of the order to cancel.
    /// * `cancelled_shares` - The number of shares to cancel.
    /// * `order_cancellation` - If true, the order is being cancelled, not executed.
    fn cancel_order(
        &mut self,
        order_reference_number: OrderReferenceNumber,
        cancelled_shares: u32,
        order_cancellation: bool, // If true, the order is being cancelled, not executed
    ) -> Result<(), OrderBookError> {
        let order = self
            .orders
            .get_mut(&order_reference_number)
            .ok_or(OrderBookError::NonExistentOrder)?;
        if cancelled_shares > self.share_quantity || cancelled_shares > order.shares {
            return Err(OrderBookError::InvalidCancellation);
        }
        self.share_quantity -= cancelled_shares;
        order.shares -= cancelled_shares;

        if order.shares == 0 && order_cancellation {
            log::warn!("Order cancellation resulted in 0 shares, deleting order");
            self.delete_order(order_reference_number)?;
        }

        Ok(())
    }

    fn delete_order(
        &mut self,
        order_reference_number: OrderReferenceNumber,
    ) -> Result<AddOrder, OrderBookError> {
        let order = self
            .orders
            .remove(&order_reference_number)
            .ok_or(OrderBookError::NonExistentOrder)?;
        self.share_quantity -= order.shares;
        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addordermessages::AddOrder;
    use crate::enums::BuySellIndicator;
    use crate::messageheader::MessageHeader;
    use crate::types::{GenerateExampleMessage, PriceConversions};

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
            price: Price4::new(15000u32),
            mpid: None,
        };
        let sell_order = AddOrder {
            header: header.clone(),
            order_reference_number: 2,
            buy_sell_indicator: BuySellIndicator::Sell,
            shares: 50,
            stock: *b"AAPL    ",
            price: Price4::new(15100u32),
            mpid: Some(*b"JPMC"),
        };

        let stock_locate = header.stock_locate;
        book_manager.add_order(buy_order).unwrap();
        book_manager.add_order(sell_order).unwrap();

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
                .get(&Price4::new(15100u32))
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
                .get(&Price4::new(15100u32)) // Price bucket
                .unwrap()
                .share_quantity,
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
            price: Price4::new(15000u32),
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
            price: Price4::new(15000u32),
            mpid: Some(*b"JPMC"),
        };

        // Add the order to the book
        book_manager.add_order(add_order.clone()).unwrap();

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
        book_manager.cancel_order(cancel_order).unwrap();

        // Verify that the order's shares have been reduced
        let order_book = book_manager
            .order_books
            .get(&add_order.header.stock_locate)
            .unwrap();
        let price_bucket = order_book.bid_book.get(&Reverse(add_order.price)).unwrap();
        let updated_order = price_bucket
            .orders
            .get(&add_order.order_reference_number)
            .unwrap();

        assert_eq!(
            updated_order.shares, 70,
            "Order shares were not correctly reduced after cancellation"
        );

        // Verify that the order is still in the order_price_map
        assert!(
            book_manager
                .order_price_map
                .contains_key(&add_order.order_reference_number),
            "Order was incorrectly removed from order_price_map"
        );

        // Test case where order shares are updated to be < 0
        let invalid_cancel_order = OrderCancel {
            header: MessageHeader {
                stock_locate: add_order.header.stock_locate,
                ..MessageHeader::parse(&MessageHeader::generate_binary_example())
            },
            order_reference_number: add_order.order_reference_number,
            canceled_shares: 80, // This would bring the total to -10 shares
        };

        // Attempt to cancel more shares than available
        let result = book_manager.cancel_order(invalid_cancel_order);

        // Verify that an error is returned
        assert!(
            result.is_err(),
            "Expected an error when canceling more shares than available"
        );

        if let Err(error) = result {
            assert!(
                matches!(error, OrderBookError::InvalidCancellation),
                "Expected InvalidCancellation error, got {:?}",
                error
            );
        }
    }

    #[test]
    fn test_replace_order() {
        let mut book_manager = OrderBookManager::new();
        let header = MessageHeader::parse(&MessageHeader::generate_binary_example());

        // Add an initial order
        let initial_order = AddOrder {
            header: header.clone(),
            order_reference_number: 1,
            buy_sell_indicator: BuySellIndicator::Buy,
            shares: 100,
            stock: *b"AAPL    ",
            price: Price4::new(15000u32),
            mpid: None,
        };
        book_manager.add_order(initial_order.clone()).unwrap();

        // Create a replace order
        let replace_order = OrderReplace {
            header: header.clone(),
            original_order_reference_number: 1,
            new_order_reference_number: 2,
            shares: 150,
            price: Price4::new(15100u32),
        };

        // Replace the order
        book_manager.replace_order(replace_order.clone()).unwrap();

        // Verify that the old order is removed
        assert!(
            !book_manager.order_price_map.contains_key(&1),
            "Original order was not removed from order_price_map"
        );

        // Verify that the new order is added
        let (stock_locate, price, buy_sell_indicator) =
            book_manager.order_price_map.get(&2).unwrap();
        assert_eq!(*stock_locate, initial_order.header.stock_locate);
        assert_eq!(*price, replace_order.price);
        assert_eq!(*buy_sell_indicator, initial_order.buy_sell_indicator);

        // Verify that the order book is updated
        let order_book = book_manager
            .order_books
            .get(&initial_order.header.stock_locate)
            .unwrap();
        let price_bucket = order_book
            .bid_book
            .get(&Reverse(replace_order.price))
            .unwrap();
        let updated_order = price_bucket
            .orders
            .get(&replace_order.new_order_reference_number)
            .unwrap();

        assert_eq!(updated_order.shares, replace_order.shares);
        assert_eq!(updated_order.price, replace_order.price);

        // Test replacing a non-existent order
        let invalid_replace_order = OrderReplace {
            header: header.clone(),
            original_order_reference_number: 999, // Non-existent order
            new_order_reference_number: 3,
            shares: 200,
            price: Price4::new(15200u32),
        };

        let result = book_manager.replace_order(invalid_replace_order);
        assert!(
            result.is_err(),
            "Expected an error when replacing a non-existent order"
        );

        if let Err(error) = result {
            assert!(
                matches!(error, OrderBookError::NonExistentOrder),
                "Expected NonExistentOrder error, got {:?}",
                error
            );
        }
    }
}
