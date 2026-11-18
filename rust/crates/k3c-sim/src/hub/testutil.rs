//! Hilfen der Tests der Gruppe `hub` (Go: `hubCopy`, `readHub7`, `hasFinding`, `withoutKind`, `siteOfKind` aus
//! `site_data_test.go` und `hub_candidate_test.go`; die Paket-Variable `hub`).

use std::path::PathBuf;

use k3c_data::Source;
use k3c_data::hub::HubData;

use super::validate::{HubReport, parse_hub, validate_hub_data_in};
use crate::island::testutil::{parse_world_in, test_data};
use crate::island::{CreateOptions, Island, WorldDef, create_island_in};
use crate::state::{Site, World};
use crate::world::base::base;

/// Go: die Paket-Variable `hub`: die eingebettete data/hub.json.
pub fn hub() -> HubData {
    base().expect("eingebettete Daten").data.hub.clone()
}

/// Go: `hubCopy`: eine frische Kopie der eingebetteten data/hub.json (Tests ändern nur Kopien).
pub fn hub_copy() -> HubData {
    let raw = Source::Embedded.read("hub.json").expect("hub.json");
    parse_hub(&raw).expect("hub.json parsen")
}

/// Go: `readHub7`: die 7-Stufen-Fixture (WZ15, AC-03).
pub fn read_hub7() -> HubData {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/hub/hub7.json");
    let raw = std::fs::read_to_string(path).expect("hub7.json");
    parse_hub(&raw).expect("hub7.json parsen")
}

/// Go: `hasFinding`: Gibt es einen Befund der Regel, dessen Platz `site` enthält?
pub fn has_finding(r: &HubReport, rule: &str, site: &str) -> bool {
    r.findings.iter().any(|f| f.rule == rule && f.site.contains(site))
}

/// Go: `withoutKind`: entfernt alle Plätze der Art aus `sites` und `islandSites`.
pub fn without_kind(h: &mut HubData, kind: &str) {
    h.sites.retain(|s| s.kind != kind);
    h.island_sites.retain(|s| s.kind != kind);
}

/// Die Standard-Welt der Go-Tests (`StandardWorld`): Ihr Laden liest `story` und wartet damit auf RU19
/// (`decode_world_story`). Die Tests rechnen darum mit einer Welt ohne Geschichte mit den Tiefen `depths` und der Kennung
/// `id`; Plätze, Gebäude und Wirtschaft kommen wie bei der Standard-Welt aus den eingebetteten Daten.
pub fn plain_world(id: &str, depths: &[i64]) -> WorldDef {
    use crate::island::switch_testutil::test_island;
    let island = test_island("a", &format!(r#","depths":{depths:?}"#));
    let raw = format!(r#"{{"id":"{id}","name":"Test","islands":[{island}]}}"#);
    parse_world_in(&raw, id).unwrap_or_else(|e| panic!("{e}"))
}

/// Go: `ValidateHubData` mit der Welt ohne Geschichte (`plain_world`).
pub fn validate(h: &HubData) -> HubReport {
    validate_hub_data_in(h, &plain_world("test", &[0, 1, 4]))
}

/// Go: `CreateIsland(seed, depths, 1, opts)` in der Welt ohne Geschichte (`plain_world`).
pub fn make_island(seed: &str, depths: &[i64], opts: CreateOptions) -> Result<Island, String> {
    let world = plain_world("test", &[0, 1, 2, 3, 4]);
    create_island_in(&test_data(), &world, seed, depths, 1.0, opts)
}

/// Go: `siteOfKind`: der erste Platz der Art in `w.sites`.
pub fn site_of_kind<'a>(w: &'a World, kind: &str) -> Option<&'a Site> {
    w.sites.iter().find(|s| s.kind == kind)
}
