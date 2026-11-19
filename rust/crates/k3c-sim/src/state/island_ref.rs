//! Bezug einer Stufe zu ihrer Insel (Go: `w.island`, Zustandsmodell Vorentscheidung 4).
//!
//! `World` hat kein Feld für die Insel. Funktionen, die in Go `w.island` lesen, bekommen `Option<&IslandRef>` bzw.
//! `Option<&mut IslandRef>` als letzten Parameter vor `dt`; `None` heißt Go `w.island == nil`. RU10 baut den
//! `IslandRef` vor dem Schritt einer Stufe aus der Insel; RU15 ergänzt besiegte Bosse und den Endboss-Kampf.

use k3c_data::buildings::Cost;
use k3c_data::difficulty::Grade;
use k3c_data::waves::WavesDataDepthScaling;

use super::World;
use crate::boss::endboss::EndbossFight;

/// Die Zahlen des Grads, die der Takt liest (Go: `Grade` ohne `DefaultDefeat`). `Copy`, damit `Island::stage_ref` je
/// Stufenschritt keinen String klont (B-1082).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GradeValues {
    pub wave_size: f64,
    pub enemy_hp: f64,
    pub enemy_damage: f64,
    pub protected_nights: i64,
}

impl From<&Grade> for GradeValues {
    fn from(g: &Grade) -> Self {
        Self {
            wave_size: g.wave_size,
            enemy_hp: g.enemy_hp,
            enemy_damage: g.enemy_damage,
            protected_nights: g.protected_nights,
        }
    }
}

/// Der Teil der Insel, den eine Stufe beim Rechnen braucht.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandRef {
    /// Tabelle der Tiefen-Skalierung der Insel (Go: `island.Scaling`, island_data.go).
    pub scaling: WavesDataDepthScaling,
    /// Der Grad der Insel (Go: `difficulty[island.Options.Grade]`).
    pub grade: GradeValues,
    /// Zahl der Spieler der Insel (Go: `len(isl.Players())`).
    pub players: i64,
    /// Zuschlag der Wellengröße je weiteren Spieler (Go: `isl.values().Waves.PerExtraPlayer`).
    pub per_extra_player: f64,
    /// Dev-Schalter `CheapBuild` der Insel (Go: `island.Dev.CheapBuild`, dev.go): Preise höchstens 1 je Posten.
    pub cheap_build: bool,
    /// Index der Stufe in der Insel (`Island::stage_ref`, RU10.1).
    pub stage: usize,
    /// Niederlage-Modus `resources` der Insel (Go: `island.Options.Defeat == "resources"`, island_defeat.go).
    pub defeat_resources: bool,
    /// Besiegte Bosse der Insel (Go: `island.DefeatedBosses`); die Insel gibt sie für den Stufenschritt her
    /// (`std::mem::take`) und bekommt sie in `put_stage_ref` zurück.
    pub defeated_bosses: Vec<String>,
    /// Der laufende Endboss-Kampf (Go: `island.endboss`), wie `defeated_bosses` herausgenommen.
    pub endboss: Option<EndbossFight>,
    /// Der Endboss der Insel ist besiegt (Go: `island.EndbossDefeated`).
    pub endboss_defeated: bool,
    /// Die Stufe hat die Insel beendet (Go: `island.Over = true`, `game_over`); die Insel übernimmt es in `put_stage_ref`.
    pub over: bool,
    /// Gold-Verlust des Modus `resources`: Die Stufe hat ihren Teil schon halbiert, die Insel halbiert das Gold der Spieler
    /// aller anderen Stufen in `put_stage_ref` (ein Stufenschritt erreicht sie nicht).
    pub halve_other_gold: bool,
    /// Zahl der Stufen der Insel (Go: `len(isl.Stages)`), für das Lager-Maximum (`island/storage.rs`, RU10.3).
    pub stages: i64,
    /// Gebaute Lager aller anderen Stufen (Go: `Kind == "storage" && State == "built"` über `isl.Stages` ohne diese Stufe);
    /// die Lager der eigenen Stufe liest `capacity` live aus `w.sites`, die anderen Stufen rechnen nicht mit.
    pub storages_elsewhere: i64,
    /// Dev-Bonus des Lager- und Gold-Maximums (Go: `island.Dev.StockBonus`, `devMaxStock`).
    pub stock_bonus: i64,
    /// Münzen, die Spieler dieser Stufe im Schritt aufgehoben haben (Go: `island.GoldCollected++`, Siegvariante gold); die
    /// Insel zählt sie in `put_stage_ref` zu ihrem Zähler.
    pub gold_collected: i64,
    /// Fund-Pool der Insel (Go: `island.SkillPool`, monarch.go); `add_pool_points` erhöht ihn, `put_stage_ref` übernimmt
    /// ihn und spiegelt ihn nach `World::skill_points` aller Stufen.
    pub skill_pool: i64,
    /// Geöffnete Truhen aller Stufen (Go: `island.ChestsOpened`); jede n-te gibt einen Pool-Punkt.
    pub chests_opened: i64,
    /// Ankünfte des Händlers auf der Insel (Go: `island.MerchantVisits`, merchant.go).
    pub merchant_visits: i64,
    /// Eine andere Stufe der Insel hat eine gebaute Taverne (Go: `tavernOnIsland` über `isl.Stages`); die eigene Stufe
    /// liest `tavern_on_island` live aus `w.sites`.
    pub tavern_elsewhere: bool,
}

