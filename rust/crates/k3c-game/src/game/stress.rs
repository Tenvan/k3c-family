//! Port von `native/k3c-game/game/stress.go`: Stresstest (Testing, B-896). Das Menü startet ihn über die Rolle
//! `view::stress::StressStarter`; `game` legt das Spiel an, schaltet Leistungsanzeige (F2) und Debug-Werte (F10) an und
//! führt die Werte über Dev-Aktionen des Raums aus: im ersten Takt Ausbau, Truppen und Zeitraffer, zu jeder neuen Welle
//! (Ereignis `wave`) zusätzliche Gegner nach der Kurve (`StressTest::enemies_for`). Der Test löst die Wellen selbst aus
//! (`stress_wave`), sobald keine Gegner mehr leben oder warten, statt den Tag-Nacht-Zyklus abzuwarten (B-920). Im reinen
//! Modus (`StressTest::pure`, B-921) ruht jede natürliche Welle; die Gegner entstehen an der Burg (`spawnCastle`). Nach den
//! eingestellten Wellen oder Minuten, bei Spielende oder Verlassen schreibt er den Bericht (`stressreport.rs`) und hält die
//! Simulation an (F7 lässt sie weiterlaufen).
//!
//! Abweichung von Go (Leitfaden § 3): Go reicht dem Menü die Rolle `StressStarter` als Typ-Zusicherung an der Session
//! (`menuSession`). In Rust hält das Menü den Starter selbst (`Menus::set_stress_starter`); `StarterLink` ist dieser
//! Starter und gibt den Auftrag an die Schleife weiter, die ihn gleich nach dem `update` des Screens ausführt
//! (`run_stress_request`). Ein Fehler des Starts erscheint darum als Hinweis (`Cue`) statt im Menü.

use std::cell::{Cell, RefCell};
use std::path::Path;
use std::rc::Rc;

use k3c_platform::log;

use super::Game;
use super::measure_load::ERR_NO_DEV;
use super::stressreport::{StressRun, write_stress_report};
use crate::devtools::report::Zeit;
use crate::view::devkeys::{DEBUG_VALUES, DevAction, DevSession};
use crate::view::frame::{Cue, CueKind, Frame};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::roles::REPORTS_DIR;
use crate::view::session::NewGame;
use crate::view::stress::{StressStarter, StressTest};

/// Der Spielstand eines Stresstests (jeder neue Test ersetzt ihn ohne Rückfrage, B-916).
pub const STRESS_SAVE: &str = "stresstest";

/// Index der Platzhalter-Pads für Spieler ohne Gerät (der Platz wartet auf ein Gerät).
pub const STRESS_PAD_BASE: i64 = 1000;

/// Der Starter, den das Menü hält: merkt den Auftrag für die Schleife (siehe Moduldoku).
#[derive(Clone, Default)]
pub struct StarterLink {
    request: Rc<RefCell<Option<(StressTest, DeviceID)>>>,
    /// die Simulation hat eine Entwickler-Rolle (die Schleife setzt es vor jedem `update` der Menüs)
    dev: Rc<Cell<bool>>,
}

impl StressStarter for StarterLink {
    fn start_stress(&mut self, t: &StressTest, by: DeviceID) -> Result<(), String> {
        if !self.dev.get() {
            return Err(ERR_NO_DEV.to_string());
        }
        *self.request.borrow_mut() = Some((t.clone(), by));
        Ok(())
    }
}

/// Zustand der Schleife rund um den Stresstest, der in Go einzelne `Game`-Felder sind (`clock`, `fpsOf`, `reports`).
#[derive(Default)]
pub struct StressEnv {
    /// Wanduhr des Stresstests (Tests setzen eine feste Uhr)
    pub clock: Option<Box<dyn Fn() -> Zeit>>,
    /// gemessene Bildrate (Tests setzen sie; sonst `Game::fps`, im Aufnahme-Modus 0)
    pub fps_of: Option<Box<dyn Fn() -> f64>>,
    /// Ordner der Berichte (Tests setzen ihn; leer = `REPORTS_DIR`)
    pub reports: String,
    link: StarterLink,
}

