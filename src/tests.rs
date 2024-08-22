use modifyordermessages::{
    OrderCancel, OrderDelete, OrderExecuted, OrderExecutedWithPrice, OrderReplace,
};
use stockmessages::{
    IPOQuotingPeriodUpdate, MWCBDeclineLevel, MWCBStatus, MarketParticipantPosition,
    RegSHOShortSalePriceTestRestriction, StockDirectory, StockTradingAction,
};
use trademessages::{BrokenTrade, CrossingTrade, NonCrossingTrade};
use types::GenerateBinaryExample;

use super::*;

// Helpers
#[test]
fn test_byte_to_bool() {
    assert_eq!(helpers::byte_to_bool(b'Y'), true);
    assert_eq!(helpers::byte_to_bool(b'N'), false);
}

#[test]
fn test_byte_to_bool_space() {
    assert_eq!(
        helpers::byte_to_bool_space(b'Y'),
        enums::BoolOrUnavailable::Bool(true)
    );
    assert_eq!(
        helpers::byte_to_bool_space(b'N'),
        enums::BoolOrUnavailable::Bool(false)
    );
    assert_eq!(
        helpers::byte_to_bool_space(b' '),
        enums::BoolOrUnavailable::Str("Not Available".to_string())
    );
}

// System Events
#[test]
fn test_system_event_message() {
    let example_msg = SystemEventMessage::generate_example_message();
    let parsed = SystemEventMessage::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the system event message failed");
}

// Stock Messages
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

// Trade Messages
#[test]
fn test_non_crossing_trade() {
    let example_msg = NonCrossingTrade::generate_example_message();
    let parsed = NonCrossingTrade::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the non-crossing trade message failed"
    );
}

#[test]
fn test_crossing_trade() {
    let example_msg = CrossingTrade::generate_example_message();
    let parsed = CrossingTrade::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the crossing trade message failed");
}

#[test]
fn test_broken_trade() {
    let example_msg = BrokenTrade::generate_example_message();
    let parsed = BrokenTrade::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the broken trade message failed");
}

// Modify Order Messages
#[test]
fn test_order_executed() {
    let example_msg = OrderExecuted::generate_example_message();
    let parsed = OrderExecuted::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the order executed message failed");
}

#[test]
fn test_order_executed_with_price() {
    let example_msg = OrderExecutedWithPrice::generate_example_message();
    let parsed = OrderExecutedWithPrice::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the order executed with price message failed"
    );
}

#[test]
fn test_order_cancel() {
    let example_msg = OrderCancel::generate_example_message();
    let parsed = OrderCancel::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the order cancel message failed");
}

#[test]
fn test_order_delete() {
    let example_msg = OrderDelete::generate_example_message();
    let parsed = OrderDelete::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the order delete message failed");
}

#[test]
fn test_order_replace() {
    let example_msg = OrderReplace::generate_example_message();
    let parsed = OrderReplace::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the order replace message failed");
}

// Add Order Messages
#[test]
fn test_add_order() {
    let example_msg = addordermessages::AddOrder::generate_example_message();
    let parsed = addordermessages::AddOrder::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the add order message failed");
}

#[test]
fn test_add_order_mpid() {
    let example_msg = addordermessages::AddOrderMPID::generate_example_message();
    let parsed = addordermessages::AddOrderMPID::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the add order mpid message failed");
}

// NOII Messages
#[test]
fn test_net_order_imbalance_indicator() {
    let example_msg = noiimessages::NetOrderImbalanceIndicator::generate_example_message();
    let parsed = noiimessages::NetOrderImbalanceIndicator::parse(&example_msg);
    assert!(
        parsed.is_ok(),
        "Parsing the net order imbalance indicator message failed"
    );
}
