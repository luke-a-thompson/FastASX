use types::GenerateBinaryExample;

use super::*;

#[test]
fn test_system_event_message() {
    let example_msg = SystemEventMessage::generate_example_message();
    let parsed = SystemEventMessage::parse(&example_msg);
    assert!(parsed.is_ok(), "Parsing the system event message failed");
}