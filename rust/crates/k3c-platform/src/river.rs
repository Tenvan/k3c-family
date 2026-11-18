//! Fluss-Shader (B-1070, B-1071): Uniforms des Aufrufs [`crate::Canvas::river`], Layout wie die Uniforms von
//! `native/k3c-game/world/water/river.kage`, und das Fragment des Shaders als Referenz in Rust.

/// Uniforms des Fluss-Shaders. `origin` ist das Fenster-px der linken oberen Ecke des Rechtecks, `rows` sind die sechs
/// Reihen als vec4 (x = oben, y = Höhe, z = Kachelbreite, w = erste Kachelkante), `phase` Bogenmaß (0 … 2π:
/// x = Verzerrung, y = Heben des Bands, z = Glanz); `base`, `wave`, `shade`, `glint` sind Farben (Kanäle 0 … 1).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RiverUniforms {
    pub origin: [f32; 2],
    pub rows: [f32; 24],
    pub phase: [f32; 3],
    pub base: [f32; 3],
    pub wave: [f32; 3],
    pub shade: [f32; 3],
    pub glint: [f32; 3],
}

/// 2π wie `tau` in `river.kage`.
pub(crate) const TAU: f32 = std::f32::consts::TAU;
/// Höhe der Wellen-Kachel in Quell-px (`srcH` in `river.kage`, `riverSrcH` in Go); Band und Schatten rechnen in
/// dieser Einheit.
pub(crate) const SRC_H: f32 = 64.0;

/// Größe der Uniforms eines Fluss-Aufrufs im GPU-Puffer in Byte (52 × f32, Layout `struct River` in `river.wgsl`).
pub(crate) const RIVER_BYTES: usize = 208;

/// Packt einen Fluss-Aufruf für `river.wgsl`: Rechteck, Ursprung, 6 Reihen, Phasen und vier Farben, jedes Feld als vec4
/// (Füllwert 0).
pub(crate) fn pack_river(c: &crate::RiverCall, out: &mut [u8]) {
    let (d, u) = (c.dst, &c.u);
    let v3 = |a: [f32; 3]| [a[0], a[1], a[2], 0.0];
    let floats = [d.x, d.y, d.w, d.h, u.origin[0], u.origin[1], 0.0, 0.0]
        .into_iter()
        .chain(u.rows)
        .chain([u.phase, u.base, u.wave, u.shade, u.glint].into_iter().flat_map(v3));
    for (chunk, f) in out[..RIVER_BYTES].as_chunks_mut::<4>().0.iter_mut().zip(floats) {
        *chunk = f.to_ne_bytes();
    }
}

/// Mischt `a` nach `b` um den Anteil `t` (wie `mix` in Kage).
fn mix3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [0, 1, 2].map(|i| a[i] * (1.0 - t) + b[i] * t)
}

/// Glanz liefert 1 auf einem Glanzstrich: drei je Kachel unter dem Band, sie fließen mit und blinken nacheinander
/// (Go: `glint` in `river.kage`).
fn glint(phase_z: f32, u: f32, v: f32, n: f32) -> f32 {
    let s = u * 3.0 + n * 0.37;
    let on = 0.5 + 0.5 * (phase_z + s.floor() * 2.3 + n * 1.7).sin();
    if s - s.floor() < 0.2 && (v - SRC_H * 0.8).abs() < 1.5 && on > 0.6 {
        1.0
    } else {
        0.0
    }
}

/// Fragment des Flusses am Fenster-px `p` (Pixelmitte, wie `Origin + srcPos` im Shader): RGBA, Alpha ist 0 oder 1
/// (Farbe also zugleich gerade und vormultipliziert). Quelle der Rechnung ist `Fragment` in `river.kage`; `f32` wie
/// auf der GPU.
pub fn river_fragment(u: &RiverUniforms, p: [f32; 2]) -> [f32; 4] {
    let row = |i: usize| [u.rows[4 * i], u.rows[4 * i + 1], u.rows[4 * i + 2], u.rows[4 * i + 3]];
    if p[1] < row(0)[0] {
        return [0.0; 4];
    }
    let (mut r, mut n) = (row(0), 0.0_f32);
    for i in 1..6 {
        if p[1] >= row(i)[0] {
            r = row(i);
            n = i as f32;
        }
    }
    // Reihenhöhe 0: v bleibt 0 (keine Division durch 0).
    let t = if r[1] > 0.0 {
        ((p[1] - r[0]) / r[1]).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let v = t * SRC_H;
    let mut x = (p[0] - r[3]) / r[2];
    // Verzerrung: die Reihe schwankt waagrecht, je Tiefe der Reihe versetzt.
    x += 0.025 * (TAU * v / SRC_H + u.phase[0] + n * 1.3).sin();
    // Wellen-Band wie paintWaves: zwei Wellen je Kachel um die Mitte, leicht gehoben.
    let line = SRC_H * 0.5 + 7.0 * (TAU * 2.0 * x).sin() + 1.5 * (u.phase[1] + n * 0.9).sin();
    let mut c = u.base;
    if v > line + 3.0 && v < line + 6.0 {
        c = u.shade;
    }
    c = mix3(c, u.wave, (1.0 - (v - line).abs() / 4.0).clamp(0.0, 1.0));
    c = mix3(c, u.glint, 0.6 * glint(u.phase[2], x, v, n));
    [c[0], c[1], c[2], 1.0]
}

#[cfg(test)]
#[path = "river_tests.rs"]
mod river_tests;
