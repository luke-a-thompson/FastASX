// RUN THIS FILE WITH:
// cargo bench --features bench

#![cfg(feature = "bench")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastasx::types::{EnumTestHelpers, GenerateExampleMessage, Parse};
use fastasx::{
    addordermessages, enums, helpers, modifyordermessages, noiimessages, stockmessages,
    systemmessages, trademessages, types,
};

fn bench_byte_to_bool(c: &mut Criterion) {
    let valid_bytes = [b'Y', b'N'];
    let byte = valid_bytes[fastrand::usize(..valid_bytes.len())];

    c.bench_function("byte_to_bool", |b| {
        b.iter(|| black_box(helpers::byte_to_bool(byte)));
    });
}

fn bench_byte_to_bool_space(c: &mut Criterion) {
    let valid_bytes = [b'Y', b'N', b' '];
    let byte = valid_bytes[fastrand::usize(..valid_bytes.len())];

    c.bench_function("byte_to_bool_space", |b| {
        b.iter(|| black_box(helpers::byte_to_bool_space(byte)));
    });
}

fn bench_issue_classification_values(c: &mut Criterion) {
    let values = enums::IssueClassificationCodes::generate_example_code();

    c.bench_function("issue_classification_values", |b| {
        b.iter(|| {
            let parsed = enums::IssueClassificationCodes::try_from(black_box(values));
            assert!(
                parsed.is_ok(),
                "Parsing the issue classification code failed"
            );
        });
    });
}

fn bench_stock_parsing(c: &mut Criterion) {
    let example_msg = types::Stock::generate_binary_example();

    c.bench_function("stock_parsing", |b| {
        b.iter(|| {
            let _parsed: types::Stock = black_box(example_msg).try_into().unwrap();
        });
    });
}

fn bench_system_event_message(c: &mut Criterion) {
    let example_msg = systemmessages::SystemEventMessage::generate_binary_example();

    c.bench_function("system_event_message", |b| {
        b.iter(|| {
            let parsed = systemmessages::SystemEventMessage::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the system event message failed");
        });
    });
}

fn bench_stock_directory(c: &mut Criterion) {
    let example_msg = stockmessages::StockDirectory::generate_binary_example();

    c.bench_function("stock_directory", |b| {
        b.iter(|| {
            let parsed = stockmessages::StockDirectory::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the stock directory message failed");
        });
    });
}

fn bench_stock_trading_action(c: &mut Criterion) {
    let example_msg = stockmessages::StockTradingAction::generate_binary_example();

    c.bench_function("stock_trading_action", |b| {
        b.iter(|| {
            let parsed = stockmessages::StockTradingAction::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the stock trading action message failed"
            );
        });
    });
}

fn bench_reg_sho_short_sale_price_test_restriction(c: &mut Criterion) {
    let example_msg = stockmessages::RegSHOShortSalePriceTestRestriction::generate_binary_example();

    c.bench_function("reg_sho_short_sale_price_test_restriction", |b| {
        b.iter(|| {
            let parsed =
                stockmessages::RegSHOShortSalePriceTestRestriction::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the reg sho short sale price test restriction message failed"
            );
        });
    });
}

fn bench_market_participant_position(c: &mut Criterion) {
    let example_msg = stockmessages::MarketParticipantPosition::generate_binary_example();

    c.bench_function("market_participant_position", |b| {
        b.iter(|| {
            let parsed = stockmessages::MarketParticipantPosition::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the market participant position message failed"
            );
        });
    });
}

fn bench_mwcb_decline_level(c: &mut Criterion) {
    let example_msg = stockmessages::MWCBDeclineLevel::generate_binary_example();

    c.bench_function("mwcb_decline_level", |b| {
        b.iter(|| {
            let parsed = stockmessages::MWCBDeclineLevel::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the mwcb decline level message failed"
            );
        });
    });
}

fn bench_mwcb_status(c: &mut Criterion) {
    let example_msg = stockmessages::MWCBStatus::generate_binary_example();

    c.bench_function("mwcb_status", |b| {
        b.iter(|| {
            let parsed = stockmessages::MWCBStatus::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the mwcb status message failed");
        });
    });
}

fn bench_ipo_quoting_period_update(c: &mut Criterion) {
    let example_msg = stockmessages::IPOQuotingPeriodUpdate::generate_binary_example();

    c.bench_function("ipo_quoting_period_update", |b| {
        b.iter(|| {
            let parsed = stockmessages::IPOQuotingPeriodUpdate::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the ipo quoting period update message failed"
            );
        });
    });
}

