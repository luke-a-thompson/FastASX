use stockmessages::{
    IPOQuotingPeriodUpdate, MWCBDeclineLevel, MWCBStatus, MarketParticipantPosition, RegSHOShortSalePriceTestRestriction, StockDirectory, StockTradingAction
};
use types::GenerateBinaryExample;

use super::*;

#[test]
fn test_system_event_message() {
    let example_msg = SystemEventMessage::generate_example_message();
    let parsed = SystemEventMessage::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the system event message failed");
}

#[test]
fn test_stock_directory() {
    let example_msg = StockDirectory::generate_example_message();
    let parsed = StockDirectory::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the stock directory message failed");
}

#[test]
fn test_stock_trading_action() {
    let example_msg = StockTradingAction::generate_example_message();
    let parsed = StockTradingAction::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the stock trading action message failed"
    );
}

#[test]
fn test_reg_sho_short_sale_price_test_restriction() {
    let example_msg = RegSHOShortSalePriceTestRestriction::generate_example_message();
    let parsed = RegSHOShortSalePriceTestRestriction::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the reg sho short sale price test restriction message failed"
    );
}

#[test]
fn test_market_participant_position() {
    let example_msg = MarketParticipantPosition::generate_example_message();
    let parsed = MarketParticipantPosition::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the market participant position message failed"
    );
}

#[test]
fn test_mwcb_decline_level() {
    let example_msg = MWCBDeclineLevel::generate_example_message();
    let parsed = MWCBDeclineLevel::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the mwcb decline level message failed"
    );
}

#[test]
fn test_mwcb_status() {
    let example_msg = MWCBStatus::generate_example_message();
    let parsed = MWCBStatus::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the mwcb status message failed");
}

#[test]
fn test_ipo_quoting_period_update() {
    let example_msg = IPOQuotingPeriodUpdate::generate_example_message();
    let parsed = IPOQuotingPeriodUpdate::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the ipo quoting period update message failed"
    );
}