//! Insel (Port von `engine/sim/island.go`, Entscheidung 003, B-100): Ein Level mit mehreren Stufen. Alle Stufen laufen im
//! selben Takt weiter, auch ohne Spieler; jeder Spieler ist an genau eine Stufe gebunden. Das Baumaterial gehört der Insel
//! (ein Vorrat für alle Stufen), Gold gehört dem Spieler.
//!
//! Zustandsmodell (`docs/standards/rust-sim-zustand.md`): Die Insel besitzt ihre Stufen und den Vorrat. Vor dem Schritt einer
//! Stufe tauscht sie den Vorrat per `std::mem::swap` in die Stufe (Entscheidung 5) und reicht ihr einen `IslandRef`
//! (`stage_ref`, Entscheidung 4); danach geht beides zurück (`put_stage_ref`).

use std::collections::BTreeMap;

use k3c_data::difficulty::DifficultyFile;
use k3c_data::hub::Stock;
use k3c_data::waves::WavesDataDepthScaling;

use std::sync::Arc;

use crate::boss::endboss::EndbossFight;
use crate::state::island_ref::{IslandEvents, IslandView};
use crate::state::links::{DevFlags, values};
use crate::state::{Event, IslandRef, PlayerCommand, World, WorldValues};
use crate::world::state_mirror::{mirror_state, mirror_switch};
use crate::world::{add_player_at, step};

pub mod create;
pub mod data;
pub mod defeat;
pub mod endboss;
pub mod errand;
pub mod harbor_start;
pub mod keys;
pub mod nature;
pub mod options;
pub mod save;
pub mod save_load;
pub mod save_parse;
pub mod save_v3;
pub mod save_v5;
pub mod save_v8;
pub mod sizes;
pub mod storage;
pub mod switch;
pub mod targets;
pub mod travel;
pub mod victory;
pub mod worlds;
pub mod worlds_decode;

#[cfg(test)]
mod carry_tests;
#[cfg(test)]
mod defeat_tests;
#[cfg(test)]
mod depths_tests;
#[cfg(test)]
mod endboss_tests;
#[cfg(test)]
mod errand_tests;
#[cfg(test)]
mod harbor_start_tests;
#[cfg(test)]
mod nature_tests;
#[cfg(test)]
mod options_tests;
#[cfg(test)]
mod planned_tests;
#[cfg(test)]
mod planned_world_tests;
#[cfg(test)]
mod save_boss_tests;
#[cfg(test)]
mod save_crown_tests;
#[cfg(test)]
mod save_null_tests;
#[cfg(test)]
mod save_planned_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod save_v3_tests;
#[cfg(test)]
mod save_v4_tests;
#[cfg(test)]
mod save_v8_tests;
#[cfg(test)]
mod sizes_tests;
#[cfg(test)]
mod start_tests;
#[cfg(test)]
mod storage_tests;
#[cfg(test)]
mod switch_etappe_tests;
#[cfg(test)]
mod switch_tests;
#[cfg(test)]
pub(crate) mod switch_testutil;
#[cfg(test)]
mod switch_world_tests;
#[cfg(test)]
mod takt_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
#[cfg(test)]
mod travel_tests;
#[cfg(test)]
mod victory_tests;
#[cfg(test)]
mod view_tests;
#[cfg(test)]
mod worlds_error_tests;
#[cfg(test)]
mod worlds_etappe_tests;
#[cfg(test)]
mod worlds_tests;

pub use create::{CreateOptions, IslandBuild, create_island, create_island_at, create_island_in};
pub use data::IslandDef;
pub use options::{IslandOptions, default_options, default_options_for, set_grade, set_options, validate_options};
pub use save::{ISLAND_SAVE_VERSION, IslandPlayerSave, IslandSave, MerchantSave, SaveTables};
pub use save_load::{from_island_save, from_island_save_in, saved_create_options};
pub use save_parse::{parse_island_save, validate_island_save};
pub use switch::{next_island, step_island_switch};
pub use targets::choose_island;
pub use travel::{IslandTravel, insert_player, remove_player, step_island_travel};
pub use victory::check_victory;
pub use worlds::{STANDARD_WORLD_ID, WorldDef, load_world, parse_world, standard_world};

