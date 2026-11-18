//! Hilfen der Tests der Gruppe `building` (Go: `fixtureIsland`, `ruinOf`, `siteAt`, `hubSiteOf`, `fighterAt`, `hasEvent`
//! aus den `building_*_test.go`, `sites_test.go`). Tests, die `create_world` oder `step` brauchen, warten auf RU18 (der
//! Zahlplatz `new_hub_site`, die Schritte der Wirtschaft) und tragen `ignore`.

pub use crate::enemies::testutil::{
    DT, add_peasant, add_player, bare_world, build_site, built_site, day_world, line_site, quiet_world, run, run_until,
    spawn,
};
pub use crate::world::testutil::{built, pay_at};

use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_level::types::PlannedChunk;

use crate::island::testutil::{ISLAND_SPEED, standard_world, test_data};
use crate::island::{CreateOptions, Island, add_island_player, create_island};
use crate::state::{Event, World};
use crate::unit::jobs::make_fighter;
use crate::unit::spawn_vagrant;
use crate::world::hub_of;

/// Go: `hubSiteOf`: der Index des Platzes `kind` laut data/hub.json › sites.
pub fn hub_site_of(w: &World, kind: &str) -> usize {
    for h in hub_of(w).sites.iter().filter(|h| h.kind == kind) {
        if let Some(si) = w
            .sites
            .iter()
            .position(|s| s.kind == kind && (s.x - (w.hub_x + h.offset_units)).abs() < 1e-9)
        {
            return si;
        }
    }
    panic!("kein Hub-Platz {kind} laut data/hub.json");
}

/// Go: `fighterAt` (building_upgrades_test.go): ein Kämpfer der Figur `kind` bei `x` auf der Seite von `x`.
pub fn fighter_at(w: &mut World, kind: &str, x: f64) -> usize {
    let ti = spawn_vagrant(w, x, x);
    make_fighter(w, ti, kind);
    let anchor = w.hub_x + (x - w.hub_x).signum();
    let t = &mut w.troops[ti];
    t.x = x;
    t.anchor_x = anchor;
    ti
}

/// Go: `hasEvent`: Unter den Ereignissen ist eines der Art `typ`.
pub fn has_event(events: &[Event], typ: &str) -> bool {
    events
        .iter()
        .any(|e| e.get("type").and_then(|v| v.as_str()) == Some(typ))
}

/// Go: `siteAt`: der Index des Platzes `kind`@`x` der Stufe.
pub fn site_at(w: &World, kind: &str, x: f64) -> Option<usize> {
    w.sites.iter().position(|s| s.kind == kind && s.x == x)
}

/// Go: `ruinOf`: der Index der Ruine `kind`.
pub fn ruin_of(w: &World, kind: &str) -> usize {
    match w.sites.iter().position(|s| s.ruin && s.kind == kind) {
        Some(si) => si,
        None => panic!("keine Ruine {kind}"),
    }
}

/// Go: `ruinStartHP`: die Start-HP einer Ruine, ein Viertel der HP der Art (aufgerundet).
pub fn ruin_start_hp(w: &World, kind: &str) -> f64 {
    (0.25 * crate::world::vals(w).buildings[kind].hp).ceil()
}

/// Go: `ruinFixturePlan`: die Planung aus testdata/planned/forest-ruin.json (Lager, Schmiede, Kaserne).
pub fn ruin_fixture_plan() -> Vec<PlannedChunk> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../testdata/planned/forest-ruin.json"
    );
    let raw = std::fs::read_to_string(path).expect("Fixture forest-ruin.json");
    let f: serde_json::Value = serde_json::from_str(&raw).expect("Fixture ist JSON");
    serde_json::from_value(f["planned"].clone()).expect("Planung der Fixture")
}

/// Go: `ruinIsland` baut die Insel „ruine-1“ (Tiefe 0) mit der Planung `plan` und zwei Spielern.
pub fn ruin_island(plan: Vec<PlannedChunk>) -> (Island, Data) {
    let data = test_data();
    let opts = CreateOptions {
        planned: Some(BTreeMap::from([(0, plan)])),
        ..CreateOptions::default()
    };
    let mut isl = create_island(&data, &standard_world(&data), "ruine-1", &[0], ISLAND_SPEED, opts).expect("Insel");
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    (isl, data)
}

/// Go: `fixtureIsland` ist `ruin_island` mit der Planung der Fixture.
pub fn fixture_island() -> (Island, Data) {
    ruin_island(ruin_fixture_plan())
}
