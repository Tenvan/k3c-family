//! Tests aus `save_test.go` (S2.2, B-147): Speichern beim Verlassen und Trennen; ein Schreibfehler lässt den vorigen
//! Stand unverändert. Dazu das Spielstand-Format (B-1014, AC-03): `testdata/saves/v9/familie.json` laden, speichern und
//! wieder laden ergibt denselben Zustands-Baum; das Format bleibt Version 9 mit Feldnamen und Reihenfolge wie Go.
use serde_json::Value;

use super::save_testutil::{create_with, drop_device, fast_cycle, leave, logged, saved};
use super::testutil::{TestManager, join_room, manager, rm, ticks, wait};
use super::types::{EMPTY_FOR_SECS, Options};
use crate::fehler::Fehler;

/// Tickt mit beschleunigtem Zyklus, bis Nacht ist (Go `tickToNight`).
fn tick_to_night(m: &mut TestManager, code: &str) {
    fast_cycle(m, code);
    for _ in 0..=300 {
        if rm(m, code).isl.stages[0].cycle.phase == "night" {
            return;
        }
        ticks(m, code, 1);
    }
    panic!("keine Nacht nach 10 s");
}

// (a) AC-03/AC-04: Gerät 2 verlässt → eine Speicherung, Gerät 1 bricht ab (das letzte) → noch eine; der Stand lädt und
// nennt Zeitpunkt, Tag, Phase und Tiefen.
#[test]
fn speichern_bei_verlassen_und_abbruch() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "nacht", true, Options::default());
    let _hi = join_room(&mut m, "handy", 2, &code, &[0]);
    tick_to_night(&mut m, &code);
    let before = m.store.calls;
    leave(&mut m, &code, "handy", 2);
    assert_eq!(m.store.calls, before + 1, "Verlassen bei verbliebenem Gerät");
    drop_device(&mut m, &code, "xbox", 1);
    assert_eq!(m.store.calls, before + 2, "Abbruch des letzten Geräts");
    let s = saved(&m, "nacht");
    assert!(
        s.phase == "night" && s.day >= 1 && !s.saved_at.is_empty() && s.players.len() == 2 && s.players[1].depth == 0,
        "Stand: Phase {:?}, Tag {}, savedAt {:?}, Spieler {:?}",
        s.phase,
        s.day,
        s.saved_at,
        s.players
    );
    assert!(m.load_in(&m.store.data["nacht"]).is_ok(), "Stand lädt nicht");
}

// (b) AC-03: Scheitert das Schreiben, bleibt die Datei Byte für Byte gleich, das Log nennt den Fehler, und der leere
// Raum fährt nach `EMPTY_FOR_SECS` trotzdem herunter.
#[test]
fn schreibfehler_laesst_stand_unveraendert() {
    let (mut m, clock) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "heil", true, Options::default());
    leave(&mut m, &code, "xbox", 1);
    wait(&mut m, &clock, EMPTY_FOR_SECS + 60);
    let prev = m.store.data["heil"].clone();

    m.store.fail = Some(Fehler::Io("platte voll".into()));
    let (code, _yi) = create_with(&mut m, "xbox", 2, "heil", false, Options::default());
    ticks(&mut m, &code, 30);
    leave(&mut m, &code, "xbox", 2);
    assert_eq!(
        m.store.data["heil"], prev,
        "die Datei hat sich trotz Schreibfehler geändert"
    );
    assert!(
        logged(&m, "Spielstand nicht gespeichert") && logged(&m, "platte voll"),
        "Log ohne Fehler:\n{:?}",
        m.log.lines
    );
    wait(&mut m, &clock, EMPTY_FOR_SECS + 60);
    assert!(m.rooms.is_empty(), "Raum nach Schreibfehler nicht aufgeräumt");
}

