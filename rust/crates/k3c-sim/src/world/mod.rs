//! Der Einstieg in eine Stufe (Port von `engine/sim/world.go`): `create_world` baut den Startzustand aus Biom und Seed,
//! `add_player` stellt einen Monarchen an die Burg, `step` (`step.rs`) rechnet einen Tick, `castle_fallen` und
//! `castle_losses` regeln den Burgfall. Ableitungen davon: Schiff (`ship.rs`) und Vorschau (`preview.rs`).
//!
//! Gos Paket-Variablen `biomes` und `globalDayNight` kommen aus `base` (die eingebetteten Daten); die Insel einer Stufe
//! steht nicht in der Stufe (Zustandsmodell, Entscheidung 4): `step` und `castle_fallen` bekommen den `IslandRef`.
//! Teilschritte späterer Sprints stehen als benannte Platzhalter in `pending.rs`.

use std::sync::{Arc, OnceLock};

use k3c_data::biome::Biome;
use k3c_data::hub::HubData;
use k3c_level::level::generate_with;
use k3c_level::lines::HubSite;
use k3c_level::planned_gen::Options as LevelOptions;
use k3c_level::rules::Rules;
use k3c_level::types::{NatureValues, PlannedChunk};
use k3c_rng::Rng;

use crate::enemies::guard::keep_guards;
use crate::island::create::CreateOptions;
use crate::island::defeat::{defeat_losses, halve_stock};
use crate::island::sizes::check_chunk_count;
use crate::state::island_ref::protected_night;
use crate::state::pending::{gather_of, is_ruin, to_hub, values};
use crate::state::{Camp, Castle, Event, IslandRef, Pickup, Player, ResourceNode, Site, World, WorldValues};
use crate::unit::hammer::{spawn_start_peasants, spawn_vagrant_with};
use crate::unit::jobs::make_archer;

pub mod base;
pub mod pending;
pub mod preview;
pub mod ship;
pub mod step;

use base::base;
use pending::{
    add_ruin_sites, apply_preset, cycle_at, default_values, new_hub_site, preset_for, ruin_site, validate_hub_data,
    world_site_specs,
};
use ship::add_ship_site;

pub use step::step;

#[cfg(test)]
mod cause_tests;
#[cfg(test)]
mod preview_hub_tests;
#[cfg(test)]
mod preview_tests;
#[cfg(test)]
mod ship_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
pub(crate) mod testutil;

/// Go: `Options` für `CreateWorld`; die kleingeschriebenen Felder (`lair`, `hub`, `vals`) sind hier öffentlich.
/// `cycle_speed` 0 bedeutet 1 (wie `?? 1` in TS).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Options {
    /// Beschleunigt den Tag/Nacht-Zyklus (Tests, Dev).
    pub cycle_speed: f64,
    /// Startzeit in Sekunden, damit der globale Zyklus beim Stufenwechsel weiterläuft.
    pub time: f64,
    /// Die geplanten Chunks dieser Stufe (docs/rules/geplante-chunks.md); leer = reiner Zufall.
    pub planned: Vec<PlannedChunk>,
    /// Chunk-Anzahl dieser Stufe (Inselgröße, geplante-chunks.md § 7); 0 = Länge des Bioms.
    pub chunk_count: i64,
    /// Die fest gesetzten Natur-Werte dieser Stufe (B-891); fehlende würfelt der Generator aus dem Seed.
    pub nature: NatureValues,
    /// Trägt die Stufe den Bau (`level.Options.Lair`); `None` = Feld `lair` des Bioms.
    pub lair: Option<bool>,
    /// Kandidaten-hub.json; `None` = die hub.json des Wertesatzes.
    pub hub: Option<Arc<HubData>>,
    /// Wertesatz der Welt; `None` = Basis-Wertesatz (`default_values`).
    pub vals: Option<Arc<WorldValues>>,
    /// Regelsatz des Level-Generators; `None` = der der eingebetteten Daten (`base`). Go: die Paket-Variablen.
    pub rules: Option<Arc<Rules>>,
}

/// Der Wertesatz der Stufe; fehlt `vals`, gilt wie in Go die Basis (`values`, RU14.4).
pub fn vals(w: &World) -> Arc<WorldValues> {
    w.vals.clone().unwrap_or_else(|| values(w))
}

/// Go: `hubOf`: die hub.json, aus der die Stufe gebaut ist: der Kandidat aus `CreateOptions` oder die der Welt. Eine Stufe
/// ohne Wertesatz (Test-Gerüst ohne `create_world`) hat eine leere hub.json.
pub fn hub_of(w: &World) -> &HubData {
    static EMPTY: OnceLock<HubData> = OnceLock::new();
    w.hub_data
        .as_deref()
        .or_else(|| w.vals.as_deref().map(|v| &v.hub))
        .unwrap_or_else(|| EMPTY.get_or_init(HubData::default))
}

