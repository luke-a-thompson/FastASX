use std::error::Error;
use std::fmt;

pub type Stock = [u8; 8];
pub type MPID = [u8; 4];

#[derive(Debug)]
pub enum ParseError {
    IncompleteMessage { expected: usize },
    InvalidBuySellType { invalid_byte: u8 },
    InvalidCrossType { invalid_byte: u8 },

    //System Messages
    InvalidSystemEventCode { invalid_byte: u8 },

    // Stock Messages
    InvalidMarketCategory { invalid_byte: u8 },
    InvalidFinancialStatusIndicator { invalid_byte: u8 },
    InvalidShortSaleThresholdIndicator { invalid_byte: u8 },
    InvalidIssueClassificationCode { invalid_byte: u8 },
    InvalidTradingReasonCode,
    InvalidTradingState { invalid_byte: u8 },
    InvalidRegSHOAction { invalid_byte: u8 },
    InvalidMarketMakerMode { invalid_byte: u8 },
    InvalidMarketParticipantState { invalid_byte: u8 },
    InvalidMWCBStatus { invalid_byte: u8 },
    InvalidMWCBLevel { invalid_byte: u8 },
    InvalidIPOQuotationReleaseQualifier { invalid_byte: u8 },
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseError::IncompleteMessage { expected } => write!(
                f,
                "Failed to parse message, slice may be incomplete. Expected {} bytes.",
                expected
            ),
            ParseError::InvalidSystemEventCode { invalid_byte } => {
                write!(
                    f,
                    "Invalid SystemEventCode encountered: {}.",
                    invalid_byte as char
                )
            }

            ParseError::InvalidBuySellType { invalid_byte } => {
                write!(
                    f,
                    "Invalid BuySellType encountered: {}.",
                    invalid_byte as char
                )
            }
            ParseError::InvalidCrossType { invalid_byte } => {
                write!(
                    f,
                    "Invalid CrossType encountered: {}.",
                    invalid_byte as char
                )
            }

            // Stock Messages
            ParseError::InvalidMarketCategory { invalid_byte } => {
                write!(f, "Invalid MarketCategory encountered: {}.", invalid_byte)
            }
            ParseError::InvalidFinancialStatusIndicator { invalid_byte } => {
                write!(
                    f,
                    "Invalid FinancialStatusIndicator encountered: {}.",
                    invalid_byte
                )
            }
            ParseError::InvalidShortSaleThresholdIndicator { invalid_byte } => {
                write!(
                    f,
                    "Invalid ShortSaleThresholdIndicator encountered: {}.",
                    invalid_byte
                )
            }
            ParseError::InvalidIssueClassificationCode { invalid_byte } => {
                write!(
                    f,
                    "Invalid IssueClassificationCode encountered: {}.",
                    invalid_byte
                )
            }
            ParseError::InvalidTradingReasonCode => {
                write!(f, "Invalid TradingReasonCode encountered.")
            }
            ParseError::InvalidTradingState { invalid_byte } => {
                write!(f, "Invalid TradingState encountered: {}.", invalid_byte)
            }
            ParseError::InvalidRegSHOAction { invalid_byte } => {
                write!(f, "Invalid RegSHOAction encountered: {}.", invalid_byte)
            }
            ParseError::InvalidMarketMakerMode { invalid_byte } => {
                write!(f, "Invalid MarketMakerMode encountered: {}.", invalid_byte)
            }
            ParseError::InvalidMarketParticipantState { invalid_byte } => {
                write!(
                    f,
                    "Invalid MarketParticipantState encountered: {}.",
                    invalid_byte
                )
            }
            ParseError::InvalidMWCBStatus { invalid_byte } => {
                write!(f, "Invalid MWCBStatus encountered: {}.", invalid_byte)
            }
            ParseError::InvalidMWCBLevel { invalid_byte } => {
                write!(f, "Invalid MWCBLevel encountered: {}.", invalid_byte)
            }
            ParseError::InvalidIPOQuotationReleaseQualifier { invalid_byte } => {
                write!(
                    f,
                    "Invalid IPOQuotationReleaseQualifier encountered: {}.",
                    invalid_byte
                )
            }
        }
    }
}

pub trait Parse: Sized {
    fn parse(input: &[u8]) -> Result<Self, ParseError>;
}