fn bench_non_crossing_trade(c: &mut Criterion) {
    let example_msg = trademessages::NonCrossingTrade::generate_binary_example();

    c.bench_function("non_crossing_trade", |b| {
        b.iter(|| {
            let parsed = trademessages::NonCrossingTrade::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the non-crossing trade message failed"
            );
        });
    });
}

fn bench_crossing_trade(c: &mut Criterion) {
    let example_msg = trademessages::CrossingTrade::generate_binary_example();

    c.bench_function("crossing_trade", |b| {
        b.iter(|| {
            let parsed = trademessages::CrossingTrade::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the crossing trade message failed");
        });
    });
}

fn bench_broken_trade(c: &mut Criterion) {
    let example_msg = trademessages::BrokenTrade::generate_binary_example();

    c.bench_function("broken_trade", |b| {
        b.iter(|| {
            let parsed = trademessages::BrokenTrade::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the broken trade message failed");
        });
    });
}

fn bench_order_executed(c: &mut Criterion) {
    let example_msg = modifyordermessages::OrderExecuted::generate_binary_example();

    c.bench_function("order_executed", |b| {
        b.iter(|| {
            let parsed = modifyordermessages::OrderExecuted::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the order executed message failed");
        });
    });
}

fn bench_order_executed_with_price(c: &mut Criterion) {
    let example_msg = modifyordermessages::OrderExecutedWithPrice::generate_binary_example();

    c.bench_function("order_executed_with_price", |b| {
        b.iter(|| {
            let parsed =
                modifyordermessages::OrderExecutedWithPrice::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the order executed with price message failed"
            );
        });
    });
}

fn bench_order_cancel(c: &mut Criterion) {
    let example_msg = modifyordermessages::OrderCancel::generate_binary_example();

    c.bench_function("order_cancel", |b| {
        b.iter(|| {
            let parsed = modifyordermessages::OrderCancel::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the order cancel message failed");
        });
    });
}

fn bench_order_delete(c: &mut Criterion) {
    let example_msg = modifyordermessages::OrderDelete::generate_binary_example();

    c.bench_function("order_delete", |b| {
        b.iter(|| {
            let parsed = modifyordermessages::OrderDelete::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the order delete message failed");
        });
    });
}

fn bench_order_replace(c: &mut Criterion) {
    let example_msg = modifyordermessages::OrderReplace::generate_binary_example();

    c.bench_function("order_replace", |b| {
        b.iter(|| {
            let parsed = modifyordermessages::OrderReplace::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the order replace message failed");
        });
    });
}

fn bench_add_order(c: &mut Criterion) {
    let example_msg = addordermessages::AddOrder::generate_binary_example();

    c.bench_function("add_order", |b| {
        b.iter(|| {
            let parsed = addordermessages::AddOrder::parse(black_box(&example_msg));
            assert!(parsed.is_ok(), "Parsing the add order message failed");
        });
    });
}

fn bench_net_order_imbalance_indicator(c: &mut Criterion) {
    let example_msg = noiimessages::NetOrderImbalanceIndicator::generate_binary_example();

    c.bench_function("net_order_imbalance_indicator", |b| {
        b.iter(|| {
            let parsed = noiimessages::NetOrderImbalanceIndicator::parse(black_box(&example_msg));
            assert!(
                parsed.is_ok(),
                "Parsing the net order imbalance indicator message failed"
            );
        });
    });
}

criterion_group!(
    benches,
    bench_byte_to_bool,
    bench_byte_to_bool_space,
    bench_issue_classification_values,
    bench_stock_parsing,
    bench_system_event_message,
    bench_stock_directory,
    bench_stock_trading_action,
    bench_reg_sho_short_sale_price_test_restriction,
    bench_market_participant_position,
    bench_mwcb_decline_level,
    bench_mwcb_status,
    bench_ipo_quoting_period_update,
    bench_non_crossing_trade,
    bench_crossing_trade,
    bench_broken_trade,
    bench_order_executed,
    bench_order_executed_with_price,
    bench_order_cancel,
    bench_order_delete,
    bench_order_replace,
    bench_add_order,
    bench_net_order_imbalance_indicator,
);

criterion_main!(benches);
