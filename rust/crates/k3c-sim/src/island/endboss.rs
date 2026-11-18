//! Endboss je Insel (Port von `engine/sim/island_endboss.go`; docs/rules/welten.md § 2 und § 3, bosse.md § 1, B-981): Das
//! Insel-Feld `endBoss` wählt die ID aus data/bosses.json › endBosses, ohne Feld gilt der Endboss der tiefsten Stufe. Der
//! Bau liegt immer in der tiefsten Stufe der Insel, in jedem Biom (`level.Options.Lair`); die Bau-Werte stehen am Endboss.
//!
//! Gos Paket-Tabellen `bosses`/`endBosses` kommen als `&BossesFile` (`Data::bosses`). `boss_by_id` und `end_boss` (Go:
//! `boss_data.go`, RU15.1) stehen bis dahin hier als reine Suche in den Daten.

use k3c_data::Data;
use k3c_data::bosses::{BossData, BossesFile};
use k3c_level::lair::check_lair_fit;

use super::data::{IslandDef, biome_for_depth};

/// Go: `bossByID` (boss_data.go): der Boss mit der ID `id`, erst unter den Minibossen, dann unter den Endbossen.
pub fn boss_by_id<'a>(bosses: &'a BossesFile, id: &str) -> Option<&'a BossData> {
    bosses.bosses.iter().chain(&bosses.end_bosses).find(|b| b.id == id)
}

/// Go: `endBoss` (boss_data.go): der Endboss, dessen Bau in der Stufe `depth` liegt (`None`: keiner).
pub fn end_boss(bosses: &BossesFile, depth: i64) -> Option<&BossData> {
    bosses.end_bosses.iter().find(|b| b.depth == depth)
}

impl IslandDef {
    /// Go: `LairDepth`, die Tiefe der Stufe mit dem Bau: die tiefste der Insel. Go bricht bei leeren Tiefen mit `panic` ab;
    /// `check_island` lehnt eine Insel ohne Tiefen vorher ab, hier gilt dann 0.
    pub fn lair_depth(&self) -> i64 {
        self.depths.iter().copied().max().unwrap_or(0)
    }

    /// Go: `endBossOf`, der Endboss der Insel (`None`: keiner gefunden, `check_end_boss` lehnt das vorher ab).
    pub fn end_boss_of<'a>(&self, bosses: &'a BossesFile) -> Option<&'a BossData> {
        if self.end_boss.is_empty() {
            return end_boss(bosses, self.lair_depth());
        }
        boss_by_id(bosses, &self.end_boss).filter(|b| b.kind == "end")
    }

    /// Go: `EndBossID`, die ID des Endbosses der Insel (Feld `endBoss`, sonst der Endboss der tiefsten Stufe); leer, wenn
    /// keiner da ist.
    pub fn end_boss_id(&self, bosses: &BossesFile) -> String {
        self.end_boss_of(bosses).map(|b| b.id.clone()).unwrap_or_default()
    }
}

/// Go: `endBossIDs`, die IDs aller Endbosse in der Reihenfolge der Datei (für Meldungen).
fn end_boss_ids(bosses: &BossesFile) -> String {
    let ids: Vec<&str> = bosses.end_bosses.iter().map(|b| b.id.as_str()).collect();
    ids.join(", ")
}

/// Go: `checkEndBoss` prüft Endboss und Bau der Insel: eine unbekannte ID oder ein Miniboss in `endBoss`, kein Endboss für
/// die tiefste Stufe und ein Biom, das den Bau samt Camp-Abstand nicht trägt (`check_lair_fit`), lehnen die Insel ab.
pub fn check_end_boss(d: &IslandDef, data: &Data) -> Result<(), String> {
    let deepest = d.lair_depth();
    let bosses = &data.bosses;
    if d.end_boss.is_empty() {
        if end_boss(bosses, deepest).is_none() {
            return Err(format!("kein Endboss in der tiefsten Stufe {deepest}"));
        }
    } else {
        match boss_by_id(bosses, &d.end_boss) {
            None => {
                return Err(format!(
                    "endBoss {:?} unbekannt (erlaubt: {})",
                    d.end_boss,
                    end_boss_ids(bosses)
                ));
            }
            Some(b) if b.kind != "end" => {
                return Err(format!(
                    "endBoss {:?} ist ein Miniboss (erlaubt: {})",
                    d.end_boss,
                    end_boss_ids(bosses)
                ));
            }
            Some(_) => {}
        }
    }
    let Some(biome) = biome_for_depth(&data.biomes, deepest) else {
        return Err(format!("unbekannte Tiefe {deepest}"));
    };
    check_lair_fit(biome, d.count_at(deepest)).map_err(|e| format!("bau in der tiefsten Stufe {deepest}: {e}"))
}

/// Go: `lairOptions`: Die Stufe der Tiefe `depth` trägt den Bau (nur die tiefste Stufe der Insel `def`), und ihr Endboss.
pub fn lair_options(def: &IslandDef, depth: i64, data: &Data) -> (Option<bool>, Option<BossData>) {
    let lair = depth == def.lair_depth();
    if !lair {
        return (Some(lair), None);
    }
    (Some(lair), def.end_boss_of(&data.bosses).cloned())
}
