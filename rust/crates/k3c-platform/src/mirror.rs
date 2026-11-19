//! Spiegel-Fluss (RU55.3, B-1044): Uniforms des Aufrufs [`crate::Canvas::mirror`] und das Fragment des Shaders als
//! Referenz in Rust (`mirror.wgsl` rechnet dasselbe).
//!
//! Der Spiegel liest das fertig gezeichnete Bild über der Gehlinie (alles, was die Zelle bis dahin gezeichnet hat) und
//! klappt es an der Unterkante des Ufers nach unten: dunkler (`brightness`), leicht in Wasserfarbe getönt (`tint`),
//! senkrecht gestaucht (`squash`), mit sanftem waagerechtem Schwanken (`wobble`) und mit hellen waagerechten
//! Glanzstrichen. Lichtquellen bleiben hell: Pixel, die nachts trotz Lichtkarte hell sind (`emissive`), behalten ihre
//! Helligkeit. Das Backend kopiert dafür den Streifen `[src_top, ground]` in eine Zwischentextur.

/// Uniforms des Spiegels; alle Längen in logischen Pixeln (Fenster-px, 1920×1080).
///
/// Der Spiegel füllt das Rechteck `dst` des Aufrufs. `src_top` ist die Oberkante des gelesenen Streifens, `ground` die
/// Gehlinie (Unterkante des Streifens), `axis` die Oberkante des Spiegels (Unterkante des Ufers).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MirrorUniforms {
    pub src_top: f32,
    pub ground: f32,
    pub axis: f32,
    /// Stauchung: ein Abstand `d` unter der Achse zeigt das Bild `d / squash` über der Gehlinie.
    pub squash: f32,
    /// Helligkeit der Spiegelung (0,65).
    pub brightness: f32,
    /// Waagerechte Auslenkung in px (Amplitude) und Phasen des Schwankens und des Glanzes (Bogenmaß).
    pub wobble: f32,
    pub phase_wobble: f32,
    pub phase_glint: f32,
    /// 0 … 1: Lichtquellen halten ihre Helligkeit (Nacht), 0 am Tag.
    pub emissive: f32,
    /// Anteil der Wasserfarbe in der Spiegelung und Verlauf zum Wasser nach unten (Anteil der Höhe von `dst`).
    pub tint: f32,
    pub depth_fade: f32,
    /// Deckkraft der Glanzstriche.
    pub glint_mix: f32,
    pub water: [f32; 3],
    pub glint: [f32; 3],
}

/// Größe der Uniforms eines Spiegel-Aufrufs im GPU-Puffer in Byte (28 × f32, `struct Mirror` in `mirror.wgsl`).
pub(crate) const MIRROR_BYTES: usize = 112;

/// Packt einen Spiegel-Aufruf für `mirror.wgsl`: sieben vec4.
pub(crate) fn pack_mirror(c: &crate::MirrorCall, out: &mut [u8]) {
    let (d, u) = (c.dst, &c.u);
    let floats = [
        d.x,
        d.y,
        d.w,
        d.h,
        d.x,
        u.src_top,
        d.w,
        (u.ground - u.src_top).max(0.0),
        u.axis,
        u.ground,
        u.squash,
        u.brightness,
        u.wobble,
        u.phase_wobble,
        u.phase_glint,
        u.emissive,
        u.tint,
        u.depth_fade,
        u.glint_mix,
        0.0,
        u.water[0],
        u.water[1],
        u.water[2],
        0.0,
        u.glint[0],
        u.glint[1],
        u.glint[2],
        0.0,
    ];
    for (chunk, f) in out[..MIRROR_BYTES].as_chunks_mut::<4>().0.iter_mut().zip(floats) {
        *chunk = f.to_ne_bytes();
    }
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [0, 1, 2].map(|i| a[i] * (1.0 - t) + b[i] * t)
}

/// Glanz liefert 1 auf einem Glanzstrich: waagerechte Striche in Zeilen von 8 px unter der Achse, sie wandern mit und
/// blinken nacheinander.
fn glint(phase: f32, x: f32, dy: f32) -> f32 {
    let row = (dy / 8.0).floor();
    let s = x / 80.0 + row * 0.37;
    let on = 0.5 + 0.5 * (phase + s.floor() * 2.3 + row * 1.7).sin();
    let line = (dy / 8.0 - row - 0.5).abs() < 0.14;
    if line && s - s.floor() < 0.35 && on > 0.6 {
        1.0
    } else {
        0.0
    }
}

/// Fragment des Spiegels am Fenster-px `p` (Pixelmitte): RGBA, Alpha 1. `src` liefert die Farbe (RGB, 0 … 1) des Bildes an
/// einer Stelle in Fenster-px (Streifen `[src_top, ground]`, außerhalb geklemmt); `rect` (x, y, Breite, Höhe) ist das Rechteck des Aufrufs.
/// Die Rechnung steht gleich in `mirror.wgsl`; `f32` wie auf der GPU.
pub fn mirror_fragment(
    u: &MirrorUniforms,
    rect: [f32; 4],
    p: [f32; 2],
    src: &dyn Fn(f32, f32) -> [f32; 3],
) -> [f32; 4] {
    let dy = (p[1] - u.axis).max(0.0);
    let sy = u.ground - dy / u.squash.max(0.01);
    let sway = u.wobble * (dy * 0.12 + u.phase_wobble).sin() * (dy / 40.0).min(1.0);
    let sx = (p[0] + sway).clamp(rect[0], rect[0] + rect[2]);
    let mut c = if sy < u.src_top {
        u.water
    } else {
        let s = src(sx, sy.min(u.ground - 0.5));
        let luma = 0.299 * s[0] + 0.587 * s[1] + 0.114 * s[2];
        // Lichtquellen (nachts trotz Lichtkarte hell) bleiben hell, alles andere wird dunkler
        let keep = u.emissive * smoothstep(0.55, 0.9, luma);
        let b = u.brightness + (1.0 - u.brightness) * keep;
        mix3([s[0] * b, s[1] * b, s[2] * b], u.water, u.tint * (1.0 - keep))
    };
    let fade = (u.depth_fade * dy / rect[3].max(1.0)).clamp(0.0, 1.0);
    c = mix3(c, u.water, fade * 0.5);
    c = mix3(c, u.glint, u.glint_mix * glint(u.phase_glint, p[0], dy));
    [c[0], c[1], c[2], 1.0]
}

#[cfg(test)]
#[path = "mirror_tests.rs"]
mod mirror_tests;
