//! Port von `stats_test.go`; dazu der Eintrag in `transcript.json`.

use std::time::{Duration, UNIX_EPOCH};

use super::*;
use crate::testutil::tmp_dir;

fn transcript() -> Transcript {
    let segment = |text: &str| Segment {
        text: text.to_owned(),
        ..Segment::default()
    };
    Transcript {
        segments: Some(vec![segment("eins zwei drei"), segment("vier")]),
        ..Transcript::default()
    }
}

#[test]
fn make_stats_rechnet_das_verhaeltnis() {
    let seconds = Duration::from_secs;
    let st = make_stats(seconds(60), seconds(6), 120_000, &transcript(), 3);
    assert_eq!(
        (st.total_ms, st.words, st.segments, st.frames),
        (66000, 4, 2, 3),
        "{st:?}"
    );
    assert!(
        (st.sec_per_min - 33.0).abs() < 1e-9 && (st.factor - 0.55).abs() < 1e-12,
        "{st:?}"
    );
    let none = make_stats(seconds(1), Duration::ZERO, 0, &transcript(), 0);
    assert!(none.factor == 0.0 && none.sec_per_min == 0.0, "{none:?}");
}

#[test]
fn save_stats_traegt_in_transcript_json_ein() {
    let dir = tmp_dir();
    std::fs::write(
        dir.path().join("transcript.json"),
        r#"{"audioStartMs":5,"language":"de","model":"m","segments":[{"startMs":1,"endMs":2,"text":"a<b"}]}"#,
    )
    .unwrap();
    std::fs::create_dir(dir.path().join("frames")).unwrap();
    std::fs::write(dir.path().join("frames").join("marker-1.jpg"), "x").unwrap();
    let at = |s| UNIX_EPOCH + Duration::from_secs(s);
    save_stats(dir.path(), at(0), at(60), at(66), 120_000).unwrap();
    let text = std::fs::read_to_string(dir.path().join("transcript.json")).unwrap();
    assert!(text.contains("\"text\": \"a\\u003cb\""), "{text}");
    assert!(text.contains("\"totalMs\": 66000"), "{text}");
    assert!(
        text.contains("\"frames\": 1,\n    \"secPerMin\": 33,\n    \"factor\": 0.55\n"),
        "{text}"
    );
    assert!(!dir.path().join("transcript.json.tmp").exists());
    assert!(save_stats(&dir.path().join("fehlt"), at(0), at(0), at(0), 0).is_err());
}
