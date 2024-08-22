use thiserror::Error;

pub type Stock = [u8; 8];
pub type MPID = [u8; 4];

impl GenerateBinaryExample<8> for Stock {
    fn generate_example_message() -> [u8; 8] {
        [b'T', b'E', b'S', b'T', b' ', b' ', b' ', b' ']
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
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
}

pub trait BinaryMessageLength {
    const LENGTH: usize;
}

pub trait Parse: Sized {
    fn parse(input: &[u8]) -> Result<Self, ParseError>;
}

pub trait GenerateBinaryExample<const N: usize> {
    fn generate_example_message() -> [u8; N];
}

pub trait EnumTestHelpers<const N: usize> {
    const VALID_CODES: [u8; N];
    fn generate_example_code() -> u8;
}
