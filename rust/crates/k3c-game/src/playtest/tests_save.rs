//! Tests von B-1221 (WZ66.2, AC-02): Ruckler-Automarker mit Schwelle und Entprellung, Spielstand am Pad-Marker.

use std::path::{Path, PathBuf};

use k3c_platform::{InputState, Key};

use super::Playtest;
use super::flag::extract;
use super::tests::{dir, frame, lines, lt_rt, strs};
use crate::view::frame::Frame;

#[test]
fn flag_schwelle_wird_gelesen() {
    let (o, rest) = extract(&strs(&["--playtest", "o", "--playtest-jank-ms", "40.5", "--w", "9"])).unwrap();
    let o = o.expect("Optionen");
    assert_eq!(o.jank_ms, 40.5);
    assert_eq!(rest, strs(&["--w", "9"]));
    let (o, _) = extract(&strs(&["--playtest=o"])).unwrap();
    assert_eq!(o.map(|o| o.jank_ms), Some(25.0));
    assert!(extract(&strs(&["--playtest", "o", "--playtest-jank-ms", "x"])).is_err());
}

/// Zwei Spieler im Spiel, Frame läuft, nicht pausiert, nicht beim Laden.
fn live_frame() -> Frame {
    frame()
}

#[test]
fn ruckler_schwelle_entprellung_und_uebersprungene() {
    let d = dir("jank");
    let mut pt = Playtest::open(&d);
    pt.step(&InputState::new(), Some(&live_frame()), 0);
    pt.frame_done(0, 16.0); // erstes Bild: nur Vorlauf
    pt.frame_done(10, 24.9); // unter der Schwelle
    assert!(lines(&d).is_empty());
    pt.frame_done(20, 41.0);
    pt.frame_done(1_000, 60.0); // gedrosselt
    pt.frame_done(2_000, 30.0); // gedrosselt
    pt.frame_done(5_100, 26.0);
    let l = lines(&d);
    assert_eq!(l.len(), 2);
    assert_eq!(l[0]["kind"], "ruckler");
    assert_eq!(l[0]["frame_ms"], 41.0);
    assert_eq!(l[0]["skipped"], 0);
    assert_eq!(l[0]["stage"], 0, "Kontext des ersten Spielers");
    assert!(l[0].get("save").is_none());
    assert_eq!(l[1]["skipped"], 2);
    assert_eq!(l[1]["worst_frame_ms"], 60.0);
}

#[test]
fn keine_ruckler_beim_laden_ohne_spiel_und_nach_dem_laden() {
    let d = dir("jank-laden");
    let mut pt = Playtest::open(&d).configure(25.0, None);
    // kein Spiel (Menü)
    pt.step(&InputState::new(), None, 0);
    pt.frame_done(0, 90.0);
    pt.frame_done(10, 90.0);
    // Laden
    let mut loading = live_frame();
    loading.loading = true;
    pt.step(&InputState::new(), Some(&loading), 20);
    pt.frame_done(20, 300.0);
    pt.frame_done(30, 300.0);
    // erstes Bild nach dem Laden zählt nicht, das zweite schon
    pt.step(&InputState::new(), Some(&live_frame()), 40);
    pt.frame_done(40, 200.0);
    assert!(lines(&d).is_empty());
    pt.frame_done(50, 200.0);
    assert_eq!(lines(&d).len(), 1);
}

#[test]
fn eigene_schwelle() {
    let d = dir("jank-schwelle");
    let mut pt = Playtest::open(&d).configure(50.0, None);
    pt.step(&InputState::new(), Some(&live_frame()), 0);
    pt.frame_done(0, 10.0);
    pt.frame_done(10, 41.0);
    assert!(lines(&d).is_empty());
    pt.frame_done(20, 51.0);
    assert_eq!(lines(&d).len(), 1);
}

/// Ordner der Spielstände des Spiels mit einer frischen Datei (so, wie das Schnellspeichern sie schreibt).
fn saves_src(d: &Path) -> PathBuf {
    let s = d.join("quelle");
    std::fs::create_dir_all(&s).unwrap();
    s
}

/// Läuft Takte, bis `n` Zeilen in `markers.jsonl` stehen (der Thread kopiert nebenbei), höchstens 3 s.
fn wait_lines(pt: &mut Playtest, d: &Path, n: usize) {
    for i in 0..300 {
        pt.step(&InputState::new(), Some(&live_frame()), 100_000 + i);
        if lines(d).len() >= n {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn pad_marker_legt_spielstand_ab_ruckler_nicht() {
    let d = dir("save");
    let src = saves_src(&d);
    let mut pt = Playtest::open(&d).configure(25.0, Some(src.clone()));
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    assert_eq!(pt.step(&input, Some(&live_frame()), 1_000).len(), 1);
    assert!(
        pt.game_input(&input).held(Key::F5),
        "das Spiel bekommt F5 (Schnellspeichern)"
    );
    std::fs::write(src.join("spiel.json"), b"{\"stand\":1}").unwrap(); // das Spiel speichert im selben Takt
    assert!(lines(&d).is_empty(), "Zeile erst nach dem Kopieren");
    wait_lines(&mut pt, &d, 1);
    let l = lines(&d);
    assert_eq!(l[0]["save"], "saves/marker-1.json");
    assert_eq!(std::fs::read(d.join("saves/marker-1.json")).unwrap(), b"{\"stand\":1}");
    assert!(
        !pt.game_input(&InputState::new()).held(Key::F5),
        "nur im Takt des Markers"
    );
    // Ruckler: kein Spielstand
    pt.frame_done(200_000, 50.0);
    pt.frame_done(200_010, 50.0);
    let l = lines(&d);
    assert!(
        l.iter()
            .filter(|v| v["kind"] == "ruckler")
            .all(|v| v.get("save").is_none())
    );
    assert!(!d.join("saves/marker-2.json").exists());
}

#[test]
fn zweiter_marker_zweite_datei_und_fehlschlag_ohne_save() {
    let d = dir("save2");
    let src = saves_src(&d);
    let mut pt = Playtest::open(&d).configure(25.0, Some(src.clone()));
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    pt.step(&input, Some(&live_frame()), 1_000);
    std::fs::write(src.join("a.json"), b"1").unwrap();
    wait_lines(&mut pt, &d, 1);
    // zweiter Marker, aber das Spiel speichert nicht (nichts Neues): Zeile ohne save, Spiel läuft weiter
    std::thread::sleep(std::time::Duration::from_millis(120));
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    pt.step(&input, Some(&live_frame()), 5_000);
    std::fs::remove_file(src.join("a.json")).unwrap();
    wait_lines(&mut pt, &d, 2);
    let l = lines(&d);
    assert_eq!(l.len(), 2);
    assert!(l[1].get("save").is_none());
}

#[test]
fn ohne_spielstand_ordner_oder_im_menue_kein_save() {
    let d = dir("nosave");
    let mut pt = Playtest::open(&d);
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    pt.step(&input, Some(&live_frame()), 1_000);
    assert!(!pt.game_input(&input).held(Key::F5));
    assert_eq!(lines(&d).len(), 1);
    assert!(!d.join("saves").exists());
    // pausiert: ebenfalls kein Auftrag, Zeile sofort
    let d2 = dir("nosave2");
    let mut pt = Playtest::open(&d2).configure(25.0, Some(saves_src(&d2)));
    let mut f = live_frame();
    f.paused = true;
    let mut input = InputState::new();
    lt_rt(&mut input, 7, true);
    pt.step(&input, Some(&f), 1_000);
    assert!(!pt.game_input(&input).held(Key::F5));
    assert_eq!(lines(&d2).len(), 1);
}
