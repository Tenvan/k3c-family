//! Abfragen der Boss-Daten (Port der Abfragen aus `engine/sim/boss_data.go`, docs/rules/bosse.md § 1). Die Lader und
//! Prüfungen (`loadBossData`, `loadBosses`, `loadEndBosses`, `checkBoss` …) liegen seit RU3 in `k3c_data::bosses`.
//! Gos Paket-Variablen `bosses` und `endBosses` sind das Feld `WorldValues::bosses`.

use k3c_data::bosses::{BossData, BossesFile};

/// Go: `bossByID`: der Miniboss oder Endboss mit dieser ID (erst die Minibosse, dann die Endbosse); `None`: keiner.
pub fn boss_by_id<'a>(bosses: &'a BossesFile, id: &str) -> Option<&'a BossData> {
    bosses
        .bosses
        .iter()
        .chain(bosses.end_bosses.iter())
        .find(|b| b.id == id)
}

/// Go: `endBoss`: der Endboss, dessen Bau in der Stufe `depth` liegt (`None`: keiner).
pub fn end_boss(bosses: &BossesFile, depth: i64) -> Option<&BossData> {
    bosses.end_bosses.iter().find(|b| b.depth == depth)
}

/// Go: `miniBoss`: der Miniboss der Stufe `depth` (`None`: keiner).
pub fn mini_boss(bosses: &BossesFile, depth: i64) -> Option<&BossData> {
    bosses.bosses.iter().find(|b| b.kind == "mini" && b.depth == depth)
}
