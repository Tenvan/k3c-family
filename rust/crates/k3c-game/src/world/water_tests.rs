//! Port von `native/k3c-game/world/water_test.go`.
//!
//! Nicht portiert (kein `ebiten.NewShader` in Rust, der Shader steht in der Plattform): `TestRiverShaderCompiles`,
//! `TestRiverShaderFallback`; dafür prüfen `river_uses_canvas_call` und `river_falls_back_to_tiles` (B-1073) den
//! Aufruf und den Rückfall, die WGSL-Quelle prüft `k3c-platform` (B-1072).

use std::f64::consts::PI;

use std::rc::Rc;

use super::*;
use crate::view::frame::{Frame, Seat, Stage};
use crate::view::settings::Settings;
use crate::world::World;
use crate::world::testutil::layer_cell;
use k3c_platform::{Affine, Filter, FontId, TexId, Vertex};
use k3c_sim::state::World as SimWorld;

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

/// Go: `TestRiverShaderLazy` prüft Übersetzen beim ersten Bedarf: `ready` bereitet einmal vor und meldet dann `true`
/// (ob der Shader zeichnet, entscheidet `Canvas::river`).
#[test]
fn river_shader_lazy() {
    let mut r = RiverWater::default();
    r.init(RiverMode::Tiles);
    assert!(!r.tried, "Kachel-Start bereitet schon vor");
    assert!(r.ready() && r.tried, "Shader-Fassung gewählt: bereit, ein Versuch");
    assert!(r.ready());
    assert_eq!(r.colors[0], rgb(RIVER_BASE));
    assert_eq!(r.colors[3], rgb(RIVER_GLINT));
    let mut s = RiverWater::default();
    s.init(RiverMode::Shader);
    assert!(s.tried, "Shader-Start bereitet vor");
}

/// Zeichenfläche, die Fluss-Aufrufe und Sprites merkt; `river_ok` ist die Antwort auf `river`.
#[derive(Default)]
struct Rec {
    rivers: Vec<(Rect, RiverUniforms)>,
    sprites: Vec<(Rect, [f32; 4])>,
    river_ok: bool,
}

impl Canvas for Rec {
    fn size(&self) -> (f32, f32) {
        (1920.0, 1080.0)
    }
    fn texture_with(&mut self, _: &[u8], _: u32, _: u32, _: Filter) -> TexId {
        TexId(0)
    }
    fn free_texture(&mut self, _: TexId) {}
    fn sprite(&mut self, _: TexId, _: Rect, dst: Rect, tint: [f32; 4]) {
        self.sprites.push((dst, tint));
    }
    fn sprite_tf(&mut self, _: TexId, _: Rect, _: Affine, _: [f32; 4]) {}
    fn mesh(&mut self, _: TexId, _: &[Vertex], _: &[u16]) {}
    fn rect(&mut self, _: Rect, _: [f32; 4]) {}
    fn river(&mut self, dst: Rect, u: &RiverUniforms) -> bool {
        self.rivers.push((dst, *u));
        self.river_ok
    }
    fn stroke_rect(&mut self, _: Rect, _: f32, _: [f32; 4]) {}
    fn fill_circle(&mut self, _: f32, _: f32, _: f32, _: [f32; 4]) {}
    fn stroke_circle(&mut self, _: f32, _: f32, _: f32, _: f32, _: [f32; 4]) {}
    fn stroke_line(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: [f32; 4]) {}
    fn fill_path(&mut self, _: &[[f32; 2]], _: [f32; 4]) {}
    fn text(&mut self, _: &str, _: f32, _: f32, _: f32, _: [f32; 4]) {}
    fn font(&mut self, _: &[u8]) -> Result<FontId, String> {
        Err("keine Schrift".to_string())
    }
    fn text_font(&mut self, _: FontId, _: &str, _: f32, _: f32, _: f32, _: [f32; 4]) {}
    fn text_width(&self, _: FontId, _: &str, _: f32) -> f32 {
        0.0
    }
}

/// Frame mit einer Stufe samt Welt und einem Platz für Spieler 0.
fn river_frame(setting: &str, clock: f64) -> Frame {
    Frame {
        clock,
        settings: Settings {
            river: setting.to_string(),
            ..Settings::default()
        },
        stages: vec![Stage {
            index: 0,
            level: "wald".to_string(),
            world: Some(Rc::new(SimWorld::default())),
            ..Stage::default()
        }],
        seats: vec![Seat::default()],
        ..Frame::default()
    }
}

/// B-1073 AC-01: In Shader-Fassung ruft `draw_river` `Canvas::river` genau einmal je Zelle, mit dem Rechteck und den
/// Uniforms aus `water_layout`; meldet der Canvas `true`, gibt es keine Kacheln.
#[test]
fn river_uses_canvas_call() {
    let mut w = World::new(None);
    w.start = RiverMode::Tiles;
    let c = layer_cell(40.0);
    let f = river_frame(RIVER_SHADER, 3.0);
    let mut cv = Rec {
        river_ok: true,
        ..Rec::default()
    };
    w.draw_river(&mut cv, &f, &c);
    assert_eq!(cv.rivers.len(), 1, "ein Aufruf je Zelle");
    assert!(cv.sprites.is_empty(), "Shader gezeichnet, keine Kacheln");
    let p = water_layout(&c, 3.0);
    let (r, u) = cv.rivers[0];
    assert_eq!(r, Rect::new(p.x as f32, p.top as f32, p.w as f32, p.h as f32));
    let mut want = RiverWater::default();
    want.build();
    want.set(&p);
    assert_eq!(u, want.uniforms());
    assert_eq!(u.base, rgb(RIVER_BASE));
    assert_eq!(u.glint, rgb(RIVER_GLINT));
}

/// B-1073 AC-02: Meldet der Canvas `false`, entstehen dieselben Kachel-Befehle wie in der Kachel-Fassung; die
/// Kachel-Fassung ruft den Shader nie auf.
#[test]
fn river_falls_back_to_tiles() {
    let c = layer_cell(40.0);
    let tiles = |setting: &str, ok: bool| {
        let mut w = World::new(None);
        w.start = RiverMode::Shader;
        let mut cv = Rec {
            river_ok: ok,
            ..Rec::default()
        };
        w.draw_river(&mut cv, &river_frame(setting, 3.0), &c);
        cv
    };
    let want = tiles(RIVER_TILES, true);
    assert!(want.rivers.is_empty(), "Kachel-Fassung ruft river nicht auf");
    assert!(!want.sprites.is_empty());
    let got = tiles(RIVER_SHADER, false);
    assert_eq!(got.rivers.len(), 1, "Shader versucht");
    assert_eq!(got.sprites, want.sprites, "Rückfall zeichnet die Kacheln");
    // "" gilt der Startwert: Shader
    assert_eq!(tiles("", false).rivers.len(), 1);
}

/// B-1073: Zelle ohne sichtbare Fläche (Breite 0): nichts zeichnen, weder Shader noch Kacheln.
#[test]
fn river_empty_cell_draws_nothing() {
    let mut c = layer_cell(40.0);
    // Breite 0 (die Höhe folgt aus dem Boden und wird bei gültiger Zelle nie 0)
    c.rect.max.x = c.rect.min.x;
    assert!(water_layout(&c, 1.0).w <= 0);
    let mut cv = Rec::default();
    let mut r = RiverWater::default();
    assert!(r.draw(&mut cv, &c, 1.0));
    assert!(cv.rivers.is_empty());
}
