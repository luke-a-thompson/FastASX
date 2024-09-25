use crate::types::ParseError;
use crate::{
    addordermessages, modifyordermessages, noiimessages, stockmessages, systemmessages,
    trademessages,
};

#[cfg(any(test, feature = "bench"))]
use crate::types::EnumTestHelpers;

#[derive(Debug, PartialEq)]
pub enum BoolOrUnavailable {
    Bool(bool),
    Str(&'static str),
}

#[derive(Debug, PartialEq)]
pub enum MessageTypes {
    SystemEvent(systemmessages::SystemEventMessage),
    StockDirectory(stockmessages::StockDirectory),
    StockTradingAction(stockmessages::StockTradingAction),
    RegSHO(stockmessages::RegSHOShortSalePriceTestRestriction),
    MarketParticipantPosition(stockmessages::MarketParticipantPosition),
    MWCBDeclineLevel(stockmessages::MWCBDeclineLevel),
    MWCBStatus(stockmessages::MWCBStatus),
    IPOQuotingPeriodUpdate(stockmessages::IPOQuotingPeriodUpdate),
    AddOrder(addordermessages::AddOrder),
    // AddOrderMPID(addordermessages::AddOrderMPID),
    OrderExecuted(modifyordermessages::OrderExecuted),
    OrderExecutedWithPrice(modifyordermessages::OrderExecutedWithPrice),
    OrderCancel(modifyordermessages::OrderCancel),
    OrderDelete(modifyordermessages::OrderDelete),
    OrderReplace(modifyordermessages::OrderReplace),
    NonCrossingTrade(trademessages::NonCrossingTrade),
    CrossingTrade(trademessages::CrossingTrade),
    BrokenTrade(trademessages::BrokenTrade),
    NOII(noiimessages::NetOrderImbalanceIndicator),
}

#[derive(Debug, PartialEq)]
pub enum SystemEventCode {
    StartOfMessages,
    StartOfSystemHours,
    StartOfMarketHours,
    EndOfMarketHours,
    EndOfSystemHours,
    EndOfMessages,
}

impl TryFrom<u8> for SystemEventCode {
    type Error = ParseError; // Set the correct error type

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'O' => Ok(SystemEventCode::StartOfMessages),
            b'S' => Ok(SystemEventCode::StartOfSystemHours),
            b'Q' => Ok(SystemEventCode::StartOfMarketHours),
            b'M' => Ok(SystemEventCode::EndOfMarketHours),
            b'E' => Ok(SystemEventCode::EndOfSystemHours),
            b'C' => Ok(SystemEventCode::EndOfMessages),
            _ => Err(ParseError::InvalidSystemEventCode {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<6> for SystemEventCode {
    const VALID_CODES: [u8; 6] = [b'O', b'S', b'Q', b'M', b'E', b'C'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum MarketCategory {
    NASDAQGlobalSelectMarket,
    NASDAQGlobalMarket,
    NASDAQCapitalMarket,
    NYSE,
    NYSEMKT,
    NYSEArca,
    InvestorsExchange,
    BATS,
    Unavailable,
}

impl TryFrom<u8> for MarketCategory {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'Q' => Ok(MarketCategory::NASDAQGlobalSelectMarket),
            b'G' => Ok(MarketCategory::NASDAQGlobalMarket),
            b'S' => Ok(MarketCategory::NASDAQCapitalMarket),
            b'N' => Ok(MarketCategory::NYSE),
            b'A' => Ok(MarketCategory::NYSEArca),
            b'P' => Ok(MarketCategory::NYSEMKT),
            b'V' => Ok(MarketCategory::InvestorsExchange),
            b'Z' => Ok(MarketCategory::BATS),
            b' ' => Ok(MarketCategory::Unavailable),
            _ => Err(ParseError::InvalidMarketCategory {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<8> for MarketCategory {
    const VALID_CODES: [u8; 8] = [b'Q', b'G', b'S', b'N', b'A', b'P', b'Z', b' '];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum Authenticity {
    Production,
    Test,
}

impl TryFrom<u8> for Authenticity {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'P' => Ok(Authenticity::Production),
            b'T' => Ok(Authenticity::Test),
            _ => Err(ParseError::InvalidAuthenticity {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<2> for Authenticity {
    const VALID_CODES: [u8; 2] = [b'P', b'T'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum ShortSaleThresholdIndicator {
    Restricted,
    NotRestricted,
    NotAvailable,
}

impl TryFrom<u8> for ShortSaleThresholdIndicator {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'Y' => Ok(ShortSaleThresholdIndicator::Restricted),
            b'N' => Ok(ShortSaleThresholdIndicator::NotRestricted),
            b' ' => Ok(ShortSaleThresholdIndicator::NotAvailable),
            _ => Err(ParseError::InvalidShortSaleThresholdIndicator {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<3> for ShortSaleThresholdIndicator {
    const VALID_CODES: [u8; 3] = [b'Y', b'N', b' '];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum FinancialStatusIndicator {
    Deficient,
    Delinquent,
    Bankrupt,
    Suspended,
    DeficientAndBankrupt,
    DeficientAndDelinquent,
    DelinquentAndBankrupt,
    DeficientDelinquentAndBankrupt,
    CreationsAndRedemptionsSuspended,
    Normal,
    NotAvailable,
}

impl TryFrom<u8> for FinancialStatusIndicator {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'D' => Ok(FinancialStatusIndicator::Deficient),
            b'E' => Ok(FinancialStatusIndicator::Delinquent),
            b'Q' => Ok(FinancialStatusIndicator::Bankrupt),
            b'S' => Ok(FinancialStatusIndicator::Suspended),
            b'G' => Ok(FinancialStatusIndicator::DeficientAndBankrupt),
            b'H' => Ok(FinancialStatusIndicator::DeficientAndDelinquent),
            b'J' => Ok(FinancialStatusIndicator::DelinquentAndBankrupt),
            b'K' => Ok(FinancialStatusIndicator::DeficientDelinquentAndBankrupt),
            b'C' => Ok(FinancialStatusIndicator::CreationsAndRedemptionsSuspended),
            b'N' => Ok(FinancialStatusIndicator::Normal),
            b' ' => Ok(FinancialStatusIndicator::NotAvailable),
            _ => Err(ParseError::InvalidFinancialStatusIndicator {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<11> for FinancialStatusIndicator {
    const VALID_CODES: [u8; 11] = [
        b'D', b'E', b'Q', b'S', b'G', b'H', b'J', b'K', b'C', b'N', b' ',
    ];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum TradingState {
    Halted,
    Paused,
    QuotationOnly,
    Trading,
}

impl TryFrom<u8> for TradingState {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'H' => Ok(TradingState::Halted),
            b'P' => Ok(TradingState::Paused),
            b'Q' => Ok(TradingState::QuotationOnly),
            b'T' => Ok(TradingState::Trading),
            _ => Err(ParseError::InvalidTradingState {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<4> for TradingState {
    const VALID_CODES: [u8; 4] = [b'H', b'P', b'Q', b'T'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum RegSHOAction {
    NoPriceTestInEffect,
    RegSHOShortSalePriceTestRestriction,
    TestRestrictionRemains,
}

impl TryFrom<u8> for RegSHOAction {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(RegSHOAction::NoPriceTestInEffect),
            b'1' => Ok(RegSHOAction::RegSHOShortSalePriceTestRestriction),
            b'2' => Ok(RegSHOAction::TestRestrictionRemains),
            _ => Err(ParseError::InvalidRegSHOAction {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<3> for RegSHOAction {
    const VALID_CODES: [u8; 3] = [b'0', b'1', b'2'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum TradingReasonCodes {
    Halt(TradingHaltReasonCodes),
    Resumption(TradingResumptionReasonCodes),
}

impl TryFrom<&[u8]> for TradingReasonCodes {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"T1" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::HaltNewsPending,
            )),
            b"T2" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::HaltNewsDisseminated,
            )),
            b"T5" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::SingleSecurityTradingPause,
            )),
            b"T6" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::ExtraordinaryMarketActivity,
            )),
            b"T8" => Ok(TradingReasonCodes::Halt(TradingHaltReasonCodes::ETFHalt)),
            b"T12" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::InformationRequested,
            )),
            b"H4" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::NonCompliance,
            )),
            b"H9" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::NonCurrentFilings,
            )),
            b"H10" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::SECTradingSuspension,
            )),
            b"H11" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::RegulatoryConcern,
            )),
            b"O1" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::OperationsHalt,
            )),
            b"LUDP" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::VolatilityPause,
            )),
            b"LUDS" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::VolatilityPauseStraddle,
            )),
            b"MWC1" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::Level1CircuitBreaker,
            )),
            b"MWC2" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::Level2CircuitBreaker,
            )),
            b"MWC3" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::Level3CircuitBreaker,
            )),
            b"MWC0" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::CarryOverCircuitBreaker,
            )),
            b"IPO1" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::IPONotYetTrading,
            )),
            b"M1" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::CorporateAction,
            )),
            b"M2" => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::QuotationUnavailable,
            )),
            b"T3" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::NewsAndResumptionTime,
            )),
            b"T7" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::SingleSecurityPauseOrQuoteOnlyPeriod,
            )),
            b"R4" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::QualificationIssuesResolved,
            )),
            b"R9" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::FilingIssuesResolved,
            )),
            b"C3" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::IssuerNewNotForthcoming,
            )),
            b"C4" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::QualificationsHaltEndedMaintenanceMet,
            )),
            b"C9" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::QualificationsHaltConcludedFilingsMet,
            )),
            b"C11" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::TradeHaltConcluded,
            )),
            b"MWCQ" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::CircuitBreakerResumption,
            )),
            b"R1" | b"R2" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::IssueAvailable,
            )),
            b"IPOQ" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::IPOSecurityQuotationReleased,
            )),
            b"IPOE" => Ok(TradingReasonCodes::Resumption(
                TradingResumptionReasonCodes::IPOPositioningWindowExtension,
            )),
            b"    " => Ok(TradingReasonCodes::Halt(
                TradingHaltReasonCodes::NotAvailable,
            )),
            _ => return Err(ParseError::InvalidTradingReasonCode),
        }
    }
}

