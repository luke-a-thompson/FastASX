use crate::enums::{
    BoolOrUnavailable, FinancialStatusIndicator, IPOReleaseQualifier, IssueClassificationValues,
    MWCBLevel, MarketCategory, MarketMakerMode, MarketParticipantState, RegSHOAction,
    ShortSaleThresholdIndicator, TradingHaltReasonCodes, TradingReasonCodes,
    TradingResumptionReasonCodes, TradingState,
};
use crate::helpers::{byte_to_bool, byte_to_bool_space, u8s_to_ticker};
use crate::messageheader::MessageHeader;
use crate::types::{Parse, ParseError, Stock};
use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq)]
pub struct StockDirectory {
    header: MessageHeader,
    stock: String,
    market_category: MarketCategory,
    financial_status_indicator: FinancialStatusIndicator,
    round_lot_size: u32,
    round_lots_only: bool,
    issue_classification: IssueClassificationValues,
    issue_sub_type: u16,
    authenticity: char,
    short_sale_threshold_indicator: ShortSaleThresholdIndicator,
    ipo_flag: char,
    luld_reference_price_tier: char,
    etp_flag: BoolOrUnavailable,
    etp_leverage_factor: u32,
    inverse_indicator: bool,
}

impl Parse for StockDirectory {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 38 {
            panic!("Invalid input length for StockDirectory");
        }

        Ok(StockDirectory {
            header: MessageHeader::parse(&input[..10]),
            stock: u8s_to_ticker(&input[10..18]),
            market_category: {
                match input[18] {
                    b'Q' => MarketCategory::NASDAQGlobalSelectMarket,
                    b'G' => MarketCategory::NASDAQGlobalMarket,
                    b'S' => MarketCategory::NASDAQCapitalMarket,
                    b'N' => MarketCategory::NYSE,
                    b'A' => MarketCategory::NYSEMKT,
                    b'P' => MarketCategory::NYSEArca,
                    b'Z' => MarketCategory::BATS,
                    b' ' => MarketCategory::Unavailable,
                    _ => {
                        return Err(ParseError::InvalidMarketCategory {
                            invalid_byte: input[18],
                        })
                    }
                }
            },
            // market_category: input[18] as char,
            financial_status_indicator: {
                match input[19] {
                    b'D' => FinancialStatusIndicator::Deficient,
                    b'E' => FinancialStatusIndicator::Delinquent,
                    b'Q' => FinancialStatusIndicator::Bankrupt,
                    b'S' => FinancialStatusIndicator::Suspended,
                    b'G' => FinancialStatusIndicator::DeficientAndBankrupt,
                    b'H' => FinancialStatusIndicator::DeficientAndDelinquent,
                    b'J' => FinancialStatusIndicator::DelinquentAndBankrupt,
                    b'K' => FinancialStatusIndicator::DeficientDelinquentAndBankrupt,
                    b'C' => FinancialStatusIndicator::CreationsAndRedemptionsSuspended,
                    b'N' => FinancialStatusIndicator::Normal,
                    b' ' => FinancialStatusIndicator::NotAvailable,
                    _ => {
                        return Err(ParseError::InvalidFinancialStatusIndicator {
                            invalid_byte: input[19],
                        })
                    }
                }
            },
            round_lot_size: BigEndian::read_u32(&input[20..24]),
            round_lots_only: input[24] == b'Y',
            issue_classification: parse_issue_classification_code(&input[25])
                .expect("Invalid Issue Classification Code"),
            issue_sub_type: BigEndian::read_u16(&input[26..28]),
            authenticity: input[28] as char,
            short_sale_threshold_indicator: {
                match input[29] {
                    b'Y' => ShortSaleThresholdIndicator::Restricted,
                    b'N' => ShortSaleThresholdIndicator::NotRestricted,
                    b' ' => ShortSaleThresholdIndicator::NotAvailable,
                    _ => {
                        return Err(ParseError::InvalidShortSaleThresholdIndicator {
                            invalid_byte: input[29],
                        })
                    }
                }
            },
            ipo_flag: input[30] as char,
            luld_reference_price_tier: input[31] as char,
            etp_flag: byte_to_bool_space(input[32]),
            etp_leverage_factor: BigEndian::read_u32(&input[33..37]),
            inverse_indicator: byte_to_bool(input[37]), // We only read up to index 37 (38 bytes), 1 less because of match. max spec offset-1
        })
    }
}