/// Go: `HubData.LevelSites`: die Hub- und Insel-Plätze für den Generator, in Dateireihenfolge (sites, dann islandSites).
pub fn level_sites(h: &HubData) -> Vec<HubSite> {
    h.sites
        .iter()
        .chain(&h.island_sites)
        .map(|s| HubSite {
            kind: s.kind.clone(),
            offset_units: s.offset_units,
        })
        .collect()
}

/// Go: `CreateWorld`: baut den Startzustand einer Stufe.
pub fn create_world(b: &Biome, seed: &str, opts: &Options) -> Result<World, String> {
    let base = base()?;
    let rules = opts.rules.clone().unwrap_or_else(|| Arc::clone(&base.rules));
    let v = opts.vals.clone().unwrap_or_else(default_values);
    // Kandidat oder hub.json der Welt: Level und Stufe rechnen mit denselben Plätzen (B-983)
    let hub_sites = level_sites(opts.hub.as_deref().unwrap_or(&v.hub));
    let lo = LevelOptions {
        planned: opts.planned.clone(),
        chunk_count: opts.chunk_count,
        nature: opts.nature.clone(),
        lair: opts.lair,
        hub_sites: Some(hub_sites),
    };
    let lv = generate_with(b, seed, &lo, &rules).map_err(|e| e.to_string())?;
    let speed = if opts.cycle_speed == 0.0 { 1.0 } else { opts.cycle_speed };
    let hub_x = lv.hub_center_units;
    let castle_hp = v.buildings.get("castle").map_or(0.0, |c| c.hp);
    let mut w = World {
        seed: seed.to_string(),
        biome: b.clone(),
        rng: Rng::new(&format!("{}:{seed}:sim", b.id)),
        time: opts.time,
        cycle_speed: speed,
        next_id: 1,
        width_units: lv.width_units,
        hub_x,
        hub_level: 1,
        hub_step: 1,
        cycle: cycle_at(&base.day_night, opts.time * speed),
        castle: Castle {
            x: hub_x,
            hp: castle_hp,
            max_hp: castle_hp,
            ..Castle::default()
        },
        hub_data: opts.hub.clone(),
        planned: opts.planned.clone(),
        chunk_count: opts.chunk_count,
        vals: Some(Arc::clone(&v)),
        level: lv,
        ..World::default()
    };
    if b.cycle.r#type == "aggressionPool" {
        w.aggression = Some(0.0);
    }
    w.castle.id = w.new_id();
    w.hub_site = Some(new_hub_site(&w));
    place_entities(&mut w, &v, &rules);
    for s in world_site_specs(&w) {
        let site = empty_site(&mut w, &s.kind, s.x);
        w.sites.push(site);
    }
    add_ruin_sites(&mut w);
    add_ship_site(&mut w);
    spawn_start_peasants(&mut w, &v, hub_x);
    for _ in 0..v.hub.start_troops.archer {
        let ti = spawn_vagrant_with(&mut w, &v, hub_x, hub_x);
        make_archer(&mut w, ti);
    }
    Ok(w)
}

/// Go: `CreateWorldWith`: baut eine Stufe wie `create_world`, ohne Insel, mit den Erzeugungs-Optionen (Vorschau des
/// Gebäude-Planers, WZ9.1, B-462). Ein Kandidat wird vorher mit `validate_hub_data` geprüft (fail closed); die Welt behält
/// eine Kopie. `chunk_count` ist die Inselgröße der Stufe (0 = Länge des Bioms, sonst eine Größe nach `check_chunk_count`,
/// B-490). Ohne Kandidat gleich `create_world` mit Zyklusgeschwindigkeit 1.
pub fn create_world_with(
    b: &Biome,
    seed: &str,
    o: &CreateOptions,
    planned: &[PlannedChunk],
    chunk_count: i64,
) -> Result<World, String> {
    if chunk_count != 0 {
        let sizes = &base()?.data.island_sizes;
        check_chunk_count(sizes, chunk_count).map_err(|e| format!("welt: {e}"))?;
    }
    let mut opts = Options {
        cycle_speed: 1.0,
        planned: planned.to_vec(),
        chunk_count,
        ..Options::default()
    };
    if let Some(candidate) = &o.candidate {
        if let Some(first) = validate_hub_data(candidate).first() {
            return Err(format!("welt: Kandidaten-hub.json abgelehnt: {first}"));
        }
        opts.hub = Some(Arc::new(candidate.clone()));
    }
    create_world(b, seed, &opts)
}

