#![feature(test)]

use addordermessages::{AddOrder, AddOrderMPID};
use modifyordermessages::{
    OrderCancel, OrderDelete, OrderExecuted, OrderExecutedWithPrice, OrderReplace,
};
use noiimessages::NetOrderImbalanceIndicator;
use stockmessages::{
    IPOQuotingPeriodUpdate, MWCBDeclineLevel, MWCBStatus, MarketParticipantPosition,
    RegSHOShortSalePriceTestRestriction, StockDirectory, StockTradingAction,
};
use trademessages::{BrokenTrade, CrossingTrade, NonCrossingTrade};
use types::GenerateBinaryExample;

use super::*;

#[bench]
fn bench_byte_to_bool(b: &mut test::Bencher) {
    let byte;

    if fastrand::bool() {
        byte = b'Y';
    } else {
        byte = b'N';
    }

    b.iter(|| helpers::byte_to_bool(byte));
}

#[bench]
fn bench_byte_to_bool_space(b: &mut test::Bencher) {
    let valid_bytes = [b'Y', b'N', b' '];
    let byte = valid_bytes[fastrand::usize(..valid_bytes.len())];

    b.iter(|| helpers::byte_to_bool_space(byte));
}

#[bench]
fn bench_system_event_message(b: &mut test::Bencher) {
    let example_msg = SystemEventMessage::generate_example_message();
    b.iter(|| {
        let parsed = SystemEventMessage::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the system event message failed");
    });
}

#[bench]
fn bench_stock_directory(b: &mut test::Bencher) {
    let example_msg = StockDirectory::generate_example_message();
    b.iter(|| {
        let parsed = StockDirectory::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the stock directory message failed");
    });
}

#[bench]
fn bench_stock_trading_action(b: &mut test::Bencher) {
    let example_msg = StockTradingAction::generate_example_message();
    b.iter(|| {
        let parsed = StockTradingAction::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the stock trading action message failed"
        );
    });
}

#[bench]
fn bench_reg_sho_short_sale_price_test_restriction(b: &mut test::Bencher) {
    let example_msg = RegSHOShortSalePriceTestRestriction::generate_example_message();
    b.iter(|| {
        let parsed = RegSHOShortSalePriceTestRestriction::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the reg sho short sale price test restriction message failed"
        );
    });
}

#[bench]
fn bench_market_participant_position(b: &mut test::Bencher) {
    let example_msg = MarketParticipantPosition::generate_example_message();
    b.iter(|| {
        let parsed = MarketParticipantPosition::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the market participant position message failed"
        );
    });
}

#[bench]
fn bench_mwcb_decline_level(b: &mut test::Bencher) {
    let example_msg = MWCBDeclineLevel::generate_example_message();
    b.iter(|| {
        let parsed = MWCBDeclineLevel::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the mwcb decline level message failed"
        );
    });
}

#[bench]
fn bench_mwcb_status(b: &mut test::Bencher) {
    let example_msg = MWCBStatus::generate_example_message();
    b.iter(|| {
        let parsed = MWCBStatus::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the mwcb status message failed");
    });
}

#[bench]
fn bench_ipo_quoting_period_update(b: &mut test::Bencher) {
    let example_msg = IPOQuotingPeriodUpdate::generate_example_message();
    b.iter(|| {
        let parsed = IPOQuotingPeriodUpdate::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the ipo quoting period update message failed"
        );
    });
}

#[bench]
fn bench_non_crossing_trade(b: &mut test::Bencher) {
    let example_msg = NonCrossingTrade::generate_example_message();
    b.iter(|| {
        let parsed = NonCrossingTrade::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the non-crossing trade message failed"
        );
    });
}

#[bench]
fn bench_crossing_trade(b: &mut test::Bencher) {
    let example_msg = CrossingTrade::generate_example_message();
    b.iter(|| {
        let parsed = CrossingTrade::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the crossing trade message failed");
    });
}

#[bench]
fn bench_broken_trade(b: &mut test::Bencher) {
    let example_msg = BrokenTrade::generate_example_message();
    b.iter(|| {
        let parsed = BrokenTrade::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the broken trade message failed");
    });
}

#[bench]
fn bench_order_executed(b: &mut test::Bencher) {
    let example_msg = OrderExecuted::generate_example_message();
    b.iter(|| {
        let parsed = OrderExecuted::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the order executed message failed");
    });
}

#[bench]
fn bench_order_executed_with_price(b: &mut test::Bencher) {
    let example_msg = OrderExecutedWithPrice::generate_example_message();
    b.iter(|| {
        let parsed = OrderExecutedWithPrice::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the order executed with price message failed"
        );
    });
}

#[bench]
fn bench_order_cancel(b: &mut test::Bencher) {
    let example_msg = OrderCancel::generate_example_message();
    b.iter(|| {
        let parsed = OrderCancel::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the order cancel message failed");
    });
}

#[bench]
fn bench_order_delete(b: &mut test::Bencher) {
    let example_msg = OrderDelete::generate_example_message();
    b.iter(|| {
        let parsed = OrderDelete::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the order delete message failed");
    });
}

#[bench]
fn bench_order_replace(b: &mut test::Bencher) {
    let example_msg = OrderReplace::generate_example_message();
    b.iter(|| {
        let parsed = OrderReplace::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the order replace message failed");
    });
}

#[bench]
fn bench_add_order(b: &mut test::Bencher) {
    let example_msg = AddOrder::generate_example_message();
    b.iter(|| {
        let parsed = AddOrder::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the add order message failed");
    });
}

#[bench]
fn bench_add_order_mpid(b: &mut test::Bencher) {
    let example_msg = AddOrderMPID::generate_example_message();
    b.iter(|| {
        let parsed = AddOrderMPID::parse(&example_msg);
        assert!(parsed.is_ok(), "Parsing the add order mpid message failed");
    });
}

#[bench]
fn bench_net_order_imbalance_indicator(b: &mut test::Bencher) {
    let example_msg = NetOrderImbalanceIndicator::generate_example_message();
    b.iter(|| {
        let parsed = NetOrderImbalanceIndicator::parse(&example_msg);
        assert!(
            parsed.is_ok(),
            "Parsing the net order imbalance indicator message failed"
        );
    });
}
