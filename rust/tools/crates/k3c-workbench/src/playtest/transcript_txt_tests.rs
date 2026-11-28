//! `TestTranscriptTextZeilenMitAufnahmezeit` aus `transcript_txt_test.go`.

use super::*;
use crate::playtest::Segment;

#[test]
fn transcript_text_zeilen_mit_aufnahmezeit() {
    let segment = |start: i64, end: i64, text: &str| Segment {
        start_ms: 1_000_000 + start,
        end_ms: 1_000_000 + end,
        text: text.to_owned(),
    };
    let transcript = Transcript {
        audio_start_ms: 1_000_000,
        segments: Some(vec![
            segment(65_000, 70_000, " Das war knapp. "),
            segment(3_000, 4_000, ""),
            segment(-500, 1_000, "Los geht's"),
        ]),
        ..Transcript::default()
    };
    assert_eq!(
        transcript_text(&transcript),
        "[01:05] Das war knapp.\n[00:00] Los geht's\n"
    );
}
