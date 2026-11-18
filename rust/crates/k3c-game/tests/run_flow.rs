//! Ablauf des Spiels ohne Fenster (RU30.6, AC-01): Hauptmenü → Neues Spiel → Lobby mit zwei Spielern → beide laufen →
//! Nacht und Überfall per Dev-Aktion (`phase`, `spawn`) → Kronenraub = Niederlage → nach `END_DELAY` Takten liegt das
//! Hauptmenü oben. Abweichung vom Auftrag: Die Dev-Aktion `castleFall` (Burgfall) nimmt nur der Stufe die Burg, das
//! Spiel endet in Go wie in Rust nur durch Kronenraub (`island/defeat.rs`); darum der Überfall. Es ist die echte
//! `local::Session` (Rust-Sim) in einem Wegwerf-Ordner unter `.work/` (nie `%TEMP%`), die Eingabe die Folge
//! `--flow --input` (Go: `e2e/flow_test.go`), die Zeichenfläche `NullCanvas`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_game::game::launch::{build_game_with, parse};
use k3c_game::game::sim::Sim;
use k3c_game::game::stack::Mode;
use k3c_game::game::{Game, Libs};
use k3c_game::local::config::Config as SessionConfig;
use k3c_game::local::session::Session;
use k3c_game::view::devkeys::{DevAction, DevSession};
use k3c_game::view::frame::Seat;
use k3c_game::view::frame::{END_DELAY, Frame, Outcome};
use k3c_game::view::input::{Command, DeviceID};
use k3c_game::view::session::{Fehler, NewGame, SaveInfo, Session as SessionApi, WorldInfo};
use k3c_game::view::settings::Settings;
use k3c_platform::{App, Canvas, InputState, run_headless};
use std::cell::Cell;
use std::rc::Rc;

/// Takt (seit dem Spielstart), ab dem überfallen wird: beide Spieler sind dann eine Weile gelaufen.
const RAID_AT: u32 = 200;
/// Takte zwischen zwei Überfällen und Zahl der Gegner je Spieler.
const RAID_EVERY: u32 = 90;
const RAID_SIZE: i64 = 12;

/// Eingabe-Folge wie im Go-Ablauf: Neues Spiel, Welt Standard, Spieler 2 tritt der Lobby bei, Grad Ultra, Starten;
/// Spieler 1 läuft nach rechts, Spieler 2 sprintet.
const FLOW: &str = "0@2=a:confirm,0@3=a:confirm,1@4=a:confirm,0@6=a:right,0@8=a:right,\
0@10=a:down,0@12=a:down,0@14=a:down,0@16=a:down,0@18=a:down,0@20=a:confirm,0@21=R,1@21=RS";

/// Wegwerf-Ordner der Spielstände unter `.work/` (nie `%TEMP%`).
fn saves() -> String {
    format!(
        "{}/../../../.work/run-flow-{}",
        env!("CARGO_MANIFEST_DIR"),
        std::process::id()
    )
}

/// Meldungen der Probe an den Test.
#[derive(Default)]
struct Seen {
    raided: Cell<bool>,
    seats_at_raid: Cell<usize>,
}

/// Die echte Sitzung, die ab dem Takt `RAID_AT` nach dem Spielstart Überfälle auslöst.
struct Fall {
    inner: Session,
    play_ticks: u32,
    seen: Rc<Seen>,
}

impl SessionApi for Fall {
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler> {
        self.inner.new_game(g)
    }
    fn continue_game(&mut self, d: &[DeviceID]) -> Result<(), Fehler> {
        self.inner.continue_game(d)
    }
    fn load(&mut self, n: &str, d: &[DeviceID]) -> Result<(), Fehler> {
        self.inner.load(n, d)
    }
    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler> {
        self.inner.list()
    }
    fn delete(&mut self, n: &str) -> Result<(), Fehler> {
        self.inner.delete(n)
    }
    fn save(&mut self) -> Result<(), Fehler> {
        self.inner.save()
    }
    fn close(&mut self) -> Result<(), Fehler> {
        self.play_ticks = 0;
        self.inner.close()
    }
    fn active(&self) -> bool {
        self.inner.active()
    }
    fn pause(&mut self, on: bool) {
        self.inner.pause(on);
    }
    fn join(&mut self, d: DeviceID) -> Result<i64, Fehler> {
        self.inner.join(d)
    }
    fn leave(&mut self, p: i64) -> Result<(), Fehler> {
        self.inner.leave(p)
    }
    fn rebind(&mut self, p: i64, d: DeviceID) -> Result<(), Fehler> {
        self.inner.rebind(p, d)
    }
    fn learn(&mut self, p: i64, s: &str) -> Result<(), Fehler> {
        self.inner.learn(p, s)
    }
    fn respec(&mut self, p: i64) -> Result<(), Fehler> {
        self.inner.respec(p)
    }
    fn choose_island(&mut self, p: i64, i: &str) -> Result<(), Fehler> {
        self.inner.choose_island(p, i)
    }
    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler> {
        self.inner.set_settings(s)
    }
    fn settings(&self) -> Settings {
        self.inner.settings()
    }
    fn seats(&self) -> Vec<Seat> {
        self.inner.seats()
    }
    fn world_list(&self) -> Vec<WorldInfo> {
        self.inner.world_list()
    }
    fn world(&self) -> String {
        self.inner.world()
    }
}

