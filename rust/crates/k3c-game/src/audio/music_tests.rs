//! Port von `native/k3c-game/audio/music_test.go`.

use std::collections::BTreeMap;

use super::duck::{DuckShape, DuckWindow, duck_gain, extend_duck};
use super::events_tests::missing_files;
use super::music::*;
use super::testutil::real_register;

const STATES: [MusicState; 9] = [
    MusicState::Day,
    MusicState::Dusk,
    MusicState::Night,
    MusicState::Battle,
    MusicState::Depth,
    MusicState::Boss,
    MusicState::Lobby,
    MusicState::Defeat,
    MusicState::Ending,
];

// AC-01: Jeder Zustand außer None hat einen Musik-Slot mit Datei im Register, kein Stück doppelt.
#[test]
fn music_slots_have_files() {
    assert!(MUSIC_SLOTS.len() == MusicState::Ending as usize + 1 && MUSIC_SLOTS[MusicState::None as usize].is_empty());
    let reg = real_register();
    let mut files = std::collections::BTreeSet::new();
    for st in STATES {
        let slot = st.slot();
        let f = reg.music.get(slot).map_or("", String::as_str);
        assert!(
            !slot.is_empty() && !f.is_empty() && files.insert(f),
            "Zustand {st:?}: Slot {slot:?}, Datei {f:?} fehlt oder doppelt"
        );
    }
}

// AC-02: Fehlt einem Musik-Slot die Datei im Register, nennt die Prüfung genau diesen Slot.
#[test]
fn missing_files_names_music_slot() {
    let mut files = real_register().music;
    let slots: BTreeMap<String, String> = MUSIC_SLOTS[MusicState::Day as usize..]
        .iter()
        .map(|s| (s.to_string(), s.to_string()))
        .collect();
    assert!(missing_files(&slots, &files).is_empty(), "Musik-Slots ohne Datei");
    files.remove(MusicState::Boss.slot());
    assert_eq!(missing_files(&slots, &files), ["music:boss → music:boss"]);
}

fn view(phase: &str, enemies: i64, depth: i64, boss: bool, victory: bool) -> MusicView<'_> {
    MusicView {
        lobby: false,
        phase,
        enemies,
        depth,
        boss,
        victory,
    }
}

// AC-02: Musikwechsel des Zustands-Automaten next_music_state: Tageszeit und Priorität.
#[test]
fn next_music_state_priority() {
    use MusicState::{Battle, Boss, Day, Depth, Dusk, Ending, Lobby, Night, None};
    let lobby = MusicView {
        lobby: true,
        ..view("night", 3, 0, true, true)
    };
    let cases = [
        ("Tag", view("day", 0, 0, false, false), Day),
        ("Abend", view("dusk", 0, 0, false, false), Dusk),
        ("Nacht", view("night", 0, 0, false, false), Night),
        ("Morgengrauen = Nacht", view("dawn", 0, 0, false, false), Night),
        ("Lobby vor allem", lobby, Lobby),
        ("Sieg", view("night", 3, 2, false, true), Ending),
        ("Boss", view("night", 3, 2, true, false), Boss),
        ("Kampf", view("night", 3, 2, false, false), Battle),
        ("Tiefe", view("night", 0, 2, false, false), Depth),
        ("unbekannte Phase = Lücke", view("?", 0, 0, false, false), None),
    ];
    for (name, v, want) in cases {
        assert_eq!(next_music_state(None, &v, false), want, "{name}");
    }
}

// AC-02: Übergänge aus einem Zustand: Niederlage und Sieg bleiben, bis die Lobby wieder läuft.
#[test]
fn next_music_state_sticky() {
    use MusicState::{Defeat, Ending, Lobby, Night};
    let day = view("day", 0, 0, false, false);
    let night = view("night", 0, 0, false, false);
    let lobby = MusicView { lobby: true, ..day };
    assert_eq!(next_music_state(Night, &night, false), Night, "anderes Ereignis");
    assert_eq!(
        next_music_state(Night, &view("night", 2, 0, false, false), true),
        Defeat,
        "Burg fällt"
    );
    assert_eq!(
        next_music_state(Defeat, &view("day", 0, 0, false, true), false),
        Defeat,
        "Niederlage bleibt"
    );
    assert_eq!(
        next_music_state(Defeat, &lobby, false),
        Lobby,
        "Lobby beendet Niederlage"
    );
    assert_eq!(next_music_state(Ending, &day, false), Ending, "Sieg bleibt");
}

