//! Port von `native/k3c-game/devtools/chronicle_test.go` (`TestChronicleQuietWithoutChange`,
//! `TestChronicleNightAndEconomy`, `TestChronicleSlowTicks`, `TestChronicleReport`), dazu Hilfen und die Zeit.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use k3c_platform::log::{Level, line};
use k3c_sim::state::{Event as SimEvent, Player, Troop, World};
use serde_json::json;

use super::chronicle::{Chronicle, LogSink, outcome_name};
use super::chronicle_stage::{num, round1};
use super::report::{Report, Zeit, sanitize};
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Event, Frame, Outcome, Seat, Stage};
use crate::view::layer::Listener;
use crate::view::roles::Finisher;

/// Lines sammelt die Logzeilen (statt `k3c_platform::log`), Go: `slog` in einen Puffer.
#[derive(Clone, Default)]
struct Lines(Rc<RefCell<Vec<String>>>);

impl Lines {
    fn push(&self, level: Level, msg: &str, fields: &[(&str, String)]) {
        let f: Vec<(&str, &dyn std::fmt::Display)> =
            fields.iter().map(|(k, v)| (*k, v as &dyn std::fmt::Display)).collect();
        self.0.borrow_mut().push(line(level, msg, &f));
    }

    fn all(&self) -> Vec<String> {
        self.0.borrow().clone()
    }

    fn text(&self) -> String {
        self.all().join("\n")
    }
}

impl LogSink for Lines {
    fn info(&self, msg: &str, fields: &[(&str, String)]) {
        self.push(Level::Info, msg, fields);
    }

    fn warn(&self, msg: &str, fields: &[(&str, String)]) {
        self.push(Level::Warn, msg, fields);
    }
}

/// days_from_civil rechnet (Jahr, Monat, Tag) in Tage seit 1970-01-01 (Gegenstück zu `civil_from_days`).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719_468
}

