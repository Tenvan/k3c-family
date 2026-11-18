//! Port von `native/k3c-game/game/play.go`: der Takt im laufenden Spiel (kein Screen offen).

use std::mem::take;

use super::Game;
use super::devices::{rebinds, seat_of};
use super::stack::Mode;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Cue, CueKind, END_DELAY, Outcome};
use crate::view::input::{Action, Command, DeviceID, Input};
use crate::view::story::END_HOLD_MAX;
use k3c_platform::log;

impl Game {
    /// Bedient das laufende Spiel (kein Screen offen): Overlays (Skill-Menü) zuerst, dann Pause bei `Action::Menu` bzw.
    /// `Action::Home`, verwaiste Plätze nach der Zuordnungs-Regel, Befehle der Geräte an ihre Spieler und Beitritt
    /// neuer Geräte mit Bestätigen. Verbrauchte Spieler bekommen einen leeren Befehl, ihre Aktionen zählen nicht. B ist
    /// im Spiel unbelegt. Endet das Spiel (Absturz oder Spielende), liegt danach das Hauptmenü oben und dieses Update
    /// tut sonst nichts.
    pub(super) fn play(&mut self, input: &Input) {
        if !self.sim.active() || self.ending() {
            self.end_in = 0;
            let main = self.parts.menus.main();
            self.stack.push(main);
            self.mode = Mode::Menu;
            return;
        }
        let consumed = self.overlays(input);
        if self.wants_pause(input, &consumed) {
            self.sim.pause(true);
            let pause = self.parts.menus.pause();
            self.stack.push(pause);
            self.mode = Mode::Menu;
            return;
        }
        self.rebind(input);
        for i in 0..self.frame.seats.len() {
            let s = self.frame.seats[i];
            let di = input.devices.iter().find(|di| di.device == s.device);
            let c = match di {
                Some(di) if !is_consumed(&consumed, s.player) => di.command,
                _ => Command::default(),
            };
            if let Err(e) = self.sim.command(s.player, c) {
                log::warn("🚫 Befehl abgelehnt", &[("player", &s.player), ("err", &e)]);
            }
        }
        self.join(input);
    }

    /// Zählt die Takte im Spiel, seit der Frame einen Ausgang trägt (`Frame::outcome`), und schließt das Spiel nach
    /// `END_DELAY`, frühestens wenn kein `EndHolder` mehr etwas zeigt (höchstens `END_HOLD_MAX` länger;
    /// `Session::close` speichert; nach einer Niederlage speichert der Raum nicht mehr). Ein Frame ohne Stufe (Menü,
    /// beendetes Spiel) setzt den Zähler zurück, damit ein späteres Spiel neu zählt. `true`: Spiel geschlossen.
    pub(super) fn ending(&mut self) -> bool {
        if self.frame.outcome == Outcome::None || self.frame.stages.is_empty() {
            self.end_in = 0;
            return false;
        }
        self.end_in += 1;
        if self.end_in < END_DELAY || (self.end_in < END_DELAY + END_HOLD_MAX && self.end_held()) {
            return false;
        }
        if let Err(e) = self.sim.close() {
            log::warn("💥 Spiel nach Spielende nicht sauber beendet", &[("err", &e)]);
        }
        log::info(
            "🛑 Spielende, zurück ins Hauptmenü",
            &[("ausgang", &format!("{:?}", self.frame.outcome))],
        );
        true
    }

    /// Meldet, ob ein Listener am Spielende noch etwas zeigt (`EndHolder`: Geschichte zum Sieg, B-974).
    fn end_held(&self) -> bool {
        self.parts.end_holders.iter().any(|h| h.end_holding())
    }

