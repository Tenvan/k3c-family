//! Seltenere Dev-Aktionen des Raums: Lager, Baukosten, Ausbau, Tageszeit, reiner Stressmodus, Zeitraffer-Auto und
//! `gold`/`material` (Port von `devExtra` und `devStock` aus `dev.go`).
use k3c_sim::dev::{
    dev_add_stock, dev_build_all, dev_drop_gold, dev_fill_stock, dev_next_phase, dev_raise_max_stock,
    dev_set_cheap_build, dev_set_pure,
};

use super::dev::{Attrs, DEV_MAX_AMOUNT, DevAction, SlotMap, attr};
use super::env::Env;
use super::types::Room;

impl Room {
    /// Führt die Aktionen für Lager, Baukosten, Ausbau, Tageszeit und Kampf aus und liefert die Log-Felder; `None`:
    /// abgelehnt.
    pub(super) fn dev_extra(&mut self, env: &mut Env, who: &str, a: &DevAction, map: &SlotMap) -> Option<Attrs> {
        match a.action.as_str() {
            "maxStock" => dev_raise_max_stock(&mut self.isl).then(|| vec![attr("bonus", self.isl.dev.stock_bonus)]),
            "fillStock" => {
                dev_fill_stock(&mut self.isl, &env.data.difficulty);
                Some(Vec::new())
            }
            "cheapBuild" => {
                let on = a.on?;
                dev_set_cheap_build(&mut self.isl, on);
                Some(vec![attr("an", on)])
            }
            "buildAll" => dev_build_all(&mut self.isl, a.level).then(|| vec![attr("stufe", a.level)]),
            "nextPhase" => Some(vec![attr("phase", dev_next_phase(&mut self.isl))]),
            "pure" => {
                // reiner Stressmodus (B-921)
                let on = a.on?;
                dev_set_pure(&mut self.isl, on);
                Some(vec![attr("an", on)])
            }
            "timescaleAuto" => {
                let on = a.on?;
                let phase = self.isl.stages.get(self.start)?.cycle.phase.clone();
                self.auto_reset = on;
                self.reset_phase = phase;
                Some(vec![attr("an", on)])
            }
            _ => self.dev_combat(env, who, a, map),
        }
    }

    /// `gold` und `material`: liefert die Log-Felder; `None`: abgelehnt.
    pub(super) fn dev_stock(&mut self, env: &Env, a: &DevAction, map: &SlotMap) -> Option<Attrs> {
        let idx = map.index(a)?;
        if a.amount < 1 || a.amount > DEV_MAX_AMOUNT {
            return None;
        }
        let mut attrs = vec![attr("slot", a.slot?), attr("amount", a.amount)];
        if a.action == "gold" {
            return dev_drop_gold(&mut self.isl, idx, a.amount).then_some(attrs);
        }
        let (taken, ok) = dev_add_stock(&mut self.isl, &env.data.difficulty, idx, &a.resource, a.amount);
        attrs.extend([attr("resource", &a.resource), attr("genommen", taken)]);
        ok.then_some(attrs)
    }
}
