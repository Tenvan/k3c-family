//! RU38.2: Aufzeichnung aus Go laden, JSON-Baum je Takt vergleichen, Wiedergabe headless mit Zwischenbildern.

use k3c_det::golden::{diff, testdata, tree};
use k3c_game::replay::ReplayApp;
use k3c_game::replay::trace::{load, parse, read_lines};
use k3c_platform::{NullCanvas, Runner, STEP, run_headless};
use serde_json::Value;

fn path() -> std::path::PathBuf {
    testdata().join("rust/trace/nacht-300.jsonl.gz")
}

#[test]
fn alle_takte_laden_und_sind_gleich_dem_json_aus_go() {
    let lines = read_lines(path()).expect("lesen");
    let t = parse(&lines).expect("laden");
    assert_eq!(t.rows.len(), 300, "Takt 0 (Startzustand) bis 299");
    assert_eq!(
        (t.head.szene.as_str(), t.head.seed.as_str(), t.head.spieler),
        ("nacht", "trace-nacht", 2)
    );
    let head: Value = serde_json::from_str(&lines[0]).unwrap();
    assert_eq!(diff("level", &head["level"], &tree(&t.head.level)), "");
    for (i, row) in t.rows.iter().enumerate() {
        let want: Value = serde_json::from_str(&lines[i + 1]).unwrap();
        assert_eq!(row.takt, i as u64);
        let at = format!("takt {i} zustand");
        assert_eq!(diff(&at, &want["zustand"], &tree(&row.zustand)), "", "{at}");
        let ev = tree(&row.ereignisse.clone().unwrap_or_default());
        assert_eq!(diff("ereignisse", &want["ereignisse"], &ev), "", "takt {i} ereignisse");
    }
}

#[test]
fn wiedergabe_headless_zeichnet_zwei_haelften() {
    let mut app = ReplayApp::new(load(path()).unwrap());
    let c = run_headless(&mut app, 300, None);
    assert!(app.playback().finished());
    assert_eq!(app.playback().pos(), 299);
    assert_eq!(c.clips, 3 * app.frames, "je Bild zwei Hälften und das Aufheben");
    assert!(c.rects > 2 * app.frames, "mehr als der Hintergrund");
    assert!(app.last_rects > 0);
}

#[test]
fn zwischenbilder_zeichnen_ohne_neuen_takt_und_stellen_wieder_her() {
    let mut app = ReplayApp::new(load(path()).unwrap());
    let mut c = NullCanvas::default();
    let mut r = Runner::new(&mut app, None);
    r.advance(STEP, &mut c);
    let (steps, rects) = (r.steps(), c.rects);
    assert_eq!(r.advance(STEP * 0.5, &mut c), 0, "kein neuer Takt");
    assert_eq!(r.steps(), steps);
    assert!(c.rects > rects, "Zwischenbild zeichnet");
    let pos = app.playback().pos();
    assert_eq!(pos as u64, steps);
    let want = load(path()).unwrap().rows[pos].zustand.clone();
    assert_eq!(app.playback().world(), Some(&want), "restore stellt die Sim wieder her");
}
