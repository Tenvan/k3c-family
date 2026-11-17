//! Port von `native/k3c-game/world/water_test.go`.
//!
//! Nicht portiert (Shader entfällt, siehe Kopf von water.rs): `TestRiverShaderCompiles`, `TestRiverShaderFallback`.

use std::f64::consts::PI;

use super::*;
use crate::view::settings::Settings;
use crate::world::testutil::layer_cell;

/// Schalter K3C_RIVER: nur "tiles" wählt die Kacheln, alles andere (auch leer) den Shader.
#[test]
fn river_mode_of_faelle() {
    let cases = [
        ("", RiverMode::Shader),
        ("shader", RiverMode::Shader),
        ("tiles", RiverMode::Tiles),
        (" Tiles ", RiverMode::Tiles),
        ("x", RiverMode::Shader),
    ];
    for (v, want) in cases {
        assert_eq!(river_mode_of(v), want, "river_mode_of({v:?})");
    }
}

/// Das Rechteck des Shaders beginnt an der ersten Reihe (unter dem Ufer) und reicht bis zum unteren Rand der Zelle;
/// die Reihen sind die der Kacheln (gleiche Lage, gleiches Fließen nach links).
#[test]
fn water_layout_rect() {
    let c = layer_cell(40.0);
    let p = water_layout(&c, 7.5);
    let rows = river_layout(&c, 7.5);
    assert!(p.rows == rows, "Reihen weichen von den Kacheln ab: {:?}", p.rows);
    assert!(
        !(p.x != c.rect.min.x || p.w != c.rect.dx() || p.top as f64 > rows[0].y || p.top as f64 <= rows[0].y - 1.0),
        "Rechteck x {} w {} top {}, erste Reihe bei {}",
        p.x,
        p.w,
        p.top,
        rows[0].y
    );
    assert!(
        !(p.top + p.h != c.rect.max.y || p.top < c.ground_y()),
        "Rechteck {} + {}, Zelle {:?}, Boden {}",
        p.top,
        p.h,
        c.rect,
        c.ground_y()
    );
}

/// Phasen hängen nur von der Uhr ab, liegen in 0 … 2π und laufen auch nach langer Spielzeit stetig weiter.
#[test]
fn water_phase() {
    let c = layer_cell(12.0);
    for tm in [0.0, 1.25, 3600.0, 360000.5] {
        let (a, b) = (water_layout(&c, tm), water_layout(&c, tm));
        assert!(a == b, "t={tm}: nicht deterministisch");
        for (i, ph) in a.phase.iter().enumerate() {
            assert!(!(*ph < 0.0 || *ph >= 2.0 * PI), "t={tm} Phase {i} = {ph}");
        }
    }
    const DT: f64 = 0.01;
    let (a, b) = (water_layout(&c, 360000.0), water_layout(&c, 360000.0 + DT));
    let want = 2.0 * PI * WATER_LIFT_HZ * DT;
    let d = (b.phase[1] - a.phase[1] + 2.0 * PI) % (2.0 * PI);
    assert!(
        (d - want).abs() <= 1e-6,
        "Phase springt nach 100 h: {d}, erwartet {want}"
    );
    assert!(
        phase_at(-0.25, 1.0) == 1.5 * PI,
        "negative Zeit: {}",
        phase_at(-0.25, 1.0)
    );
}

/// Die Uniforms je Zelle landen in den Puffern (Go prüft zusätzlich „legt nichts an“ und die Map-Einträge des
/// Shaders; beides entfällt ohne Shader).
#[test]
fn water_uniforms() {
    let mut r = RiverWater::default();
    let c = layer_cell(40.0);
    let p = water_layout(&c, 3.0);
    r.set(&p);
    assert!(
        r.rows.len() == 4 * RIVER_ROWS && r.rows[4 * 5 + 1] == p.rows[5].h as f32 && r.rows[3] == p.rows[0].x0 as f32,
        "Rows-Uniform {:?}",
        r.rows
    );
    assert_eq!(r.origin, [p.x as f32, p.top as f32]);
    assert_eq!(r.phase[1], p.phase[1] as f32);
    assert_eq!(rgb(RIVER_SHADE), [50.0 / 255.0, 104.0 / 255.0, 150.0 / 255.0]);
}

/// F3 (B-816): Die Einstellung bestimmt die Fassung, "" gilt der Startwert; zwei Umschaltungen führen zurück.
#[test]
fn river_mode_for_faelle() {
    let cases = [
        ("", RiverMode::Shader, RiverMode::Shader),
        ("", RiverMode::Tiles, RiverMode::Tiles),
        (RIVER_TILES, RiverMode::Shader, RiverMode::Tiles),
        (RIVER_SHADER, RiverMode::Tiles, RiverMode::Shader),
    ];
    for (setting, start, want) in cases {
        assert_eq!(
            river_mode_for(setting, start),
            want,
            "river_mode_for({setting:?}, {start:?})"
        );
    }
    let mut s = Settings::default();
    let start = river_mode_of(river_start());
    s.river = s.next_river(river_start()).to_string();
    assert!(
        river_mode_for(&s.river, start) != start,
        "Umschalten ändert die Fassung nicht: {:?}",
        s.river
    );
    s.river = s.next_river(river_start()).to_string();
    assert_eq!(s.river, river_start(), "zweimal umschalten");
}

/// Go: `TestRiverShaderLazy` prüft Übersetzen beim ersten Bedarf; ohne Shader bleibt nur: `ready` versucht einmal und
/// meldet immer `false`.
#[test]
fn river_shader_lazy() {
    let mut r = RiverWater::default();
    r.init(RiverMode::Tiles);
    assert!(!r.tried, "Kachel-Start übersetzt schon");
    assert!(
        !r.ready() && r.tried,
        "kein Shader in Rust: Rückfall auf Kacheln, ein Versuch"
    );
    assert!(!r.ready());
}