/// Der Pfad der ersten Abweichung zweier JSON-Bäume ("" = gleich; Zahlen über `==` von `serde_json`, also als `f64`).
pub(super) fn diff(at: &str, want: &Value, got: &Value) -> String {
    match (want, got) {
        (Value::Object(a), Value::Object(b)) => {
            if !a.keys().eq(b.keys()) {
                return format!("{at}: Schlüssel {:?} statt {:?}", b.keys(), a.keys());
            }
            a.iter()
                .map(|(k, v)| diff(&format!("{at}.{k}"), v, &b[k]))
                .find(|d| !d.is_empty())
                .unwrap_or_default()
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => a
            .iter()
            .zip(b)
            .enumerate()
            .map(|(i, (x, y))| diff(&format!("{at}[{i}]"), x, y))
            .find(|d| !d.is_empty())
            .unwrap_or_default(),
        _ if want == got => String::new(),
        _ => format!("{at}: {got} statt {want}"),
    }
}

/// Die Schlüssel jedes Objekts des JSON-Textes in Textreihenfolge, mit seiner Tiefe (`serde_json::Value` sortiert sie).
fn objects(text: &[u8]) -> Vec<(usize, Vec<String>)> {
    let (mut out, mut open, mut nest, mut want_key, mut i) = (Vec::new(), Vec::new(), Vec::new(), false, 0);
    while i < text.len() {
        match text[i] {
            b'{' => {
                nest.push(b'{');
                open.push(Vec::new());
                want_key = true;
            }
            b'[' => nest.push(b'['),
            b'}' | b']' => {
                if nest.pop() == Some(b'{') {
                    out.push((nest.len(), open.pop().unwrap_or_default()));
                }
            }
            b',' => want_key = nest.last() == Some(&b'{'),
            b'"' => {
                let start = i + 1;
                i += 1;
                while text[i] != b'"' {
                    i += if text[i] == b'\\' { 2 } else { 1 };
                }
                if want_key && let Some(keys) = open.last_mut() {
                    keys.push(String::from_utf8_lossy(&text[start..i]).into_owned());
                    want_key = false;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// Prüft die Feldreihenfolge von `got` gegen `want` (Go): Steht in einem Objekt von `want` derselben Tiefe `b` vor `a`
/// und nie `a` vor `b`, darf `got` `a` nicht vor `b` schreiben. Die Inhalte dürfen sich unterscheiden (andere Plätze,
/// weggelassene leere Felder). Liefert die erste Abweichung ("" = keine).
fn order_conflict(want: &[u8], got: &[u8]) -> String {
    let mut before = std::collections::BTreeSet::new();
    for (d, keys) in objects(want) {
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                before.insert((d, a.clone(), b.clone()));
            }
        }
    }
    for (d, keys) in objects(got) {
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                let (ab, ba) = ((d, a.clone(), b.clone()), (d, b.clone(), a.clone()));
                if before.contains(&ba) && !before.contains(&ab) {
                    return format!("Tiefe {d}: {a} vor {b}, Go schreibt {b} vor {a}");
                }
            }
        }
    }
    String::new()
}

// B-1014/AC-03: `familie.json` (Version 9) laden, über den Raum speichern und wieder laden: der Zustands-Baum aller
// Stufen ist gleich, der gespeicherte Stand ist Version 12 (RU62.10) mit den Feldern in der Reihenfolge der Datei, und ein zweites
// Speichern ergibt dieselben Bytes.
#[test]
fn spielstand_v9_laden_speichern_gleicher_zustand() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../testdata/saves/v9/familie.json");
    let raw = std::fs::read(path).expect("familie.json");
    let (mut m, _) = manager();
    m.store.data.insert("familie".into(), raw.clone());
    let (code, _xi) = create_with(&mut m, "xbox", 1, "familie", false, Options::default());
    let before = serde_json::to_value(&m.load_in(&raw).expect("laden").stages).expect("Stufen");
    assert_eq!(m.with_room(&code, |r, env| r.save_now(env).is_ok()), Some(true));
    let first = m.store.data["familie"].clone();
    let after = serde_json::to_value(&m.load_in(&first).expect("laden").stages).expect("Stufen");
    assert_eq!(diff("stages", &before, &after), "", "Zustand nach Speichern und Laden");

    let out: Value = serde_json::from_slice(&first).expect("JSON");
    assert_eq!(out["version"], 12, "Version");
    assert_eq!(
        order_conflict(&raw, &first),
        "",
        "Feldreihenfolge wie Go (familie.json)"
    );
    let (file, mine) = (objects(&raw), objects(&first));
    let top = |o: &[(usize, Vec<String>)]| {
        o.iter()
            .find(|(d, _)| *d == 0)
            .map(|(_, k)| k.clone())
            .unwrap_or_default()
    };
    assert_eq!(top(&mine), top(&file), "Feldnamen und Reihenfolge des Stands");

    m.store.data.insert("zwei".into(), first.clone());
    let (code2, _yi) = create_with(&mut m, "handy", 2, "zwei", false, Options::default());
    assert_eq!(m.with_room(&code2, |r, env| r.save_now(env).is_ok()), Some(true));
    assert_eq!(m.store.data["zwei"], first, "zweites Speichern weicht ab");
}
