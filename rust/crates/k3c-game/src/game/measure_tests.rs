//! Tests zu `measure.rs`: `TestPercentile` aus `native/k3c-game/game/modes_test.go`, dazu Feldliste und Reihenfolge des
//! Berichts (Go hat keinen Test dafür), die Phasen des `Meter`, der Bericht aus dem Spiel und das Schreiben der Datei. Die
//! Datei entsteht im Ordner `.work/measure-test/` des Repos (ignoriert) und wird danach entfernt.

use std::path::Path;
use std::rc::Rc;

use k3c_sim::state::{Enemy, World};

use super::*;
use crate::game::play_testutil::play_game;
use crate::view::frame::Stage;

/// Die Felder des Go-`Report` (`measure.go`) in ihrer Reihenfolge; `bilder` kommt nur mit `--measure-frames`.
const GO_FIELDS: [&str; 28] = [
    "szene",
    "spieler",
    "ebitengine",
    "backend",
    "fenster",
    "vsync",
    "sheets",
    "textur_mib_rgba",
    "rig_mib_rgba",
    "messdauer_s",
    "frames",
    "fps_mittel",
    "fps_1pct_low",
    "frametime_ms",
    "rechenzeit_ms",
    "takt_ms",
    "takt_max_ms",
    "takte",
    "takt_ende",
    "phase",
    "gegner",
    "go_heap_mib",
    "go_sys_mib",
    "bilder_ohne_takt",
    "takt_abstand_ms",
    "gc_laeufe",
    "gc_pause_ms",
    "bilder",
];

/// Go: `TestPercentile`.
#[test]
fn percentile_wie_go() {
    let v = [1.0, 2.0, 3.0, 4.0, 100.0];
    assert!(
        percentile(&v, 0.5) == 3.0 && percentile(&v, 0.99) == 100.0,
        "Perzentile falsch"
    );
    assert!(
        percentile(&[], 0.5) == 0.0 && sum(&v) == 110.0,
        "leer oder Summe falsch"
    );
    // Rang wie Go: round((n-1)*p), gerundet weg von 0 (bei .5 nach oben)
    let w = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(percentile(&w, 0.5), 3.0);
    assert_eq!(percentile(&w, 0.95), 4.0);
}

#[test]
fn round1_rundet_auf_eine_stelle() {
    assert_eq!(round1(12.34), 12.3);
    assert_eq!(round1(12.35), 12.4);
    assert_eq!(round1(0.04), 0.0);
}

/// Steht die Zeile `  "name":` (zwei Leerzeichen, oberste Ebene) im Text, liefert sie die Stelle.
fn top_level(text: &str, name: &str) -> Option<usize> {
    text.find(&format!("\n  \"{name}\":"))
}

/// Alle Namen des Go-Berichts kommen in derselben Reihenfolge vor, `bilder` nur mit Wert.
#[test]
fn bericht_hat_alle_felder_in_der_reihenfolge_von_go() {
    let r = Report {
        bilder: Some(FrameReport::default()),
        ..Report::default()
    };
    let text = serde_json::to_string_pretty(&r).unwrap();
    let mut last = 0;
    for name in GO_FIELDS {
        let at = top_level(&text, name).unwrap_or_else(|| panic!("Feld {name} fehlt:\n{text}"));
        assert!(at > last, "Feld {name} steht an falscher Stelle");
        last = at;
    }
    assert_eq!(
        text.matches("\n  \"").count(),
        GO_FIELDS.len(),
        "unbekannte Felder:\n{text}"
    );
    let ohne = serde_json::to_string_pretty(&Report::default()).unwrap();
    assert!(top_level(&ohne, "bilder").is_none(), "bilder nur mit Wert");
    assert!(top_level(&ohne, "gc_pause_ms").is_some());
}

/// Die verschachtelten Kennwerte tragen `p50`, `p95`, `p99`, `max`; die Bilder die Namen von Go.
#[test]
fn verschachtelte_namen_wie_go() {
    let r = Report {
        bilder: Some(FrameReport::default()),
        ..Report::default()
    };
    let v = serde_json::to_value(&r).unwrap();
    for key in ["frametime_ms", "takt_abstand_ms"] {
        let names: Vec<&String> = v[key].as_object().unwrap().keys().collect();
        assert_eq!(names.len(), 4, "{key}");
        for n in ["p50", "p95", "p99", "max"] {
            assert!(v[key].get(n).is_some(), "{key}.{n}");
        }
    }
    let text = serde_json::to_string(&v["bilder"]).unwrap();
    for n in [
        "datei",
        "bilder",
        "verworfen",
        "median_ms",
        "p99_ms",
        "p999_ms",
        "max_ms",
        "lang",
        "lang_mit_gc",
        "lang_ohne_takt",
        "lang_mehr_takt",
        "ohne_takt",
        "mehr_takt",
        "mit_gc",
        "gc_laeufe",
    ] {
        assert!(text.contains(&format!("\"{n}\":")), "bilder.{n}");
    }
}

