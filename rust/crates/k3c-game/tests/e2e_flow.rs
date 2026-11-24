//! Ende-zu-Ende-Ablauf ohne Fenster gegen das Rust-Spiel (RU32.2, AC-01), Gegenstück zu
//! `native/k3c-game/e2e/flow_test.go`: Hauptmenü, Lobby mit zwei Geräten, Spiel (laufen, Münzen, Skill, bauen, Nacht),
//! Pause, Spiel verlassen, Weiter, Niederlage durch Kronenraub, Hauptmenü. Die Eingabe-Folge kommt aus der Go-Referenz
//! `testdata/rust/e2e/flow.json` (beide Spiele bekommen dieselben Eingaben), je Station stimmen Name und Takt, und der
//! kanonische Welt-Hash wird mit Go verglichen (Abweichungen: `docs/messung/rust-bildvergleich.md`). Zwei Läufe
//! müssen dieselben Stationen liefern. Spielstände liegen unter `.work/` (nie `%TEMP%`).
//!
//! Abweichung zu Go: Ob eine Sitzung läuft (`r.sim.Active()`), lesen die Tests an `Frame::stages` ab (ohne Spiel
//! leer); den Bildschirm (`*menu.mainScreen` usw.) an `Mode`, `screen()` und `Frame::paused`. Die Gegenprobe von Go,
//! den Spielstand direkt über `engine/room` zu laden, entfällt: Der Welt-Hash der Station `weiter` steht in der
//! Referenz.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::flow::{Run, Station, enemy_near, has_cue, has_event};
use k3c_det::golden;
use k3c_game::game::stack::Mode;
use k3c_game::view::frame::{CueKind, END_DELAY, Outcome};
use k3c_game::view::input::DeviceKind;
use serde_json::Value;

// Takte der Eingabe-Folge (wie `flow_test.go`).
const START_AT: u32 = 20;
const PAUSE_AT: u32 = 15900;
const LEAVE_AT: u32 = 15912;
const CONTINUE_AT: u32 = 15916;
const REJOIN_AT: u32 = 15920;
const MAX_TICKS: u32 = 60000;
const MENU_AFTER: u32 = 20;
const SAVE_NAME: &str = "spiel-1";

/// Die Go-Referenz (`flow.json`).
struct Reference {
    script: String,
    found: u32,
    stations: Vec<Station>,
}

fn reference() -> Reference {
    let v: Value = golden::read(golden::testdata().join("rust/e2e/flow.json"));
    let num = |x: &Value| u32::try_from(x.as_u64().expect("Zahl")).expect("u32");
    let stations = v["stations"]
        .as_array()
        .expect("stations")
        .iter()
        .map(|s| Station {
            name: s["name"].as_str().expect("name").to_string(),
            tick: num(&s["tick"]),
            hash: s["hash"].as_str().expect("hash").to_string(),
        })
        .collect();
    Reference {
        script: v["script"].as_str().expect("script").to_string(),
        found: num(&v["found"]),
        stations,
    }
}

/// Fährt den ganzen Ablauf mit der Eingabe-Folge der Referenz und prüft die Bedingungen jeder Station.
fn flow(tag: &str, r: &Reference) -> Run {
    let mut run = Run::new(tag, &["--flow", "--input", &r.script]);
    menus(&mut run);
    day(&mut run, r.found);
    night(&mut run);
    let at = pause_and_leave(&mut run);
    continue_game(&mut run, at);
    defeat(&mut run);
    run
}

/// Hauptmenü, Lobby mit zwei Geräten, Start mit zwei Spielern.
fn menus(r: &mut Run) {
    r.to(1);
    assert!(r.in_main_menu(), "Hauptmenü oben, kein Spiel");
    r.mark("hauptmenue");
    r.until("Spieler 2 tritt der Lobby bei", 10, |f| has_cue(f, CueKind::Joined));
    assert!(r.screen(), "Lobby liegt oben");
    r.to(START_AT);
    r.mark("lobby");
    r.to(START_AT + 1);
    assert!(!r.screen(), "Spiel läuft ohne Menü");
    let f = r.frame();
    assert_eq!(f.seats.len(), 2, "Plätze");
    assert_eq!(f.stages.len(), 1, "Stufen");
    assert_eq!(f.stages[0].level, "forest");
    let kinds: Vec<_> = f.seats.iter().map(|x| x.device.kind).collect();
    assert_eq!(kinds, [DeviceKind::Keyboard, DeviceKind::Gamepad], "Geräte der Plätze");
    r.mark("spielstart");
}

/// Laufen, Münzen geben, Skill-Punkt holen und lernen, Mauer gebaut.
fn day(r: &mut Run, found: u32) {
    let x1 = r.frame().monarch(1).expect("Spieler 2").x;
    let (x0, gold) = {
        let m = r.frame().monarch(0).expect("Spieler 1");
        (m.x, m.gold)
    };
    r.until("Spieler 1 gibt Münzen", 700, |f| has_event(f, "coinGive", 0));
    r.mark("muenzen");
    r.until("Spieler 2 findet einen Skill-Punkt", found, |f| {
        has_event(f, "skillPoint", 1)
    });
    let x = r.frame().monarch(1).expect("Spieler 2").x;
    assert!(x > x1, "laufen: Spieler 2 {x1:.1} -> {x:.1}");
    r.mark("skillpunkt");
    let menu = found + MENU_AFTER;
    r.to(menu + 4);
    r.mark("skillmenue");
    r.to(menu + 20);
    // Spieler 2 lernt über das Skill-Menü genau einen Skill (B-1106); `deviations` muss am Ende leer sein.
    let learned = r.frame().monarch(1).is_some_and(|m| m.skills.len() == 1);
    r.soft(learned, "Spieler 2 lernt keinen Skill (B-1106)");
    r.until("Mauer gebaut", 3000, |f| has_event(f, "built", -1));
    r.mark("gebaut");
    let m0 = r.frame().monarch(0).expect("Spieler 1");
    assert!(
        m0.x >= x0 + 40.0 && m0.gold < gold,
        "laufen/zahlen: Spieler 1 {x0:.1} -> {:.1} (Gold {gold} -> {})",
        m0.x,
        m0.gold
    );
}

