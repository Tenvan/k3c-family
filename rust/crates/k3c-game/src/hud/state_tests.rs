//! Port von `native/k3c-game/hud/state_test.go`.

use serde_json::json;

use super::state::SAVED_SECONDS;
use super::testutil::{hud, set_world, two_players};
use crate::view::frame::{Cue, CueKind, Event, Frame};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::settings::{LANGUAGE_ENGLISH, Settings, default_settings};
use k3c_sim::state::{Event as SimEvent, World};

fn event(v: serde_json::Value) -> Event {
    let data: SimEvent = v.as_object().cloned().unwrap_or_default();
    Event {
        stage: 0,
        tick: 0,
        data,
    }
}

#[test]
fn listen_je_spieler_getrennt() {
    let (w, mut f) = two_players(0);
    f.settings = default_settings();
    let mut h = hud();
    f.events = vec![event(json!({"type": "chest", "player": 1, "gold": 5}))];
    f.cues = vec![Cue {
        kind: CueKind::Denied,
        player: 1,
        text: "abgelehnt".to_string(),
        ..Default::default()
    }];
    h.listen(&f);
    let (a, b) = (&h.seats[0], &h.seats[1]);
    assert!(
        a.active && b.active && !a.gold_s.is_empty() && !a.clock_s.is_empty() && !a.shared_s.is_empty(),
        "beide Spieler mit Werten: {a:?}"
    );
    assert!(
        a.banner.showing == "P2 findet 5 Gold" && b.banner.queue == ["P2 findet 5 Gold"],
        "Meldung der Stufe in beiden Zellen: {:?} {:?}",
        a.banner.showing,
        b.banner.queue
    );
    assert!(
        a.banner.queue.is_empty() && b.banner.showing == "abgelehnt",
        "Ablehnung nur beim betroffenen Spieler: {:?} {:?}",
        a.banner.queue,
        b.banner.showing
    );
    assert!(
        a.place_s == "P1 · Oberwelt (Wald)" && b.place_s == "P2 · Oberwelt (Wald)",
        "Ort je Spieler: {:?} {:?}",
        a.place_s,
        b.place_s
    );
    gold_und_gesichert(&mut h, w, f);
}

/// gold_und_gesichert: Goldtopf nur beim Spieler mit geändertem Gold, „gesichert“ läuft ab, ein gehender Spieler fällt weg.
fn gold_und_gesichert(h: &mut super::Hud, mut w: World, mut f: Frame) {
    w.players[1].gold += 3;
    set_world(&mut f, &w);
    f.events.clear();
    f.cues = vec![Cue {
        kind: CueKind::Saved,
        player: -1,
        ..Default::default()
    }];
    f.clock = 1.0;
    h.listen(&f);
    let (a, b) = (&h.seats[0], &h.seats[1]);
    assert!(!a.flash && b.flash, "Goldtopf nur beim Spieler mit geändertem Gold");
    assert!(
        a.clock_s.ends_with("gesichert") && b.clock_s.ends_with("gesichert"),
        "gesichert nach CueSaved: {:?}",
        a.clock_s
    );
    f.cues.clear();
    f.clock = 1.0 + SAVED_SECONDS;
    h.listen(&f);
    assert!(!h.seats[0].clock_s.ends_with("gesichert"), "gesichert läuft ab");
    f.seats.remove(0);
    h.listen(&f);
    assert!(
        !h.seats[0].active && h.cells == 1,
        "Spieler 0 gegangen: Spieler 1 allein"
    );
}

#[test]
fn listen_englisch_und_beitritt() {
    let (_, mut f) = two_players(0);
    f.settings = Settings {
        language: LANGUAGE_ENGLISH.to_string(),
        ..Default::default()
    };
    f.unseated = vec![DeviceID::new(DeviceKind::Gamepad, 4)];
    let mut h = hud();
    h.listen(&f);
    assert!(
        h.join_s == "Press A to join" && h.seats[0].clock_s.starts_with("Day 1"),
        "englisch: {:?} {:?}",
        h.join_s,
        h.seats[0].clock_s
    );
}

