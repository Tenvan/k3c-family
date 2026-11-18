//! Tests aus `fullsave_rules_test.go` (B-500, B-425, B-426): Der Voll-Stand hält die Ausbauregeln und läuft über einen
//! echten Spielstand-Ordner (unter `.work/`, statt Gos `t.TempDir`).
use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_sim::island::IslandSave;

use super::log::RecordingLog;
use super::manager::Manager;
use super::testutil::{dev_manager, test_data};
use crate::store::{Saves, Stamp};

/// Kleinste Hub-Stufe je Gebäudeart (`sites` und `islandSites` aus data/hub.json) und die Hub-Stufe der Tore.
fn hub_rules() -> (BTreeMap<String, i64>, i64) {
    let raw = Source::Embedded.read("hub.json").expect("hub.json");
    let h: serde_json::Value = serde_json::from_str(&raw).expect("hub.json lesbar");
    let mut need: BTreeMap<String, i64> = BTreeMap::new();
    for list in ["sites", "islandSites"] {
        for s in h[list].as_array().into_iter().flatten() {
            let (kind, level) = (
                s["kind"].as_str().unwrap_or_default(),
                s["hubLevel"].as_i64().unwrap_or(0).max(1),
            );
            let e = need.entry(kind.to_string()).or_insert(level);
            *e = (*e).min(level);
        }
    }
    (need, h["wallLines"]["gateHubLevel"].as_i64().unwrap_or(0))
}

// B-500/AC-01, B-425/AC-01: Der Voll-Stand hat die Tiefen der Welt-Datei; kein gebauter Platz hat eine höhere Stufe als
// die Hub-Stufe seiner Inselstufe, Gebäude mit Hub-Voraussetzung und Tore stehen nur, wo die Hub-Stufe sie zulässt.
#[test]
fn voll_ausgebaut_haelt_ausbauregeln() {
    let m = dev_manager();
    let s: IslandSave = m.full_island("abc", "2026-10-25T12:00:00.000Z").expect("full_island");
    let depths: Vec<i64> = s.stages.iter().map(|h| h.depth).collect();
    let world = m.world("").expect("Standard-Welt");
    assert_eq!(depths, world.islands[0].depths, "Tiefen wie die Welt-Datei");
    let (min_hub, gate_hub) = hub_rules();
    for h in &s.stages {
        let hub = h.hub_level.max(1);
        for site in h.sites.iter().filter(|s| s.state == "built") {
            let at = format!("Tiefe {}, Hub {hub}: {}@{}", h.depth, site.kind, site.x);
            assert!(site.level <= hub, "{at} hat Stufe {}", site.level);
            if let Some(&l) = min_hub.get(&site.kind) {
                assert!(l <= hub, "{at} gebaut, braucht Hub {l}");
            }
            assert!(
                site.kind != "gate" || hub >= gate_hub,
                "{at}: Tor braucht Hub {gate_hub}"
            );
        }
    }
}

// B-426/AC-02: `full_save` schreibt voll-<seed> über einen echten `Saves` (Datei im Ordner), der Stand lädt.
#[test]
fn voll_ausgebaut_mit_echtem_store() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work")
        .join(format!("room-fullsave-{}", std::process::id()));
    let now = || Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0);
    let saves = Saves::new(root.join("saves"), now);
    let mut m = Manager::new(saves, RecordingLog::default(), now, "codes", test_data());
    m.dev = true;
    let res = m.full_save("abc");
    let file = root.join("saves").join("voll-abc.json");
    let (exists, loaded) = (file.is_file(), m.store.load("voll-abc"));
    let _ = std::fs::remove_dir_all(&root); // vor den Prüfungen aufräumen, ein Rest in `.work/` stört nicht
    assert_eq!(res.as_deref(), Ok("voll-abc"));
    assert!(exists, "Datei fehlt: {}", file.display());
    let raw = loaded.expect("laden");
    let s: IslandSave = serde_json::from_slice(&raw).expect("Spielstand");
    let depths = m.world("").expect("Standard-Welt").islands[0].depths.len();
    assert!(
        s.stages.len() == depths && s.stages[0].hub_level == 5,
        "Stufen {}, Hub der ersten Stufe {}",
        s.stages.len(),
        s.stages[0].hub_level
    );
    assert!(m.load_in(&raw).is_ok(), "Stand lädt");
}