#[inline]
fn parse_issue_classification_code(code: &u8) -> Result<IssueClassificationValues, ParseError> {
    match code {
        b'A' => Ok(IssueClassificationValues::AmericanDepositaryShare),
        b'B' => Ok(IssueClassificationValues::Bond),
        b'C' => Ok(IssueClassificationValues::CommonStock),
        b'F' => Ok(IssueClassificationValues::DepositoryReceipt),
        b'I' => Ok(IssueClassificationValues::UnregisteredSecurity), // SEC Rule 144a
        b'L' => Ok(IssueClassificationValues::LimitedPartnership),
        b'N' => Ok(IssueClassificationValues::Notes),
        b'O' => Ok(IssueClassificationValues::OrdinaryShare),
        b'P' => Ok(IssueClassificationValues::PreferredStock),
        b'Q' => Ok(IssueClassificationValues::OtherSecurity),
        b'R' => Ok(IssueClassificationValues::Right),
        b'S' => Ok(IssueClassificationValues::ShareOfBeneficialInterest),
        b'T' => Ok(IssueClassificationValues::ConvertibleDebenture),
        b'U' => Ok(IssueClassificationValues::Unit),
        b'V' => Ok(IssueClassificationValues::UnitBenifInt),
        b'W' => Ok(IssueClassificationValues::Warrant),
        _ => {
            return Err(ParseError::InvalidIssueClassificationCode {
                invalid_byte: *code,
            })
        }
    }
}

