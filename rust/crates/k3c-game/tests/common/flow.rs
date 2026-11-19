//! Treiber des Ende-zu-Ende-Laufs (RU32.2), Gegenstück zu `native/k3c-game/e2e/run_test.go`: die echte Schleife
//! (`build_game` mit allen Fachpaketen, ohne Fenster und ohne Zeichenfläche) und die echte `local::Session` im
//! Aufnahme-Modus (Speichern im Takt, feste Uhr), getaktet über `Game::step` wie im Fenster.

use super::canon::worlds_hash;
use k3c_det::golden;
use k3c_game::game::launch::{build_game, parse};
use k3c_game::game::stack::Mode;
use k3c_game::game::{Game, Libs};
use k3c_game::local::config::Config;
use k3c_game::local::session::Session;
use k3c_game::view::frame::{CueKind, Frame};
use k3c_platform::{App, InputState, STEP};
use k3c_room::store::Stamp;
use serde_json::Value;
use std::path::PathBuf;

/// Uhr der Spielstände: 2026-01-01 12:00 UTC (Go: `fixedNow`).
pub const FIXED_NOW_SECS: i64 = 1_767_268_800;

/// Eine Station des Ablaufs: Name, Takt (Zahl der Updates seit dem Start) und kanonischer Welt-Hash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Station {
    pub name: String,
    pub tick: u32,
    pub hash: String,
}

/// Ein Lauf des Ablaufs.
pub struct Run {
    pub g: Game,
    pub dir: PathBuf,
    /// Zahl der Updates seit dem Start
    pub tick: u32,
    pub stations: Vec<Station>,
    /// Bekannte Abweichungen von Go (Meldung je verfehlter weicher Bedingung, `soft`)
    pub deviations: Vec<String>,
}

/// Wegwerf-Ordner der Spielstände unter `.work/` (nie `%TEMP%`); `tag` trennt Läufe im selben Prozess.
pub fn saves_dir(tag: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}/../../../.work/e2e-{}-{tag}",
        env!("CARGO_MANIFEST_DIR"),
        std::process::id()
    ))
}

impl Run {
    /// Baut das Spiel wie `main` (ohne Fenster) mit den Schaltern `argv` und der festen Uhr.
    pub fn new(tag: &str, argv: &[&str]) -> Run {
        let dir = saves_dir(tag);
        let _ = std::fs::remove_dir_all(&dir);
        let mut rest: Vec<String> = argv.iter().map(|s| (*s).to_string()).collect();
        rest.extend(["--saves".to_string(), dir.to_string_lossy().into_owned()]);
        let args = parse(&rest).expect("Schalter");
        let session = Session::open(Config {
            dir: dir.to_string_lossy().into_owned(),
            now: Some(Box::new(|| Stamp::new(FIXED_NOW_SECS, 0))),
            recording: true,
            ..Config::default()
        });
        let g = build_game(&args, session, &Libs::default()).expect("Spiel");
        Run {
            g,
            dir,
            tick: 0,
            stations: Vec::new(),
            deviations: Vec::new(),
        }
    }

    /// Der Frame des letzten Updates.
    pub fn frame(&self) -> &Frame {
        self.g.frame()
    }

    /// Rechnet ein Update.
    pub fn step(&mut self) {
        self.g.step(&InputState::default(), STEP);
        self.tick += 1;
    }

    /// Rechnet bis zum Takt `tick`.
    pub fn to(&mut self, tick: u32) {
        while self.tick < tick {
            self.step();
        }
    }

    /// Rechnet, bis `cond` gilt; spätestens beim Takt `limit` bricht der Test mit `what` ab.
    pub fn until(&mut self, what: &str, limit: u32, cond: impl Fn(&Frame) -> bool) {
        while !cond(self.frame()) {
            assert!(self.tick < limit, "{what}: bis Takt {limit} nicht erreicht");
            self.step();
        }
    }

    /// Merkt die Station `name` beim aktuellen Takt mit dem kanonischen Welt-Hash des Frames.
    pub fn mark(&mut self, name: &str) {
        let hash = self.hash();
        self.stations.push(Station {
            name: name.to_string(),
            tick: self.tick,
            hash,
        });
    }

    /// Weiche Bedingung: gilt `ok` nicht, merkt der Lauf `msg` als bekannte Abweichung von Go (statt abzubrechen).
    pub fn soft(&mut self, ok: bool, msg: &str) {
        if !ok {
            self.deviations.push(format!("Takt {}: {msg}", self.tick));
        }
    }

    /// Kanonischer Welt-Hash des Frames (`-` ohne Spiel).
    pub fn hash(&self) -> String {
        worlds_hash(&self.trees())
    }

    /// Die Bäume der Welten nach Stufen-Index.
    pub fn trees(&self) -> Vec<(i64, Value)> {
        self.frame()
            .stages
            .iter()
            .filter_map(|s| s.world.as_deref().map(|w| (s.index, golden::tree(w))))
            .collect()
    }

    /// Steht ein Menü oben (`false` im Spiel)?
    pub fn screen(&self) -> bool {
        self.g.screen().is_some()
    }

    /// Das Hauptmenü liegt oben und kein Spiel läuft.
    pub fn in_main_menu(&self) -> bool {
        self.g.mode() == Mode::Menu && self.screen() && self.frame().stages.is_empty() && !self.frame().paused
    }
}

/// Meldet ein Ereignis des Typs `typ` im Frame; `player >= 0` verlangt diesen Spieler (Feld `player`).
pub fn has_event(f: &Frame, typ: &str, player: i64) -> bool {
    f.events.iter().any(|e| {
        e.type_() == typ
            && (player < 0
                || e.data
                    .get("player")
                    .and_then(Value::as_f64)
                    .is_some_and(|p| (p - player as f64).abs() < 1e-9))
    })
}

/// Meldet einen Cue der Art `k`.
pub fn has_cue(f: &Frame, k: CueKind) -> bool {
    f.cues.iter().any(|c| c.kind == k)
}

/// Meldet einen lebenden Gegner höchstens `d` Units neben einem Monarchen der Spieler.
pub fn enemy_near(f: &Frame, d: f64) -> bool {
    f.seats.iter().any(|s| {
        let (Some(m), Some(w)) = (f.monarch(s.player), f.stage(s.stage).and_then(|st| st.world.as_deref())) else {
            return false;
        };
        w.enemies.iter().any(|e| e.hp > 0.0 && e.x > m.x - d && e.x < m.x + d)
    })
}
