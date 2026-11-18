//! Tests der Hub-Stufen-Vorschau (Go: `preview_test.go`, `PreviewHubLevel`, WZ7.8, B-460/AC-03, AC-08). Sie bauen eine Welt mit
//! `create_world` und brauchen den Zyklus (RU14.2), Plätze und Ruinen (RU18) und `dev_build_site` (RU19); `own_*` läuft ohne.

use super::preview::{preview_cycle, preview_hub_level};
use super::testutil::*;
use super::{Options, hub_of, step, vals};
use crate::state::{PlayerCommand, Site, World};

fn preview_json(w: &World) -> String {
    serde_json::to_string(w).expect("JSON")
}

#[test]
#[ignore = "wartet auf RU19"]
fn preview_hub_level_baut_nach_den_regeln_der_sim() {
    let hub_levels = |w: &World| hub_of(w).levels.len() as i64;
    let mut level = 1;
    while level <= hub_levels(&new_world(0, "x", Options::default())) {
        let mut w = new_world(0, "x", Options::default());
        preview_hub_level(&mut w, level).unwrap_or_else(|e| panic!("Stufe {level}: {e}"));
        assert_eq!(w.hub_level, level, "Stufe {level}: HubLevel");
        let h = w.hub_site.as_ref().expect("Hub-Platz");
        assert!(
            h.upgrade.is_empty() && h.state == "built",
            "Stufe {level}: Hub-Platz {h:?}"
        );
        let v = vals(&w);
        for s in &w.sites {
            // Regel einzeln aus den Daten: Tor bleibt, Mauerlinie k ab Hub-Stufe k, Platz ab seiner Hub-Stufe.
            let mut want = v.buildings.get(&s.kind).is_some_and(|b| b.hp > 0.0) && site_hub_level(&w, s) <= level;
            if site_line(&w, s).is_some_and(|index| index > level) {
                want = false;
            }
            assert_eq!(
                s.state == "built",
                want,
                "Hub {level}: {} bei {} gebaut, erwartet {want}",
                s.kind,
                s.x
            );
        }
        level += 1;
    }
}

/// Go: `siteHubLevel` (hub_level.go), RU18.
fn site_hub_level(_w: &World, _s: &Site) -> i64 {
    panic!("RU18: site_hub_level")
}

/// Go: `siteLine` (lines_shim_test.go): der Index der Mauerlinie des Platzes.
fn site_line(w: &World, s: &Site) -> Option<i64> {
    super::testutil::site_line(w, s).map(|l| l.index)
}

/// Go: `builtCount`: zählt die gebauten Plätze einer Art.
fn built_count(w: &World, kind: &str) -> usize {
    w.sites.iter().filter(|s| s.kind == kind && s.state == "built").count()
}

#[test]
#[ignore = "wartet auf RU19"]
fn preview_hub_level_feste_faelle() {
    // Feste Erwartung aus data/hub.json (healer ab Hub 3, armory ab Hub 4, Mauerlinien 1 bis 5), unabhängig vom Code.
    let mut w = new_world(0, "x", Options::default());
    preview_hub_level(&mut w, 3).expect("Stufe 3");
    assert!(
        built_count(&w, "healer") == 1 && built_count(&w, "armory") == 0,
        "Hub 3"
    );
    for s in &w.sites {
        if let Some(index) = site_line(&w, s)
            && s.state == "built"
            && index > 3
        {
            panic!("Hub 3: {} der Linie {index} gebaut", s.kind);
        }
    }
    let mut w = new_world(0, "x", Options::default());
    preview_hub_level(&mut w, 1).expect("Stufe 1");
    assert!(
        built_count(&w, "healer") == 0 && built_count(&w, "workshop") == 1,
        "Hub 1"
    );
}

/// Go: `rotationFehler`: prüft die Folge der Stufen einer Art bei Hub-Stufe `level`: 1…level, dann von vorn, jede Stufe
/// vertreten.
fn rotation_fehler(kind: &str, level: i64, levels: &[i64]) -> Vec<String> {
    let mut errs = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (i, &got) in levels.iter().enumerate() {
        seen.insert(got);
        let want = i as i64 % level + 1;
        if got != want {
            errs.push(format!(
                "{kind} Nr. {i} hat Stufe {got}, erwartet {want} (Folge {levels:?})"
            ));
        }
    }
    if levels.len() as i64 >= level && seen.len() as i64 != level {
        errs.push(format!("{kind} zeigt Stufen {seen:?}, erwartet genau 1…{level}"));
    }
    errs
}

