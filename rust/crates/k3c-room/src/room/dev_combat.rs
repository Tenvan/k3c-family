//! Kampf-Aktionen des Dev-Modus (B-603, B-766, B-921). Port von `engine/room/dev_combat.go`.
use k3c_sim::dev::combat::{dev_boss_phase, dev_spawn_boss, dev_start_event};
use k3c_sim::dev::dev_set_phase;
use k3c_sim::dev::spawn::{dev_spawn_at_castle, dev_spawn_units};
use k3c_sim::state::Fehler as SimFehler;

use super::dev::{Attrs, DevAction, SlotMap, attr};
use super::env::Env;
use super::log::Level;
use super::types::Room;

impl Room {
    /// Führt die Kampf-Aktionen aus und liefert die Log-Felder (`None`: abgelehnt, bad_request): `boss` stellt den Boss an
    /// das nächste Portal der Stufe des Monarchen von `slot`, `bossPhase` schaltet den laufenden Endboss eine Phase weiter,
    /// `event` startet ein Event (Mond-Events in der nächsten Nacht, zu deren Beginn alle Stufen springen), `night` springt
    /// zur Nacht, `spawn` und `spawnCastle` stellen Figuren um den Monarchen bzw. um die Burg (Stressmodus, B-921). Lebt
    /// schon ein Boss, ist `boss` abgelehnt mit Hinweis im Log.
    pub(super) fn dev_combat(&mut self, env: &mut Env, who: &str, a: &DevAction, map: &SlotMap) -> Option<Attrs> {
        match a.action.as_str() {
            "boss" => {
                let (idx, slot) = (map.index(a)?, a.slot?);
                let res = dev_spawn_boss(&mut self.isl, &env.data.difficulty, idx, &a.boss);
                if matches!(res, Err(SimFehler::DevBossPresent)) {
                    self.log(
                        env,
                        Level::Warn,
                        "🚫 Dev-Aktion abgelehnt: Boss schon da",
                        &[("device", &who), ("aktion", &a.action), ("boss", &a.boss)],
                    );
                }
                res.ok().map(|()| vec![attr("slot", slot), attr("boss", &a.boss)])
            }
            "spawn" | "spawnCastle" => {
                let (idx, slot) = (map.index(a)?, a.slot?);
                let spawn = if a.action == "spawnCastle" {
                    dev_spawn_at_castle
                } else {
                    dev_spawn_units
                };
                let n = spawn(&mut self.isl, &env.data.difficulty, idx, &a.kind, &a.unit, a.amount).ok()?;
                Some(vec![
                    attr("slot", slot),
                    attr("art", &a.kind),
                    attr("sorte", &a.unit),
                    attr("anzahl", n),
                ])
            }
            "bossPhase" => dev_boss_phase(&mut self.isl).map(|phase| vec![attr("phase", phase)]),
            "event" => {
                dev_start_event(&mut self.isl, &env.data.difficulty, &a.event).then(|| vec![attr("event", &a.event)])
            }
            "night" => dev_set_phase(&mut self.isl, "night").then(|| vec![attr("phase", "night")]),
            _ => None,
        }
    }
}