fn cfg(seconds: f64, warmup: f64) -> Measure {
    Measure {
        seconds,
        warmup,
        ..Measure::default()
    }
}

/// Aufwärmen (Phase 0), Messen (1), fertig (2); `done` und das Ende kommen erst nach `seconds` Messzeit.
#[test]
fn meter_durchlaeuft_die_phasen() {
    let mut m = Meter::new(&cfg(0.1, 0.05));
    let mut now = 10.0;
    let mut ended_at = None;
    for i in 0..40 {
        now += 1.0 / 60.0;
        // Takt wächst nur in jedem zweiten Bild (wie 60 Hz Bild bei 30 Hz Takt)
        if m.frame(now, 0.002, 0.5, i / 2) {
            ended_at = Some(i);
            break;
        }
        assert!(!m.done() || ended_at.is_some());
    }
    assert!(m.done(), "Messung nie beendet");
    assert!(ended_at.is_some_and(|i| i >= 9), "zu früh beendet: {ended_at:?}");
    assert!(m.frames.len() >= 5, "{} Bilder", m.frames.len());
    assert!(
        m.repeated > 0 && m.repeated <= m.frames.len(),
        "Bilder ohne Takt: {}",
        m.repeated
    );
    assert!((m.draw_ms / m.frames.len() as f64 - 2.0).abs() < 1e-9, "Draw-Mittel");
    let was = m.frames.len();
    m.frame(now + 1.0, 0.0, 0.0, 99); // nach dem Ende sammelt nichts mehr
    assert_eq!(m.frames.len(), was);
}

/// Takt-Zeiten und Abstände der Updates zählen nur in der Messung.
#[test]
fn meter_sammelt_takte_nur_im_messen() {
    let mut m = Meter::new(&cfg(5.0, 0.0));
    m.started(1.0);
    m.tick(0.003);
    assert!(m.ticks.is_empty() && m.gaps.is_empty(), "Aufwärmen sammelt nicht");
    m.frame(1.5, 0.0, 0.0, 1); // Warmup 0 → Messen
    m.started(2.0); // erstes Update der Messung: Abstand zum Update davor (1.0 s)
    m.tick(0.004);
    m.started(2.0334);
    m.tick(0.002);
    assert_eq!(m.ticks, [4.0, 2.0]);
    assert_eq!(m.gaps.len(), 2);
    assert!(
        (m.gaps[0] - 1000.0).abs() < 1e-9 && (m.gaps[1] - 33.4).abs() < 1e-6,
        "{:?}",
        m.gaps
    );
}

/// Der Bericht rechnet Frames, Mittel, Perzentile und Takte aus den Sammlungen und dem Stand des Spiels.
#[test]
fn bericht_rechnet_aus_den_sammlungen() {
    let (mut g, _) = play_game(Vec::new());
    g.args.scene.tick = 7;
    g.frame.tick = 42;
    let mut m = Meter::new(&cfg(1.0, 0.0));
    m.frames = (1..=100).map(f64::from).collect(); // 1 … 100 ms
    m.draw_ms = 200.0;
    m.ticks = vec![1.0, 3.0];
    m.repeated = 9;
    m.gaps = vec![33.0, 34.0, 80.0];
    let r = g.report(&m);
    assert_eq!(r.szene, "seed=t tick=7 spieler=2");
    assert_eq!((r.spieler, r.frames, r.takte, r.takt_ende), (2, 100, 2, 42));
    assert_eq!(r.messdauer_s, 5.05);
    assert_eq!(r.fps_mittel, 100.0 * 1000.0 / 5050.0);
    assert_eq!(r.fps_1pct_low, 1000.0 / 100.0); // schlechtestes 1 % = das längste Bild
    assert_eq!(
        r.frametime_ms,
        FrameStats {
            p50: 51.0,
            p95: 95.0,
            p99: 99.0,
            max: 100.0
        }
    );
    assert_eq!((r.rechenzeit_ms, r.takt_ms, r.takt_max_ms), (2.0, 2.0, 3.0));
    assert_eq!(r.bilder_ohne_takt, 9);
    assert_eq!(
        r.takt_abstand_ms,
        FrameStats {
            p50: 34.0,
            p95: 80.0,
            p99: 80.0,
            max: 80.0
        }
    );
    assert!(r.vsync && r.fenster == [1920, 1080], "{r:?}");
    assert_eq!(
        (r.go_heap_mib, r.go_sys_mib, r.gc_laeufe, r.gc_pause_ms),
        (0.0, 0.0, 0, 0.0)
    );
    assert_eq!((r.backend.as_str(), r.bilder.is_none()), ("wgpu", true));
    assert_eq!(r.sheets, 0, "ohne Grafik");
}