/// Go: `Island`, eine Insel mit ihren Stufen. Ohne JSON: der Spielstand ist `IslandSave` (`save.rs`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Island {
    /// Gleiche ID = gleiches Spiel (Spielstand).
    pub id: String,
    pub seed: String,
    pub cycle_speed: f64,
    /// Stufe i ist die Welt der i-ten Tiefe aus `create_island`.
    pub stages: Vec<World>,
    /// Vorrat der Insel; kein Zeiger wie in Go: Während eine Stufe rechnet, liegt er in ihr (`step_island`).
    pub stock: Stock,
    /// Grad, Ziel, Niederlage-Modus (`options.rs`).
    pub options: IslandOptions,
    /// Fund-Pool der Insel (monarch.md § 3), alle Stufen spiegeln ihn nach `World::skill_points`.
    pub skill_pool: i64,
    /// Geöffnete Truhen aller Stufen (jede n-te gibt einen Pool-Punkt).
    pub chests_opened: i64,
    /// IDs der besiegten Bosse in Reihenfolge des Siegs (boss.go); ein besiegter Boss kommt nie wieder.
    pub defeated_bosses: Vec<String>,
    /// Der Endboss der Insel ist besiegt (Siegvariante, Inselwechsel).
    pub endboss_defeated: bool,
    /// Der laufende Kampf gegen den Endboss (boss_endboss.go); `None`, solange er wartet oder nach dem Sieg.
    pub endboss: Option<EndbossFight>,
    /// Münzen, die Spieler auf der Insel aufgehoben haben (Siegvariante gold, island_victory.go).
    pub gold_collected: i64,
    /// Die Insel hat ihr Ziel erreicht, `victory` kam schon.
    pub won: bool,
    /// Es stand ein endliches Ressourcenobjekt (mineAll). Nur im Speicher.
    pub had_finite: bool,
    /// Die Welt der Insel (welten.md, B-488).
    pub world: WorldDef,
    /// Ihre Insel-ID darin (Spielstand, welten.md § 5).
    pub island_id: String,
    /// Index der Insel in ihrer Welt (0 = Insel 1).
    pub number: i64,
    /// Tabelle der Gegner-Skalierung der Insel.
    pub scaling: WavesDataDepthScaling,
    /// Wechselpunkt offen (island_switch.go).
    pub gate_open: bool,
    /// Alle stehen am Wechselpunkt, der Raum tauscht die Insel.
    pub switch_ready: bool,
    pub switch_progress: f64,
    /// IDs der abgeschlossenen Inseln der Welt (Endboss beim Ablegen besiegt, B-961).
    pub completed: Vec<String>,
    /// Gewähltes Ziel am Hafen (leer = erstes Ziel).
    pub chosen: String,
    /// Wahlen des Takts je Spielerindex (island_targets.go); nur über den Schlüssel zugreifen.
    pub pending_choice: BTreeMap<i64, String>,
    /// Niederlage-Modus „Komplett verloren“ (defeat.rs): Die Insel ist zu Ende, `step_island` ändert nichts mehr.
    pub over: bool,
    /// Schalter der Dev-Aktionen (dev.go), nur im Speicher.
    pub dev: DevFlags,
    /// Ankünfte des Händlers auf der Insel (merchant.go), Rhythmus des Händler-Überfalls (K3.2).
    pub merchant_visits: i64,
    pub next_player: i64,
    /// Reisefortschritt je Spielerindex (island_travel.go); nur über den Schlüssel zugreifen.
    pub travel: BTreeMap<i64, IslandTravel>,
    /// Fracht auf dem Schiff (RU62.1, B-967); ohne `cfg` trägt die Insel keine.
    pub cargo: crate::cargo::Cargo,
    /// Der Einsiedler der Insel (RU62.10, B-972); ohne `cfg` hat die Insel keinen.
    pub hermit: crate::hermit::Hermit,
    /// Das Fest der Insel (RU62.12, B-973); ohne Eintrag in festival.json trägt die Insel keines.
    pub festival: crate::festival::Festival,
}

impl Island {
    /// Go: `WorldID`, die ID der Welt der Insel (Spielstand `world`, welten.md § 5).
    pub fn world_id(&self) -> &str {
        &self.world.id
    }

    /// Go: `IslandID`, die ID der Insel in ihrer Welt (Spielstand `islandId`).
    pub fn island_id(&self) -> &str {
        &self.island_id
    }

    /// Anzahl aller Spieler der Insel (Go: `len(isl.Players())`, die Liste ist `players()`).
    pub fn player_count(&self) -> i64 {
        self.stages.iter().map(|w| w.players.len() as i64).sum()
    }

