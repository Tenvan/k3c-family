//! Fluss-Shader (B-1070): Uniforms des Aufrufs [`crate::Canvas::river`], Layout wie die Uniforms von
//! `native/k3c-game/world/water/river.kage`.

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
