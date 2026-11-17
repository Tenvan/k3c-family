//! Zusatz-Tests zu `lightdraw.rs` (Go hat keine eigene Datei dazu): Die Lichtkarte wird gerechnet, auch wenn der
//! Canvas keinen Multiply-Blend kennt.

use k3c_sim::state::{Castle, World as SimWorld};

use super::*;
use crate::world::light::LightSource;
use crate::world::testutil::{cycle, layer_cell, near};

#[test]
fn light_mesh_fill() {
    let mut light = LightData {
        night: 0.2,
        glow_color: "#ffffff".into(),
        ..LightData::default()
    };
    light.sources.insert(
        "campfire".into(),
        LightSource {
            radius: 12.0,
            strength: 1.0,
        },
    );
    light.prepare();
    let wd = SimWorld {
        castle: Castle {
            x: 40.0,
            ..Castle::default()
        },
        hub_level: 1,
        cycle: cycle("night", 0.0),
        ..SimWorld::default()
    };
    let c = layer_cell(40.0);
    let mut m = LightMesh::default();
    m.fill(&c, &light, &wd);
    assert_eq!(m.verts.len(), 2 * (LIGHT_COLUMNS + 1));
    assert_eq!(m.indices.len(), 6 * LIGHT_COLUMNS);
    // Mitte der Zelle = Kamera bei der Burg: voll hell; Rand: Nacht-Helligkeit
    let mid = &m.verts[2 * (LIGHT_COLUMNS / 2)];
    assert!(near(f64::from(mid.color[0]), 1.0), "Mitte {:?}", mid.color);
    let edge = &m.verts[0];
    assert!(
        near(f64::from(edge.color[0]), f64::from(0.2f32)) || (edge.color[0] - 0.2).abs() < 1e-6,
        "Rand {:?}",
        edge.color
    );
    assert_eq!((m.verts[0].pos[1], m.verts[1].pos[1]), (540.0, 1080.0));
    assert!(column_x(&c, 0) == 0.0 && column_x(&c, LIGHT_COLUMNS) == 1920.0);
}