/// Gleicher Frame, gleicher Zustand: zwei HUDs mit derselben Folge liefern dieselben Texte (Bildgleichheit).
#[test]
fn listen_deterministisch() {
    let run = || {
        let (mut w, mut f) = two_players(1);
        let mut h = hud();
        for tick in 0..120_u64 {
            w.players[0].x += 1.0 / 30.0 * 6.0;
            w.players[1].x -= 1.0 / 30.0 * 9.0;
            w.players[0].gold = (tick / 30) as i64;
            set_world(&mut f, &w);
            f.tick = tick;
            f.clock = tick as f64 / 30.0;
            f.events.clear();
            if tick == 60 {
                f.events.push(event(json!({"type": "night", "day": 1})));
            }
            h.listen(&f);
        }
        h
    };
    let (a, b) = (run(), run());
    for p in 0..a.seats.len() {
        let (x, y) = (&a.seats[p], &b.seats[p]);
        assert!(
            x.gold_s == y.gold_s
                && x.clock_s == y.clock_s
                && x.banner.showing == y.banner.showing
                && x.x == y.x
                && x.guide_s == y.guide_s,
            "Spieler {p} verschieden"
        );
    }
}

/// is_german ist die Regex `[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}` aus `state_test.go`.
fn is_german(s: &str) -> bool {
    if s.chars().any(|c| "äöüßÄÖÜ".contains(c)) {
        return true;
    }
    let c: Vec<char> = s.chars().collect();
    let run = |from: usize| c[from..].iter().take_while(|x| x.is_ascii_alphabetic()).count();
    let mut i = 0;
    while i < c.len() {
        if !c[i].is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let end = i + run(i);
        let gap = c[end..].iter().take_while(|x| x.is_whitespace()).count();
        if end - i >= 2 && gap > 0 && run(end + gap) >= 2 {
            return true;
        }
        i = end;
    }
    false
}

/// string_literals liefert die Zeichenketten-Literale des Quelltexts (ohne Kommentare und Zeichenliterale).
fn string_literals(src: &str) -> Vec<String> {
    let c: Vec<char> = src.chars().collect();
    let (mut out, mut i) = (Vec::new(), 0);
    while i < c.len() {
        match c[i] {
            '/' if c.get(i + 1) == Some(&'/') => {
                while i < c.len() && c[i] != '\n' {
                    i += 1;
                }
            }
            '/' if c.get(i + 1) == Some(&'*') => {
                i += 2;
                while i + 1 < c.len() && !(c[i] == '*' && c[i + 1] == '/') {
                    i += 1;
                }
                i += 2;
            }
            '\'' => {
                // Zeichenliteral ('x', '\n') überspringen; sonst Lebenszeit ('a)
                if c.get(i + 1) == Some(&'\\') {
                    i += 2;
                    while i < c.len() && c[i] != '\'' {
                        i += 1;
                    }
                } else if c.get(i + 2) == Some(&'\'') {
                    i += 2;
                }
                i += 1;
            }
            '"' => {
                let raw = i > 0 && (c[i - 1] == 'r' || c[i - 1] == '#');
                i += 1;
                let mut s = String::new();
                while i < c.len() && c[i] != '"' {
                    if c[i] == '\\' && !raw {
                        i += 1;
                    }
                    if i < c.len() {
                        s.push(c[i]);
                    }
                    i += 1;
                }
                out.push(s);
                i += 1;
            }
            _ => i += 1,
        }
    }
    out
}

/// Textregel (textRule.test.ts): Spieltexte stehen nur in `texts*.rs`; kein anderes Literal mit Umlaut oder zwei
/// Wörtern. Der Go-Test parst Quelltext mit go/parser; hier liest ein Zeichenscan die Literale (ohne Kommentare, ohne
/// Zeichenliterale, ohne Literale mit Emoji-Präfix wie die Log-Zeilen).
#[test]
fn keine_texte_ausserhalb_der_textdatei() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/hud");
    let mut bad = Vec::new();
    for e in std::fs::read_dir(&dir).expect("hud-Ordner") {
        let path = e.expect("Eintrag").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let skip = !name.ends_with(".rs")
            || name.starts_with("texts")
            || name.ends_with("_tests.rs")
            || name == "pending.rs"
            || name == "testutil.rs";
        if skip {
            continue;
        }
        let src = std::fs::read_to_string(&path).expect("Quelltext");
        for lit in string_literals(&src) {
            let emoji_prefix = lit
                .chars()
                .next()
                .is_some_and(|c| !c.is_ascii() && !c.is_alphanumeric());
            if is_german(&lit) && !emoji_prefix {
                bad.push(format!("{name}: Text-Literal {lit:?} gehört nach texts.rs"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn textregel_erkennt_texte() {
    assert!(is_german("Die Burg") && is_german("Größe") && is_german("ab  cd"));
    assert!(!is_german("hud.place") && !is_german("wood") && !is_german("{}:{:02}") && !is_german("a b"));
    assert_eq!(string_literals("let a = \"x y\"; // \"nein\"\nlet c = '\"';"), ["x y"]);
}
