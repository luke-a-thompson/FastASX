use crate::orderbook::{OrderBookManager, StockLocateCode};
use crate::stockdirectory::StockDirectoryManager;
use crate::types::PriceConversions;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::Stylize,
    widgets::{BarChart, Paragraph},
    DefaultTerminal,
};
use std::io;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub fn run(
    mut terminal: DefaultTerminal,
    order_book_manager: Arc<RwLock<OrderBookManager>>,
    stock_directory_manager: Arc<RwLock<StockDirectoryManager>>,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        ratatui::layout::Constraint::Percentage(100),
                        ratatui::layout::Constraint::Min(100),
                    ]
                    .as_ref(),
                )
                .split(size);

            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, chunks[0]);

            // Acquire read lock within the draw closure
            let order_manager = order_book_manager.read().unwrap();
            let stock_directory_manager = stock_directory_manager.read().unwrap();

            let stock = b"AAPL    ";
            let stock_str = std::str::from_utf8(stock).unwrap().trim();

            let stock_locate_code = match stock_directory_manager.stock_to_locate(*stock) {
                Some(stock_locate_code) => stock_locate_code,
                None => {
                    // Render a stock not found popup
                    let popup = ratatui::widgets::Paragraph::new("Stock not found")
                        .block(
                            ratatui::widgets::Block::default()
                                .title("Error")
                                .borders(ratatui::widgets::Borders::ALL),
                        )
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
                        .alignment(ratatui::layout::Alignment::Center);

                    let area = ratatui::layout::Rect::new(0, 0, size.width, size.height);
                    frame.render_widget(ratatui::widgets::Clear, area); // Clear the area first
                    frame.render_widget(popup, area);

                    return; // Exit the draw closure early
                }
            };

            let num_bins = 25;
            let price_quantities = match order_manager.order_books.get(&stock_locate_code) {
                Some(book) => book
                    .bid_book
                    .iter()
                    .map(|(price, bucket)| (price.0.to_f64(), bucket.share_quantity as u64))
                    .collect(),
                None => Vec::new(),
            };
            let binned_bid_book_data = bin_orderbook_data(
                &price_quantities,
                num_bins,
                BinningStrategy::Uniform, // or BinningStrategy::Logarithmic
            );

            // let ask_book_data = match order_manager.order_books.get(&stock_locate_code) {
            //     Some(book) => book
            //         .ask_book
            //         .iter()
            //         .map(|(price, bucket)| (price.to_string(), bucket.share_quantity as u64))
            //         .collect(),
            //     None => Vec::new(),
            // };

            let bid_barchart = BarChart::default()
                .block(
                    ratatui::widgets::Block::default()
                        .title(format!("Bid Book (Stock Locate: {})", stock_str))
                        .borders(ratatui::widgets::Borders::ALL),
                )
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
                .data(
                    &binned_bid_book_data
                        .iter()
                        .map(|(s, u)| (s.as_str(), *u))
                        .collect::<Vec<_>>(),
                )
                .bar_width(1)
                .bar_gap(0)
                .direction(ratatui::layout::Direction::Horizontal)
                .bar_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
                .value_style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Black)
                        .bg(ratatui::style::Color::Green),
                );

            frame.render_widget(bid_barchart, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            // If an event is available, read it
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

use std::collections::HashMap;

/// Binning strategy for order book visualization.
enum BinningStrategy {
    Uniform,
    Logarithmic,
}

/// Bin the order book data based on the given strategy.
///
/// # Arguments
/// * `price_quantities` - A slice of tuples containing the price and quantity.
/// * `num_bins` - The number of bins to use.
/// * `binning_strategy` - The binning strategy to use.
///
/// # Returns
/// A vector of tuples containing the bin label and the quantity.
fn bin_orderbook_data(
    price_quantities: &[(f64, u64)], // (price, quantity)
    num_bins: usize,
    binning_strategy: BinningStrategy,
) -> Vec<(String, u64)> {
    if price_quantities.is_empty() || num_bins == 0 {
        return Vec::new();
    }

    // Extract min and max prices
    let min_price = price_quantities
        .iter()
        .map(|(price, _)| *price)
        .fold(f64::INFINITY, f64::min);
    let max_price = price_quantities
        .iter()
        .map(|(price, _)| *price)
        .fold(f64::NEG_INFINITY, f64::max);

    if min_price == max_price {
        // All prices are the same; place everything in one bin
        let total_quantity: u64 = price_quantities.iter().map(|(_, qty)| *qty).sum();
        return vec![(format!("{:.2}", min_price), total_quantity)];
    }

    // Generate bin edges based on the chosen strategy
    let bin_edges: Vec<f64> = match binning_strategy {
        BinningStrategy::Uniform => {
            let bin_width = (max_price - min_price) / num_bins as f64;
            (0..=num_bins)
                .map(|i| min_price + i as f64 * bin_width)
                .collect()
        }
        BinningStrategy::Logarithmic => {
            // Ensure prices are positive for logarithmic binning
            let min_price = if min_price <= 0.0 {
                f64::MIN_POSITIVE
            } else {
                min_price
            };
            let log_min = min_price.ln();
            let log_max = max_price.ln();
            let bin_width = (log_max - log_min) / num_bins as f64;
            (0..=num_bins)
                .map(|i| (log_min + i as f64 * bin_width).exp())
                .collect()
        }
    };

    // Initialize bins
    let mut bins: HashMap<usize, u64> = HashMap::new();

    // Assign each price to a bin
    for (price, quantity) in price_quantities {
        // Find the appropriate bin index
        let bin_index = match binning_strategy {
            BinningStrategy::Uniform => {
                let relative_position = (price - min_price) / (max_price - min_price);
                let index = (relative_position * num_bins as f64).floor() as usize;
                if index >= num_bins {
                    num_bins - 1
                } else {
                    index
                }
            }
            BinningStrategy::Logarithmic => {
                let price = if *price <= 0.0 {
                    f64::MIN_POSITIVE
                } else {
                    *price
                };
                let log_price = price.ln();
                let relative_position = (log_price - bin_edges[0].ln())
                    / (bin_edges[bin_edges.len() - 1].ln() - bin_edges[0].ln());
                let index = (relative_position * num_bins as f64).floor() as usize;
                if index >= num_bins {
                    num_bins - 1
                } else {
                    index
                }
            }
        };

        // Sum quantities per bin
        *bins.entry(bin_index).or_insert(0) += *quantity;
    }

    // Prepare the binned data for visualization
    let binned_data: Vec<(String, u64)> = bins
        .iter()
        .map(|(&bin_index, &quantity)| {
            let bin_label = match binning_strategy {
                BinningStrategy::Uniform => {
                    let bin_start = bin_edges[bin_index];
                    let bin_end = bin_edges[bin_index + 1];
                    format!("{:.2}-{:.2}", bin_start, bin_end)
                }
                BinningStrategy::Logarithmic => {
                    let bin_start = bin_edges[bin_index];
                    let bin_end = bin_edges[bin_index + 1];
                    format!("{:.2}-{:.2}", bin_start, bin_end)
                }
            };
            (bin_label, quantity)
        })
        .collect();

    binned_data
}