#[inline]
fn parse_trading_reason_code(code: &[u8]) -> Result<TradingReasonCodes, ParseError> {
    match code {
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

#[derive(Debug, PartialEq)]
pub struct StockTradingAction {
    header: MessageHeader,
    stock: u64,
    trading_state: TradingState,
    reserved: u8,
    reason: TradingReasonCodes,
}

impl Parse for StockTradingAction {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 24 {
            return Err(ParseError::IncompleteMessage { expected: 24 });
        }

        Ok(StockTradingAction {
            header: MessageHeader::parse(&input[..10]),
            stock: BigEndian::read_u64(&input[10..18]),
            trading_state: {
                match input[18] {
                    b'H' => TradingState::Halted,
                    b'P' => TradingState::Paused,
                    b'Q' => TradingState::QuotationOnly,
                    b'T' => TradingState::Trading,
                    _ => {
                        return Err(ParseError::InvalidTradingState {
                            invalid_byte: input[18],
                        })
                    }
                }
            },
            reserved: input[19],
            reason: parse_trading_reason_code(&input[19..23])?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct RegSHOShortSalePriceTestRestriction {
    header: MessageHeader,
    stock: Stock,
    reg_sho_action: RegSHOAction,
}

impl Parse for RegSHOShortSalePriceTestRestriction {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 19 {
            return Err(ParseError::IncompleteMessage { expected: 19 });
        }

        Ok(RegSHOShortSalePriceTestRestriction {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(), // We only read up to index 18, 1 less because of match. max spec offset-1
            reg_sho_action: {
                match input[18] {
                    b'0' => RegSHOAction::NoPriceTestInEffect,
                    b'1' => RegSHOAction::RegSHOShortSalePriceTestRestriction,
                    b'2' => RegSHOAction::TestRestrictionRemains,
                    _ => {
                        return Err(ParseError::InvalidRegSHOAction {
                            invalid_byte: input[18],
                        })
                    }
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MarketParticipantPosition {
    header: MessageHeader,
    mp_id: u32,
    stock: Stock,
    primary_market_maker: bool,
    market_maker_mode: MarketMakerMode,
    market_participant_state: MarketParticipantState,
}

impl Parse for MarketParticipantPosition {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 25 {
            return Err(ParseError::IncompleteMessage { expected: 25 });
        }

        Ok(MarketParticipantPosition {
            header: MessageHeader::parse(&input[..10]),
            mp_id: BigEndian::read_u32(&input[10..14]),
            stock: input[14..22].try_into().unwrap(),
            primary_market_maker: byte_to_bool(input[22]),
            market_maker_mode: {
                match input[23] {
                    b'N' => MarketMakerMode::Normal,
                    b'P' => MarketMakerMode::Passive,
                    b'S' => MarketMakerMode::Syndicate,
                    b'R' => MarketMakerMode::PreSyndicate,
                    b'L' => MarketMakerMode::Penalty,
                    _ => {
                        return Err(ParseError::InvalidMarketMakerMode {
                            invalid_byte: input[23],
                        })
                    }
                }
            },
            market_participant_state: {
                match input[24] {
                    b'A' => MarketParticipantState::Active,
                    b'E' => MarketParticipantState::ExcusedWithdrawn,
                    b'W' => MarketParticipantState::Withdrawn,
                    b'S' => MarketParticipantState::Suspended,
                    b'D' => MarketParticipantState::Deleted,
                    _ => {
                        return Err(ParseError::InvalidMarketMakerMode {
                            invalid_byte: input[24],
                        })
                    }
                }
            },
        })
    }
}

// Market-Wide Circuit Breaker (MWCB) Messaging
#[derive(Debug, PartialEq)]
pub struct MWCBDeclineLevel {
    header: MessageHeader,
    level1: u64,
    level2: u64,
    level3: u64,
}

impl Parse for MWCBDeclineLevel {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 35 {
            return Err(ParseError::IncompleteMessage { expected: 35 });
        }

        Ok(MWCBDeclineLevel {
            header: MessageHeader::parse(&input[..10]),
            level1: BigEndian::read_u64(&input[10..18]),
            level2: BigEndian::read_u64(&input[18..26]),
            level3: BigEndian::read_u64(&input[26..34]),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MWCBStatus {
    header: MessageHeader,
    breached_level: MWCBLevel,
}

impl Parse for MWCBStatus {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 11 {
            return Err(ParseError::IncompleteMessage { expected: 11 });
        }

        Ok(MWCBStatus {
            header: MessageHeader::parse(&input[..10]),
            breached_level: {
                match input[10] {
                    b'1' => MWCBLevel::Level1,
                    b'2' => MWCBLevel::Level2,
                    b'3' => MWCBLevel::Level3,
                    _ => {
                        return Err(ParseError::InvalidMWCBLevel {
                            invalid_byte: input[24],
                        })
                    }
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct IPOQuotingPeriodUpdate {
    header: MessageHeader,
    stock: u64,
    IPO_quotation_release_time: u32,
    IPO_quotation_release_qualifier: IPOReleaseQualifier,
    IPO_price: u32,
}

impl Parse for IPOQuotingPeriodUpdate {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 27 {
            return Err(ParseError::IncompleteMessage { expected: 27 });
        }

        Ok(IPOQuotingPeriodUpdate {
            header: MessageHeader::parse(&input[..10]),
            stock: BigEndian::read_u64(&input[10..18]),
            IPO_quotation_release_time: BigEndian::read_u32(&input[18..22]),
            IPO_quotation_release_qualifier: {
                match input[22] {
                    b'A' => IPOReleaseQualifier::Anticipated,
                    b'P' => IPOReleaseQualifier::Postponed,
                    _ => {
                        return Err(ParseError::InvalidIPOQuotationReleaseQualifier {
                            invalid_byte: input[22],
                        })
                    }
                }
            },
            IPO_price: BigEndian::read_u32(&input[23..27]),
        })
    }
}