// TODO: Implement tests for TradingReasonCodes

#[derive(Debug, PartialEq)]
pub enum TradingHaltReasonCodes {
    HaltNewsPending,
    HaltNewsDisseminated,
    SingleSecurityTradingPause,
    ExtraordinaryMarketActivity,
    ETFHalt,
    InformationRequested, // maybe give reason not available
    NonCompliance,
    NonCurrentFilings,
    SECTradingSuspension,
    RegulatoryConcern,
    OperationsHalt, // contact operators
    VolatilityPause,
    VolatilityPauseStraddle,
    Level1CircuitBreaker, // Note they're mkt-wide?
    Level2CircuitBreaker,
    Level3CircuitBreaker,
    CarryOverCircuitBreaker,
    IPONotYetTrading,
    CorporateAction,
    QuotationUnavailable,
    NotAvailable,
}

#[derive(Debug, PartialEq)]
pub enum TradingResumptionReasonCodes {
    NewsAndResumptionTime,
    SingleSecurityPauseOrQuoteOnlyPeriod,
    QualificationIssuesResolved,
    FilingIssuesResolved,
    IssuerNewNotForthcoming,
    QualificationsHaltEndedMaintenanceMet,
    QualificationsHaltConcludedFilingsMet,
    TradeHaltConcluded,
    CircuitBreakerResumption,
    IssueAvailable,
    IPOSecurityQuotationReleased,
    IPOPositioningWindowExtension,
    NotAvailable,
}

