//! Spielseite des Cheat-Fensters `k3c-devwin` (RU56, B-1133): Pipe-Client, Zustandsabzug und Befehle über dieselben
//! Cheat-Pfade wie das F9-Menü (`DevSession::dev`, `DevCmd`). Das Fenster ist ein eigener Prozess; ohne Fenster ist
//! `DevWin` leer (kein Thread, keine Allokation je Bild, `update` kehrt sofort zurück).
//!
//! Die Schleife liefert über [`Host`] nur, was die Befehle brauchen; Tests setzen einen Mock ein (`tests/devwin_pipe.rs`).
//! Befehle nach Bereich: `world.rs` (Welt und Zeit), `eco.rs` (Wirtschaft), `players.rs` (Spieler und Skills),
//! `foes.rs` (Gegner und Welle), `view.rs` (Anzeige); `commands.rs` leitet weiter und antwortet mit `ack`.

pub mod commands;
pub mod eco;
pub mod foes;
pub mod link;
pub mod mock;
pub mod players;
pub mod spawn;
pub mod state;
pub mod view;
pub mod world;

use std::time::{Duration, Instant};

use k3c_devproto::{Catalog, GameInfo, Msg, PROTO_VERSION, State};

pub use link::{Event, Link, Opener, STATE_QUEUE, Streams, open_pipe};
pub use state::Throttle;

use crate::view::devkeys::{DevCmd, DevSession};

/// RETRY ist der Abstand der Verbindungsversuche, solange das Fenster gewollt ist.
pub const RETRY: Duration = Duration::from_secs(2);
/// FAST_RETRY ist der Abstand der ersten Versuche nach dem Start (das Fenster legt seine Pipe erst an), `FAST_FOR` ihre Dauer.
pub const FAST_RETRY: Duration = Duration::from_millis(250);
const FAST_FOR: Duration = Duration::from_secs(10);
/// MAX_PER_UPDATE sind die Befehle, die ein Takt höchstens abarbeitet (der Rest folgt im nächsten).
const MAX_PER_UPDATE: usize = 32;

/// Host ist das Spiel aus Sicht der Befehle: Zugriff auf die Cheat-Pfade und Lesen des Zustands.
pub trait Host {
    /// Zahl der Spieler im laufenden Spiel (0 ohne Spiel).
    fn player_count(&self) -> i64;
    /// Die Entwickler-Rolle der Session (`DevSession`); `None` ohne Rolle oder ohne Spiel.
    fn dev_session(&mut self) -> Option<&mut dyn DevSession>;
    /// Lernt einen Skill für den Spieler (wie das Skill-Menü).
    fn learn(&mut self, player: i64, skill: &str) -> Result<(), String>;
    /// Setzt die Skills des Spielers zurück.
    fn respec(&mut self, player: i64) -> Result<(), String>;
    /// Führt den Befehl einer F-Taste aus (wie die Taste; Hilfe, Debug, Leistung, Fluss, Speichern, Zeitraffer,
    /// Screenshot, Vollbild).
    fn dev_cmd(&mut self, c: DevCmd) -> Result<(), String>;
    /// Wechselt Welt, Insel und Stufe.
    fn goto(&mut self, world: &str, island: i64, stage: i64) -> Result<(), String>;
    /// Holt das Spielfenster nach vorn.
    fn focus(&mut self) -> Result<(), String>;
    /// Der aktuelle Zustand für die Anzeige im Fenster.
    fn state(&mut self) -> State;
    /// Die Auswahllisten für die Controls.
    fn catalog(&self) -> Catalog;
}

struct Run {
    opener: Opener,
    link: Option<Link>,
    next_try: Option<Instant>,
    /// Bis dahin gilt `FAST_RETRY`, solange noch nie verbunden war.
    fast_until: Instant,
    ever: bool,
    throttle: Throttle,
    game: GameInfo,
}

/// DevWin ist die Verbindung des Spiels zum Fenster; Standard ist aus.
#[derive(Default)]
pub struct DevWin {
    run: Option<Run>,
}

impl DevWin {
    /// is_on meldet, ob das Fenster gewollt ist.
    pub fn is_on(&self) -> bool {
        self.run.is_some()
    }

    /// connected meldet, ob eine Verbindung steht.
    pub fn connected(&self) -> bool {
        self.run.as_ref().is_some_and(|r| r.link.is_some())
    }

    /// start will das Fenster: ab dem nächsten `update` wird verbunden (alle `RETRY`, solange es fehlt).
    pub fn start(&mut self, opener: Opener, game: GameInfo) {
        self.run = Some(Run {
            opener,
            link: None,
            next_try: None,
            fast_until: Instant::now() + FAST_FOR,
            ever: false,
            throttle: Throttle::default(),
            game,
        });
    }

    /// focus bittet das Fenster, sich nach vorn zu holen (zweites F4); ohne Verbindung geschieht nichts.
    pub fn focus(&mut self) {
        if let Some(link) = self.run.as_ref().and_then(|r| r.link.as_ref()) {
            link.send_ctl(Msg::Focus { id: 0 });
        }
    }

    /// stop beendet die Verbindung (`bye`) und verwirft den Wunsch.
    pub fn stop(&mut self) {
        self.run = None;
    }

    /// update nimmt Befehle an, antwortet und sendet den Zustand (höchstens 4 Hz); es blockiert nie.
    pub fn update(&mut self, host: &mut dyn Host, now: Instant) {
        let Some(run) = self.run.as_mut() else {
            return;
        };
        if run.link.is_none() {
            run.try_connect(host, now);
        }
        let mut lost = false;
        if let Some(link) = run.link.as_mut() {
            for _ in 0..MAX_PER_UPDATE {
                match link.poll() {
                    Some(Event::Msg(m)) => {
                        if let Some(ack) = commands::handle(host, m) {
                            link.send_ctl(ack);
                        }
                    }
                    Some(Event::Lost) => {
                        lost = true;
                        break;
                    }
                    None => break,
                }
            }
            if !lost && let Some(st) = run.throttle.next(now, || host.state()) {
                link.send_state(Msg::State(st));
            }
        }
        if lost {
            k3c_platform::log::info("🔌 Cheat-Fenster getrennt", &[]);
            run.link = None;
            run.next_try = Some(now + RETRY);
        }
    }
}

impl Run {
    fn try_connect(&mut self, host: &dyn Host, now: Instant) {
        if self.next_try.is_some_and(|t| now < t) {
            return;
        }
        let gap = if !self.ever && now < self.fast_until {
            FAST_RETRY
        } else {
            RETRY
        };
        self.next_try = Some(now + gap);
        let Ok(streams) = (self.opener)() else {
            return;
        };
        match Link::connect(streams) {
            Ok(link) => {
                link.send_ctl(Msg::Hello {
                    v: PROTO_VERSION,
                    game: Some(self.game.clone()),
                });
                link.send_ctl(Msg::Catalog(host.catalog()));
                self.throttle = Throttle::default();
                self.ever = true;
                self.link = Some(link);
                k3c_platform::log::info("🔌 Cheat-Fenster verbunden", &[]);
            }
            Err(e) => k3c_platform::log::warn("💥 Cheat-Fenster: Threads nicht gestartet", &[("err", &e)]),
        }
    }
}

#[cfg(test)]
mod menu_tests;
