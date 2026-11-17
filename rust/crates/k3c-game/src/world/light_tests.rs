//! Port von `native/k3c-game/world/light_test.go`.

use std::collections::BTreeMap;
use std::path::Path;

use k3c_sim::state::{Castle, Site, World as SimWorld};
use serde::Deserialize;

use super::*;
use crate::world::testutil::{cycle, near};

/// testLight sind Lichtwerte wie data/light.json (eigene Zahlen, damit Datenänderungen die Fälle nicht ändern).
fn test_light() -> LightData {
    let mut d = LightData {
        night: 0.2,
        back: 0.05,
        glow_color: "#ffffff".into(),
        sources: BTreeMap::from([
            (
                "campfire".to_string(),
                LightSource {
                    radius: 12.0,
                    strength: 1.0,
                },
            ),
            (
                "torch".to_string(),
                LightSource {
                    radius: 6.0,
                    strength: 0.8,
                },
            ),
        ]),
        torches: BTreeMap::from([
            ("wall".to_string(), vec![0.0]),
            ("tower".to_string(), vec![-0.75, 0.75]),
        ]),
        ..LightData::default()
    };
    d.prepare();
    d
}

/// AC-03 (Fälle aus daylight.test.ts und nightLight.test.ts): Helligkeit je Tageszeit.
#[test]
fn brightness_by_phase() {
    let d = test_light();
    let cases = [
        (cycle("day", 0.5), 1.0, 1.0),
        (cycle("dusk", 0.5), 0.6, 0.525),
        (cycle("night", 0.95), 0.2, 0.05),
        (cycle("dawn", 0.0), 0.2, 0.05),
        (cycle("dawn", 0.5), 0.6, 0.525),
        (cycle("eclipse", 0.5), 0.2, 0.05),
    ];
    for (c, front, back) in cases {
        let (f, b) = (d.front(&c), d.backdrop(&c));
        assert!(
            near(f, front) && near(b, back),
            "{} {:.2}: Vordergrund {f} Hintergrund {b}, erwartet {front} / {back}",
            c.phase,
            c.progress
        );
    }
    assert!(
        near(daylight(&cycle("dusk", 0.25)), 0.75),
        "Dämmerung dunkelt nicht linear ab"
    );
}

/// Lichtkarte: außen Nacht-Helligkeit, an der Burg (Lagerfeuer) voll, dazwischen linear, am Tag überall 1;
/// eine ausgebaute Burg gibt kein Licht.
#[test]
fn light_level() {
    let d = test_light();
    let mut w = SimWorld {
        castle: Castle {
            x: 10.0,
            ..Castle::default()
        },
        hub_x: 10.0,
        hub_level: 1,
        cycle: cycle("night", 0.0),
        ..SimWorld::default()
    };
    for (x, want) in [(40.0, 0.2), (10.0, 1.0), (16.0, 0.6)] {
        let got = d.level_at(x, &w);
        assert!(near(got, want), "x={x}: {got}, erwartet {want}");
    }
    w.hub_level = 2;
    let got = d.level_at(10.0, &w);
    assert!(near(got, 0.2), "Burg leuchtet: {got}");
    w.hub_level = 1;
    w.cycle = cycle("day", 0.0);
    let got = d.level_at(40.0, &w);
    assert!(got == 1.0, "Tag: {got}");
}

/// Deckkraft des Scheins: am Tag 0, nachts die Stärke, im Morgengrauen fallend, begrenzt auf 0 … 1.
#[test]
fn glow_alpha_faelle() {
    assert!(
        glow_alpha(0.7, &cycle("day", 0.3)) == 0.0 && near(glow_alpha(0.7, &cycle("night", 0.3)), 0.7),
        "Tag/Nacht"
    );
    let (a, b) = (
        glow_alpha(0.7, &cycle("dawn", 0.0)),
        glow_alpha(0.7, &cycle("dawn", 0.5)),
    );
    assert!(a > b, "Morgengrauen steigt: {a} → {b}");
    assert!(
        glow_alpha(3.0, &cycle("night", 0.0)) == 1.0 && glow_alpha(-1.0, &cycle("night", 0.0)) == 0.0,
        "Begrenzung"
    );
}

#[derive(Deserialize)]
struct TorchCase {
    #[serde(rename = "hubX")]
    hub_x: f64,
    #[serde(rename = "offsets")]
    offsets: BTreeMap<String, Vec<f64>>,
    #[serde(rename = "sites")]
    sites: Vec<Site>,
    #[serde(rename = "torches")]
    torches: Vec<f64>,
}

/// Fackeln stehen wie in der Simulation (testdata/torches.json): Jeder Ort leuchtet, gebaute Plätze ohne Fackel nicht.
#[test]
fn torches_like_sim() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/torches.json");
    let raw = std::fs::read_to_string(path).unwrap();
    let tc: TorchCase = serde_json::from_str(&raw).unwrap();
    let mut d = test_light();
    d.torches = tc.offsets.clone();
    let w = SimWorld {
        hub_x: tc.hub_x,
        hub_level: 2,
        sites: tc.sites,
        cycle: cycle("night", 0.0),
        ..SimWorld::default()
    };
    for x in &tc.torches {
        let got = d.level_at(*x, &w);
        assert!(got > d.front(&w.cycle), "Fackel bei {x} leuchtet nicht ({got})");
    }
    let got = torch_xs_of(&d, &w);
    assert_eq!(got, tc.torches, "Fackeln {got:?}, erwartet {:?}", tc.torches);
    let real = load_light();
    let got = torch_xs_of(&real, &w);
    let want = k3c_sim::state::torches::torch_xs_of(&w, &real.torches);
    assert_eq!(got, want, "mit data/light.json: {got:?}, Simulation {want:?}");
}

/// torchXsOf sammelt die Fackel-Orte wie drawTorches (nur für den Test).
fn torch_xs_of(d: &LightData, w: &SimWorld) -> Vec<f64> {
    let mut out = Vec::new();
    for s in &w.sites {
        if s.state != "built" {
            continue;
        }
        for o in d.torches.get(&s.kind).into_iter().flatten() {
            out.push(s.x + torch_side(s.x, w.hub_x) * o);
        }
    }
    out
}

/// Lichtfarbe: Schein in glowColor, nie über 1; data/light.json ist lesbar. (Go prüft hier auch „ohne Allokation“;
/// Rust zählt keine Allokationen, `color_at` legt nichts an: nur Rechnung auf Stapel-Werten.)
#[test]
fn light_color_and_data() {
    let mut d = test_light();
    d.glow_color = "#ff8000".into();
    d.prepare();
    let w = SimWorld {
        castle: Castle {
            x: 10.0,
            ..Castle::default()
        },
        hub_level: 1,
        cycle: cycle("night", 0.0),
        ..SimWorld::default()
    };
    let (r, g, b) = d.color_at(10.0, &w);
    assert!(
        r == 1.0 && near(g, 0.2 + 0.8 * 128.0 / 255.0) && near(b, 0.2),
        "Farbe {r} {g} {b}"
    );
    let real = load_light();
    assert!(
        real.night > 0.0 && real.night < 1.0 && real.campfire.valid() && real.torch.valid(),
        "data/light.json: {real:?}"
    );
}