/// Go: `placeEntities`: legt Ressourcen, Truhen, Skill-Punkte, Camps und Portale aus dem Level an.
pub fn place_entities(w: &mut World, v: &WorldValues, rules: &Rules) {
    let entities = std::mem::take(&mut w.level.entities);
    for e in &entities {
        let gatherable = gather_of(v, &e.kind).is_some();
        let rock_or_tree = e.kind == "tree" || e.kind == "rock";
        if gatherable && (!rock_or_tree || !w.level.on_free_ground(e.x, rules)) {
            let id = w.new_id();
            w.nodes.push(ResourceNode {
                id,
                kind: e.kind.clone(),
                x: e.x,
                variant: e.variant.clone(),
                ..ResourceNode::default()
            });
        } else if e.kind == "chest" || e.kind == "skillPoint" {
            let id = w.new_id();
            w.pickups.push(Pickup {
                id,
                kind: e.kind.clone(),
                x: e.x,
            });
        } else if e.kind == "recruitCamp" {
            let camp = &v.economy.recruit_camp;
            w.camps.push(Camp {
                x: e.x,
                respawn_in: camp.respawn_seconds,
            });
            for i in 0..camp.max_vagrants {
                spawn_vagrant_with(w, v, e.x, e.x + (i as f64 - 0.5) * 3.0);
            }
        } else if e.kind == "portal" {
            w.portals.push(e.x);
        }
    }
    w.level.entities = entities;
    crate::portal::sync_portals(w);
}

/// Go: `emptySite`: ein unbezahlter Platz der Art `kind` bei `x` (zieht eine ID).
pub fn empty_site(w: &mut World, kind: &str, x: f64) -> Site {
    let id = w.new_id();
    let max_hp = vals(w).buildings.get(kind).map_or(0.0, |b| b.hp);
    Site {
        id,
        kind: kind.to_string(),
        x,
        state: "unpaid".to_string(),
        max_hp,
        ruin: is_ruin(w, kind, x),
        ..Site::default()
    }
}

/// Go: `AddPlayer`: stellt einen neuen Monarchen an die Burg (Couch-Koop); liefert seinen Platz in `w.players`.
pub fn add_player(w: &mut World) -> usize {
    add_player_at(w, w.players.len() as i64)
}

/// Go: `addPlayerAt`: stellt den Monarchen mit dem angegebenen Spielerindex an die Burg (in einer Insel inselweit
/// eindeutig); liefert seinen Platz in `w.players`.
pub fn add_player_at(w: &mut World, index: i64) -> usize {
    let (offset, facing) = if index % 2 != 0 { (3.0, 1) } else { (-3.0, -1) };
    let id = w.new_id();
    let gold = vals(w).economy.purse.start_gold;
    w.players.push(Player {
        id,
        index,
        x: w.hub_x + offset,
        facing,
        gold,
        pay_cooldown: 0.5, // der Beitritts-Tastendruck soll nicht gleich eine Münze ausgeben
        ..Player::default()
    });
    let pi = w.players.len() - 1;
    let _ = apply_preset(w, pi, preset_for(index)); // Startverteilung beim Beitritt, soweit Pool-Punkte frei sind
    pi
}

/// Go: `castleFallen`: Die Burg fällt. Respawn am Hub, die Burg steht danach wieder; die Verluste folgen dem Niederlage-Modus
/// der Insel (`island/defeat.rs`). In einer geschützten Nacht (`protected_night`) bleiben Gebäude, Ressourcen, Gold und
/// Truppen. Der Endboss einer Insel bleibt dem Kampf erhalten (`park_endboss`), alle anderen Gegner außer den Wachen gehen.
pub fn castle_fallen(w: &mut World, mut isl: Option<&mut IslandRef>) {
    let mut ev = Event::new();
    ev.insert("type".to_string(), "castleFallen".into());
    w.events.push(ev);
    if w.cycle.phase == "night" {
        w.castle_fell_night = w.cycle.day; // keine Vollmond-Belohnung für diese Nacht (events_moon.go)
    }
    w.castle.hp = w.castle.max_hp;
    if !protected_night(w, isl.as_deref()) {
        defeat_losses(w, isl.as_deref_mut());
    }
    for pi in 0..w.players.len() {
        to_hub(w, pi);
    }
    crate::boss::endboss::park_endboss(w, isl);
    keep_guards(&mut w.enemies);
    w.spawn_queue.clear();
    w.projectiles.clear();
}

/// Go: `castleLosses`: Stufenverlust. Bauplätze der Stufe leer (Ruinen zurück in den Ausgangszustand, auch zerstörte),
/// Vorrat und Gold ihrer Spieler −50 %, Truppen außer Landstreichern weg.
pub fn castle_losses(w: &mut World) {
    w.hub_site = Some(new_hub_site(w)); // ein laufender Hub-Ausbau ist verloren, die Hub-Stufe bleibt
    for si in 0..w.sites.len() {
        let (id, kind, x) = (w.sites[si].id, w.sites[si].kind.clone(), w.sites[si].x);
        let mut fresh = if is_ruin(w, &kind, x) {
            ruin_site(w, &kind, x)
        } else {
            empty_site(w, &kind, x) // verbraucht wie in TS eine ID je Bauplatz
        };
        fresh.id = id;
        w.sites[si] = fresh;
    }
    halve_stock(&mut w.stock);
    for p in &mut w.players {
        p.gold /= 2;
    }
    for n in &mut w.nodes {
        n.worker_id = None;
    }
    w.troops.retain(|t| t.kind == "vagrant");
}
