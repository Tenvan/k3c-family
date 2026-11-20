//! Fälle aus src/scenes/render/buildingSprites.test.ts, hubGrowth.test.ts, backWallRules.test.ts, damageRules.test.ts
//! und scaffoldView.test.ts.

use k3c_sim::state::Site;

use super::buildings::*;
use super::data::building_level_count;
use super::testutil::wall_at;

#[test]
fn site_look_baustellen_ohne_grafik() {
    for state in ["unpaid", "waitingMaterial", "waitingWorker"] {
        let l = site_look("workshop", state, 3);
        assert!(!l.ok(), "{state}: {l:?}");
    }
    assert_eq!(
        site_look("wall", "built", 0),
        Look::new("gebaeude-paper/mauer", "stand"),
        "Mauer ohne Stufe"
    );
    assert_eq!(
        site_look("stairsUp", "built", 4),
        Look::new("gebaeude-paper/treppe-hoch", "stand"),
        "Treppe hoch"
    );
    assert!(!site_look("castle", "built", 1).ok(), "unbekannte Art");
}

#[test]
fn site_look_mauer_und_turm_je_stufe() {
    for (kind, id) in [("wall", "gebaeude-paper/mauer"), ("tower", "gebaeude-paper/turm")] {
        for s in 1..=5 {
            let want = Look::new(id, stage_anim(s));
            assert_eq!(site_look(kind, "built", s), want, "{kind} {s}");
        }
        assert_eq!(
            site_look(kind, "built", 9).anim,
            "stand",
            "{kind} 9: Grundform erwartet"
        );
    }
}

#[test]
fn site_look_hub_gebaeude_und_schiff() {
    for (kind, id) in HUB_BUILDINGS {
        assert_eq!(
            site_look(kind, "built", 6),
            Look::new(id, "stand.s5"),
            "{kind} 6: höchste Stufe erwartet"
        );
        assert_eq!(site_look(kind, "built", 0).anim, "stand", "{kind} 0");
        let want = building_level_count(kind) == 0;
        assert_eq!(
            is_hub_building(kind),
            want,
            "{kind}: Hub-Gebäude, erwartet {want} (B-569)"
        );
    }
    assert!(
        site_stage("workshop", 0, 3) == 3 && site_stage("storage", 2, 5) == 2 && site_stage("wall", 0, 4) == 1,
        "Stufe der Grafik: Werkstatt je Hub-Stufe, Lager und Mauer je Platz"
    );
    for (n, anim) in [(1, "stand"), (2, "stand.s2"), (3, "stand.s3"), (4, "stand.s3")] {
        assert_eq!(
            site_look("ship", "built", n),
            Look::new("gebaeude-paper/schiff", anim),
            "Schiff {n}"
        );
    }
}