impl Fall {
    /// Nacht, und eine Gruppe Gegner um jeden der beiden Spieler (Dev-Aktionen `phase` und `spawn`).
    fn raid(&mut self) {
        let ds = self.inner.dev_session().expect("die Sitzung hat die Entwickler-Rolle");
        let night = DevAction {
            action: "phase".to_string(),
            phase: "night".to_string(),
            ..DevAction::default()
        };
        let _ = ds.dev(0, &night);
        for p in 0..2 {
            let spawn = DevAction {
                action: "spawn".to_string(),
                kind: "enemy".to_string(),
                unit: "goblin".to_string(),
                amount: RAID_SIZE,
                ..DevAction::default()
            };
            let r = ds.dev(p, &spawn);
            assert!(r.is_ok() || self.seen.raided.get(), "spawn: {r:?}");
        }
    }
}

impl Sim for Fall {
    fn command(&mut self, p: i64, c: Command) -> Result<(), Fehler> {
        self.inner.command(p, c)
    }

    fn step(&mut self) -> Frame {
        if self.inner.active() {
            self.play_ticks += 1;
            if self.play_ticks >= RAID_AT && (self.play_ticks - RAID_AT).is_multiple_of(RAID_EVERY) {
                if !self.seen.raided.get() {
                    self.seen.seats_at_raid.set(self.inner.seats().len());
                }
                self.raid();
                self.seen.raided.set(true);
            }
        }
        self.inner.step()
    }

    fn dev_session(&mut self) -> Option<&mut dyn DevSession> {
        self.inner.dev_session()
    }
}

/// Spiel und Beobachtung; zählt Takte und merkt sich den Verlauf.
struct Probe {
    g: Game,
    seen: Rc<Seen>,
    ticks: u32,
    lost_at: Option<u32>,
    menu_at: Option<u32>,
    max_seats: usize,
}

impl App for Probe {
    fn step(&mut self, input: &InputState, dt: f64) {
        self.g.step(input, dt);
        self.ticks += 1;
        self.max_seats = self.max_seats.max(self.g.frame().seats.len());
        if self.lost_at.is_none() && self.g.frame().outcome == Outcome::Lost {
            self.lost_at = Some(self.ticks);
        }
        if self.lost_at.is_some() && self.menu_at.is_none() && self.g.mode() == Mode::Menu {
            self.menu_at = Some(self.ticks);
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.g.draw(c, alpha);
    }

    fn done(&self) -> bool {
        self.g.done()
    }
}

#[test]
fn zwei_spieler_vom_hauptmenue_bis_zur_niederlage() {
    let dir = saves();
    let args = parse(&["--saves", &dir, "--flow", "--input", FLOW].map(String::from)).expect("Schalter");
    let seen = Rc::new(Seen::default());
    let session = Session::open(SessionConfig {
        dir: dir.clone(),
        ..SessionConfig::default()
    });
    let sim = Fall {
        inner: session,
        play_ticks: 0,
        seen: Rc::clone(&seen),
    };
    // derselbe Weg wie `k3c-game`, nur mit der umhüllten Sitzung
    let g = build_game_with(&args, Box::new(sim), &Libs::default()).expect("Spiel");
    assert_eq!(g.mode(), Mode::Menu, "Start im Hauptmenü");
    assert!(g.screen().is_some());
    let mut p = Probe {
        g,
        seen: Rc::clone(&seen),
        ticks: 0,
        lost_at: None,
        menu_at: None,
        max_seats: 0,
    };
    run_headless(&mut p, 4000, None);
    let _ = std::fs::remove_dir_all(&dir);

    assert!(p.seen.raided.get(), "Spiel gestartet, Überfall ausgelöst");
    assert_eq!(p.seen.seats_at_raid.get(), 2, "beide Spieler im Spiel");
    assert_eq!(p.max_seats, 2);
    let lost = p.lost_at.expect("Niederlage (Kronenraub, Insel verloren)");
    let menu = p.menu_at.expect("Hauptmenü nach der Niederlage");
    assert!(
        menu - lost >= u32::try_from(END_DELAY).unwrap() - 1,
        "Hauptmenü erst nach END_DELAY Takten: Niederlage {lost}, Menü {menu}"
    );
    assert_eq!(p.g.mode(), Mode::Menu);
    assert!(p.g.screen().is_some(), "Hauptmenü liegt oben");
    assert!(p.g.frame().stages.is_empty(), "kein Spiel mehr");
}