#[test]
fn rotation_fehler_erkennt_die_reihum_folge() {
    assert!(rotation_fehler("wall", 3, &[1, 2, 3, 1, 2]).is_empty());
    assert_eq!(rotation_fehler("wall", 3, &[1, 2, 2]).len(), 2);
    assert_eq!(rotation_fehler("wall", 2, &[2, 2]).len(), 2);
}

#[test]
#[ignore = "wartet auf RU19"]
fn preview_hub_level_stufen_laufen_reihum() {
    // Feste Reihum-Folgen: Mauer und Turm haben fünf Stufen; bei Hub n zählt der Zähler 1…n und beginnt dann von vorn.
    let n = hub_of(&new_world(0, "x", Options::default())).levels.len() as i64;
    for level in 1..=n {
        let mut w = new_world(0, "x", Options::default());
        preview_hub_level(&mut w, level).expect("Vorschau");
        let v = vals(&w);
        let mut seq: std::collections::BTreeMap<&str, Vec<i64>> = std::collections::BTreeMap::new();
        for s in &w.sites {
            let levels = v.buildings.get(&s.kind).map_or(&[][..], |b| b.levels.as_slice());
            if s.state != "built" || levels.is_empty() {
                continue;
            }
            seq.entry(s.kind.as_str()).or_default().push(s.level);
            let ok = s.level >= 1 && s.level <= level && s.hp == levels[(s.level - 1) as usize].hp && s.max_hp == s.hp;
            assert!(
                ok,
                "Hub {level}: {} Stufe {} HP {}/{} verletzt die Regeln",
                s.kind, s.level, s.hp, s.max_hp
            );
        }
        for (kind, levels) in &seq {
            let errs = rotation_fehler(kind, level, levels);
            assert!(errs.is_empty(), "Hub {level}: {errs:?}");
        }
        assert!(level < 3 || seq.contains_key("wall"), "Hub {level}: keine Mauer gebaut");
    }
}

#[test]
#[ignore = "wartet auf RU19"]
fn preview_hub_level_fehler_und_unabhaengigkeit() {
    let n = hub_of(&new_world(0, "x", Options::default())).levels.len() as i64;
    for level in [0, -1, n + 1] {
        let mut w = new_world(0, "x", Options::default());
        let before = preview_json(&w);
        assert!(preview_hub_level(&mut w, level).is_err(), "Stufe {level}: kein Fehler");
        assert_eq!(before, preview_json(&w), "Stufe {level}: Welt nach Fehler verändert");
    }
    let build = |level: i64| {
        let mut w = new_world(0, "x", Options::default());
        preview_hub_level(&mut w, level).expect("Hub-Stufe");
        preview_cycle(&mut w, "dusk", 0.5).expect("Tageszeit");
        preview_json(&w)
    };
    let (a1, b, a2) = (build(2), build(n), build(2));
    assert!(
        a1 == a2 && a1 != b,
        "Aufrufe beeinflussen sich oder sind nicht deterministisch"
    );
}

#[test]
fn own_preview_hub_level_fehler_ohne_create_world() {
    let (mut w, _) = built_wall_world();
    let n = hub_of(&w).levels.len() as i64;
    assert!(n >= 3, "hub.json hat {n} Stufen");
    for level in [0, -1, n + 1] {
        let before = preview_json(&w);
        let err = preview_hub_level(&mut w, level).expect_err("Stufe außerhalb");
        assert_eq!(err, format!("vorschau: Hub-Stufe {level} außerhalb von 1 bis {n}"));
        assert_eq!(before, preview_json(&w), "Stufe {level}: Welt nach Fehler verändert");
    }
}

#[test]
#[ignore = "wartet auf RU19"]
fn preview_welt_laeuft_weiter() {
    // Gleiche Eingaben → nach 120 Ticks dieselbe Welt, und der Zyklus läuft von der gesetzten Tageszeit aus weiter.
    let run = || {
        let mut w = new_world(0, "x", Options::default());
        let n = hub_of(&w).levels.len() as i64;
        preview_hub_level(&mut w, n).expect("Hub-Stufe");
        preview_cycle(&mut w, "night", 0.5).expect("Tageszeit");
        for _ in 0..120 {
            let cmds = vec![PlayerCommand::default(); w.players.len()];
            step(&mut w, &cmds, None, DT);
        }
        (preview_json(&w), w.cycle.phase.clone())
    };
    let (a, phase) = run();
    let (b, _) = run();
    assert_eq!(a, b, "gleiche Eingaben liefern nach 120 Ticks verschiedene Welten");
    assert_eq!(phase, "night", "nach 120 Ticks Phase {phase:?}");
}
