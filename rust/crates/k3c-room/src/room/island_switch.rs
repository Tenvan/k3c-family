//! Inselwechsel im Raum (B-345, `docs/rules/stufen.md` § 1). Port von `island_switch.go`: Meldet die Sim `switch_ready`,
//! ersetzt der Raum die Insel im selben Takt durch die nächste. Monarchen und Slots bleiben (Spieler-Index = Monarch),
//! jedes Gerät bekommt Level und vollen Zustand jeder Stufe neu (kein Delta über den Wechsel), der Spielstand enthält
//! die neue Insel.
use k3c_sim::island::next_island;

use super::env::Env;
use super::log::Level;
use super::types::Room;

impl Room {
    /// Gos `switchIsland`: tauscht die Insel, wenn der Wechsel bereit ist und es in der Welt des Raums eine nächste gibt.
    /// `true`: Die Insel wurde getauscht.
    pub fn switch_island(&mut self, env: &mut Env) -> bool {
        if !self.isl.switch_ready {
            return false;
        }
        let Some(def) = self.isl.next_def() else {
            return false;
        };
        let mut next = match next_island(env.data, &self.isl, &def) {
            Ok(next) => next,
            Err(e) => {
                self.log(env, Level::Error, "💥 Inselwechsel fehlgeschlagen", &[("err", &e)]);
                return false;
            }
        };
        let (von, nach, tick) = (self.isl.number, next.number, self.tick);
        self.log(
            env,
            Level::Info,
            "🪜 Inselwechsel",
            &[("von", &von), ("nach", &nach), ("tick", &tick)],
        );
        next.dev = self.isl.dev.clone();
        next.dev.nights.clear(); // erzwungene Dev-Nächte gehören zur alten Insel, ihr Tageszähler gilt dort nicht (Review KB4)
        self.isl = next;
        self.start = 0;
        for d in self.devices.values_mut() {
            d.stages.clear();
        }
        self.broadcast_seats();
        true
    }
}
