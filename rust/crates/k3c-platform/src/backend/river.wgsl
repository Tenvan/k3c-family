// Fluss als WGSL-Shader (B-1072): Fragment 1:1 nach native/k3c-game/world/water/river.kage. Sechs Wellen-Reihen,
// Verzerrung, Papier-Schatten unter dem Band, Glanzstriche. Alle Werte kommen als Uniforms (Packung: river.rs,
// pack_river); der Shader kennt keine Uhr. Jedes Feld ist ein vec4, damit das Layout keine Füllbytes braucht.

// Rechteck (x, y, breite, höhe) in logischen Pixeln (1920x1080).
struct River {
  rect: vec4<f32>,
  // origin.xy: Fenster-px der linken oberen Ecke des Rechtecks (wie Origin in Kage).
  origin: vec4<f32>,
  // Reihen in Fenster-px: x = oben, y = Höhe, z = Kachelbreite, w = erste Kachelkante.
  rows: array<vec4<f32>, 6>,
  // phase.xyz: Verzerrung, Heben des Bands, Glanz (Bogenmaß).
  phase: vec4<f32>,
  base: vec4<f32>,
  wave: vec4<f32>,
  shade: vec4<f32>,
  glint: vec4<f32>,
};

@group(0) @binding(0) var<uniform> r: River;

// tau ist 2π; srcH ist die Höhe der Wellen-Kachel in Quell-px (riverSrcH), Band und Schatten rechnen in dieser Einheit.
const tau: f32 = 6.283185307179586;
const srcH: f32 = 64.0;

struct Out {
  @builtin(position) pos: vec4<f32>,
  // Position in logischen Pixeln (Fenster-px), in der Pixelmitte interpoliert.
  @location(0) win: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) v: u32) -> Out {
  var q = array<vec2<f32>, 6>(
    vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
    vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0));
  let px = r.rect.xy + q[v] * r.rect.zw;
  var o: Out;
  o.pos = vec4(px.x / 960.0 - 1.0, 1.0 - px.y / 540.0, 0.0, 1.0);
  o.win = px;
  return o;
}

// glint liefert 1 auf einem Glanzstrich: drei je Kachel unter dem Band, sie fließen mit und blinken nacheinander.
fn glint(u: f32, v: f32, n: f32) -> f32 {
  let s = u * 3.0 + n * 0.37;
  let on = 0.5 + 0.5 * sin(r.phase.z + floor(s) * 2.3 + n * 1.7);
  if (fract(s) < 0.2 && abs(v - srcH * 0.8) < 1.5 && on > 0.6) {
    return 1.0;
  }
  return 0.0;
}

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
  let p = o.win;
  if (p.y < r.rows[0].x) {
    return vec4(0.0);
  }
  var row = r.rows[0];
  var n = 0.0;
  for (var i = 1u; i < 6u; i++) {
    if (p.y >= r.rows[i].x) {
      row = r.rows[i];
      n = f32(i);
    }
  }
  // Reihenhöhe 0: v bleibt 0 (keine Division durch 0 im Ergebnis).
  let t = select(0.0, clamp((p.y - row.x) / row.y, 0.0, 1.0), row.y > 0.0);
  let v = t * srcH;
  var u = (p.x - row.w) / row.z;
  // Verzerrung: die Reihe schwankt waagrecht, je Tiefe der Reihe versetzt.
  u += 0.025 * sin(tau * v / srcH + r.phase.x + n * 1.3);
  // Wellen-Band wie paintWaves: zwei Wellen je Kachel um die Mitte, leicht gehoben.
  let line = srcH * 0.5 + 7.0 * sin(tau * 2.0 * u) + 1.5 * sin(r.phase.y + n * 0.9);
  var c = r.base.rgb;
  if (v > line + 3.0 && v < line + 6.0) {
    c = r.shade.rgb;
  }
  c = mix(c, r.wave.rgb, clamp(1.0 - abs(v - line) / 4.0, 0.0, 1.0));
  c = mix(c, r.glint.rgb, 0.6 * glint(u, v, n));
  // Deckend, Farbe also zugleich gerade und vormultipliziert (wie shader.wgsl: Blend PREMULTIPLIED_ALPHA).
  return vec4(c, 1.0);
}
