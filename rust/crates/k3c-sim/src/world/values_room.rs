//! Wertesatz je Raum (Port von `engine/sim/world_values_room.go`, welten.md § 1/§ 6, B-681): Jede Stufe rechnet mit dem
//! Wertesatz ihrer Welt (`World::vals`, alle Stufen einer Insel teilen ihn), aufgelöst einmal beim Anlegen oder Laden,
//! nie je Tick. Welten ohne Overrides in ihrer Kette rechnen mit dem Basis-Wertesatz (`default_values`); nur Welten mit
//! Overrides bekommen einen eigenen Wertesatz aus `resolve_world_values`.
//!
//! Abweichungen von Go: Gos Paket-Variablen (`dataFS`, `buildings` …) gibt es nicht. `resolve_world` bekommt Quelle, Daten
//! und Regelsatz als Parameter; `values_of` (Aufrufer ohne Quelle: Insel anlegen und laden) und `default_values` rechnen
//! mit den eingebetteten Daten (`base`). Der Basis-Wertesatz wird einmal gebaut und als `Arc` geteilt (Go baut ihn je
//! Aufruf neu, teilt aber die Maps; Rust ändert ihn nie). `values` merkt sich bei einer Stufe ohne Wertesatz nichts (die
//! Stufe ist nur geliehen), liefert aber jedes Mal denselben `Arc` (Go: B-692, keine Allokation je Aufruf).

use std::sync::{Arc, OnceLock};

use k3c_data::{Data, Source};
use k3c_level::rules::Rules;

use super::base::base;
use super::hub_sites::check_world_plans;
use super::ship::with_ship;
use super::skins::chain_skins;
use super::values::{LookupFn, resolve_world_values, world_chain};
use super::values_from::ReadFn;
use crate::combat::data::chain_combat;
use crate::island::WorldDef;
use crate::island::worlds::load_world;
pub use crate::monarch::new_skill_table; // Von RU18.4 portiert (monarch_data.go)
use crate::state::{World, WorldValues};

/// Go: `defaultValues` für die Daten `data`: der Basis-Wertesatz (Gebäude samt Platz `ship`, Skill-Katalog, Bosse).
pub fn default_values_from(data: &Data) -> Result<WorldValues, String> {
    let mut buildings = data.buildings.clone();
    with_ship(&mut buildings, &data.ship)?;
    Ok(WorldValues {
        buildings,
        troops: data.troops.clone(),
        economy: data.economy.clone(),
        hub: data.hub.clone(),
        monarch: data.monarch.clone(),
        enemies: data.enemies.clone(),
        waves: data.waves.clone(),
        goals: data.goals.clone(),
        events: data.events.events.clone(),
        skills: new_skill_table(&data.monarch.skills),
        bosses: data.bosses.clone(),
        ..WorldValues::default()
    })
}

/// Go: `defaultValues`: der Basis-Wertesatz der eingebetteten Daten, einmal gebaut. Scheitern die eingebetteten Daten
/// (nicht zu erwarten, `base` meldet es jedem Aufrufer mit `Result`), ist er leer.
pub fn default_values() -> Arc<WorldValues> {
    static DEFAULTS: OnceLock<Arc<WorldValues>> = OnceLock::new();
    Arc::clone(DEFAULTS.get_or_init(|| {
        let built = base().and_then(|b| default_values_from(&b.data));
        Arc::new(built.unwrap_or_default())
    }))
}

/// Go: `World.values`: der Wertesatz der Stufe; eine Stufe ohne `create_world` (Test-Gerüst `World::default()`) rechnet
/// mit der Basis.
pub fn values(w: &World) -> Arc<WorldValues> {
    match &w.vals {
        Some(v) => Arc::clone(v),
        None => default_values(),
    }
}

/// Go: `ResolveWorld`: `world` mit ihrem aufgelösten Wertesatz (`WorldDef::values`), den das Anlegen und Laden einer
/// Insel übernehmen. `lookup` liefert die Eltern-Welten der Kette; `world` selbst muss `lookup` nicht kennen. Ohne
/// Overrides in der Kette gilt der Basis-Wertesatz aus `data`. Fehler nennen die Welt (welten.md § 1.2).
pub fn resolve_world(
    world: &WorldDef,
    src: &Source,
    data: &Data,
    rules: &Rules,
    lookup: LookupFn,
) -> Result<WorldDef, String> {
    let defaults = Arc::new(default_values_from(data)?);
    resolve_world_with(world, &|name| src.read(name), defaults, data, rules, lookup)
}

/// `ResolveWorld` mit schon gebautem Basis-Wertesatz (geteilt, solange die Welt ihn nicht ändert).
fn resolve_world_with(
    world: &WorldDef,
    read: ReadFn,
    defaults: Arc<WorldValues>,
    data: &Data,
    rules: &Rules,
    lookup: LookupFn,
) -> Result<WorldDef, String> {
    let this = |id: &str| {
        if id == world.id { Ok(world.clone()) } else { lookup(id) }
    };
    let chain = world_chain(&this, &world.id)?;
    let mut out = world.clone();
    out.skins = chain_skins(&chain); // Kind überschreibt Eltern je Schlüssel (B-984)
    let mut vals = defaults;
    if chain.iter().any(|w| !w.overrides.is_empty()) {
        vals = Arc::new(resolve_world_values(read, &this, &world.id)?);
    }
    check_world_plans(&out, &vals.hub, &data.biomes, rules)?; // Planung gegen die Plätze der Welt
    let combat = chain_combat(&chain.iter().map(|w| w.combat.clone()).collect::<Vec<_>>()); // Kind überschreibt Eltern
    if vals.combat != combat {
        Arc::make_mut(&mut vals).combat = combat;
    }
    out.values = Some(vals);
    Ok(out)
}

/// Go: `valuesOf`: der Wertesatz der Welt: aus `resolve_world`, sonst aufgelöst mit den eingebetteten Welten als Eltern.
pub fn values_of(world: &WorldDef) -> Result<Arc<WorldValues>, String> {
    if let Some(v) = &world.values {
        return Ok(Arc::clone(v));
    }
    let b = base()?;
    let src = Source::Embedded;
    let lookup = |id: &str| load_world(&src, id, &b.data, &b.rules);
    let resolved = resolve_world_with(
        world,
        &|name| src.read(name),
        default_values(),
        &b.data,
        &b.rules,
        &lookup,
    )?;
    resolved.values.ok_or_else(|| "ResolveWorld ohne Wertesatz".to_string())
}
