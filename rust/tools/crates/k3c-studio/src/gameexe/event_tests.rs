//! Port von `event_test.go`.

use super::parse_event;

#[test]
fn parse_event_wie_go() {
    let ev = parse_event(r#"{"event":"progress","done":3,"total":6}"#).unwrap();
    assert_eq!((ev.event.as_str(), ev.done, ev.total), ("progress", 3, 6));
    let ev = parse_event(r#"{"event":"result","report":{"a":1}}"#).unwrap();
    assert_eq!(ev.report.unwrap().get(), r#"{"a":1}"#);
    let ev = parse_event(r#"{"event":"report","files":["a.json","b.md"]}"#).unwrap();
    assert_eq!(ev.files.len(), 2);
    let ev = parse_event(r#"{"event":"progress","done":null,"x":1}"#).unwrap();
    assert_eq!(ev.done, 0);
    for bad in ["", "text", r#"{"x":1}"#, r#"{"event":""}"#, r#"["result"]"#] {
        assert!(parse_event(bad).is_none(), "{bad} gilt nicht als Event");
    }
}