/// Die Geräte der Spieler: `by`, dann Platzhalter-Pads (Go: `stressDevices`).
pub fn stress_devices(t: &StressTest, by: DeviceID) -> Vec<DeviceID> {
    let mut out = vec![by];
    for i in 1..t.players {
        out.push(DeviceID::new(DeviceKind::Gamepad, STRESS_PAD_BASE + i));
    }
    out
}

/// Die Angaben des Spiels zum Test `t` (Go: `stressGame`).
pub fn stress_game(t: &StressTest, by: DeviceID) -> NewGame {
    NewGame {
        name: STRESS_SAVE.to_string(),
        grade: "normal".to_string(),
        depth: t.depth,
        devices: stress_devices(t, by),
        seed: if t.seed > 0 { t.seed.to_string() } else { String::new() },
        ..NewGame::default()
    }
}

impl Game {
    /// Der Starter für das Menü (`Menus::set_stress_starter`); er gehört zu dieser Schleife.
    pub fn stress_starter(&self) -> Box<dyn StressStarter> {
        Box::new(self.stress_env.link.clone())
    }

    /// Gibt dem Menü den Starter dieser Schleife (nach `Game::new`, B-1117); ohne diesen Aufruf zeigt Testing
    /// `TstNoRunner`.
    pub fn wire_stress_starter(&mut self) {
        let st = self.stress_starter();
        self.parts.menus.set_stress_starter(st);
    }

    /// Meldet dem Starter, ob die Simulation Dev-Aktionen kann (vor dem `update` der Menüs).
    pub(super) fn sync_stress_link(&mut self) {
        let dev = self.sim.dev_session().is_some();
        self.stress_env.link.dev.set(dev);
    }

    /// Führt den Auftrag des Starters aus, den der Screen im `update` gab (Go: Aufruf von `StartStress` im `update`).
    pub(super) fn run_stress_request(&mut self) {
        let req = self.stress_env.link.request.borrow_mut().take();
        let Some((t, by)) = req else { return };
        if let Err(e) = self.start_stress(&t, by) {
            log::warn("💥 Stresstest nicht gestartet", &[("err", &e)]);
            self.cues.push(Cue {
                kind: CueKind::Error,
                player: -1,
                text: format!("Stresstest: {e}"),
                ..Cue::default()
            });
        }
    }

    /// Legt das Spiel an, schaltet F2 und F10 an und merkt den Lauf (Go: `startStress`).
    pub fn start_stress(&mut self, t: &StressTest, by: DeviceID) -> Result<(), String> {
        if self.sim.dev_session().is_none() {
            return Err(ERR_NO_DEV.to_string());
        }
        let t = t.clone().normalized();
        self.drop_stress_save()?;
        self.sim.new_game(stress_game(&t, by)).map_err(|e| e.to_string())?;
        let mut set = self.sim.settings();
        set.stats = true;
        if let Err(e) = self.sim.set_settings(set) {
            log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
        }
        self.dev.debug = DEBUG_VALUES;
        self.stress = Some(StressRun::new(t.clone(), self.now()));
        log::info(
            "🚀 Stresstest gestartet",
            &[
                ("vorgabe", &t.preset),
                ("spieler", &t.players),
                ("tiefe", &t.depth),
                ("seed", &t.seed),
                ("gegner", &t.enemies),
                ("anstieg", &t.growth),
                ("faktor", &t.factor),
                ("wellen", &t.waves),
                ("minuten", &t.minutes),
                ("rein", &t.pure),
            ],
        );
        Ok(())
    }

