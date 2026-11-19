//! Hilfen der Tests dieser Gruppe. Die Hilfen aus den Go-Testdateien `boss_test.go` und `boss_abilities_test.go`
//! (`bossIsland`, `deepIsland`, `bossesIn`, `countKind`, `waveUntil`) stehen hier. Hilfen, die Inseln (RU10) oder den
//! Takt (RU14) brauchen, melden den liefernden Sprint; Tests, die sie rufen, tragen `ignore`.

use std::sync::Arc;

use k3c_data::{Source, load_all};

use crate::enemies::testutil::{Island, bare_world, test_values};
use crate::state::island_ref::IslandRef;
use crate::state::{World, WorldValues};

/// Wertesatz aus den eingebetteten Daten samt Bossen (Go: `w.values()` und die Paket-Variablen `bosses`/`endBosses`).
pub fn boss_values() -> Arc<WorldValues> {
    let d = load_all(&Source::Embedded).expect("eingebettete Daten");
    Arc::new(WorldValues {
        bosses: d.bosses,
        ..test_values()
    })
}

/// Eine leere Stufe der Tiefe `depth` mit Wertesatz samt Bossen, Hub bei 200 und einem Portal bei 20; dazu der Bezug zu
/// ihrer Insel (Grad `normal`-Werte 1,0, `players` Spieler). Ersetzt für Tests ohne `CreateIsland` die Insel.
pub fn boss_stage(depth: i64, players: i64) -> (World, IslandRef) {
    let mut w = bare_world();
    w.vals = Some(boss_values());
    w.biome.depth = depth;
    w.portals = vec![20.0];
    let isl = IslandRef {
        scaling: boss_values().waves.depth_scaling.clone(), // die Tabelle der Insel ist die der Daten (island_data.go)
        players,
        grade: crate::state::island_ref::GradeValues {
            enemy_hp: 1.0,
            enemy_damage: 1.0,
            wave_size: 1.0,
            ..Default::default()
        },
        ..IslandRef::default()
    };
    (w, isl)
}

/// Go: `bossIsland`: Insel mit Wald, Höhle und Mine und `players` Spielern im Wald; ohne Truppen und Gegner, damit nur
/// Boss und Beschwörungen kämpfen.
pub fn boss_island(_players: usize) -> Island {
    panic!("RU10: boss_island (CreateIsland, AddIslandPlayer)")
}

/// Go: `deepIsland`: Insel mit allen fünf Stufen der Insel 1 und `players` Spielern in der Stufe `stage`.
pub fn deep_island(_players: usize, _stage: usize) -> Island {
    panic!("RU10: deep_island (CreateIsland, AddIslandPlayer)")
}

/// Go: `bossesIn`: die Bosse der Stufe (Indizes).
pub fn bosses_in(w: &World) -> Vec<usize> {
    (0..w.enemies.len()).filter(|&i| w.enemies[i].boss).collect()
}

/// Go: `countKind`: Zahl der Gegner dieser Art.
pub fn count_kind(w: &World, kind: &str) -> usize {
    w.enemies.iter().filter(|e| e.kind == kind).count()
}

/// Go: `waveUntil`: startet Wellen, bis die Stufe bei Welle `n` ist (`startWave` braucht die Insel, RU10).
pub fn wave_until(_w: &mut World, _n: i64) {
    panic!("RU10: wave_until (startWave mit Insel)")
}

/// Go: `startWave` einer Stufe der Insel (RU10).
pub fn start_wave(_isl: &mut Island, _stage: usize) {
    panic!("RU10: start_wave")
}