/// Ohne gemessene Bilder rechnet der Bericht nichts durch null (JSON kennt weder Unendlich noch NaN).
#[test]
fn bericht_ohne_bilder_ist_endlich() {
    let (g, _) = play_game(Vec::new());
    let r = g.report(&Meter::new(&cfg(1.0, 0.0)));
    assert!(
        r.fps_mittel == 0.0 && r.fps_1pct_low == 0.0 && r.messdauer_s == 0.0,
        "{r:?}"
    );
    assert!(serde_json::to_string(&r).unwrap().contains("\"fps_mittel\":0.0"));
}

/// `ebitengine` nennt Plattform und wgpu-Version; die Version steht fest in `rust/Cargo.toml`.
#[test]
fn engine_name_nennt_die_wgpu_version_der_workspace_datei() {
    let toml = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml")).unwrap();
    let line = toml
        .lines()
        .find(|l| l.starts_with("wgpu = "))
        .expect("wgpu in Cargo.toml");
    assert!(line.contains(&format!("\"={WGPU_VERSION}\"")), "{line}");
    let name = engine_name();
    assert!(
        name.starts_with("k3c-platform ") && name.ends_with(WGPU_VERSION),
        "{name}"
    );
}

/// Go: `worldState`: Tageszeit der ersten Stufe mit Welt und die lebenden Gegner aller Stufen.
#[test]
fn world_state_zaehlt_lebende_gegner() {
    let enemy = |hp| Enemy { hp, ..Enemy::default() };
    let mut w = World::default();
    w.cycle.phase = "night".to_string();
    w.enemies = vec![enemy(5.0), enemy(0.0), enemy(1.0)];
    let mut w2 = World::default();
    w2.cycle.phase = "day".to_string();
    w2.enemies = vec![enemy(2.0)];
    let mut f = Frame::default();
    assert_eq!(world_state(&f), (String::new(), 0));
    f.stages = vec![
        Stage::default(), // ohne Welt
        Stage {
            world: Some(Rc::new(w)),
            ..Stage::default()
        },
        Stage {
            world: Some(Rc::new(w2)),
            ..Stage::default()
        },
    ];
    assert_eq!(world_state(&f), ("night".to_string(), 3));
}

/// Der Bericht geht als JSON in die Datei `cfg.report`; ein Fehler beim Schreiben meldet `write_report`.
#[test]
fn bericht_wird_als_json_geschrieben() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/measure-test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bericht.json");
    let r = Report {
        szene: "seed=t tick=1 spieler=2".to_string(),
        frames: 3,
        fenster: [1920, 1080],
        ..Report::default()
    };
    let ok = write_report(&r, &path.to_string_lossy());
    let text = std::fs::read_to_string(&path);
    let fehl = write_report(&r, &dir.join("fehlt").join("x.json").to_string_lossy());
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(ok, Ok(()));
    let text = text.unwrap();
    assert!(
        text.starts_with("{\n  \"szene\": \"seed=t tick=1 spieler=2\""),
        "zwei Leerzeichen: {text}"
    );
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!((v["frames"].as_i64(), v["fenster"][1].as_i64()), (Some(3), Some(1080)));
    assert!(fehl.is_err());
}

/// Die Szene heißt wie in Go.
#[test]
fn scene_name_wie_go() {
    let s = Scene {
        seed: "abc".to_string(),
        tick: 90,
        players: 2,
        ..Scene::default()
    };
    assert_eq!(s.name(), "seed=abc tick=90 spieler=2");
}