/// scratch_dir liefert einen leeren Ordner unter `.work/tmp` (Schreiben nur im Repo).
fn scratch_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work/tmp")
        .join(format!("chronicle-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// chronicle_for liefert eine Chronik, die in `Lines` loggt und Berichte nach dir schreibt, mit fester Uhr
/// (2026-11-14T20:00:00 UTC, jeder Aufruf eine Minute später).
fn chronicle_for(dir: &str) -> (Chronicle, Lines) {
    let lines = Lines::default();
    let t0 = days_from_civil(2026, 11, 14) * 86_400 + 20 * 3600;
    let n = Cell::new(0_i64);
    let now = move || {
        n.set(n.get() + 1);
        Zeit::from_unix(t0 + n.get() * 60, 0)
    };
    (Chronicle::with(Box::new(lines.clone()), Box::new(now), dir), lines)
}

/// chron_frame ist ein Frame mit einem Spieler in Stufe 0 einer Tag-Welt.
fn chron_frame(tick: u64) -> Frame {
    let w = World {
        seed: "wald".to_string(),
        cycle: k3c_sim::state::CycleInfo {
            phase: "day".to_string(),
            day: 1,
            ..Default::default()
        },
        troops: vec![
            Troop {
                id: 1,
                ..Default::default()
            },
            Troop {
                id: 2,
                ..Default::default()
            },
        ],
        players: vec![Player {
            index: 0,
            gold: 10,
            ..Default::default()
        }],
        ..World::default()
    };
    Frame {
        tick,
        seats: vec![Seat {
            player: 0,
            monarch: 0,
            stage: 0,
            ..Default::default()
        }],
        stages: vec![Stage {
            index: 0,
            world: Some(Rc::new(w)),
            ..Default::default()
        }],
        ..Frame::default()
    }
}

fn world_mut(f: &mut Frame) -> &mut World {
    Rc::make_mut(f.stages[0].world.as_mut().expect("Welt"))
}

fn ev(v: serde_json::Value) -> SimEvent {
    v.as_object().cloned().expect("Ereignis ist ein Objekt")
}

/// AC-06 (B-889): Ohne Wechsel bleibt das Log leer, auch über viele Takte; ein Tageszeitwechsel steht genau einmal da.
#[test]
fn chronicle_quiet_without_change() {
    let (mut c, log) = chronicle_for("");
    let mut f = chron_frame(1);
    for i in 0..300 {
        f.tick = i + 1;
        c.listen(&f);
    }
    assert!(log.all().is_empty(), "Log ohne Wechsel: {}", log.text());
    world_mut(&mut f).cycle.phase = "dusk".to_string();
    c.listen(&f);
    c.listen(&f);
    let l = log.all();
    assert!(
        l.len() == 1 && l[0].contains("🚦 Tageszeit") && l[0].contains("phase=dusk"),
        "Wechsel: {l:?}"
    );
}

fn send(c: &mut Chronicle, f: &mut Frame, data: Vec<SimEvent>) {
    f.events = data
        .into_iter()
        .map(|d| Event {
            stage: 0,
            tick: 0,
            data: d,
        })
        .collect();
    f.tick += 1;
    c.listen(f);
}

/// AC-06: Eine Nacht fasst gespawnt, getötet, Verluste und Schaden je Seite in einer Zeile zusammen; Gold-Änderungen
/// stehen am Tageswechsel je Spieler; Bau fertig und Burgfall sofort.
#[test]
fn chronicle_night_and_economy() {
    let (mut c, log) = chronicle_for("");
    let mut f = chron_frame(1);
    c.listen(&f);
    {
        let w = world_mut(&mut f);
        w.cycle.phase = "night".to_string();
        w.players[0].gold = 25;
    }
    let night = vec![
        ev(json!({"type": "night"})),
        ev(json!({"type": "wave", "count": 6})),
        ev(json!({"type": "built", "kind": "wall"})),
    ];
    send(&mut c, &mut f, night);
    world_mut(&mut f).players[0].gold = 20;
    let fight = vec![
        ev(json!({"type": "kill"})),
        ev(json!({"type": "kill"})),
        ev(json!({"type": "hit", "target": "enemy", "damage": 4.0})),
        ev(json!({"type": "hit", "target": "troop", "damage": 2.5})),
        ev(json!({"type": "castleFallen"})),
    ];
    send(&mut c, &mut f, fight);
    {
        let w = world_mut(&mut f);
        w.troops.truncate(1);
        w.cycle.phase = "dawn".to_string();
        w.cycle.day = 2;
    }
    send(&mut c, &mut f, vec![ev(json!({"type": "dawn"}))]);
    let text = log.text();
    for want in [
        "🚦 Bau fertig",
        "art=wall",
        "🚦 Burgfall",
        "🏹 Kampf der Nacht",
        "gespawnt=6",
        "getötet=2",
        "verluste=1",
        "schaden_an_gegnern=4",
        "schaden_eigene_seite=2.5",
        "🪙 Wirtschaft des Tags",
        "einnahmen=15",
        "ausgaben=5",
    ] {
        assert!(text.contains(want), "fehlt {want:?} in\n{text}");
    }
    assert!(
        text.matches('🏹').count() == 1 && text.matches('🪙').count() == 1,
        "Zusammenfassungen mehrfach:\n{text}"
    );
}

/// AC-06: Ein Takt über Budget loggt mit Objektanzahl, weitere langsame Takte im Fenster nicht.
#[test]
fn chronicle_slow_ticks() {
    let (mut c, log) = chronicle_for("");
    let mut f = chron_frame(100);
    f.step_ms = c.vals.step_budget_ms + 1.0;
    for i in 0..20 {
        f.tick = 100 + i;
        c.listen(&f);
    }
    let l = log.all();
    assert!(
        l.len() == 1 && l[0].contains("🐢 Takt über Budget") && l[0].contains("truppen=2"),
        "langsame Takte: {l:?}"
    );
    f.tick += (c.vals.slow_log_every_seconds * 30.0) as u64;
    c.listen(&f);
    assert!(
        log.text().contains("langsame_takte=20"),
        "zweite Meldung: {}",
        log.text()
    );
}

/// AC-06: Am Spielende liegt einmal der Sitzungsbericht als JSON in reports/; Programmende ohne Spiel schreibt nichts.
#[test]
fn chronicle_report() {
    let dir = scratch_dir("report");
    let (mut c, log) = chronicle_for(dir.to_str().expect("Pfad"));
    let mut f = chron_frame(10);
    c.listen(&f);
    f.tick = 70;
    world_mut(&mut f).cycle.day = 3;
    c.listen(&f);
    f.outcome = Outcome::Lost;
    c.listen(&f);
    c.listen(&Frame {
        outcome: Outcome::Lost,
        ..Frame::default()
    });
    c.finish();
    let files: Vec<_> = std::fs::read_dir(&dir)
        .expect("Ordner")
        .map(|e| e.expect("Eintrag").path())
        .collect();
    assert_eq!(files.len(), 1, "Berichte: {files:?}");
    let name = files[0].file_name().and_then(|n| n.to_str()).expect("Name").to_string();
    assert_eq!(name, "native-session-2026-11-14T20-02-00-t60.json");
    let raw = std::fs::read_to_string(&files[0]).expect("lesen");
    let want = "{\n  \"kind\": \"native-session\",\n  \"seed\": \"wald\",\n  \"start\": \"2026-11-14T20:01:00\",\n  \
                \"end\": \"2026-11-14T20:02:00\",\n  \"dauerSekunden\": 60,\n  \"takte\": 60,\n  \"ersterTag\": 1,\n  \
                \"letzterTag\": 3,\n  \"tage\": 3,\n  \"endeUrsache\": \"verloren\"\n}";
    assert_eq!(raw, want);
    let r: Report = serde_json::from_str(&raw).expect("JSON");
    assert!(r.seed == "wald" && r.days == 3 && r.cause == "verloren" && r.ticks == 60 && r.seconds > 0.0);
    assert!(log.text().contains("💾 Sitzungsbericht"), "Log: {}", log.text());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Programmende mit laufender Sitzung schreibt den Bericht mit Ursache `beendet`; ohne Ordner (kein Grafik-Bibliothek) nie.
#[test]
fn chronicle_finish_on_exit() {
    let dir = scratch_dir("beendet");
    let (mut c, _) = chronicle_for(dir.to_str().expect("Pfad"));
    c.listen(&chron_frame(5));
    c.finish();
    c.finish();
    let files: Vec<_> = std::fs::read_dir(&dir)
        .expect("Ordner")
        .map(|e| e.expect("Eintrag").path())
        .collect();
    assert_eq!(files.len(), 1, "Berichte: {files:?}");
    assert!(
        std::fs::read_to_string(&files[0])
            .expect("lesen")
            .contains("\"endeUrsache\": \"beendet\"")
    );
    let _ = std::fs::remove_dir_all(&dir);

    let (mut c, log) = chronicle_for("");
    c.listen(&chron_frame(5));
    c.finish();
    assert!(log.all().is_empty());
}

/// B-889 Ausnahmefall: Ist `reports/` nicht beschreibbar (hier: der Pfad ist eine Datei), steht eine 💥-Warnung im
/// Log, `finish` kehrt zurück und die Chronik läuft weiter (kein zweiter Versuch, kein Absturz).
#[test]
fn chronicle_report_dir_not_writable() {
    let blocker = scratch_dir("blockiert");
    std::fs::create_dir_all(blocker.parent().expect("Elternordner")).expect("Ordner");
    std::fs::write(&blocker, "Datei statt Ordner").expect("Datei");
    let (mut c, log) = chronicle_for(blocker.to_str().expect("Pfad"));
    c.listen(&chron_frame(5));
    c.finish();
    let all = log.text();
    assert!(all.contains("💥 Sitzungsbericht nicht geschrieben"), "Log: {all}");
    c.finish();
    c.listen(&chron_frame(6));
    assert_eq!(
        all.matches("💥").count(),
        log.text().matches("💥").count(),
        "nur eine Warnung"
    );
    let _ = std::fs::remove_file(&blocker);
}

/// Der Aufnahme-Modus loggt und schreibt nichts.
#[test]
fn chronicle_silent_when_recording() {
    let (mut c, log) = chronicle_for("");
    let mut f = chron_frame(1);
    f.recording = true;
    c.listen(&f);
    world_mut(&mut f).cycle.phase = "night".to_string();
    c.listen(&f);
    assert!(log.all().is_empty());
}

#[test]
fn round_num_and_names() {
    assert_eq!(round1(2.54), 2.5);
    assert_eq!(round1(2.56), 2.6);
    assert_eq!(round1(0.04), 0.0);
    assert_eq!(num(Some(&json!(3))), 3.0);
    assert_eq!(num(Some(&json!("x"))), 0.0);
    assert_eq!(num(None), 0.0);
    assert_eq!(outcome_name(Outcome::Won), "gewonnen");
    assert_eq!(outcome_name(Outcome::Lost), "verloren");
    assert_eq!(outcome_name(Outcome::None), "verlassen");
    assert_eq!(sanitize("a-12:30:05.json"), "a-12-30-05.json");
    assert_eq!(MAX_PLAYERS, 2);
}

#[test]
fn zeit_matches_known_instants() {
    assert_eq!(Zeit::from_unix(0, 0).format(), "1970-01-01T00:00:00");
    assert_eq!(Zeit::from_unix(951_782_400, 0).format(), "2000-02-29T00:00:00");
    assert_eq!(Zeit::from_unix(1_000_000_000, 0).format(), "2001-09-09T01:46:40");
    let t = Zeit::from_unix(days_from_civil(2026, 11, 14) * 86_400 + 20 * 3600, 0);
    assert_eq!(t.format(), "2026-11-14T20:00:00");
    let later = Zeit::from_unix(t.unix_secs + 90, 500_000_000);
    assert!((later.since(&t) - 90.5).abs() < 1e-9);
    assert!(Zeit::now().year >= 2026);
}