type HubCase = (i64, i64, &'static str, HubLook);

/// Fälle der Wachstumsreihe (Hub-Stufe, Schritt, Ausbau-Zustand, erwarteter Look).
fn hub_cases() -> [HubCase; 6] {
    let bg = |id: &'static str, anim: &'static str| Look::new(id, anim);
    [
        (
            1,
            2,
            "",
            HubLook {
                main: bg("gebaeude-paper/lagerfeuer", "stand.s2"),
                ..Default::default()
            },
        ),
        (
            0,
            9,
            "",
            HubLook {
                main: bg("gebaeude-paper/lagerfeuer", "stand.s3"),
                ..Default::default()
            },
        ),
        (
            3,
            1,
            "",
            HubLook {
                main: bg("gebaeude-paper/burg", "stand.s3"),
                ..Default::default()
            },
        ),
        (
            3,
            3,
            "",
            HubLook {
                main: bg("gebaeude-paper/burg", "stand.s3"),
                overlay: bg("gebaeude-paper/burg-ausbau", "stand.s3"),
                scaffold: false,
            },
        ),
        (
            3,
            3,
            "waitingWorker",
            HubLook {
                main: bg("gebaeude-paper/burg", "stand.s3"),
                scaffold: true,
                ..Default::default()
            },
        ),
        (
            7,
            0,
            "",
            HubLook {
                main: bg("gebaeude-paper/burg", "stand.s5"),
                ..Default::default()
            },
        ),
    ]
}

#[test]
fn hub_look_wachstumsreihe() {
    let mut seen: Vec<HubLook> = Vec::new();
    for lv in 1..=5 {
        for step in 1..=3 {
            let h = hub_look_of(lv, step, "");
            if !seen.contains(&h) {
                seen.push(h);
            }
        }
    }
    assert_eq!(seen.len(), 15, "15 Kombinationen, {} verschiedene Looks", seen.len());
    let cases = hub_cases();
    for (lv, step, state, want) in cases {
        let got = hub_look_of(lv, step, state);
        assert_eq!(got, want, "Hub {lv}/{step} {state:?}");
    }
}

fn bw(id: i64, from: f64, to: f64, stage: i64) -> BackWall {
    BackWall { id, from, to, stage }
}

fn walls(list: &[(i64, f64, i64, &str)]) -> Vec<Site> {
    list.iter()
        .map(|&(id, x, level, state)| wall_at(id, x, level, state))
        .collect()
}

/// Fälle der Hintergrundmauern (Plätze, erwartete Abschnitte).
fn back_wall_cases() -> [(Vec<Site>, Vec<BackWall>); 4] {
    let mut mit_turm = walls(&[(1, 520.0, 0, "built"), (2, 560.0, 0, "built")]);
    mit_turm.push(Site {
        id: 9,
        kind: "tower".to_string(),
        x: 540.0,
        state: "built".to_string(),
        ..Default::default()
    });
    [
        (
            walls(&[(1, 480.0, 3, "built"), (2, 440.0, 1, "built"), (3, 400.0, 2, "unpaid")]),
            vec![bw(2, 440.0, 480.0, 1)],
        ),
        (
            walls(&[
                (1, 520.0, 2, "built"),
                (2, 560.0, 4, "built"),
                (3, 600.0, 5, "built"),
                (4, 480.0, 1, "built"),
                (5, 440.0, 2, "built"),
            ]),
            vec![bw(5, 440.0, 480.0, 2), bw(2, 520.0, 560.0, 4), bw(3, 560.0, 600.0, 5)],
        ),
        (
            walls(&[
                (1, 520.0, 1, "built"),
                (2, 560.0, 1, "waitingWorker"),
                (3, 600.0, 1, "built"),
            ]),
            vec![],
        ),
        (mit_turm, vec![bw(2, 520.0, 560.0, 1)]),
    ]
}

#[test]
fn back_walls_abschnitte() {
    let cases = back_wall_cases();
    for (i, (sites, want)) in cases.iter().enumerate() {
        let got = back_walls(Vec::new(), &mut Vec::new(), sites, 500.0);
        assert_eq!(&got, want, "Fall {i}");
    }
    assert!(
        !back_wall_look(6).ok() && back_wall_look(2) == Look::new("gebaeude-paper/hintergrundmauer", "stand.s2"),
        "Hintergrundmauer je Stufe 1–5"
    );
}

#[test]
fn damage_stage_und_ruine() {
    for (hp, want) in [
        (90.0, 0),
        (89.0, 1),
        (60.0, 1),
        (59.0, 2),
        (30.0, 2),
        (29.0, 3),
        (1.0, 3),
    ] {
        assert_eq!(damage_stage("built", hp, 90.0), want, "HP {hp}");
    }
    assert!(
        damage_stage("unpaid", 0.0, 90.0) == 0 && damage_stage("built", 0.0, 0.0) == 0,
        "nicht gebaut oder ohne MaxHP: keine Überlagerung"
    );
    assert!(
        show_rubble(true, "built", 23.0, 90.0)
            && !show_rubble(true, "built", 45.0, 90.0)
            && !show_rubble(false, "built", 23.0, 90.0)
            && !show_rubble(true, "unpaid", 0.0, 90.0),
        "Trümmer nur an einer gebauten Ruine unter 50 %"
    );
    assert_eq!(DAMAGE_SHADE, [0.0, 0.12, 0.25, 0.4]);
}

#[test]
fn geruest_und_bauschild() {
    let want = [
        ("wall", "1"),
        ("tower", "3"),
        ("nursery", "4"),
        ("stairsUp", "4"),
        ("stairsDown", "4"),
        ("workshop", "6"),
        ("storage", "6"),
        ("barracks", "6"),
        ("tavern", "6"),
        ("healer", "6"),
        ("smithy", "6"),
        ("armory", "6"),
        ("ship", "6"),
    ];
    for (kind, c) in want {
        let l = scaffold_look(kind);
        assert_eq!(l.asset, format!("{BUILDING_GROUP}geruest-{c}"), "{kind}: {l:?}");
    }
    assert_eq!(scaffold_look("unbekannt").asset, "gebaeude-paper/geruest-6");
    for state in ["waitingMaterial", "waitingWorker"] {
        assert_eq!(site_marker(state, true), Marker::Scaffold, "{state}: Gerüst");
    }
    assert!(
        site_marker("unpaid", true) == Marker::Sign
            && site_marker("unpaid", false) == Marker::None
            && site_marker("built", true) == Marker::None,
        "Bauschild nur am bezahlbaren ungebauten Platz"
    );
    assert!(
        sign_variant("wall") == 1 && sign_variant("stairsDown") == 2 && sign_variant("tavern") == 0,
        "Varianten des Bauschilds"
    );
}

#[test]
fn stage_anim_begrenzt() {
    for (n, want) in [
        (-1, "stand"),
        (0, "stand"),
        (1, "stand"),
        (2, "stand.s2"),
        (5, "stand.s5"),
        (9, "stand.s5"),
    ] {
        assert_eq!(stage_anim(n), want, "{n}");
    }
}

/// `data.rs` hat in Go keinen Test; Stichprobe, dass die Daten über k3c-data ankommen.
#[test]
fn data_werte_kommen_an() {
    use super::data::{build_cost, materials, site_name, values};
    assert_eq!(site_name("ship"), "Schiff");
    assert_eq!(site_name("bowyer"), "Bogner");
    assert_eq!(site_name("unbekannt"), "unbekannt");
    assert!(build_cost("bowyer", 0).wood > 0 && materials(&build_cost("bowyer", 0)) == vec!["wood"]);
    assert!(!values().depths.is_empty() && values().roots_by_biom.values().all(|r| *r >= 1));
}

/// RU36.3: Nur das fertige Schiff (Segel, ab Stufe 3) liegt im Fluss; Kiel, Rumpf und andere Plätze stehen auf der Gehlinie.
#[test]
fn schiff_liegt_erst_fertig_im_wasser() {
    use super::nature::{SHIP_DROP, ship_drop};
    for (kind, level, want) in [
        ("ship", 1, 0.0),
        ("ship", 2, 0.0),
        ("ship", 3, SHIP_DROP),
        ("ship", 4, SHIP_DROP),
        ("wall", 3, 0.0),
    ] {
        assert_eq!(ship_drop(kind, level), want, "{kind} Stufe {level}");
    }
}
