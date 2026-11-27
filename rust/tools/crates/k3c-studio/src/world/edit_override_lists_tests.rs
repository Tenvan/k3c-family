use super::world_tests::{fixture, read_world};

fn path(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| (*p).to_owned()).collect()
}

// WE6.3 AC-03 (B-987): Ereignisse (`events`, je id) und Hub-Plätze (`hub › sites`, je Index) lassen sich über
// world_override_set übersteuern; neue Einträge werden abgelehnt.
#[test]
fn override_events_und_hub_sites() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("hanse", "Hanse", "standard").unwrap();
    let view = e.set_override("hanse", "events", &path(&["events", "bloodMoon", "everyNights"]), "9");
    assert!(view.error.is_empty(), "events: {}", view.error);
    let got = read_world(e, "hanse");
    assert!(got.contains(r#""everyNights": 9"#), "{got}");
    let view = e.set_override("hanse", "hub", &path(&["sites", "0", "topOnly"]), "false");
    assert!(view.error.is_empty(), "hub sites: {}", view.error);
    let got = read_world(e, "hanse");
    assert!(got.contains(r#""topOnly": false"#), "{got}");
    let view = e.set_override(
        "hanse",
        "events",
        &path(&["events", "neuesEreignis", "everyNights"]),
        "3",
    );
    assert!(!view.error.is_empty(), "neuer Ereignis-Eintrag muss abgelehnt werden");
}
