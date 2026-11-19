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
pub fn boss_island(players: usize) -> Island {
    island_of("boss", &[0, 1, 2], players, 0)
}

/// Go: `bossIsland` und `deepIsland`: Insel mit den Stufen `depths`, `players` Spieler in `stage`, ohne Truppen,
/// Gegner und Warteschlange.
fn island_of(seed: &str, depths: &[i64], players: usize, stage: usize) -> Island {
    use crate::island::testutil::{standard_world, test_data};
    let data = test_data();
    let mut isl = crate::island::create_island(&data, &standard_world(&data), seed, depths, 1.0, Default::default())
        .expect("Insel");
    for _ in 0..players {
        crate::island::add_island_player(&mut isl, stage);
    }
    for w in &mut isl.stages {
        w.troops.clear();
        w.enemies.clear();
        w.spawn_queue.clear();
    }
    isl
}

/// Go: `deepIsland`: Insel mit allen fünf Stufen der Insel 1 und `players` Spielern in der Stufe `stage`.
pub fn deep_island(players: usize, stage: usize) -> Island {
    island_of("boss", &[0, 1, 2, 3, 4], players, stage)
}

/// Go: `bossesIn`: die Bosse der Stufe (Indizes).
pub fn bosses_in(w: &World) -> Vec<usize> {
    (0..w.enemies.len()).filter(|&i| w.enemies[i].boss).collect()
}

/// Go: `countKind`: Zahl der Gegner dieser Art.
pub fn count_kind(w: &World, kind: &str) -> usize {
    w.enemies.iter().filter(|e| e.kind == kind).count()
}

/// Go: `waveUntil`: startet Wellen der Stufe `i`, bis sie bei Welle `n` ist (mit dem Bezug zur Insel wie `w.island`).
pub fn wave_until(isl: &mut Island, i: usize, n: i64) {
    while isl.stages[i].wave < n {
        start_wave(isl, i);
    }
}

/// Go: `startWave` einer Stufe der Insel (RU10).
pub fn start_wave(isl: &mut Island, stage: usize) {
    use crate::island::testutil::test_data;
    let data = test_data();
    let mut r = isl.stage_ref(stage, &data.difficulty);
    crate::enemies::waves::start_wave(&mut isl.stages[stage], Some(&mut r));
    isl.put_stage_ref(r);
}

/// Ruft `f` auf der Stufe `i` wie `step_island` (Entscheidung 4 und 5): der Vorrat der Insel liegt in der Stufe, der
/// veränderliche `IslandRef` kommt aus der Insel; danach geht beides zurück (Go: `w.island`).
pub fn on_stage_mut<R>(isl: &mut Island, i: usize, f: impl FnOnce(&mut World, &mut IslandRef) -> R) -> R {
    let data = crate::island::testutil::test_data();
    let mut r = isl.stage_ref(i, &data.difficulty);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    let out = f(&mut isl.stages[i], &mut r);
    std::mem::swap(&mut isl.stock, &mut isl.stages[i].stock);
    isl.put_stage_ref(r);
    out
}
