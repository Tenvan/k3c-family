//! Test aus `native/k3c-game/menu/keys_test.go`: `TestTextsComplete`.

use super::texts::{TEXT_COUNT, TextKey, tr, tr1};
use super::texts_data::TEXTS;
use crate::view::settings::LANGUAGE_ENGLISH;

#[test]
fn texts_complete() {
    assert_eq!(TEXTS.len(), TEXT_COUNT);
    for (k, pair) in TEXTS.iter().enumerate() {
        for (l, s) in pair.iter().enumerate() {
            assert!(!s.is_empty(), "Text {k} Sprache {l} fehlt");
            for r in s.chars() {
                assert!(
                    u32::from(r) <= 0xff,
                    "Text {s:?}: Zeichen {r:?} fehlt in der Debug-Schrift (nur Latin-1)"
                );
            }
        }
    }
    assert!(
        tr("fr", TextKey::New) == "Neues Spiel" && tr1(LANGUAGE_ENGLISH, TextKey::Player, "2") == "Player 2",
        "Sprache oder Platzhalter falsch"
    );
}

/// Die letzte Variante steht am Ende der Tabelle (Reihenfolge von Enum und Texten stimmt überein).
#[test]
fn text_order_matches_enum() {
    assert_eq!(TextKey::PadTestHint as usize, TEXT_COUNT - 1);
    assert_eq!(tr("de", TextKey::PadTestHint), "Tasten drücken · B zurück");
    assert_eq!(tr(LANGUAGE_ENGLISH, TextKey::Continue), "Continue");
}