    /// Nimmt den vorhandenen Stresstest-Spielstand aus der Liste (Sicherung wie beim Löschen im Menü); andere Spielstände
    /// bleiben unberührt. Lehnt `delete` ab, weil er der des laufenden Spiels ist, wird dieses (der alte Stresstest)
    /// beendet und erneut gelöscht; ein laufendes normales Spiel beendet erst `new_game` (Go: `dropStressSave`).
    fn drop_stress_save(&mut self) -> Result<(), String> {
        let list = self.sim.list().map_err(|e| e.to_string())?;
        if !list.iter().any(|e| e.name == STRESS_SAVE) {
            return Ok(());
        }
        match self.sim.delete(STRESS_SAVE) {
            Ok(()) => return Ok(()),
            Err(e) if !self.sim.active() => return Err(e.to_string()),
            Err(_) => {}
        }
        if let Err(e) = self.sim.close() {
            log::warn("💥 Laufender Stresstest nicht sauber beendet", &[("err", &e)]);
        }
        self.sim.delete(STRESS_SAVE).map_err(|e| e.to_string())
    }

    /// Die Wanduhr des Stresstests (Go: `now`).
    pub(super) fn now(&self) -> Zeit {
        self.stress_env.clock.as_ref().map_or_else(Zeit::now, |c| c())
    }

    /// Die gemessene Bildrate (Go: `fps`; Tests setzen `fps_of`, im Aufnahme-Modus 0).
    fn fps_value(&self) -> f64 {
        if let Some(f) = &self.stress_env.fps_of {
            return f();
        }
        if self.args.shots.is_empty() { self.fps } else { 0.0 }
    }

    /// Ordner der Berichte (Go: `reportDir`).
    fn report_dir(&self) -> &str {
        if self.stress_env.reports.is_empty() {
            REPORTS_DIR
        } else {
            &self.stress_env.reports
        }
    }

    /// Führt den laufenden Test nach jedem Takt fort: Vorspann, Wellen, Messwerte, Ende (Go: `stressStep`).
    pub(super) fn stress_step(&mut self) {
        let Some(mut r) = self.stress.take() else { return };
        let (now, fps) = (self.now(), self.fps_value());
        match self.stress_tick(&mut r, &now, fps) {
            Some(cause) => self.finish_stress(&r, &now, &cause),
            None => self.stress = Some(r),
        }
    }

    /// Ein Takt des Tests; liefert die Ursache, wenn er endet.
    fn stress_tick(&mut self, r: &mut StressRun, now: &Zeit, fps: f64) -> Option<String> {
        let active = self.sim.active();
        let ds = self.sim.dev_session();
        let (Some(ds), true, Some(seat)) = (ds, active, self.frame.seats.first()) else {
            return Some("verlassen".to_string());
        };
        let (p, stage) = (seat.player, seat.stage);
        let f = &self.frame;
        if !r.prelude {
            r.prelude = true;
            r.start_tick = f.tick;
            stress_prelude(ds, p, &r.cfg);
        }
        for e in &f.events {
            if e.type_() == "wave" && e.stage == stage {
                // nur die Stufe von Spieler 1 (dort der Spawn)
                r.waves += 1;
                r.fought = false;
                stress_dev(ds, p, &spawn_enemies(&r.cfg, r.waves, "spawn"));
            }
        }
        stress_wave(ds, p, r, f);
        r.sample(f, fps);
        let cause = r.over(f, now);
        (!cause.is_empty()).then_some(cause)
    }

    /// Schreibt den Bericht, meldet ihn und hält die Simulation an (nicht nach Verlassen; Go: `finishStress`).
    fn finish_stress(&mut self, r: &StressRun, now: &Zeit, cause: &str) {
        let rep = r.report(&self.frame, now, cause);
        match write_stress_report(Path::new(self.report_dir()), &rep) {
            Err(e) => {
                log::warn("💥 Stresstest-Bericht nicht geschrieben", &[("err", &e)]);
                self.cues.push(Cue {
                    kind: CueKind::Error,
                    player: -1,
                    text: format!("Stresstest-Bericht: {e}"),
                    ..Cue::default()
                });
                return;
            }
            Ok(path) => {
                let datei = path.display().to_string();
                log::info(
                    "💾 Stresstest-Bericht",
                    &[
                        ("datei", &datei),
                        ("ursache", &cause),
                        ("wellen", &rep.waves),
                        ("takte_s", &rep.ticks_per_second),
                    ],
                );
                self.cues.push(Cue {
                    kind: CueKind::Denied,
                    player: -1,
                    text: format!("Stresstest beendet: {datei}"),
                    ..Cue::default()
                });
            }
        }
        if cause != "verlassen"
            && self.sim.active()
            && let Some(ds) = self.sim.dev_session()
        {
            ds.halt(true);
        }
    }
}

