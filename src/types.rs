use thiserror::Error;

pub type Stock = [u8; 8];
pub type MPID = [u8; 4];

#[cfg(any(test, feature = "bench"))]
impl GenerateExampleMessage<8> for Stock {
    fn generate_binary_example() -> [u8; 8] {
        [b'T', b'E', b'S', b'T', b' ', b' ', b' ', b' ']
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
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
