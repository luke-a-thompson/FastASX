use crate::enums::{
    BoolOrUnavailable, FinancialStatusIndicator, IPOReleaseQualifier, IssueClassificationValues,
    MWCBLevel, MarketCategory, MarketMakerMode, MarketParticipantState, RegSHOAction,
    ShortSaleThresholdIndicator, TradingReasonCodes, TradingState,
};
use crate::helpers::{byte_to_bool, byte_to_bool_space};
use crate::messageheader::MessageHeader;
use crate::types::{BinaryMessageLength, Parse, ParseError, Stock};
use byteorder::{BigEndian, ByteOrder};

#[cfg(test)]
use crate::types::{EnumTestHelpers, GenerateBinaryExample};
#[cfg(test)]
use fastrand::Rng;

#[derive(Debug, PartialEq)]
pub struct StockDirectory {
    header: MessageHeader,
    stock: Stock,
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
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(StockDirectory {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(),
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

impl BinaryMessageLength for StockDirectory {
    const LENGTH: usize = 38;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for StockDirectory {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let stock = rng.u64(..).to_be_bytes();
        let market_category = MarketCategory::generate_example_code();
        let financial_status_indicator = FinancialStatusIndicator::generate_example_code();
        let round_lot_size = rng.u32(..).to_be_bytes();
        let round_lots_only = b'Y';
        let issue_classification = IssueClassificationValues::generate_example_code();
        let issue_sub_type = rng.u16(..).to_be_bytes();
        let authenticity = b'P';
        let short_sale_threshold_indicator = ShortSaleThresholdIndicator::generate_example_code();
        let ipo_flag = b'N';
        let luld_reference_price_tier = b' ';
        let etp_flag = b' ';
        let etp_leverage_factor = rng.u32(..).to_be_bytes();
        let inverse_indicator = b'N';

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&stock);
        message[18] = market_category;
        message[19] = financial_status_indicator;
        message[20..24].copy_from_slice(&round_lot_size);
        message[24] = round_lots_only;
        message[25] = issue_classification;
        message[26..28].copy_from_slice(&issue_sub_type);
        message[28] = authenticity as u8;
        message[29] = short_sale_threshold_indicator;
        message[30] = ipo_flag as u8;
        message[31] = luld_reference_price_tier as u8;
        message[32] = etp_flag as u8;
        message[33..37].copy_from_slice(&etp_leverage_factor);
        message[37] = inverse_indicator;

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct StockTradingAction {
    header: MessageHeader,
    stock: Stock,
    trading_state: TradingState,
    reserved: u8,
    reason: TradingReasonCodes,
}

impl Parse for StockTradingAction {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(StockTradingAction {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(),
            trading_state: TradingState::try_from(input[18])?,
            reserved: input[19],
            reason: TradingReasonCodes::try_from(&input[19..23])?,
        })
    }
}

impl BinaryMessageLength for StockTradingAction {
    const LENGTH: usize = 24;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for StockTradingAction {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let stock = rng.u64(..).to_be_bytes();
        let trading_state = TradingState::generate_example_code();
        let reserved = rng.u8(..);
        let reason = b"IPO1";

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&stock);
        message[18] = trading_state;
        message[19] = reserved;
        message[19..23].copy_from_slice(reason);

        message
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
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(RegSHOShortSalePriceTestRestriction {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(), // We only read up to index 18, 1 less because of match. max spec offset-1
            reg_sho_action: RegSHOAction::try_from(input[18])?,
        })
    }
}

impl BinaryMessageLength for RegSHOShortSalePriceTestRestriction {
    const LENGTH: usize = 19;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for RegSHOShortSalePriceTestRestriction {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let stock = rng.u64(..).to_be_bytes();
        let reg_sho_action = RegSHOAction::generate_example_code();

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&stock);
        message[18] = reg_sho_action;

        message
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
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
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

impl BinaryMessageLength for MarketParticipantPosition {
    const LENGTH: usize = 25;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for MarketParticipantPosition {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let mp_id = rng.u32(..).to_be_bytes();
        let stock = rng.u64(..).to_be_bytes();
        let primary_market_maker = b'Y';
        let market_maker_mode = MarketMakerMode::generate_example_code();
        let market_participant_state = MarketParticipantState::generate_example_code();

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..14].copy_from_slice(&mp_id);
        message[14..22].copy_from_slice(&stock);
        message[22] = primary_market_maker;
        message[23] = market_maker_mode;
        message[24] = market_participant_state;

        message
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
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(MWCBDeclineLevel {
            header: MessageHeader::parse(&input[..10]),
            level1: BigEndian::read_u64(&input[10..18]),
            level2: BigEndian::read_u64(&input[18..26]),
            level3: BigEndian::read_u64(&input[26..34]),
        })
    }
}

impl BinaryMessageLength for MWCBDeclineLevel {
    const LENGTH: usize = 35;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for MWCBDeclineLevel {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let level1 = rng.u64(..).to_be_bytes();
        let level2 = rng.u64(..).to_be_bytes();
        let level3 = rng.u64(..).to_be_bytes();

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&level1);
        message[18..26].copy_from_slice(&level2);
        message[26..34].copy_from_slice(&level3);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct MWCBStatus {
    header: MessageHeader,
    breached_level: MWCBLevel,
}

impl Parse for MWCBStatus {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(MWCBStatus {
            header: MessageHeader::parse(&input[..10]),
            breached_level: MWCBLevel::try_from(input[10])?,
        })
    }
}

impl BinaryMessageLength for MWCBStatus {
    const LENGTH: usize = 11;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for MWCBStatus {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let header = MessageHeader::generate_example_message();
        let breached_level = MWCBLevel::generate_example_code();

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10] = breached_level;

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct IPOQuotingPeriodUpdate {
    header: MessageHeader,
    stock: Stock,
    ipo_quotation_release_time: u32,
    ipo_quotation_release_qualifier: IPOReleaseQualifier,
    ipo_price: u32,
}

impl Parse for IPOQuotingPeriodUpdate {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(IPOQuotingPeriodUpdate {
            header: MessageHeader::parse(&input[..10]),
            stock: input[10..18].try_into().unwrap(),
            ipo_quotation_release_time: BigEndian::read_u32(&input[18..22]),
            ipo_quotation_release_qualifier: IPOReleaseQualifier::try_from(input[22])?,
            ipo_price: BigEndian::read_u32(&input[23..27]),
        })
    }
}

impl BinaryMessageLength for IPOQuotingPeriodUpdate {
    const LENGTH: usize = 27;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for IPOQuotingPeriodUpdate {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let stock = rng.u64(..).to_be_bytes();
        let ipo_quotation_release_time = rng.u32(..).to_be_bytes();
        let ipo_quotation_release_qualifier = IPOReleaseQualifier::generate_example_code();
        let ipo_price = rng.u32(..).to_be_bytes();

        // Concatenate the arrays into a final message
        let mut message = [0u8; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&stock);
        message[18..22].copy_from_slice(&ipo_quotation_release_time);
        message[22] = ipo_quotation_release_qualifier;
        message[23..27].copy_from_slice(&ipo_price);

        message
    }
}