/// Die Dev-Aktion `spawn` (oder `spawnCastle`) für die zusätzlichen Gegner der Welle `wave`.
fn spawn_enemies(c: &StressTest, wave: i64, action: &str) -> DevAction {
    DevAction {
        action: action.to_string(),
        kind: "enemy".to_string(),
        unit: c.enemy.clone(),
        amount: c.enemies_for(wave),
        ..DevAction::default()
    }
}

/// Löst die nächste Welle aus, wenn die Stufe leer ist: die erste gleich nach dem Vorspann, jede weitere, sobald die
/// vorige geschlagen ist (`fought`), bis `cfg.waves`. Das Ereignis `wave` der Dev-Aktion geht im Takt verloren (die
/// Simulation leert die Ereignisse zu Beginn jedes Takts), darum zählt der Test die Welle selbst (Go: `stressWave`).
fn stress_wave(ds: &mut dyn DevSession, p: i64, r: &mut StressRun, f: &Frame) {
    let c = &r.cfg;
    if (c.waves > 0 && r.waves >= c.waves) || (r.waves > 0 && !r.fought) || !wave_idle(f) {
        return;
    }
    r.waves += 1;
    r.fought = false;
    let c = &r.cfg;
    if c.pure {
        stress_dev(ds, p, &spawn_enemies(c, r.waves, "spawnCastle"));
    } else {
        let wave = DevAction {
            action: "wave".to_string(),
            ..DevAction::default()
        };
        stress_dev(ds, p, &wave);
        stress_dev(ds, p, &spawn_enemies(c, r.waves, "spawn"));
    }
}

/// Keine Gegner einer Welle leben oder warten (Wachen zählen nicht, sie bleiben stehen; Go: `waveIdle`).
fn wave_idle(f: &Frame) -> bool {
    f.stages
        .iter()
        .filter_map(|st| st.world.as_ref())
        .all(|w| w.spawn_queue.is_empty() && w.enemies.iter().all(|e| e.guard))
}

/// Ausbau, Truppen und Zeitraffer im ersten Takt des Tests (Go: `stressPrelude`).
fn stress_prelude(ds: &mut dyn DevSession, p: i64, c: &StressTest) {
    let act = |a: &str| DevAction {
        action: a.to_string(),
        ..DevAction::default()
    };
    if c.pure {
        // vor allem anderen: keine natürlichen Wellen, Lager und Händler
        stress_dev(
            ds,
            p,
            &DevAction {
                on: Some(true),
                ..act("pure")
            },
        );
    }
    if c.build > 0 {
        stress_dev(
            ds,
            p,
            &DevAction {
                level: c.build,
                ..act("buildAll")
            },
        );
    }
    if c.troops > 0 {
        let a = DevAction {
            kind: "troop".to_string(),
            unit: c.troop.clone(),
            amount: c.troops,
            ..act("spawn")
        };
        stress_dev(ds, p, &a);
    }
    if c.timescale > 1 {
        stress_dev(
            ds,
            p,
            &DevAction {
                factor: c.timescale,
                ..act("timescale")
            },
        );
    }
}

/// Führt eine Dev-Aktion aus (Menge 0: nichts); eine Ablehnung steht im Log, der Test läuft weiter (Go: `stressDev`).
fn stress_dev(ds: &mut dyn DevSession, p: i64, a: &DevAction) {
    if a.action == "spawn" && a.amount <= 0 {
        return;
    }
    if let Err(e) = ds.dev(p, a) {
        log::warn(
            "🚫 Stresstest: Dev-Aktion abgelehnt",
            &[("aktion", &a.action), ("err", &e)],
        );
    }
}

#[cfg(test)]
#[path = "stress_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "stress_menu_tests.rs"]
mod menu_tests;
