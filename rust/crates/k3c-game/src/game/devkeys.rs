//! Port von `native/k3c-game/game/devkeys.go`: Entwickler-Tasten (B-816, B-888). `game` liest die Tasten der Tabelle
//! `view::DEV_KEYS`, die keine Geräte-Aktion sind (F9 und F11 laufen über `input` als `Cheats` bzw. `Fullscreen`). F2
//! und F3 ändern die Einstellungen (`Session::set_settings`, bleiben erhalten), F1 und F10 den Frame-Zustand `DevView`,
//! F5 bis F8 die Session (`DevSession`), F12 nimmt das nächste Bild auf. Keine Taste gilt im Aufnahme-Modus; dort
//! setzen `--river` und `--stats` die Werte.
//!
//! Abweichung von Go: Die frisch gedrückten Tasten kommen aus `raw_input` (Kopie der Eingabe des Takts, Go: `pressed`).

use k3c_platform::log;

use super::{Game, Mode};
use crate::view::devkeys::{DEV_KEYS, DevAction, DevCmd, DevSession, next_debug, next_timescale};
use crate::view::frame::{Cue, CueKind};
use crate::view::settings::Settings;
use crate::world::water::river_start;

impl Game {
    /// Wertet die Entwickler-Tasten aus (nicht vor dem ersten Frame, dessen Einstellungen sind noch leer).
    pub(super) fn dev_keys(&mut self) {
        if !self.args.shots.is_empty() || self.frame.settings.language.is_empty() {
            return;
        }
        let mut set = self.frame.settings.clone();
        for k in &DEV_KEYS {
            if k.action.is_none() && self.raw_input.just_pressed(k.key) {
                self.dev_command(k.cmd, &mut set);
            }
        }
        if set == self.frame.settings {
            return;
        }
        // F2 und F3 zugleich: einmal speichern
        if let Err(e) = self.sim.set_settings(set) {
            log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
        }
    }

    /// Führt den Befehl einer Entwickler-Taste aus; F2 und F3 ändern `set` (`dev_keys` speichert). `false`: `game`
    /// wertet ihn nicht selbst aus (Geräte-Aktion).
    pub(super) fn dev_command(&mut self, c: DevCmd, set: &mut Settings) -> bool {
        match c {
            DevCmd::Help => {
                self.dev.help = !self.dev.help;
                log::info("🐛 Tastenübersicht", &[("an", &self.dev.help)]);
            }
            DevCmd::Debug => {
                self.dev.debug = next_debug(self.dev.debug);
                log::info("🐛 Debug-Overlay", &[("seite", &self.dev.debug)]);
            }
            DevCmd::Shot => self.shot_next = true,
            DevCmd::Window => self.dev_window(),
            DevCmd::Stats | DevCmd::River => dev_settings(c, set),
            DevCmd::Save | DevCmd::Load | DevCmd::Pause | DevCmd::Speed => self.dev_session_cmd(c),
            _ => return false,
        }
        true
    }

    /// Führt F5 bis F8 über die Session aus; ohne laufendes Spiel, mit offenem Menü oder ohne Entwickler-Rolle
    /// geschieht nichts.
    fn dev_session_cmd(&mut self, c: DevCmd) {
        let has_dev = self.sim.dev_session().is_some();
        // nicht unter einem Menü: F6 hebt sonst dessen Pause auf
        if !has_dev || !self.sim.active() || self.mode != Mode::Play {
            log::info("🚫 Entwickler-Taste nur im laufenden Spiel", &[("befehl", &(c as i64))]);
            return;
        }
        let res = if c == DevCmd::Save {
            self.sim
                .save()
                .map(|()| log::info("💾 Schnellspeichern", &[]))
                .map_err(|e| e.to_string())
        } else {
            self.dev_session_run(c)
        };
        if let Err(e) = res {
            log::warn("🚫 Entwickler-Taste abgelehnt", &[("befehl", &(c as i64)), ("err", &e)]);
            self.cues.push(Cue {
                kind: CueKind::Denied,
                player: -1,
                text: e,
                ..Cue::default()
            });
        }
    }

    /// Lädt neu (F6), hält an bzw. rechnet einen Takt (F7), schaltet den Zeitraffer weiter (F8).
    fn dev_session_run(&mut self, c: DevCmd) -> Result<(), String> {
        let first = self.frame.seats.first().map(|s| s.player);
        let scale = self.frame.timescale;
        let Some(ds) = self.sim.dev_session() else {
            return Ok(());
        };
        match c {
            DevCmd::Load => ds.reload(),
            DevCmd::Pause => {
                if ds.halted() {
                    ds.step_once();
                } else {
                    ds.halt(true);
                }
                Ok(())
            }
            DevCmd::Speed => dev_speed(ds, first, scale),
            _ => Ok(()),
        }
    }

    /// Übernimmt `--river` und `--stats` in die Einstellungen (Aufnahme der beiden Fassungen und der Anzeige).
    pub(super) fn apply_dev_args(&mut self) {
        if self.args.river.is_empty() && !self.args.stats {
            return;
        }
        let mut s = self.sim.settings();
        if !self.args.river.is_empty() {
            s.river.clone_from(&self.args.river);
        }
        s.stats = s.stats || self.args.stats;
        if let Err(e) = self.sim.set_settings(s) {
            log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
        }
    }
}

/// Schaltet Leistungsanzeige bzw. Fluss-Fassung in `s`.
fn dev_settings(c: DevCmd, s: &mut Settings) {
    if c == DevCmd::River {
        s.river = s.next_river(river_start()).to_string();
        log::info("🚦 Fluss-Fassung umgeschaltet", &[("fassung", &s.river)]);
    } else {
        s.stats = !s.stats;
        log::info("🚦 Leistungsanzeige", &[("an", &s.stats)]);
    }
}

/// Beendet den Halt bzw. schaltet den Zeitraffer eine Stufe weiter (über den ersten Spieler, raumweit).
fn dev_speed(ds: &mut dyn DevSession, first: Option<i64>, scale: i64) -> Result<(), String> {
    if ds.halted() {
        ds.halt(false);
        return Ok(());
    }
    let Some(player) = first else {
        return Ok(());
    };
    let a = DevAction {
        action: "timescale".to_string(),
        factor: next_timescale(scale),
        ..DevAction::default()
    };
    ds.dev(player, &a)
}

#[cfg(test)]
#[path = "devkeys_tests.rs"]
mod devkeys_tests;

#[cfg(test)]
#[path = "devtable_tests.rs"]
mod devtable_tests;

#[cfg(test)]
#[path = "devwinhost_tests.rs"]
mod devwinhost_tests;