    /// Go: `values`, der Wertesatz der Insel, gleich dem ihrer ersten Stufe (ohne Stufen der Standard-Wertesatz).
    pub(crate) fn values(&self) -> Arc<WorldValues> {
        match self.stages.first() {
            Some(w) => w.vals.clone().unwrap_or_else(|| values(w)),
            None => Arc::default(),
        }
    }

    /// Go: `isl.values().Waves.PerExtraPlayer`: Zuschlag der Wellengröße je weiteren Spieler aus dem Wertesatz der Insel.
    pub(crate) fn per_extra_player(&self) -> f64 {
        self.values().waves.per_extra_player
    }

    /// Gebaute Lager aller Stufen außer `i` (Lager-Maximum der Insel, `storage.rs`).
    fn storages_except(&self, i: usize) -> i64 {
        let others = self.stages.iter().enumerate().filter(|(j, _)| *j != i);
        others.map(|(_, w)| built_storages(w)).sum()
    }

    /// Der Bezug der Stufe `i` zur Insel für ihren Schritt (Vorentscheidung 2): Tabellen und Zahlen werden kopiert, die
    /// veränderlichen Teile (`defeated_bosses`, `endboss`) herausgenommen; `put_stage_ref` legt sie zurück. Der Grad kommt
    /// aus `difficulty` (Go: Paket-Variable `difficulty`); ein unbekannter Grad ergibt wie bei Gos Map-Zugriff Nullwerte.
    pub fn stage_ref(&mut self, i: usize, difficulty: &DifficultyFile) -> IslandRef {
        let per_extra_player = self.per_extra_player();
        IslandRef {
            scaling: self.scaling.clone(),
            grade: difficulty
                .grades
                .get(&self.options.grade)
                .map(Into::into)
                .unwrap_or_default(),
            players: self.player_count(),
            per_extra_player,
            cheap_build: self.dev.cheap_build,
            stage: i,
            defeat_resources: self.options.defeat == "resources",
            defeated_bosses: std::mem::take(&mut self.defeated_bosses),
            endboss: self.endboss.take(),
            endboss_defeated: self.endboss_defeated,
            over: false,
            halve_other_gold: false,
            stages: self.stages.len() as i64,
            storages_elsewhere: self.storages_except(i),
            stock_bonus: self.dev.stock_bonus,
            gold_collected: 0,
            skill_pool: self.skill_pool,
            chests_opened: self.chests_opened,
            merchant_visits: self.merchant_visits,
            tavern_elsewhere: self.tavern_except(i),
            gear_above: errand::gear_above(self, i),
        }
    }

    /// Steht in einer anderen Stufe als `i` eine gebaute Taverne (Go: `tavernOnIsland` über `isl.Stages`)?
    fn tavern_except(&self, i: usize) -> bool {
        let built = |w: &World| w.sites.iter().any(|s| s.kind == "tavern" && s.state == "built");
        self.stages.iter().enumerate().any(|(j, w)| j != i && built(w))
    }

    /// Go: alle Stufen zeigen auf denselben Vorrat. Rust spiegelt den Vorrat der Insel in jede Stufe, damit `to_save` und
    /// Leser zwischen den Schritten den echten Wert sehen (der Tausch im Schritt bleibt, Entscheidung 5).
    pub fn sync_stock(&mut self) {
        let total: i64 = self.stages.iter().map(built_storages).sum();
        let (stages, stock_bonus, cheap_build) = (self.stages.len() as i64, self.dev.stock_bonus, self.dev.cheap_build);
        let boss_gone = self.endboss.is_some() || self.endboss_defeated;
        for (i, w) in self.stages.iter_mut().enumerate() {
            w.stock.clone_from(&self.stock);
            w.island_events = Some(IslandEvents {
                first_stage: i == 0,
                nights: self.dev.nights.clone(),
            });
            // Lesende Sicht für HUD und Anzeige (B-1125): `IslandView::to_ref` baut daraus den `IslandRef` ohne Allokation
            let lair_closed = boss_gone
                || w.lair_boss
                    .as_ref()
                    .is_some_and(|b| self.defeated_bosses.contains(&b.id));
            w.island_view = Some(IslandView {
                stage: i,
                stages,
                storages_elsewhere: total - built_storages(w),
                stock_bonus,
                cheap_build,
                lair_closed,
            });
        }
    }

