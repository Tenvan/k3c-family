//! Die Insel aus einem Spielstand bauen (Port von `island_save.go` ab `FromIslandSave`): `from_island_save`,
//! `from_island_save_in`, `saved_create_options`.
//!
//! Die Stufen entstehen über `create_island_at` (`create_world_with`, RU14); bis dahin laufen die Tests dazu mit `ignore`.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::hub::Stock;
use k3c_level::types::NatureValues;

use super::create::{CreateOptions, IslandBuild, create_island_at};
use super::save::{IslandSave, SaveTables};
use super::save_parse::validate_island_save;
use super::save_v3::restore_pool;
use super::save_v5::restore_progress;
use super::save_v8::island_in;
use super::{Island, WorldDef};
use crate::state::Fehler;
use crate::state::links::values_of;
use crate::state::save_apply::apply_hub;
use crate::world::add_player_at;

/// Go: `FromIslandSave` baut die Insel aus einem Spielstand der Standard-Welt (`from_island_save_in`).
pub fn from_island_save(t: &SaveTables, s: &IslandSave, cycle_speed: f64) -> Result<Island, Fehler> {
    from_island_save_in(t, s, cycle_speed, t.standard_world)
}

/// Go: `FromIslandSaveIn` baut die Insel aus einem Spielstand (aus `parse_island_save`) in der Welt `world`; Welt-ID und
/// Insel-ID des Stands müssen zu ihr passen (`island_in`). Gespeicherte Spieler stehen wieder in ihrer Stufe, mit ihrer
/// gespeicherten Verteilung (das Beitritts-Preset gilt beim Laden nicht).
pub fn from_island_save_in(
    t: &SaveTables,
    s: &IslandSave,
    cycle_speed: f64,
    world: &WorldDef,
) -> Result<Island, Fehler> {
    let fail = |m: String| Fehler::Spielstand(m);
    let vals = values_of(world).map_err(|e| fail(format!("insel: {e}")))?;
    let mut world = world.clone();
    world.values = Some(Arc::clone(&vals));
    validate_island_save(s, Some(&vals), t)?;
    let number = island_in(&world, s)?;
    let current = world.islands.get(number).cloned().ok_or_else(|| {
        fail(format!(
            "spielstand: Insel {:?} gibt es in Welt {:?} nicht",
            s.island_id, s.world
        ))
    })?;
    for id in &s.completed {
        if !world.islands.iter().any(|d| &d.id == id) {
            return Err(fail(format!(
                "spielstand: abgeschlossene Insel {id:?} gibt es in Welt {:?} nicht",
                s.world
            )));
        }
        if *id == current.id {
            return Err(fail(format!(
                "spielstand: aktuelle Insel {id:?} steht unter den abgeschlossenen"
            )));
        }
    }
    let cycle_speed = if cycle_speed == 0.0 { 1.0 } else { cycle_speed };
    let depths: Vec<i64> = s.stages.iter().map(|h| h.depth).collect();
    let build = IslandBuild {
        data: t.data,
        world: &world,
        first: &current,
        hub: None,
    };
    let mut isl =
        create_island_at(&build, &s.seed, &depths, cycle_speed, s.time, &saved_create_options(s)).map_err(fail)?;
    isl.id = s.campaign_id.clone();
    isl.options = s.options.clone();
    isl.number = number as i64;
    isl.island_id = current.id.clone();
    isl.scaling = current.depth_scaling.clone();
    for (w, h) in isl.stages.iter_mut().zip(&s.stages) {
        apply_hub(w, h);
        w.stock = Stock::default(); // der Vorrat gehört der Insel (Entscheidung 5), nicht der Stufe
    }
    isl.stock = s.stock.clone(); // `apply_hub` setzt den Vorrat je Hub; maßgeblich ist der der Insel
    isl.sync_stock();
    restore_pool(&mut isl, s);
    restore_progress(&mut isl, s, t.data);
    isl.completed = s.completed.clone();
    isl.chosen = s.chosen.clone();
    if !s.chosen.is_empty() && !isl.targets().iter().any(|x| x.id == s.chosen) {
        return Err(fail(format!(
            "spielstand: gewähltes Ziel {:?} ist kein Ziel der Insel {:?}",
            s.chosen, current.id
        )));
    }
    crate::cargo::save::restore_opt(&mut isl.cargo, s.cargo.as_ref(), t.data)?; // RU62.1: Fracht und Aufträge
    isl.restore_hermit(t.data, s.hermit_paid).map_err(fail)?; // RU62.10: Einsiedler bezahlt?
    place_players(&mut isl, s)?;
    Ok(isl)
}

/// Stellt die gespeicherten Spieler in ihre Stufe und ordnet jede Stufe nach Index.
fn place_players(isl: &mut Island, s: &IslandSave) -> Result<(), Fehler> {
    // Die Stelle des Monarchen ist flüchtig wie Gegner und Münzen am Boden (B-814, gewollt): `IslandPlayerSave` trägt keine
    // Position, `add_player_at` stellt den Monarchen wie beim Beitritt an die Burg seiner Stufe (hub_x ± 3).
    for p in &s.players {
        let si = isl
            .stage_by_depth(p.depth)
            .ok_or_else(|| Fehler::Spielstand(format!("spielstand: Spieler {} ungültig", p.index)))?;
        let pi = add_player_at(&mut isl.stages[si], p.index);
        let q = &mut isl.stages[si].players[pi];
        q.gold = p.gold;
        q.skills = p.skills.clone(); // schon geprüft (validate_skills)
        q.slots = p.slots.clone();
        q.skill_base = p.skill_base;
        q.mount.clone_from(&p.mount); // RU62.8: schon gegen den Katalog geprüft (validate_mounts)
        isl.next_player = isl.next_player.max(p.index + 1);
    }
    for w in &mut isl.stages {
        w.players.sort_by_key(|p| p.index); // stabile Reihenfolge nach Index
    }
    Ok(())
}

/// Go: `savedCreateOptions` sind die Erzeugungs-Parameter aus dem Stand: je Stufe die Planung und Chunk-Anzahl, mit denen
/// sie gebaut wurde (B-489). Spätere Änderungen von data/worlds/*.json betreffen nur neu erzeugte Stufen.
pub fn saved_create_options(s: &IslandSave) -> CreateOptions {
    let (mut planned, mut counts) = (BTreeMap::new(), BTreeMap::new());
    let mut nature: BTreeMap<i64, NatureValues> = BTreeMap::new();
    for h in &s.stages {
        if !h.nature.is_empty() {
            nature.insert(h.depth, h.nature.clone());
        }
        if !h.planned.is_empty() {
            planned.insert(h.depth, h.planned.clone());
        }
        if h.chunk_count > 0 {
            counts.insert(h.depth, h.chunk_count);
        }
    }
    CreateOptions {
        planned: Some(planned),
        chunk_counts: Some(counts),
        nature: Some(nature),
        candidate: None,
    }
}
