use crate::enums::{
    BoolOrUnavailable, FinancialStatusIndicator, IssueClassificationCodes, LuldReferencePriceTier,
    MarketCategory, ShortSaleThresholdIndicator,
};
use crate::stockmessages::StockDirectory;
use crate::types::Stock;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct StockDirectoryManager {
    directory: Arc<RwLock<HashMap<u16, StockData>>>,
}

impl StockDirectoryManager {
    pub fn new() -> Self {
        Self {
            directory: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_stock(&self, stock: StockDirectory) {
        log::debug!(
            "Adding stock: {}",
            std::str::from_utf8(&stock.stock).unwrap()
        );
        let mut directory = self.directory.write().unwrap(); // Lock for writing
        directory
            .entry(stock.header.stock_locate)
            .or_insert(StockData::new(stock));
    }

    pub fn get_stock_data(&self, stock_locate: u16) -> Option<StockData> {
        let directory = self.directory.read().unwrap(); // Lock for reading
        let stock_data = directory.get(&stock_locate)?; // Return a cloned version of StockData
        Some(stock_data.clone())
    }

    pub fn get_stock(&self, stock_locate: u16) -> Option<Stock> {
        let directory = self.directory.read().unwrap(); // Lock for reading
        let stock_data = directory.get(&stock_locate)?;
        Some(stock_data.stock.clone())
    }

    // Get the Arc clone: Allows sharing the manager across threads
    pub fn get_shared_manager(&self) -> Arc<RwLock<HashMap<u16, StockData>>> {
        Arc::clone(&self.directory)
    }
}

#[derive(Clone)]
pub struct StockData {
    pub stock: Stock,
    pub market_category: MarketCategory,
    pub financial_status: FinancialStatusIndicator,
    pub round_lot_size: u32,
    pub round_lots_only: bool,
    pub issue_classification: IssueClassificationCodes,
    pub issue_sub_type: u16,
    pub authenticity: char,
    pub short_sale_threshold_indicator: ShortSaleThresholdIndicator,
    pub ipo_flag: BoolOrUnavailable,
    pub luld_reference_price_tier: LuldReferencePriceTier,
    pub etp_flag: BoolOrUnavailable,
    pub etp_leverage_factor: u32,
    pub inverse_indicator: bool,
}

impl StockData {
    pub fn new(input: StockDirectory) -> Self {
        StockData {
            stock: input.stock,
            market_category: input.market_category,
            financial_status: input.financial_status_indicator,
            round_lot_size: input.round_lot_size,
            round_lots_only: input.round_lots_only,
            issue_classification: input.issue_classification,
            issue_sub_type: input.issue_sub_type,
            authenticity: input.authenticity,
            short_sale_threshold_indicator: input.short_sale_threshold_indicator,
            ipo_flag: input.ipo_flag,
            luld_reference_price_tier: input.luld_reference_price_tier,
            etp_flag: input.etp_flag,
            etp_leverage_factor: input.etp_leverage_factor,
            inverse_indicator: input.inverse_indicator,
        }
    }
}

impl fmt::Debug for StockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stock_str = std::str::from_utf8(&self.stock).unwrap_or("Invalid UTF-8");
        f.debug_struct("StockData")
            .field("stock", &stock_str)
            .field("market_category", &self.market_category)
            .field("financial_status", &self.financial_status)
            .field("round_lot_size", &self.round_lot_size)
            .field("round_lots_only", &self.round_lots_only)
            .field("issue_classification", &self.issue_classification)
            .field("issue_sub_type", &self.issue_sub_type)
            .field("authenticity", &self.authenticity)
            .field(
                "short_sale_threshold_indicator",
                &self.short_sale_threshold_indicator,
            )
            .field("ipo_flag", &self.ipo_flag)
            .field("luld_reference_price_tier", &self.luld_reference_price_tier)
            .field("etp_flag", &self.etp_flag)
            .field("etp_leverage_factor", &self.etp_leverage_factor)
            .field("inverse_indicator", &self.inverse_indicator)
            .finish()
    }
}