fn run(d: &mut Fader, n: i64) {
    for _ in 0..n {
        d.step();
    }
}

// Überblenden über Takte: neues Stück steigt in CROSSFADE_TICKS auf 1, altes fällt auf 0; gleiche Leistung im Übergang.
#[test]
fn fader_crossfade() {
    let mut d = Fader::default();
    d.target("a");
    run(&mut d, CROSSFADE_TICKS);
    let v = d.voices();
    assert!(
        v[0].slot == "a" && v[0].gain == 1.0 && v[1].slot.is_empty(),
        "eingeblendet: {v:?}"
    );
    d.target("b");
    for i in 0..CROSSFADE_TICKS {
        d.step();
        let v = d.voices();
        if i < CROSSFADE_TICKS - 1 {
            let p = v[0].gain * v[0].gain + v[1].gain * v[1].gain;
            assert!((p - 1.0).abs() < 1e-9, "Takt {i}: Leistung {p}");
        }
    }
    let v = d.voices();
    assert!(
        v[0].slot == "b" && v[0].gain == 1.0 && v[1].slot.is_empty(),
        "nach dem Wechsel: {v:?}"
    );
    d.target("b");
    assert_eq!(d.voices()[0].gain, 1.0, "dasselbe Stück läuft einfach weiter");
}

// Wechsel mitten im Übergang: kein Sprung; kehrt das ausblendende Stück zurück, steigt es vom aktuellen Pegel.
#[test]
fn fader_retarget_without_jump() {
    let mut d = Fader::default();
    d.target("a");
    run(&mut d, CROSSFADE_TICKS);
    d.target("b");
    run(&mut d, CROSSFADE_TICKS / 2);
    let before = d.voices();
    d.target("a"); // zurück
    let after = d.voices();
    assert!(
        after[0].slot == "a" && (after[0].gain - before[1].gain).abs() <= 0.03,
        "Rückkehr: vorher {before:?}, nachher {after:?}"
    );
    assert!(
        after[1].slot == "b" && after[1].gain == before[0].gain,
        "b blendet vom aktuellen Pegel aus: vorher {before:?}, nachher {after:?}"
    );
    d.target("");
    let v = d.voices();
    assert!(
        v[0].slot.is_empty() && v[1].slot == "a",
        "Lücke: Stille, a blendet aus: {v:?}"
    );
}

// Ducking: sinkt, hält, steigt; überlappend verlängert; in der Rampe hinauf ohne Sprung.
#[test]
fn duck() {
    let near = |a: f64, b: f64| (a - b).abs() < 1e-9;
    assert_eq!(duck_gain(3.0, &DuckWindow::default()), 1.0, "ohne Warnung voll");
    let d = DuckShape {
        level: 0.4,
        attack: 0.3,
        hold: 1.0,
        release: 1.5,
    };
    let w = extend_duck(&DuckWindow::default(), 10.0, d);
    let checks = [
        (10.0, 1.0),
        (10.0 + d.attack / 2.0, f64::midpoint(1.0, d.level)),
        (10.0 + d.attack, d.level),
        (w.end, d.level),
        (w.end + d.release / 2.0, f64::midpoint(1.0, d.level)),
        (w.end + d.release, 1.0),
    ];
    for (t, want) in checks {
        let g = duck_gain(t, &w);
        assert!(near(g, want), "t={t}: {g}, erwartet {want}");
    }
    let o = extend_duck(
        &extend_duck(&extend_duck(&DuckWindow::default(), 0.0, d), 0.5, d),
        1.0,
        d,
    );
    assert!(o.start == 0.0 && near(o.end, 1.0 + d.hold), "verlängert: {o:?}");
    let mut x = 0.0;
    while x < 4.0 {
        assert!(duck_gain(x, &o) >= d.level - 1e-9, "t={x} unter level");
        x += 0.05;
    }
    let at = w.end + d.release / 2.0;
    let w2 = extend_duck(&w, at, d);
    assert!(
        near(duck_gain(at, &w2), duck_gain(at, &w)) && near(duck_gain(at + d.attack, &w2), d.level),
        "Sprung in der Rampe hinauf: {w2:?}"
    );
}
