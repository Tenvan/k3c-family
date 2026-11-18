//! Hub- und Insel-Plätze im Wertesatz der Welt (Teil-Port von `engine/sim/world_hub_sites.go`, welten.md § 1.3, B-983):
//! `hub.sites` und `hub.islandSites` sind übersteuerbar; die Planung der Inseln wird gegen die Plätze des Wertesatzes
//! geprüft. `level_sites` steht in `world/mod.rs` (RU14.1); `check_world_hub_sites` und `check_site_bounds` liefert RU18.3
//! mit `hub_validate`.

use k3c_data::biome::Biome;
use k3c_data::hub::HubData;
use k3c_level::rules::Rules;

use super::level_sites;
use crate::island::WorldDef;
use crate::island::data::check_planned_chunks_with;
use crate::island::worlds::world_path;

/// Go: `checkWorldPlans`: prüft die geplanten Chunks der Inseln einer Welt gegen die Plätze ihres Wertesatzes (`hub`):
/// Ruinen dürfen nicht auf der Lage eines Platzes gleicher Art liegen. Die Meldung nennt Welt, Insel, Tiefe und Platz.
/// Go liest `world.values.Hub`; hier kommt die hub.json des Wertesatzes als Parameter, Biome und Regelsatz dazu (Gos
/// Paket-Variablen).
pub fn check_world_plans(world: &WorldDef, hub: &HubData, biomes: &[Biome], rules: &Rules) -> Result<(), String> {
    let sites = level_sites(hub);
    for d in &world.islands {
        check_planned_chunks_with(d, Some(&sites), biomes, rules)
            .map_err(|e| format!("{}: Insel {}: {e}", world_path(&world.id), d.id))?;
    }
    Ok(())
}