    /// Ruft alle Overlays in Anmelde-Reihenfolge und vereinigt die verbrauchten Spieler; ihre Cues kommen in den
    /// nächsten Frame. Ein Spieler, den ein Overlay verbraucht, fehlt in der Eingabe der späteren (Cheat-Menü vor
    /// Skill-Menü).
    fn overlays(&mut self, input: &Input) -> [bool; MAX_PLAYERS] {
        let mut consumed = [false; MAX_PLAYERS];
        let mut os = take(&mut self.parts.overlays);
        for o in &mut os {
            let st = if consumed.contains(&true) {
                let hidden = self.hide_consumed(input, &consumed);
                let st = o.update(&hidden, &self.frame, self.sim.as_mut());
                self.in_buf = hidden.devices; // Puffer zurück (gilt bis zum nächsten Aufruf)
                st
            } else {
                o.update(input, &self.frame, self.sim.as_mut())
            };
            self.cues.extend(st.cues);
            for (c, n) in consumed.iter_mut().zip(st.consumed) {
                *c = *c || n;
            }
        }
        self.parts.overlays = os;
        consumed
    }

    /// Liefert `input` ohne die Geräte verbrauchter Spieler; die Liste nutzt den Puffer `in_buf` (nur mit einem
    /// verbrauchten Spieler gerufen, der Aufrufer legt ihn danach zurück).
    fn hide_consumed(&mut self, input: &Input, consumed: &[bool; MAX_PLAYERS]) -> Input {
        let mut devices = take(&mut self.in_buf);
        devices.clear();
        for di in &input.devices {
            if !is_consumed(consumed, seat_of(&self.frame.seats, di.device)) {
                devices.push(di.clone());
            }
        }
        Input { devices }
    }

    /// Meldet `Action::Menu` oder `Action::Home` eines Geräts, dessen Spieler nicht im Overlay ist.
    fn wants_pause(&self, input: &Input, consumed: &[bool; MAX_PLAYERS]) -> bool {
        input.devices.iter().any(|di| {
            !is_consumed(consumed, seat_of(&self.frame.seats, di.device))
                && (di.actions.contains(&Action::Menu) || di.actions.contains(&Action::Home))
        })
    }

    /// Gibt verwaisten Plätzen ein freies Gerät (Neuzuordnung) über die Session.
    fn rebind(&mut self, input: &Input) {
        self.devs.rebinds.clear();
        rebinds(&mut self.devs.rebinds, &self.frame.seats, input, &self.devs.fresh);
        for i in 0..self.devs.rebinds.len() {
            let r = self.devs.rebinds[i];
            if let Err(e) = self.sim.rebind(r.player, r.device) {
                self.cues.push(Cue {
                    kind: CueKind::Denied,
                    player: r.player,
                    text: e.to_string(),
                    ..Cue::default()
                });
            }
        }
    }

    /// Nimmt freie Geräte, die Bestätigen drücken und nicht gerade einen verwaisten Platz übernommen haben, als
    /// weitere Spieler auf.
    fn join(&mut self, input: &Input) {
        for di in &input.devices {
            if seat_of(&self.frame.seats, di.device) >= 0
                || !di.actions.contains(&Action::Confirm)
                || self.rebound(di.device)
            {
                continue;
            }
            match self.sim.join(di.device) {
                Err(e) => self.cues.push(Cue {
                    kind: CueKind::Denied,
                    player: -1,
                    text: e.to_string(),
                    ..Cue::default()
                }),
                Ok(p) => log::info("👑 Gerät spielt mit", &[("device", &di.device), ("player", &p)]),
            }
        }
    }

    /// Meldet, ob das Gerät in diesem Update einen verwaisten Platz bekommen hat.
    fn rebound(&self, d: DeviceID) -> bool {
        self.devs.rebinds.iter().any(|r| r.device == d)
    }
}

/// Meldet, ob Spieler `p` von einem Overlay verbraucht ist.
fn is_consumed(consumed: &[bool; MAX_PLAYERS], p: i64) -> bool {
    usize::try_from(p).is_ok_and(|i| consumed.get(i).copied().unwrap_or(false))
}