#[derive(Debug, PartialEq)]
pub enum IssueClassificationCodes {
    AmericanDepositaryShare,
    Bond,
    CommonStock,
    DepositoryReceipt,
    UnregisteredSecurity,
    LimitedPartnership,
    Notes,
    OrdinaryShare,
    PreferredStock,
    OtherSecurity,
    Right,
    ShareOfBeneficialInterest,
    ConvertibleDebenture,
    Unit,
    UnitBenifInt,
    Warrant,
}

impl TryFrom<u8> for IssueClassificationCodes {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(IssueClassificationCodes::AmericanDepositaryShare),
            b'B' => Ok(IssueClassificationCodes::Bond),
            b'C' => Ok(IssueClassificationCodes::CommonStock),
            b'F' => Ok(IssueClassificationCodes::DepositoryReceipt),
            b'I' => Ok(IssueClassificationCodes::UnregisteredSecurity), // SEC Rule 144a
            b'L' => Ok(IssueClassificationCodes::LimitedPartnership),
            b'N' => Ok(IssueClassificationCodes::Notes),
            b'O' => Ok(IssueClassificationCodes::OrdinaryShare),
            b'P' => Ok(IssueClassificationCodes::PreferredStock),
            b'Q' => Ok(IssueClassificationCodes::OtherSecurity),
            b'R' => Ok(IssueClassificationCodes::Right),
            b'S' => Ok(IssueClassificationCodes::ShareOfBeneficialInterest),
            b'T' => Ok(IssueClassificationCodes::ConvertibleDebenture),
            b'U' => Ok(IssueClassificationCodes::Unit),
            b'V' => Ok(IssueClassificationCodes::UnitBenifInt),
            b'W' => Ok(IssueClassificationCodes::Warrant),
            _ => Err(ParseError::InvalidIssueClassificationCode {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<16> for IssueClassificationCodes {
    const VALID_CODES: [u8; 16] = [
        b'A', b'B', b'C', b'F', b'I', b'L', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
        b'W',
    ];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum MarketMakerMode {
    Normal,
    Passive,
    Syndicate,
    PreSyndicate,
    Penalty,
}

impl TryFrom<u8> for MarketMakerMode {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'N' => Ok(MarketMakerMode::Normal),
            b'P' => Ok(MarketMakerMode::Passive),
            b'S' => Ok(MarketMakerMode::Syndicate),
            b'R' => Ok(MarketMakerMode::PreSyndicate),
            b'L' => Ok(MarketMakerMode::Penalty),
            _ => Err(ParseError::InvalidMarketMakerMode {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<5> for MarketMakerMode {
    const VALID_CODES: [u8; 5] = [b'N', b'P', b'S', b'R', b'L'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum MarketParticipantState {
    Active,
    ExcusedWithdrawn,
    Withdrawn,
    Suspended,
    Deleted,
}

impl TryFrom<u8> for MarketParticipantState {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(MarketParticipantState::Active),
            b'E' => Ok(MarketParticipantState::ExcusedWithdrawn),
            b'W' => Ok(MarketParticipantState::Withdrawn),
            b'S' => Ok(MarketParticipantState::Suspended),
            b'D' => Ok(MarketParticipantState::Deleted),
            _ => Err(ParseError::InvalidMarketParticipantState {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<5> for MarketParticipantState {
    const VALID_CODES: [u8; 5] = [b'A', b'E', b'W', b'S', b'D'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum MWCBLevel {
    Level1,
    Level2,
    Level3,
}

impl TryFrom<u8> for MWCBLevel {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'1' => Ok(MWCBLevel::Level1),
            b'2' => Ok(MWCBLevel::Level2),
            b'3' => Ok(MWCBLevel::Level3),
            _ => Err(ParseError::InvalidMWCBLevel {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<3> for MWCBLevel {
    const VALID_CODES: [u8; 3] = [b'1', b'2', b'3'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum IPOReleaseQualifier {
    Anticipated,
    Postponed,
}

impl TryFrom<u8> for IPOReleaseQualifier {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(IPOReleaseQualifier::Anticipated),
            b'C' => Ok(IPOReleaseQualifier::Postponed),
            _ => Err(ParseError::InvalidIPOReleaseQualifier {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<2> for IPOReleaseQualifier {
    const VALID_CODES: [u8; 2] = [b'A', b'C'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum BuySellIndicator {
    Sell,
    Buy,
}

impl TryFrom<u8> for BuySellIndicator {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'S' => Ok(BuySellIndicator::Sell),
            b'B' => Ok(BuySellIndicator::Buy),
            _ => Err(ParseError::InvalidBuySellIndicator {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<2> for BuySellIndicator {
    const VALID_CODES: [u8; 2] = [b'S', b'B'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum CrossType {
    OpeningCross,
    ClosingCross,
    IPOCrossOrHaltedSecurity,
    IntradayOrPostCloseCross,
}

impl TryFrom<u8> for CrossType {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'O' => Ok(CrossType::OpeningCross),
            b'C' => Ok(CrossType::ClosingCross),
            b'H' => Ok(CrossType::IPOCrossOrHaltedSecurity),
            b'I' => Ok(CrossType::IntradayOrPostCloseCross), // Should be invalid for Noii
            _ => Err(ParseError::InvalidCrossType {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<4> for CrossType {
    const VALID_CODES: [u8; 4] = [b'O', b'C', b'H', b'I'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum ImbalanceDirection {
    BuyImbalance,
    SellImbalance,
    NoImbalance,
    InsufficnetOrdersToCalculate,
}

impl TryFrom<u8> for ImbalanceDirection {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'B' => Ok(ImbalanceDirection::BuyImbalance),
            b'S' => Ok(ImbalanceDirection::SellImbalance),
            b'N' => Ok(ImbalanceDirection::NoImbalance),
            b'O' => Ok(ImbalanceDirection::InsufficnetOrdersToCalculate),
            _ => Err(ParseError::InvalidImbalanceDirection {
                invalid_byte: value,
            }),
        }
    }
}

#[cfg(any(test, feature = "bench"))]
impl EnumTestHelpers<4> for ImbalanceDirection {
    const VALID_CODES: [u8; 4] = [b'B', b'S', b'N', b'O'];

    fn generate_example_code() -> u8 {
        let i = fastrand::usize(..Self::VALID_CODES.len());
        Self::VALID_CODES[i]
    }
}

#[derive(Debug, PartialEq)]
pub enum PriceVariationIndicator {}
