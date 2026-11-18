//! Tests aus `preview_test.go` (WZ7.1, B-460/AC-03, AC-08): Vorschau-Welt für den Welt-Betrachter. Die Hilfen (`preview_opts`,
//! `must_preview`, `json_of`, `built_of`, `built_kinds`, `hub_levels`) nutzen auch die übrigen `preview_*_tests`. Gos
//! `fullData()`/`level.LoadBiome` sind hier die geteilten Test-Daten (`testutil::test_data`).
use std::collections::BTreeMap;

use k3c_data::biome::Biome;
use k3c_sim::state::World;
use k3c_sim::world::preview::{SiteLook, preview_hub_level};
use k3c_sim::world::{Options as WorldOptions, create_world};
use serde::Serialize;

use super::testutil::test_data;
use super::{PreviewOptions, PreviewSource, build_preview, preview_site_key};
use crate::fehler::Fehler;

pub(super) fn preview_opts() -> PreviewOptions {
    PreviewOptions {
        seed: "abc".into(),
        source: PreviewSource { biome: "forest".into() },
        ..PreviewOptions::default()
    }
}

pub(super) fn preview(o: &PreviewOptions) -> Result<World, Fehler> {
    build_preview(o, &test_data())
}

pub(super) fn must_preview(o: &PreviewOptions) -> World {
    match preview(o) {
        Ok(w) => w,
        Err(e) => panic!("{e}"),
    }
}

pub(super) fn json_of<T: Serialize>(v: &T) -> String {
    serde_json::to_string(v).expect("JSON")
}

/// Zahl der gebauten Plätze der Art `kind`.
pub(super) fn built_of(w: &World, kind: &str) -> usize {
    w.sites.iter().filter(|s| s.kind == kind && s.state == "built").count()
}

/// Zahl der Hub-Stufen aus data/hub.json.
pub(super) fn hub_levels() -> i64 {
    (test_data().hub.levels.len() as i64).max(1)
}

pub(super) fn biome(id: &str) -> Biome {
    test_data().biomes.iter().find(|b| b.id == id).cloned().expect("Biom")
}

/// Eine frische Welt ohne Vorschau (Gos `sim.CreateWorld(biome, seed, sim.Options{CycleSpeed: 1})`).
pub(super) fn fresh_world(biome_id: &str, seed: &str) -> World {
    let opts = WorldOptions {
        cycle_speed: 1.0,
        ..WorldOptions::default()
    };
    create_world(&biome(biome_id), seed, &opts).expect("Welt")
}

/// Gebaute Plätze je Art.
fn built_kinds(w: &World) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for s in w.sites.iter().filter(|s| s.state == "built") {
        *out.entry(s.kind.clone()).or_insert(0) += 1;
    }
    out
}

#[test]
fn build_preview_lehnt_ungueltige_optionen_ab() {
    let top = hub_levels();
    type Mutate = Box<dyn Fn(&mut PreviewOptions)>;
    let cases: Vec<(&str, Mutate)> = vec![
        ("Seed leer", Box::new(|o| o.seed = String::new())),
        ("Seed zu lang", Box::new(|o| o.seed = "a".repeat(65))),
        ("Quelle fehlt", Box::new(|o| o.source = PreviewSource::default())),
        ("Biom unbekannt", Box::new(|o| o.source.biome = "nirgendwo".into())),
        ("Biom mit Pfad", Box::new(|o| o.source.biome = "../hub".into())),
        ("Hub-Stufe -1", Box::new(|o| o.hub_level = -1)),
        ("Hub-Stufe zu hoch", Box::new(move |o| o.hub_level = top + 1)),
        ("Phase unbekannt", Box::new(|o| o.phase = "mittag".into())),
        ("Fortschritt -0.1", Box::new(|o| o.progress = -0.1)),
        ("Fortschritt 1", Box::new(|o| o.progress = 1.0)),
        ("Fortschritt 1.5", Box::new(|o| o.progress = 1.5)),
    ];
    for (name, mutate) in cases {
        let mut o = preview_opts();
        mutate(&mut o);
        assert!(preview(&o).is_err(), "{name}: kein Fehler");
    }
}