/// Dämmerung, Nacht mit Welle, Gegner bei den Monarchen.
fn night(r: &mut Run) {
    r.until("Dämmerung", PAUSE_AT, |f| has_event(f, "dusk", -1));
    r.until("Nacht", PAUSE_AT, |f| has_event(f, "night", -1));
    assert!(has_event(r.frame(), "wave", -1), "Nacht ohne Welle");
    r.until("Gegner bei den Monarchen", PAUSE_AT, |f| enemy_near(f, 12.0));
    r.mark("nacht");
}

/// Pause hält die Simulation an, „Spiel verlassen“ speichert und führt ins Hauptmenü.
fn pause_and_leave(r: &mut Run) -> f64 {
    r.to(PAUSE_AT + 1);
    assert!(r.screen() && r.frame().paused, "Pause-Menü oben, Spiel pausiert");
    let tick = r.frame().tick;
    let at = r.frame().stages[0].world.as_deref().expect("Welt").time;
    r.mark("pause");
    r.to(LEAVE_AT);
    assert_eq!(r.frame().tick, tick, "Simulation lief in der Pause weiter");
    r.to(LEAVE_AT + 1);
    assert!(r.in_main_menu(), "Hauptmenü nach Spiel verlassen");
    assert!(has_cue(r.frame(), CueKind::Saved), "Spiel verlassen speichert");
    assert!(
        r.dir.join(format!("{SAVE_NAME}.json")).is_file(),
        "Spielstand {SAVE_NAME}.json fehlt in {}",
        r.dir.display()
    );
    r.mark("verlassen");
    at
}

/// Weiter lädt denselben Stand, Spieler 2 tritt wieder bei.
fn continue_game(r: &mut Run, at: f64) {
    r.to(CONTINUE_AT + 1);
    assert!(!r.screen(), "Weiter: Spiel läuft");
    let f = r.frame();
    assert_eq!((f.seats.len(), f.stages.len()), (1, 1), "Weiter: Plätze und Stufen");
    let t = f.stages[0].world.as_deref().expect("Welt").time;
    assert!(
        (t - (at + 1.0 / 30.0)).abs() < 1e-6,
        "Weiter: Spielzeit {t} statt {at} + 1 Takt"
    );
    r.mark("weiter");
    r.until("Spieler 2 spielt wieder mit", REJOIN_AT + 3, |f| f.seats.len() == 2);
    r.mark("zu-zweit");
}

/// Ein Gegner trägt die Krone von Spieler 2 ins Portal (Kronenraub); nach `END_DELAY` Takten liegt das Hauptmenü oben.
fn defeat(r: &mut Run) {
    r.until("Niederlage", MAX_TICKS, |f| f.outcome == Outcome::Lost);
    r.mark("niederlage");
    let end = r.tick + u32::try_from(END_DELAY).unwrap() + 3;
    r.until("Hauptmenü nach Spielende", end, |f| f.stages.is_empty());
    assert_eq!(r.g.mode(), Mode::Menu);
    assert_eq!(r.frame().outcome, Outcome::Lost, "Ausgang im Hauptmenü");
    r.to(r.tick + 2);
    assert!(r.in_main_menu(), "Hauptmenü nach Spielende");
    r.mark("ende");
}

fn describe(ss: &[Station]) -> String {
    ss.iter()
        .map(|s| format!("{} @{} {}", s.name, s.tick, s.hash))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Der Ablauf läuft zweimal gleich, und Name und Takt jeder Station stimmen mit Go überein.
#[test]
#[ignore = "Referenz aus RU32 (Go-Zeit) veraltet: Spiel hat sich seitdem geändert, Takte nicht mehr gültig; neu aufnehmen (Ticket B-1258)"]
fn ablauf_wie_go_zwei_laeufe_gleich() {
    let r = reference();
    assert_eq!(r.stations.len(), 14);
    let a = flow("a", &r);
    let b = flow("b", &r);
    let _ = std::fs::remove_dir_all(&a.dir);
    let _ = std::fs::remove_dir_all(&b.dir);
    assert_eq!(
        a.stations,
        b.stations,
        "zwei Läufe weichen ab:\n{}\n--\n{}",
        describe(&a.stations),
        describe(&b.stations)
    );
    let names = |ss: &[Station]| ss.iter().map(|s| (s.name.clone(), s.tick)).collect::<Vec<_>>();
    assert_eq!(
        names(&a.stations),
        names(&r.stations),
        "Stationen und Takte weichen von Go ab"
    );
    let diverged: Vec<&str> = a
        .stations
        .iter()
        .zip(&r.stations)
        .filter(|(x, y)| x.hash != y.hash)
        .map(|(x, _)| x.name.as_str())
        .collect();
    assert!(
        diverged.is_empty(),
        "Welt-Hash gegen Go: abweichende Stationen {diverged:?}; Hashes:
{}
Go:
{}",
        describe(&a.stations),
        describe(&r.stations)
    );
    assert!(a.deviations.is_empty(), "weiche Bedingungen: {:?}", a.deviations);
}
