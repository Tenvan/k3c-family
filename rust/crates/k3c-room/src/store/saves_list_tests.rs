//! Tests aus `saves_list_test.go`.
use std::time::Duration;

use super::saves::Saves;
use super::stamp::Stamp;
use super::testutil::{TempDir, clock};

// (d) S2.2, B-147/AC-03: `list` nennt neue und alte Stände mit ihren Feldern, keine Sicherungen, eine kaputte Datei als
// Eintrag mit error.
#[test]
fn saves_list_nennt_felder_ohne_sicherungen() {
    let dir = TempDir::new();
    let mut s = Saves::new(dir.path(), clock());
    s.fs.pause = Duration::ZERO;
    let v4 = r#"{"version":4,"campaignId":"neu","savedAt":"2026-10-04T22:00:00Z","day":3,"phase":"night","players":[{"index":0,"depth":1},{"index":1,"depth":0},{"index":2,"depth":1}]}"#;
    for data in [v4, v4] {
        // zweimal: der erste Stand wandert in die rotierenden Sicherungen
        s.store("neu", data.as_bytes()).expect("neu");
    }
    s.store(
        "alt",
        br#"{"version":3,"campaignId":"alt","savedAt":"2026-10-01T10:00:00Z","players":[]}"#,
    )
    .expect("alt");
    // Sicherung eines fremden Spiels (<slot>-<savedAt>.json) und eine kaputte Datei liegen daneben.
    std::fs::write(dir.path().join("neu-2026-10-01T10-00-00Z.json"), v4).expect("Sicherung");
    std::fs::write(dir.path().join("kaputt.json"), "{kaputt").expect("kaputt");
    let got = serde_json::to_string(&s.list()).expect("JSON");
    let want = concat!(
        r#"[{"name":"alt","savedAt":"2026-10-01T10:00:00Z","version":3},"#,
        r#"{"name":"kaputt","error":"ungültig"},"#,
        r#"{"name":"neu","savedAt":"2026-10-04T22:00:00Z","version":4,"day":3,"phase":"night","depths":[0,1]}]"#
    );
    assert_eq!(got, want);
    let ohne_ordner = Saves::new(dir.path().join("fehlt"), || Stamp::new(0, 0));
    assert_eq!(
        serde_json::to_string(&ohne_ordner.list()).expect("JSON"),
        "[]",
        "ohne Ordner"
    );
}