#[test]
fn build_preview_hat_keine_buerger_keine_gegner() {
    let raw = fresh_world("forest", "abc");
    assert!(
        !raw.troops.is_empty(),
        "Ausgangswelt hat keine Truppen, der Test prüfte sonst ins Leere"
    );
    let w = must_preview(&preview_opts());
    assert!(
        w.troops.is_empty() && w.enemies.is_empty() && w.spawn_queue.is_empty() && w.players.is_empty(),
        "Truppen {}, Gegner {}, Warteschlange {}, Spieler {}",
        w.troops.len(),
        w.enemies.len(),
        w.spawn_queue.len(),
        w.players.len()
    );
    let js = json_of(&w);
    for key in [r#""troops":[]"#, r#""enemies":[]"#, r#""spawnQueue":[]"#] {
        assert!(
            js.contains(key),
            "JSON enthält {key} nicht (Listen müssen [] sein, nicht null)"
        );
    }
    assert!(
        !w.nodes.is_empty() && !w.sites.is_empty(),
        "Umgebung fehlt: Ressourcen {}, Plätze {}",
        w.nodes.len(),
        w.sites.len()
    );
}

// Feste Erwartungen aus data/hub.json (healer ab Hub 3, armory ab Hub 4); Standard (0) ist die höchste Stufe.
// Dazu je Stufe alle Arten wie bei `preview_hub_level` auf einer frischen Welt (genau die freigeschalteten Plätze).
#[test]
fn build_preview_hub_stufen() {
    let top = hub_levels();
    for level in 0..=top {
        let expected = if level == 0 { top } else { level };
        let mut o = preview_opts();
        o.hub_level = level;
        let w = must_preview(&o);
        let mut reference = fresh_world("forest", "abc");
        preview_hub_level(&mut reference, expected).expect("Hub-Stufe");
        assert!(
            w.hub_level == expected && built_kinds(&w) == built_kinds(&reference),
            "Hub {level}: HubLevel {}, gebaut {:?}; erwartet {expected} und {:?}",
            w.hub_level,
            built_kinds(&w),
            built_kinds(&reference)
        );
        let healer = usize::from(expected >= 3);
        let armory = usize::from(expected >= 4);
        assert!(
            built_of(&w, "healer") == healer && built_of(&w, "armory") == armory,
            "Hub {level}: healer {} (erwartet {healer}), armory {} (erwartet {armory})",
            built_of(&w, "healer"),
            built_of(&w, "armory")
        );
    }
}

#[test]
fn build_preview_tageszeit() {
    for phase in ["", "day", "dusk", "night"] {
        let mut o = preview_opts();
        o.phase = phase.into();
        o.progress = 0.5;
        let w = must_preview(&o);
        let want = if phase.is_empty() { "day" } else { phase };
        assert!(
            w.cycle.phase == want && w.cycle.progress == 0.5,
            "Phase {phase:?}: Zyklus {:?}",
            w.cycle
        );
    }
}

#[test]
fn build_preview_gebaeude_auswahl() {
    let mut o = preview_opts();
    let all = must_preview(&o);
    let off = all
        .sites
        .iter()
        .find(|s| s.kind == "wall" && s.state == "built")
        .expect("kein gebauter Mauerplatz im Vergleichsstand");
    let off_key = preview_site_key(&off.kind, off.x);
    o.buildings = BTreeMap::from([
        (off_key.clone(), false),
        ("gibt@es-nicht".to_string(), false),
        (preview_site_key("nursery", 123456.0), true),
    ]);
    let part = must_preview(&o);
    assert_eq!(
        part.sites.len(),
        all.sites.len() - 1,
        "Plätze {}, erwartet {} (einer abgewählt, unbekannte Schlüssel ignoriert)",
        part.sites.len(),
        all.sites.len() - 1
    );
    assert!(
        part.sites.iter().all(|s| preview_site_key(&s.kind, s.x) != off_key),
        "abgewählter Platz ist noch da"
    );
    let rest: BTreeMap<String, i64> = all
        .sites
        .iter()
        .map(|s| (preview_site_key(&s.kind, s.x), s.level))
        .collect();
    for s in &part.sites {
        let want = rest[&preview_site_key(&s.kind, s.x)];
        assert_eq!(
            s.level, want,
            "{} bei {}: Stufe {} statt {want} (Abwahl darf die Stufen der übrigen nicht ändern)",
            s.kind, s.x, s.level
        );
    }
}

