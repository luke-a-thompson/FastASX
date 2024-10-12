use std::fmt;

use thiserror::Error;

pub type Stock = [u8; 8];

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<8> for Stock {
    fn generate_binary_example() -> [u8; 8] {
        [b'T', b'E', b'S', b'T', b' ', b' ', b' ', b' ']
    }
}

pub type MPID = [u8; 4];

/// `Price4` uses `u32` for value and has a fixed precision of 4 decimal places.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price4 {
    pub value: u32, // Price value in fixed-point format
}

pub trait PriceConversions<T> {
    fn new(value: T) -> Self;
    fn to_f64(&self) -> f64;
    fn to_f32(&self) -> f32;
    fn convert_to_string(&self) -> String;
}

impl PriceConversions<u32> for Price4 {
    /// Creates a new `Price4` with precision 4.
    fn new(value: u32) -> Self {
        Price4 { value }
    }

    /// Converts the fixed-point price to an f64.
    fn to_f64(&self) -> f64 {
        self.value as f64 / 10_000.0
    }

    /// Converts the fixed-point price to an f32.
    fn to_f32(&self) -> f32 {
        self.value as f32 / 10_000.0
    }

    /// Converts the price to a formatted string with 4 decimal places.
    fn convert_to_string(&self) -> String {
        let integer_part = self.value / 10_000;
        let fractional_part = self.value % 10_000;
        format!("{}.{}", integer_part, format!("{:04}", fractional_part))
    }
}

/// `Price8` uses `u64` for value and has a fixed precision of 8 decimal places.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price8 {
    value: u64, // Price value in fixed-point format
}

impl PriceConversions<u64> for Price8 {
    /// Creates a new `Price8` with precision 8.
    fn new(value: u64) -> Self {
        Price8 { value }
    }

    /// Converts the fixed-point price to an f64.
    fn to_f64(&self) -> f64 {
        self.value as f64 / 100_000_000.0
    }

    /// Converts the fixed-point price to an f32.
    fn to_f32(&self) -> f32 {
        self.value as f32 / 100_000_000.0
    }

    /// Converts the price to a formatted string with 8 decimal places.
    fn convert_to_string(&self) -> String {
        let integer_part = self.value / 100_000_000;
        let fractional_part = self.value % 100_000_000;
        format!("{}.{}", integer_part, format!("{:08}", fractional_part))
    }
}

// Implement Display for easier printing for both types

impl fmt::Display for Price4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let price_str = self.convert_to_string();
        write!(f, "{}", price_str)
    }
}

impl fmt::Display for Price8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let price_str = self.convert_to_string();
        write!(f, "{}", price_str)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid precision for price: {precision}")]
    InvalidPrecision { precision: u32 },

    #[error("Invalid byte for boolean expression: {invalid_byte}")]
    InvalidBooleanByte { invalid_byte: u8 },

    #[error("Failed to parse message, slice may be incomplete. Expected {expected} bytes.")]
    IncompleteMessage { expected: usize },

    //System Messages
    #[error("Invalid SystemEventCode encountered: {invalid_byte}")]
    InvalidSystemEventCode { invalid_byte: u8 },

    // Trade Messages
    #[error("Invalid CrossType encountered: {invalid_byte}")]
    InvalidCrossType { invalid_byte: u8 },

    // Stock Messages
    #[error("Invalid MarketCategory encountered: {invalid_byte}")]
    InvalidMarketCategory { invalid_byte: u8 },

    #[error("Invalid FinancialStatusIndicator encountered: {invalid_byte}")]
    InvalidFinancialStatusIndicator { invalid_byte: u8 },

    #[error("Invalid IssueClassification encountered: {invalid_byte}")]
    InvalidIssueClassificationCode { invalid_byte: u8 },

    #[error("Invalid Authenticity encountered: {invalid_byte}")]
    InvalidAuthenticity { invalid_byte: u8 },

    #[error("Invalid ShortSaleThresholdIndicator encountered: {invalid_byte}")]
    InvalidShortSaleThresholdIndicator { invalid_byte: u8 },

    #[error("Invalid LuldReferencePriceTier encountered: {invalid_byte}")]
    InvalidLuldReferencePriceTier { invalid_byte: u8 },

    #[error("Invalid TradingState encountered: {invalid_byte}")]
    InvalidTradingState { invalid_byte: u8 },

    #[error("Invalid TradingReason encountered")]
    InvalidTradingReasonCode,

    #[error("Invalid RegSHOAction encountered: {invalid_byte}")]
    InvalidRegSHOAction { invalid_byte: u8 },

    #[error("Invalid MarketMakerMode encountered: {invalid_byte}")]
    InvalidMarketMakerMode { invalid_byte: u8 },

    #[error("Invalid MarketParticipantState encountered: {invalid_byte}")]
    InvalidMarketParticipantState { invalid_byte: u8 },

    #[error("Invalid MWCBLevel encountered: {invalid_byte}")]
    InvalidMWCBLevel { invalid_byte: u8 },

    #[error("Invalid IPOReleaseQualifier encountered: {invalid_byte}")]
    InvalidIPOReleaseQualifier { invalid_byte: u8 },

    // Add Order Messages
    #[error("Invalid BuySellType encountered: {invalid_byte}")]
    InvalidBuySellIndicator { invalid_byte: u8 },

    // Noii Messages
    #[error("Invalid ImbalanceDirection encountered: {invalid_byte}")]
    InvalidImbalanceDirection { invalid_byte: u8 },

    #[error("Invalid PriceVariationIndicator encountered: {invalid_byte}")]
    InvalidPriceVariationIndicator { invalid_byte: u8 },
}

#[derive(Debug, Error)]
pub enum OrderBookError {
    #[error("Attempted to add duplicate order")]
    DuplicateOrder,

    #[error("Attempted to remove non-existent order")]
    NonExistentOrder,

    #[error("Attempted to cancel more shares than available")]
    InvalidCancellation,
}

pub trait BinaryMessageLength {
    const LENGTH: usize;
}

pub trait MessageHeaderType {
    const MESSAGE_TYPE: u8;
}

//  The below are for more detailed versions of the AddOrder message (with price)
pub trait AltBinaryMessageLength {
    const ALT_LENGTH: usize;
}

pub trait AltMessageHeaderType {
    const ALT_MESSAGE_TYPE: u8;
}

pub trait Parse: Sized {
    fn parse(input: &[u8]) -> Result<Self, ParseError>;
}

pub trait GenerateExampleMessage<const N: usize> {
    fn generate_binary_example() -> [u8; N];
}

pub trait EnumTestHelpers<const N: usize> {
    const VALID_CODES: [u8; N];
    fn generate_example_code() -> u8;
}
