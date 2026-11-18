//! Vorgezogen aus RU24: die kleinen Teile von Pause (`pause.go`), Zeitraffer (`dev.go`), Speichern (`saver.go`) und
//! Beobachtern (`observer.go`), ohne die `Room::tick` nicht laufen kann. Sie rechnen wie Go; RU24 ergänzt den Rest
//! (Pause-Aktion, Dev-Aktionen, Hintergrund-Schreiber, Beobachter) und bewegt sie in ihre Dateien.
use super::env::Env;
use super::log::{Level, short};
use super::types::{MonarchState, Room, SAVE_TICKS, WAIT_FOR_SECS};

impl Room {
    /// Pausiert das verbundene Gerät `id`?
    pub fn device_paused(&self, id: &str) -> bool {
        self.devices.get(id).is_some_and(|d| d.connected && d.paused)
    }

    /// Couch-Pause: genau ein Gerät mit Platz ist verbunden und pausiert (B-214).
    pub fn couch_paused(&self) -> bool {
        let mut seated = self.devices.values().filter(|d| d.connected && !d.observer);
        match (seated.next(), seated.next()) {
            (Some(d), None) => d.paused,
            _ => false,
        }
    }

    /// Zeitraffer-Faktor, mindestens 1.
    pub fn scale(&self) -> i64 {
        self.timescale.max(1)
    }

    /// Schritte der Insel je Tick: im Zeitraffer `scale()`, bei Dev- oder Couch-Pause keiner.
    pub fn steps(&self) -> i64 {
        if self.paused || self.couch_paused() {
            return 0;
        }
        self.scale()
    }

    /// Beendet den automatischen Zeitraffer, sobald die Tageszeit wechselt.
    pub fn reset_scale_on_phase(&mut self, env: &mut Env) {
        let phase = self.isl.stages[self.start].cycle.phase.clone();
        if !self.auto_reset || phase == self.reset_phase {
            return;
        }
        let factor = self.timescale;
        self.log(
            env,
            Level::Info,
            "⏩ Zeitraffer endet mit der Tageszeit",
            &[("faktor", &factor), ("phase", &phase)],
        );
        self.timescale = 1;
        self.auto_reset = false;
    }

    /// Schutz der Pause: Spieler eines pausierten Geräts sind geschützt (nicht bei Couch-Pause), der Schutz zählt ab.
    pub fn pause_tick(&mut self, env: &mut Env) {
        let couch = self.couch_paused();
        let protected: Vec<bool> = self
            .monarchs
            .iter()
            .map(|mo| {
                let d = self.devices.get(&mo.device);
                !couch
                    && mo.state == MonarchState::Taken
                    && self.device_paused(&mo.device)
                    && d.is_some_and(|d| d.protect > 0)
            })
            .collect();
        for p in self.isl.stages.iter_mut().flat_map(|w| w.players.iter_mut()) {
            if let Some(&on) = usize::try_from(p.index).ok().and_then(|i| protected.get(i)) {
                p.protected = on;
            }
        }
        if couch || self.paused {
            return;
        }
        let mut ended = Vec::new();
        for (id, d) in self.devices.iter_mut() {
            if !d.connected || !d.paused || d.protect == 0 {
                continue;
            }
            d.protect -= 1;
            if d.protect == 0 {
                ended.push(short(id));
            }
        }
        for device in ended {
            let frist = format!("{}m0s", WAIT_FOR_SECS / 60);
            self.log(
                env,
                Level::Info,
                "⏳ Schutz der Pause endet",
                &[("device", &device), ("frist", &frist)],
            );
        }
    }

    /// Beobachter mit Verbindung (`observer`, B-352).
    pub fn watching(&self) -> usize {
        self.devices.values().filter(|d| d.connected && d.observer).count()
    }

    /// Zählt einen Tick für das Speichern; im Takt `SAVE_TICKS`, bei Stufenwechsel und bei Tagesanbruch wird gespeichert.
    /// Vorläufig synchron (`save`), RU24 ersetzt es durch den Hintergrund-Schreiber.
    pub fn autosave(&mut self, env: &mut Env, travelled: bool, dawn: bool) {
        self.save_due += 1;
        if travelled || dawn || self.save_due >= SAVE_TICKS {
            self.save(env);
        }
    }

    /// Sichert den Spielstand (Go `save`): Zähler zurück, nach Game Over nichts. Vorläufig synchron, RU24 schreibt im
    /// Hintergrund.
    pub fn save(&mut self, env: &mut Env) {
        self.save_due = 0;
        self.save_sync(env);
    }

    /// Schickt Zustände; ein „saved“-Ereignis nach einer Hintergrund-Speicherung kommt mit RU24 (ohne Schreiber ist nie
    /// etwas „geschrieben“).
    pub fn push_states_saved(&mut self, env: &mut Env) -> bool {
        self.push_states(env)
    }
}
