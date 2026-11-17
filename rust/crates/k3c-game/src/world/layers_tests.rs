//! Port von `native/k3c-game/world/layers_test.go`.

use std::collections::BTreeMap;

use k3c_data::Source;

use super::*;
use crate::view::cell::{new_camera, rect};
use crate::world::backdrop::{BIOME_LAYERS, LAYER_NAMES, Palette, backdrop_group, build_looks, read_palettes};

/// layerCell ist eine Zelle 1920 × 540 px, Maßstab 32 px je Unit, Kamera bei x.
fn layer_cell(x: f64) -> Cell {
    let r = rect(0, 540, 1920, 1080);
    Cell {
        rect: r,
        camera: new_camera(r, x, 32.0),
        ..Cell::default()
    }
}

/// tileX ist das linke Fenster-x der Kachel k einer Schicht (Kulissen-Kachel 768 × 480, Fuß links unten).
fn tile_x(c: &Cell, factor: f64, k: i64) -> RectF {
    tile_rect(c, factor, k, 768, 480, 0.0, 1.0, 1.0)
}

/// AC-02: Bei zwei Kamera-Positionen steht `fern`, `mitte` und `nah` verschieben sich um ΔX · Faktor · Maßstab.
#[test]
fn layer_offsets() {
    let looks = build_looks("hintergruende", 2, &BTreeMap::new());
    let forest = looks.of("forest");
    assert!(forest.scroll[0] == 0.0, "fern scrollt: {}", forest.scroll[0]);
    let (a, b) = (layer_cell(40.0), layer_cell(60.0));
    for (i, factor) in forest.scroll.iter().enumerate() {
        let (ra, rb) = (tile_x(&a, *factor, 2), tile_x(&b, *factor, 2));
        let want = -20.0 * factor * 32.0;
        assert!(
            ((rb.x - ra.x) - want).abs() <= 1.0,
            "{}: Versatz {}, erwartet {want}",
            LAYER_NAMES[i],
            rb.x - ra.x
        );
    }
    assert!(
        forest.scroll[1] > 0.0 && forest.scroll[2] > forest.scroll[1],
        "mitte und nah müssen scrollen, nah schneller: {:?}",
        forest.scroll
    );
}

/// Kacheln: Fuß auf der Boden-Linie, 24 Units breit, Nachbarn stoßen ohne Lücke aneinander.
#[test]
fn tile_rect_faelle() {
    let c = layer_cell(13.3);
    let (r0, r1) = (tile_x(&c, 0.35, 0), tile_x(&c, 0.35, 1));
    assert!(
        r0.y + r0.h == c.ground_y() as f64,
        "Fuß bei {}, Boden-Linie {}",
        r0.y + r0.h,
        c.ground_y()
    );
    assert!(
        r0.x + r0.w == r1.x + TILE_OVERLAP && r0.x == r0.x.round(),
        "Naht: {} + {} ≠ {}",
        r0.x,
        r0.w,
        r1.x
    );
    assert!(
        (r0.w - TILE_UNITS * 32.0).abs() <= 1.0 + TILE_OVERLAP,
        "Breite {}",
        r0.w
    );
}

/// Asset-Quelle: alle Varianten aller Schichten des Levels, unbekanntes Level ohne Grafik; Varianten laufen reihum.
#[test]
fn assets_and_variants() {
    let pal = BTreeMap::from([(
        "forest".to_string(),
        Palette {
            sky: "#8ecae6".into(),
            ground: "#588157".into(),
        },
    )]);
    let ls = build_looks("hintergruende", 2, &pal);
    let got = ls.assets("forest").unwrap_or_default();
    assert!(
        got.len() == 6
            && got.contains(&"hintergruende/hg-wald-fern-v1".to_string())
            && got.contains(&"hintergruende/hg-wald-nah-v2".to_string()),
        "Assets: {got:?}"
    );
    assert!(ls.assets("nirgends").is_none(), "unbekanntes Level mit Grafik");
    let g = ls.of("forest").ground;
    assert!(g[0] == 0x58 && g[1] == 0x81 && g[2] == 0x57, "Boden-Farbe {g:?}");
    let cases: [(&[i64], i64, i64, i64); 5] = [
        (&[], 3, 2, 1),
        (&[1, 2, 3], 1, 2, 2),
        (&[1, 2, 3], 2, 2, 1),
        (&[1, 2, 3], -1, 2, 1),
        (&[4, 5], 4, 5, 4),
    ];
    for (seq, k, n, v) in cases {
        let got = variant(seq, k, n);
        assert_eq!(got, v, "variant({seq:?}, {k}, {n}) = {got}, erwartet {v}");
    }
}

/// Die Daten im Repo: jedes Biom mit Kulisse hat eine Palette, paper.json nennt Gruppe und Varianten.
#[test]
fn load_looks_repo_daten() {
    let src = Source::from_env();
    let (group, n, err) = backdrop_group(&src);
    assert!(
        err.is_none() && group == "hintergruende" && n >= 1,
        "paper.json: {group:?} {n} {err:?}"
    );
    let (pal, err) = read_palettes(&src);
    assert!(err.is_none(), "{err:?}");
    for (id, _, _) in BIOME_LAYERS {
        assert!(pal.contains_key(id), "Biom {id} ohne Palette");
    }
}

/// B-947/AC-01: Bei jedem Maßstab und jeder Kameralage deckt die Kachel k bis mindestens zur linken Kante der Kachel
/// k+1; keine Lücke, auch mit Schicht-Faktoren und gebrochenen Maßstäben.
#[test]
fn tiles_have_no_gap() {
    for scale in [17.3, 24.0, 28.8, 32.0, 37.5, 41.1, 64.7] {
        for x in [0.0, 13.3, 77.77, -5.5, 1234.567] {
            for factor in [0.0, 0.2, 0.35, 0.5, 1.0] {
                let mut c = layer_cell(x);
                c.camera = new_camera(c.rect, x, scale);
                for k in -3..6 {
                    let (a, b) = (tile_x(&c, factor, k), tile_x(&c, factor, k + 1));
                    assert!(
                        a.x + a.w >= b.x,
                        "Lücke {} px: Maßstab {scale}, x {x}, Faktor {factor}, Kachel {k}",
                        b.x - a.x - a.w
                    );
                }
            }
        }
    }
}