    /// Legt den Bezug nach dem Schritt einer Stufe zurück und übernimmt, was die Stufe an der Insel geändert hat.
    pub fn put_stage_ref(&mut self, r: IslandRef) {
        self.defeated_bosses = r.defeated_bosses;
        self.endboss = r.endboss;
        self.endboss_defeated = r.endboss_defeated;
        self.gold_collected += r.gold_collected;
        self.chests_opened = r.chests_opened;
        self.merchant_visits = r.merchant_visits;
        if r.skill_pool != self.skill_pool {
            // `add_pool_points`: alle Stufen spiegeln den Pool nach `skill_points` (der Spielstand speichert ihn)
            self.skill_pool = r.skill_pool;
            self.stages.iter_mut().for_each(|w| w.skill_points = r.skill_pool);
        }
        if r.over {
            self.over = true;
        }
        if r.halve_other_gold {
            let stage = r.stage;
            for (j, w) in self.stages.iter_mut().enumerate() {
                if j != stage {
                    w.players.iter_mut().for_each(|p| p.gold /= 2);
                }
            }
        }
    }
}

/// Go: `AddIslandPlayer`: stellt einen neuen Monarchen an die Burg der Stufe `stage`; liefert seinen Platz in
/// `stages[stage].players`. Der Index zählt inselweit hoch und bleibt auch nach einem Stufenwechsel derselbe (Eingaben
/// werden über ihn zugeordnet).
pub fn add_island_player(isl: &mut Island, stage: usize) -> usize {
    let pi = add_player_at(&mut isl.stages[stage], isl.next_player);
    let index = isl.next_player;
    let v = crate::world::vals(&isl.stages[stage]);
    if let Some(x) = harbor_start::start_x(&isl.stages[stage], &v, index) {
        isl.stages[stage].players[pi].x = x; // HB1.5: neue Spiele starten am Hafen
    }
    isl.next_player += 1;
    pi
}

/// Go: `StepIsland`: rechnet einen Tick in allen Stufen, von Stufe 0 aufwärts. `commands` wird nach dem Spielerindex der
/// Insel gelesen (`commands[p.index]`); jede Stufe bekommt dasselbe Feld und nimmt sich ihre Spieler heraus.
pub fn step_island(isl: &mut Island, commands: &[PlayerCommand], difficulty: &DifficultyFile, dt: f64) {
    if isl.over {
        // Komplett verloren: alle Stufen stehen, keine Ereignisse mehr
        for w in &mut isl.stages {
            w.events.clear();
        }
        return;
    }
    isl.prepare_coop(); // RU62.3: zwei gesteuerte Monarchen auf der Insel? (Doppel-Truhe)
    isl.prepare_weather(); // RU62.5: Wetter-Zustand und Taifun-Faktoren je Stufe
    isl.prepare_hermit(); // RU62.10: Hütte offen? Bonus des Bauwerks je Stufe
    isl.prepare_festival(); // RU62.12: Ziel Endboss? (Frieden und Fest)
    for i in 0..isl.stages.len() {
        let mut r = isl.stage_ref(i, difficulty);
        std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock); // Entscheidung 5: der Vorrat gehört der Stufe, solange sie rechnet
        step(&mut isl.stages[i], commands, Some(&mut r), dt);
        std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
        isl.put_stage_ref(r);
    }
    errand::step_errands(isl, dt); // ECO1.7: Bürger gehen für Ausrüstung nach oben und zurück
    isl.sync_stock();
    crate::hermit::step::step_hermit(isl, dt); // RU62.10: Einwurf an der Hütte startet den Bau im Hub
    crate::festival::step_festival(isl); // RU62.12: Sieg über den Endboss: Fest-Zustand und festivalStart
    if !isl.over {
        crate::cargo::step::step_cargo(isl, commands, dt);
        step_island_travel(isl, dt);
        step_island_switch(isl, dt);
        mirror_switch(isl);
        check_victory(isl);
    }
    for (i, w) in isl.stages.iter_mut().enumerate() {
        mirror_state(w, i, isl.endboss.as_ref());
    }
    for (i, w) in isl.stages.iter_mut().enumerate() {
        // nach dem Wechsel, damit auch `arrived` seine Stufe trägt
        for ev in &mut w.events {
            tag_stage(ev, i);
        }
    }
}

/// Setzt `stage` im Ereignis.
fn tag_stage(ev: &mut Event, stage: usize) {
    ev.insert("stage".to_string(), serde_json::Value::from(stage as i64));
}

/// Gebaute Lager einer Stufe (Go: `Kind == "storage" && State == "built"`).
fn built_storages(w: &World) -> i64 {
    w.sites
        .iter()
        .filter(|s| s.kind == "storage" && s.state == "built")
        .count() as i64
}
