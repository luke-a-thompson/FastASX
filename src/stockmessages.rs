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
            market_category: MarketCategory::try_from(input[18])?,
            financial_status_indicator: FinancialStatusIndicator::try_from(input[19])?,
            round_lot_size: BigEndian::read_u32(&input[20..24]),
            round_lots_only: byte_to_bool(input[24]),
            issue_classification: IssueClassificationValues::try_from(input[25])?,
            issue_sub_type: BigEndian::read_u16(&input[26..28]),
            authenticity: input[28] as char,
            short_sale_threshold_indicator: ShortSaleThresholdIndicator::try_from(input[29])?,
            ipo_flag: input[30] as char,
            luld_reference_price_tier: input[31] as char,
            etp_flag: byte_to_bool_space(input[32]),
            etp_leverage_factor: BigEndian::read_u32(&input[33..37]),
            inverse_indicator: byte_to_bool(input[37]), // We only read up to index 37 (38 bytes), 1 less because of match. max spec offset-1
        })
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
            trading_state: TradingState::try_from(input[18])?,
            reserved: input[19],
            reason: TradingReasonCodes::try_from(&input[19..23])?,
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
            reg_sho_action: RegSHOAction::try_from(input[18])?,
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
            market_maker_mode: MarketMakerMode::try_from(input[23])?,
            market_participant_state: MarketParticipantState::try_from(input[24])?,
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
            breached_level: MWCBLevel::try_from(input[10])?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct IPOQuotingPeriodUpdate {
    header: MessageHeader,
    stock: u64,
    ipo_quotation_release_time: u32,
    ipo_quotation_release_qualifier: IPOReleaseQualifier,
    ipo_price: u32,
}

impl Parse for IPOQuotingPeriodUpdate {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != 27 {
            return Err(ParseError::IncompleteMessage { expected: 27 });
        }

        Ok(IPOQuotingPeriodUpdate {
            header: MessageHeader::parse(&input[..10]),
            stock: BigEndian::read_u64(&input[10..18]),
            ipo_quotation_release_time: BigEndian::read_u32(&input[18..22]),
            ipo_quotation_release_qualifier: IPOReleaseQualifier::try_from(input[22])?,
            ipo_price: BigEndian::read_u32(&input[23..27]),
        })
    }
}
