//! Port von `native/k3c-game/local/dev.go`: Entwickler-Rolle der Session (`view::DevSession`, B-888): Dev-Aktionen über den
//! Raum (Cheat-Menü, Zeitraffer), Halt mit Einzelschritt (F7) und Neu-Laden ohne Speichern (F6). `running` und `timescale`
//! gehören dazu und stehen hier (sie lagen bis RU50.1 in `step.rs`).

use k3c_room::room::{DevAction as RoomDevAction, Log as _, Options};

use super::error::{UserError, err_no_game, user_err};
use super::session::{CONN, DEVICE_ID, Session};
use crate::view::devkeys::{DevAction, DevSession};

impl DevSession for Session {
    /// Go: `Dev`: führt die Dev-Aktion `a` für den Spieler aus (`Room::dev_local`, ohne Dev-Mode des Managers: das native
    /// Spiel läuft nur lokal; der Raum loggt sie mit 🐛).
    fn dev(&mut self, player: i64, a: &DevAction) -> Result<(), String> {
        self.seated(player).map_err(|e| e.msg)?;
        let ra = RoomDevAction {
            action: a.action.clone(),
            slot: Some(player),
            amount: a.amount,
            resource: a.resource.clone(),
            factor: a.factor,
            paused: None,
            phase: a.phase.clone(),
            grade: a.grade.clone(),
            on: a.on,
            level: a.level,
            boss: a.boss.clone(),
            event: a.event.clone(),
            kind: a.kind.clone(),
            unit: a.unit.clone(),
        };
        self.room_call(|r, env| r.dev_local(env, DEVICE_ID, CONN, &ra))
            .map_err(|_| format!("Cheat „{}“ geht gerade nicht", a.action))
    }

    /// Go: `Halt`: hält die Simulation an bzw. lässt sie weiterlaufen (unabhängig von der Pause des Menüs).
    fn halt(&mut self, on: bool) {
        if self.halt == on {
            return;
        }
        (self.halt, self.single) = (on, false);
        self.log.info("🚦 Halt", &[("an", &on)]);
    }

    fn halted(&self) -> bool {
        self.halt
    }

    /// Go: `StepOnce`: lässt im Halt den nächsten Step genau einen Takt rechnen.
    fn step_once(&mut self) {
        if self.halt {
            self.single = true;
        }
    }

    fn reload(&mut self) -> Result<(), String> {
        self.reload_game().map_err(|e| e.msg)
    }
}

impl Session {
    /// Go: `Reload`: verwirft das laufende Spiel ohne Speichern und lädt seinen zuletzt gespeicherten Stand (F5 oder
    /// Autosave) mit denselben Geräten.
    fn reload_game(&mut self) -> Result<(), UserError> {
        if self.m.is_none() {
            return Err(err_no_game());
        }
        let (name, devs) = (self.name.clone(), self.devices());
        if !self.loadable(&name) {
            return Err(user_err(
                "Noch kein ladbarer Spielstand dieses Spiels (F5 speichert)",
                None,
            ));
        }
        self.reset();
        self.start(&name, false, 0, Options::default(), &devs)?;
        self.log.info("📂 Spielstand neu geladen", &[("save", &name)]);
        Ok(())
    }

    /// Go: `loadable`: Der Spielstand steht in der Liste und ist ladbar.
    pub(super) fn loadable(&mut self, name: &str) -> bool {
        self.saves_list()
            .is_ok_and(|l| l.iter().any(|e| e.name == name && e.loadable))
    }

    /// Meldet, ob `step` in diesem Aufruf einen Takt rechnet (Spiel läuft, keine Pause, kein Halt außer Einzelschritt).
    pub(super) fn running(&mut self) -> bool {
        if self.m.is_none() || self.paused {
            self.single = false; // kein Einzelschritt auf Vorrat
            return false;
        }
        if !self.halt {
            return true;
        }
        std::mem::take(&mut self.single)
    }

    /// Der Zeitraffer des Raums (1 ohne Spiel).
    pub(super) fn timescale(&mut self) -> i64 {
        self.with_room(|r, _| r.timescale()).unwrap_or(1)
    }
}
