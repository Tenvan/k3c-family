//! Port von `native/k3c-game/game/stress_test.go` (acht Tests) plus Tests der Rust-Verdrahtung (Starter des Menüs,
//! zwei Spieler) und der Feldliste des Stressberichts. Die Schein-Simulation ist `FakeSim` mit Entwickler-Rolle; ihre
//! Aufrufe (neue Spiele, Löschen, Dev-Aktionen) stehen in `Probe`.

use std::cell::Cell as Counter;
use std::path::PathBuf;
use std::rc::Rc;

use serde_json::Value;

use super::*;
use crate::game::play_testutil::play_game_with;
use crate::game::testutil::{FakeSim, Probe, keys1, pad};
use crate::view::devkeys::DevAction;
use crate::view::frame::{Event, Seat, Stage};
use crate::view::session::SaveInfo;
use crate::view::stress::{GROWTH_EXP, GROWTH_LINEAR, STRESS_MAX_ENEMIES};
use k3c_sim::state::{Enemy, World};

/// Sekunde 0 der festen Uhr (2026-11-06T22:40:00 UTC).
pub(super) const T0: i64 = 1_794_000_000;

/// Ordner der Berichte eines Tests unter `.work/tmp` (Schreiben nur im Repo), leer.
pub(super) fn scratch_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work/tmp")
        .join(format!("stress-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Schleife mit Schein-Simulation samt Entwickler-Rolle, fester Uhr (Sekunden seit `T0` im Zähler), Bildrate 60 und
/// Berichtsordner `name` (Go: `stressGameOf`).
pub(super) fn stress_game_of(name: &str) -> (Game, Probe, Rc<Counter<i64>>, PathBuf) {
    let (sim, probe) = FakeSim::new(2);
    let (mut g, probe) = play_game_with(sim.with_dev(), probe, Vec::new());
    probe.games.borrow_mut().clear();
    let secs = Rc::new(Counter::new(0));
    let dir = scratch_dir(name);
    let s = Rc::clone(&secs);
    g.stress_env.clock = Some(Box::new(move || Zeit::from_unix(T0 + s.get(), 0)));
    g.stress_env.fps_of = Some(Box::new(|| 60.0));
    g.stress_env.reports = dir.to_string_lossy().to_string();
    (g, probe, secs, dir)
}

/// Frame mit Spieler 1 an der Tastatur und einer Stufe mit `enemies` Gegnern; `events` gelten für die Stufe 0, dazu ein
/// Ereignis `wave` der Stufe 1, das nicht zählt (Go: `stressFrame`).
#[allow(clippy::cast_precision_loss)]
pub(super) fn stress_frame(tick: u64, enemies: usize, events: &[&str]) -> Frame {
    let w = World {
        enemies: vec![Enemy::default(); enemies],
        ..World::default()
    };
    let event = |stage: i64, t: &str| Event {
        stage,
        data: serde_json::json!({ "type": t })
            .as_object()
            .cloned()
            .unwrap_or_default(),
        ..Event::default()
    };
    let mut f = Frame {
        tick,
        step_ms: (tick % 7) as f64,
        stages: vec![Stage {
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![Seat {
            player: 0,
            device: keys1(),
            ..Seat::default()
        }],
        ..Frame::default()
    };
    f.events = events.iter().map(|t| event(0, t)).collect();
    f.events.push(event(1, "wave"));
    f
}

pub(super) fn cfg() -> StressTest {
    StressTest {
        players: 2,
        depth: 1,
        seed: 42,
        enemies: 10,
        growth: GROWTH_LINEAR.to_string(),
        factor: 5.0,
        build: 3,
        troops: 20,
        timescale: 4,
        waves: 2,
        enemy: "goblin".to_string(),
        troop: "archer".to_string(),
        ..StressTest::default()
    }
}

pub(super) fn act(action: &str, kind: &str, unit: &str, amount: i64) -> DevAction {
    DevAction {
        action: action.to_string(),
        kind: kind.to_string(),
        unit: unit.to_string(),
        amount,
        ..DevAction::default()
    }
}

/// Takt `tick` mit `enemies` Gegnern und den Ereignissen `ev`.
pub(super) fn step_with(g: &mut Game, tick: u64, enemies: usize, ev: &[&str]) {
    g.frame = stress_frame(tick, enemies, ev);
    g.stress_step();
}

/// Die Namen der Dev-Aktionen, die ein Test bisher ausgelöst hat (Go: `devNames`).
pub(super) fn dev_names(p: &Probe) -> String {
    let names: Vec<String> = p
        .dev
        .borrow()
        .actions
        .iter()
        .map(|a| format!("{}:{}", a.action, a.kind))
        .collect();
    names.join(" ")
}

pub(super) fn reports_in(dir: &std::path::Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|d| d.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    out.retain(|p| {
        p.file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("native-stress-"))
    });
    out
}

/// B-896/AC-02: Ein gestarteter Test läuft mit den eingestellten Werten: Spiel mit Seed, Tiefe und Geräten, F2 und F10
/// an, im ersten Takt Ausbau, Truppen und Zeitraffer, zu jeder Welle zusätzliche Gegner nach der Kurve.
#[test]
fn stress_starts_with_config() {
    let (mut g, p, _, _) = stress_game_of("starts");
    let by = keys1();
    g.start_stress(&cfg(), by).unwrap();
    let games = p.games.borrow();
    assert_eq!(games.len(), 1, "{} Spiele", games.len());
    let ng = &games[0];
    assert!(
        ng.seed == "42" && ng.depth == 1 && ng.name == STRESS_SAVE,
        "Spiel {ng:?}"
    );
    assert_eq!(ng.devices.len(), 2);
    assert_eq!(ng.devices[0], by);
    assert_eq!(ng.devices[1], pad(STRESS_PAD_BASE + 1), "Platzhalter-Pad für Spieler 2");
    let set = p.set.borrow();
    assert!(set.len() == 1 && set[0].stats, "F2: {} Einstellungen", set.len());
    assert_eq!(g.dev.debug, DEBUG_VALUES, "F10");
    step_with(&mut g, 10, 0, &[]);
    step_with(&mut g, 11, 10, &[]);
    step_with(&mut g, 12, 10, &["wave"]);
    let want = vec![
        DevAction {
            level: 3,
            ..act("buildAll", "", "", 0)
        },
        act("spawn", "troop", "archer", 20),
        DevAction {
            factor: 4,
            ..act("timescale", "", "", 0)
        },
        act("wave", "", "", 0),
        act("spawn", "enemy", "goblin", 10),
        act("spawn", "enemy", "goblin", 15),
    ];
    assert_eq!(p.dev.borrow().actions, want);
    assert!(g.stress.is_some(), "Test endet, solange Gegner der letzten Welle leben");
}

/// Taktet zwei Wellen (Takt 110 und 130); ab Takt 151 sind alle Gegner geschlagen (Go: `runWaves`).
pub(super) fn run_waves(g: &mut Game, secs: &Counter<i64>) {
    let mut tick = 100;
    while tick < 160 && g.stress.is_some() {
        let mut ev: Vec<&str> = Vec::new();
        if tick == 110 || tick == 130 {
            ev.push("wave");
        }
        // Gegner der letzten Welle erscheinen erst nach dem Ereignis
        let enemies = if tick > 150 || tick == 130 || tick == 131 { 0 } else { 5 };
        secs.set(secs.get() + 1);
        step_with(g, tick, enemies, &ev);
        tick += 1;
    }
}

/// Liest den einzigen Bericht in `dir` als JSON-Wert.
pub(super) fn read_report(dir: &std::path::Path) -> Value {
    let files = reports_in(dir);
    assert_eq!(files.len(), 1, "Berichte {files:?}");
    let raw = std::fs::read_to_string(&files[0]).unwrap();
    serde_json::from_str(&raw).unwrap()
}

/// B-896/AC-03: Ist die letzte Welle geschlagen, liegt ein Bericht mit den Leistungswerten im Berichtsordner, die
/// Simulation steht und der Test ist vorbei.
#[test]
fn stress_writes_report() {
    let (mut g, p, secs, dir) = stress_game_of("report");
    g.start_stress(&cfg(), keys1()).unwrap();
    run_waves(&mut g, &secs);
    assert!(g.stress.is_none(), "Test läuft noch");
    assert!(p.dev.borrow().halt, "Simulation nicht angehalten");
    let r = read_report(&dir);
    let num = |k: &str| r[k].as_f64().unwrap_or(-1.0);
    assert_eq!(r["kind"], "native-stress");
    assert_eq!(r["endeUrsache"], "wellen");
    assert!(num("wellen") == 2.0 && num("takte") == 51.0, "Bericht {r}");
    assert!(num("takteJeSekunde") > 0.0 && num("fpsMittel") == 60.0, "Bericht {r}");
    assert!(r["taktdauerMs"]["max"].as_f64().unwrap_or(0.0) > 0.0, "Bericht {r}");
    assert!(
        r["taktdauerMs"]["p95"].as_f64() >= r["taktdauerMs"]["p50"].as_f64(),
        "Bericht {r}"
    );
    assert_eq!(r["hoechstwerte"]["gegner"], 5);
    assert_eq!(r["amEnde"]["gegner"], 0);
    assert_eq!(r["einstellungen"]["id"], "");
    assert_eq!(r["start"], Zeit::from_unix(T0, 0).format());
    let cues: Vec<&str> = g.cues.iter().map(|c| c.text.as_str()).collect();
    assert!(
        cues.iter().any(|t| t.starts_with("Stresstest beendet: ")),
        "Hinweise {cues:?}"
    );
}

/// Verlassen oder Spielende beenden den Test mit Bericht; ohne Entwickler-Rolle startet er nicht.
#[test]
fn stress_ends_on_leave_and_needs_dev() {
    let (mut g, p, _, dir) = stress_game_of("leave");
    g.start_stress(&cfg(), keys1()).unwrap();
    step_with(&mut g, 1, 0, &[]);
    p.active.set(false);
    g.stress_step();
    assert!(g.stress.is_none(), "Verlassen: läuft noch");
    assert_eq!(reports_in(&dir).len(), 1, "Berichte");
    assert!(!p.dev.borrow().halt, "Verlassen: angehalten");
    assert_eq!(read_report(&dir)["endeUrsache"], "verlassen");
    let (sim, probe) = FakeSim::new(2);
    let (mut plain, _) = play_game_with(sim, probe, Vec::new());
    assert_eq!(plain.start_stress(&cfg(), keys1()), Err(ERR_NO_DEV.to_string()));
}

/// Gegner je Welle: linear + Faktor, exponentiell × Faktor, gedeckelt bei 200.
#[test]
fn stress_enemies_for() {
    let lin = StressTest {
        enemies: 10,
        growth: GROWTH_LINEAR.to_string(),
        factor: 5.0,
        ..StressTest::default()
    };
    let exp = StressTest {
        enemies: 10,
        growth: GROWTH_EXP.to_string(),
        factor: 2.0,
        ..StressTest::default()
    };
    let cases = [
        (lin.enemies_for(0), 0),
        (lin.enemies_for(1), 10),
        (lin.enemies_for(3), 20),
        (exp.enemies_for(1), 10),
        (exp.enemies_for(3), 40),
        (exp.enemies_for(10), STRESS_MAX_ENEMIES),
    ];
    for (i, (got, want)) in cases.into_iter().enumerate() {
        assert_eq!(got, want, "Fall {i}");
    }
}

/// B-920/AC-01: Der Test löst die Wellen selbst aus (Dev-Aktion wave plus zusätzliche Gegner), ohne auf die erste Nacht
/// zu warten: die erste gleich nach dem Vorspann, die nächste erst nach der geschlagenen, nicht über `cfg.waves` hinaus.
#[test]
fn stress_triggers_waves_itself() {
    let (mut g, p, _, _) = stress_game_of("waves");
    let c = StressTest {
        players: 1,
        waves: 2,
        troops: 0,
        build: 0,
        timescale: 0,
        ..cfg()
    };
    g.start_stress(&c, keys1()).unwrap();
    // leer, Welle 1 kämpft, geschlagen, Welle 2 kämpft, geschlagen
    for (i, enemies) in [0, 3, 3, 0, 4, 0, 0].into_iter().enumerate() {
        step_with(&mut g, 10 + i as u64, enemies, &[]);
    }
    assert_eq!(dev_names(&p), "wave: spawn:enemy wave: spawn:enemy");
    let acts = p.dev.borrow().actions.clone();
    assert_eq!((acts[1].amount, acts[3].amount), (10, 15), "Gegner je Welle");
}

/// B-921/AC-01 bis AC-03: Im reinen Modus schaltet der Test zuerst `pure` ein, löst keine natürliche Welle aus und setzt
/// die Gegner an die Burg (`spawnCastle`); Truppen kommen wie eingestellt.
#[test]
fn stress_pure_mode() {
    let (mut g, p, _, _) = stress_game_of("pure");
    let c = StressTest {
        players: 1,
        troops: 20,
        build: 0,
        timescale: 0,
        pure: true,
        ..cfg()
    };
    g.start_stress(&c, keys1()).unwrap();
    for (i, enemies) in [0, 3, 0, 4, 0].into_iter().enumerate() {
        step_with(&mut g, 10 + i as u64, enemies, &[]);
    }
    assert_eq!(dev_names(&p), "pure: spawn:troop spawnCastle:enemy spawnCastle:enemy");
    let acts = p.dev.borrow().actions.clone();
    assert_eq!(acts[0].on, Some(true));
    assert_eq!((acts[1].amount, acts[2].amount, acts[3].amount), (20, 10, 15));
}

/// B-916: Ein vorhandener Stresstest-Spielstand wird ohne Rückfrage ersetzt; ist er der des laufenden Spiels, wird dieses
/// vorher beendet. Ein laufendes normales Spiel und andere Spielstände bleiben unberührt (`new_game` beendet es selbst).
#[test]
fn stress_replaces_old_save() {
    let save = |n: &str| SaveInfo {
        name: n.to_string(),
        ..SaveInfo::default()
    };
    for (running, closed) in [(STRESS_SAVE, true), ("spiel-1", false), ("", false)] {
        let (mut g, p, _, _) = stress_game_of("replace");
        *p.saves.borrow_mut() = vec![save("spiel-1"), save(STRESS_SAVE)];
        *p.running.borrow_mut() = running.to_string();
        p.active.set(true);
        p.closed.set(false);
        g.start_stress(&cfg(), keys1()).unwrap();
        let (deleted, games) = (p.deleted.borrow().clone(), p.games.borrow().len());
        assert!(
            deleted == [STRESS_SAVE] && p.closed.get() == closed && games == 1,
            "läuft {running:?}: gelöscht {deleted:?}, geschlossen {}, Spiele {games}",
            p.closed.get()
        );
    }
    let (mut g, p, _, _) = stress_game_of("replace-none");
    *p.saves.borrow_mut() = vec![save("spiel-1")];
    assert_eq!(g.start_stress(&cfg(), keys1()), Ok(()));
    assert!(p.deleted.borrow().is_empty() && p.games.borrow().len() == 1);
}
