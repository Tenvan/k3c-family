//! Port von `native/k3c-game/world/river_test.go`.

use super::*;
use crate::world::testutil::layer_cell;

/// AC-02: Ufer und Fluss füllen genau das untere Drittel der Zelle; sechs Reihen, nach vorn höher und breiter.
#[test]
fn river_fills_lower_third() {
    let c = layer_cell(40.0);
    let rows = river_layout(&c, 0.0);
    let third = c.river_rect();
    assert!(
        third.min.y == c.ground_y() && third.max.y == c.rect.max.y,
        "Fluss-Drittel {third:?}, Zelle {:?}",
        c.rect
    );
    let got = rows[0].y - bank_height(&c);
    assert!(
        (got - third.min.y as f64).abs() <= 1e-9,
        "erste Reihe beginnt bei {got} unter dem Ufer, Drittel bei {}",
        third.min.y
    );
    let last = rows[RIVER_ROWS - 1];
    assert!(
        (last.y + last.h - third.max.y as f64).abs() <= 1e-9,
        "letzte Reihe endet bei {}, Zelle bei {}",
        last.y + last.h,
        third.max.y
    );
    for i in 1..RIVER_ROWS {
        assert!(
            !(rows[i].h <= rows[i - 1].h
                || rows[i].tile <= rows[i - 1].tile
                || (rows[i].y - (rows[i - 1].y + rows[i - 1].h)).abs() > 1e-9),
            "Reihe {i}: {:?} nach {:?}",
            rows[i],
            rows[i - 1]
        );
    }
}

/// Die Kacheln decken die Zelle ab dem linken Rand: Die erste liegt höchstens eine Kachel links davon.
#[test]
fn river_covers_left_edge() {
    let c = layer_cell(17.25);
    for tm in [0.0, 3.7, 1000.0] {
        for (i, r) in river_layout(&c, tm).iter().enumerate() {
            assert!(
                !(r.x0 > c.rect.min.x as f64 || r.x0 <= c.rect.min.x as f64 - r.tile),
                "t={tm} Reihe {i}: x0 {}, Kachel {}",
                r.x0,
                r.tile
            );
        }
    }
}

/// shift ist die Verschiebung von a nach b, auf eine halbe Kachel um 0 gefaltet.
fn shift(a: &RiverRow, b: &RiverRow) -> f64 {
    let mut d = (b.x0 - a.x0) % a.tile;
    if d > a.tile / 2.0 {
        d -= a.tile;
    }
    if d < -a.tile / 2.0 {
        d += a.tile;
    }
    d
}

/// Der Fluss fließt nach links, vordere Reihen schneller; eine Kamerafahrt nach rechts schiebt ihn zusätzlich nach links.
#[test]
fn river_flows_left() {
    const DT: f64 = 0.1;
    let c = layer_cell(40.0);
    let (a, b) = (river_layout(&c, 5.0), river_layout(&c, 5.0 + DT));
    for i in 0..RIVER_ROWS {
        let g = 1.0 + i as f64 * RIVER_GROW;
        let want = -RIVER_FLOW_UNITS * g * c.camera.scale * DT;
        let d = shift(&a[i], &b[i]);
        assert!((d - want).abs() <= 1e-6, "Reihe {i}: Weg {d}, erwartet {want}");
    }
    let moved = layer_cell(40.5);
    let m = river_layout(&moved, 5.0);
    let d = shift(&a[0], &m[0]);
    assert!(
        (d + 0.5 * c.camera.scale).abs() <= 1e-6,
        "Kamerafahrt: Ufer-Reihe verschiebt sich um {d}, erwartet wie die Welt"
    );
}

// Go: TestRiverNoAlloc (AllocsPerRun) entfällt: Rust zählt keine Allokationen; `river_layout` gibt ein Array
// auf dem Stapel zurück und legt nichts an.

/// Zusatz: Die Wellen-Kachel ist oben wie unten Grundfarbe, auf der Mittellinie Wellenfarbe.
#[test]
fn paint_waves_rows() {
    let mut img = vec![0u8; RIVER_SRC_W * RIVER_SRC_H * 4];
    paint_waves(&mut img);
    let px = |x: usize, y: usize| img[4 * (y * RIVER_SRC_W + x)..4 * (y * RIVER_SRC_W + x) + 4].to_vec();
    assert_eq!(px(0, 0), RIVER_BASE.to_vec());
    assert_eq!(px(RIVER_SRC_W - 1, RIVER_SRC_H - 1), RIVER_BASE.to_vec());
    assert_eq!(px(0, RIVER_SRC_H / 2), RIVER_WAVE.to_vec());
}
