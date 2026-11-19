// Spiegel-Fluss (RU55.3): Fragment 1:1 nach `mirror_fragment` in mirror.rs. Das Backend kopiert den Streifen über der
// Gehlinie in eine Zwischentextur; dieser Shader tastet sie gespiegelt ab (dunkler, getönt, gestaucht, schwankend)
// und legt waagerechte Glanzstriche darüber. Alle Werte kommen als Uniforms (Packung: mirror.rs, pack_mirror).

struct Mirror {
  // Rechteck des Aufrufs (x, y, breite, höhe) in logischen Pixeln (1920x1080).
  rect: vec4<f32>,
  // Streifen der Zwischentextur (x, oben, breite, höhe) in logischen Pixeln.
  src: vec4<f32>,
  // axis, ground, squash, brightness
  a: vec4<f32>,
  // wobble, phase_wobble, phase_glint, emissive
  b: vec4<f32>,
  // tint, depth_fade, glint_mix, -
  c: vec4<f32>,
  water: vec4<f32>,
  glint: vec4<f32>,
};

@group(0) @binding(0) var<uniform> m: Mirror;
@group(0) @binding(1) var strip: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct Out {
  @builtin(position) pos: vec4<f32>,
  @location(0) win: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) v: u32) -> Out {
  var q = array<vec2<f32>, 6>(
    vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
    vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0));
  let px = m.rect.xy + q[v] * m.rect.zw;
  var o: Out;
  o.pos = vec4(px.x / 960.0 - 1.0, 1.0 - px.y / 540.0, 0.0, 1.0);
  o.win = px;
  return o;
}

fn glint(phase: f32, x: f32, dy: f32) -> f32 {
  let row = floor(dy / 8.0);
  let s = x / 80.0 + row * 0.37;
  let on = 0.5 + 0.5 * sin(phase + floor(s) * 2.3 + row * 1.7);
  let line = abs(dy / 8.0 - row - 0.5) < 0.14;
  if (line && fract(s) < 0.35 && on > 0.6) {
    return 1.0;
  }
  return 0.0;
}

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
  let p = o.win;
  let axis = m.a.x;
  let ground = m.a.y;
  let squash = max(m.a.z, 0.01);
  let brightness = m.a.w;
  let dy = max(p.y - axis, 0.0);
  let sy = ground - dy / squash;
  let sway = m.b.x * sin(dy * 0.12 + m.b.y) * min(dy / 40.0, 1.0);
  let sx = clamp(p.x + sway, m.rect.x, m.rect.x + m.rect.z);
  var c = m.water.rgb;
  if (sy >= m.src.y) {
    // Die Zwischentextur hat die Größe der Fläche; der Streifen liegt an seinem Platz (Anteil des Fensters).
    let uv = vec2(sx / 1920.0, min(sy, ground - 0.5) / 1080.0);
    let s = textureSampleLevel(strip, samp, uv, 0.0).rgb;
    let luma = 0.299 * s.r + 0.587 * s.g + 0.114 * s.b;
    // Lichtquellen (nachts trotz Lichtkarte hell) bleiben hell, alles andere wird dunkler
    let keep = m.b.w * smoothstep(0.55, 0.9, luma);
    let b = brightness + (1.0 - brightness) * keep;
    c = mix(s * b, m.water.rgb, m.c.x * (1.0 - keep));
  }
  let fade = clamp(m.c.y * dy / max(m.rect.w, 1.0), 0.0, 1.0);
  c = mix(c, m.water.rgb, fade * 0.5);
  c = mix(c, m.glint.rgb, m.c.z * glint(m.b.z, p.x, dy));
  return vec4(c, 1.0);
}
