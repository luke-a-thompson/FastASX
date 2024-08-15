#[derive(Debug, PartialEq)]
pub enum BoolOrUnavailable {
    Bool(bool),
    Str(String),
}

use crate::{modifyordermessages, noiimessages, stockmessages, systemmessages, trademessages, addordermessages};

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
    AddOrderMPID(addordermessages::AddOrderMPID),
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

#[derive(Debug, PartialEq)]
pub enum MarketCategory {
    NASDAQGlobalSelectMarket,
    NASDAQGlobalMarket,
    NASDAQCapitalMarket,
    NYSE,
    NYSEMKT,
    NYSEArca,
    BATS,
    Unavailable,
}

#[derive(Debug, PartialEq)]
pub enum Authenticity {
    Production,
    Test,
}

#[derive(Debug, PartialEq)]
pub enum ShortSaleThresholdIndicator {
    Restricted,
    NotRestricted,
    NotAvailable,
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

#[derive(Debug, PartialEq)]
pub enum TradingState {
    Halted,
    Paused,
    QuotationOnly,
    Trading,
}

#[derive(Debug, PartialEq)]
pub enum RegSHOAction {
    NoPriceTestInEffect,
    RegSHOShortSalePriceTestRestriction,
    TestRestrictionRemains,
}

#[derive(Debug, PartialEq)]
pub enum TradingReasonCodes {
    Halt(TradingHaltReasonCodes),
    Resumption(TradingResumptionReasonCodes),
}

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
pub enum IssueClassificationValues {
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

#[derive(Debug, PartialEq)]
pub enum MarketMakerMode {
    Normal,
    Passive,
    Syndicate,
    PreSyndicate,
    Penalty,
}

#[derive(Debug, PartialEq)]
pub enum MarketParticipantState {
    Active,
    ExcusedWithdrawn,
    Withdrawn,
    Suspended,
    Deleted,
}

#[derive(Debug, PartialEq)]
pub enum MWCBLevel {
    Level1,
    Level2,
    Level3,
}

#[derive(Debug, PartialEq)]
pub enum IPOReleaseQualifier {
    Anticipated,
    Postponed,
}

#[derive(Debug, PartialEq)]
pub enum BuySellIndicator {
    Sell,
    Buy,
}

#[derive(Debug, PartialEq)]
pub enum CrossType {
    OpeningCross,
    ClosingCross,
    IPOCrossOrHaltedSecurity,
    IntradayOrPostCloseCross,
}

#[derive(Debug, PartialEq)]
pub enum ImbalanceDirection {
    BuyImbalance,
    SellImbalance,
    NoImbalance,
    InsufficnetOrdersToCalculate,
}