/// Was die Ereignisse einer Stufe von ihrer Insel lesen (Go: `w.island` in `eventInNight` und `eventStage`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandEvents {
    /// Die Stufe ist Stufe 0 der Insel und meldet die Events (Go: `eventStage`).
    pub first_stage: bool,
    /// Event-Id → Nacht, in der das Event zusätzlich zum Rhythmus läuft (Go: `island.Dev.Nights`).
    pub nights: std::collections::BTreeMap<String, i64>,
}

/// Die lesbaren Teile der Insel für die Anzeige (HUD, Lager-Maximum, Zahlbarkeit, Lair; B-1125). Klein und `Copy`:
/// `Island::sync_stock` spiegelt sie je Stufe in `World::island_view`, ohne die veränderlichen Teile der Insel zu berühren.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IslandView {
    /// Index der Stufe in der Insel (`IslandRef::stage`).
    pub stage: usize,
    /// Zahl der Stufen der Insel (`IslandRef::stages`).
    pub stages: i64,
    /// Gebaute Lager aller anderen Stufen (`IslandRef::storages_elsewhere`).
    pub storages_elsewhere: i64,
    /// Dev-Bonus des Lager- und Gold-Maximums (`IslandRef::stock_bonus`).
    pub stock_bonus: i64,
    /// Dev-Schalter `CheapBuild` (`IslandRef::cheap_build`).
    pub cheap_build: bool,
    /// Der Endboss-Bau der Stufe löst nicht mehr aus: Endboss im Kampf oder besiegt, oder der Boss des Baus ist besiegt
    /// (Go: `AtLair` liest `island.endboss`, `EndbossDefeated`, `DefeatedBosses`).
    pub lair_closed: bool,
}

impl IslandView {
    /// Der `IslandRef` für lesende Funktionen (`purse_max_of`, `economy_of`, `at_lair` …). Nur die Felder der Sicht sind
    /// gesetzt, der Rest ist leer und allokiert nicht; `lair_closed` steht als `endboss_defeated`.
    pub fn to_ref(&self) -> IslandRef {
        IslandRef {
            stage: self.stage,
            stages: self.stages,
            storages_elsewhere: self.storages_elsewhere,
            stock_bonus: self.stock_bonus,
            cheap_build: self.cheap_build,
            endboss_defeated: self.lair_closed,
            ..IslandRef::default()
        }
    }
}

impl World {
    /// Die Insel-Sicht als `IslandRef` (Go: `w.island`); `None` ohne Insel (Kampagne, Standbild, Wiedergabe).
    pub fn island_ref_view(&self) -> Option<IslandRef> {
        self.island_view.as_ref().map(IslandView::to_ref)
    }
}

impl IslandRef {
    /// Go: `Island.waveFactors`: Wellengröße (Spieleranzahl der Insel mal Grad), HP- und Schadensfaktor zum Zeitpunkt
    /// des Wellenstarts.
    pub fn wave_factors(&self) -> (f64, f64, f64) {
        let extra = (self.players.max(1) - 1) as f64;
        (
            (1.0 + self.per_extra_player * extra) * self.grade.wave_size,
            self.grade.enemy_hp,
            self.grade.enemy_damage,
        )
    }
}

/// Go: `cheap` (dev.go): der Preis `c` der Stufe; mit dem Dev-Schalter `CheapBuild` höchstens 1 je Posten (0 bleibt 0).
pub fn cheap(isl: Option<&IslandRef>, c: &Cost) -> Cost {
    if !isl.is_some_and(|i| i.cheap_build) {
        return c.clone();
    }
    Cost {
        gold: c.gold.min(1),
        wood: c.wood.min(1),
        stone: c.stone.min(1),
        copper: c.copper.min(1),
        iron: c.iron.min(1),
        crystal: c.crystal.min(1),
    }
}

/// Go: `World.protectedNight`: Die Stufe gehört zu einer Insel, deren Grad die laufende Nacht schützt
/// (`protectedNights`): Gegner zerstören kein Gebäude, rauben kein Gold, ein Burgfall kostet nichts.
pub fn protected_night(w: &World, isl: Option<&IslandRef>) -> bool {
    isl.is_some_and(|i| w.cycle.phase == "night" && w.cycle.day <= i.grade.protected_nights)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grade() -> GradeValues {
        GradeValues {
            wave_size: 1.5,
            enemy_hp: 1.2,
            enemy_damage: 1.1,
            protected_nights: 2,
        }
    }

    fn island(players: i64) -> IslandRef {
        IslandRef {
            grade: grade(),
            players,
            per_extra_player: 0.5,
            ..IslandRef::default()
        }
    }

    #[test]
    fn wave_factors_von_hand_nachgerechnet() {
        // Ein Spieler: extra 0, Größe 1 * 1,5. Drei Spieler: extra 2, (1 + 0,5 * 2) * 1,5 = 3.
        assert_eq!(island(1).wave_factors(), (1.5, 1.2, 1.1));
        assert_eq!(island(3).wave_factors(), (3.0, 1.2, 1.1));
        // Null Spieler zählen wie einer (`max(…, 1)`).
        assert_eq!(island(0).wave_factors(), (1.5, 1.2, 1.1));
    }

    #[test]
    fn protected_night_nur_in_geschuetzten_naechten_einer_insel() {
        let mut w = World::default();
        w.cycle.phase = "night".into();
        w.cycle.day = 2;
        let isl = island(1);
        assert!(protected_night(&w, Some(&isl)));
        assert!(!protected_night(&w, None), "ohne Insel nie");
        w.cycle.day = 3;
        assert!(!protected_night(&w, Some(&isl)), "Nacht 3 ist nicht mehr geschützt");
        w.cycle.day = 1;
        w.cycle.phase = "day".into();
        assert!(!protected_night(&w, Some(&isl)), "am Tag nie");
    }
}