#[test]
fn build_preview_ist_deterministisch_ohne_globalen_zustand() {
    let mut a = preview_opts();
    a.hub_level = 3;
    a.phase = "dusk".into();
    a.progress = 0.25;
    let mut b = preview_opts();
    b.seed = "anderer".into();
    b.hub_level = 5;
    b.phase = "night".into();
    let first = json_of(&must_preview(&a));
    let _ = must_preview(&b);
    assert_eq!(
        first,
        json_of(&must_preview(&a)),
        "gleiche Optionen liefern nach einem anderen Aufruf eine andere Welt"
    );
    assert_ne!(
        json_of(&must_preview(&a)),
        json_of(&must_preview(&b)),
        "verschiedene Optionen liefern dieselbe Welt"
    );
}

#[test]
fn build_preview_nullwert_verhaelt_sich_wie_standard() {
    let zero = must_preview(&preview_opts());
    let mut explicit = preview_opts();
    explicit.hub_level = hub_levels();
    explicit.phase = "day".into();
    explicit.buildings = BTreeMap::new();
    assert_eq!(
        json_of(&zero),
        json_of(&must_preview(&explicit)),
        "Nullwerte (Hub 0, Phase leer, keine Auswahl) entsprechen nicht der höchsten Hub-Stufe, day, leere Auswahl"
    );
}

#[test]
fn build_preview_sites() {
    let mut o = PreviewOptions {
        seed: "k3c".into(),
        source: PreviewSource { biome: "forest".into() },
        ..PreviewOptions::default()
    };
    let base = must_preview(&o);
    let key = base
        .sites
        .iter()
        .find(|s| s.kind == "wall" && s.state == "built")
        .map(|s| preview_site_key(&s.kind, s.x))
        .expect("Mauerplatz");
    o.sites = BTreeMap::from([
        (
            key.clone(),
            SiteLook {
                unbuilt: true,
                ..SiteLook::default()
            },
        ),
        (
            "nix@1".to_string(),
            SiteLook {
                level: 3,
                ..SiteLook::default()
            },
        ),
    ]);
    let w = must_preview(&o);
    assert!(
        w.sites
            .iter()
            .all(|s| preview_site_key(&s.kind, s.x) != key || s.state != "built"),
        "Platz blieb gebaut"
    );
    o.sites = BTreeMap::from([(
        key,
        SiteLook {
            level: 99,
            ..SiteLook::default()
        },
    )]);
    assert!(preview(&o).is_err(), "ungültige Stufe wurde angenommen");
}

// Schlüssel wie Gos `strconv.FormatFloat(x, 'f', -1, 64)`: kürzester Text ohne Exponent.
#[test]
fn preview_site_key_formatiert_wie_go() {
    let cases = [
        ("wall", -3.5, "wall@-3.5"),
        ("wall", 120.0, "wall@120"),
        ("tower", 0.1, "tower@0.1"),
        ("tower", 0.0, "tower@0"),
        ("farm", 1e21, "farm@1000000000000000000000"),
        ("farm", f64::INFINITY, "farm@+Inf"),
    ];
    for (kind, x, want) in cases {
        assert_eq!(preview_site_key(kind, x), want, "{kind} {x}");
    }
}

#[test]
fn valid_preview_biome_wie_regex() {
    let long = "a".repeat(33);
    let max = "a".repeat(32);
    let cases = [
        ("forest", true),
        ("a-1", true),
        (max.as_str(), true),
        ("", false),
        (long.as_str(), false),
        ("Forest", false),
        ("../hub", false),
        ("a_b", false),
        ("äö", false),
    ];
    for (biome, want) in cases {
        assert_eq!(super::valid_preview_biome(biome), want, "{biome:?}");
    }
}
