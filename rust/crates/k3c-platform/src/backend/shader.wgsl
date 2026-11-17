// Logische Pixel (1920x1080) → NDC.
fn ndc(px: vec2<f32>) -> vec4<f32> {
  return vec4(px.x / 960.0 - 1.0, 1.0 - px.y / 540.0, 0.0, 1.0);
}

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;

// Rechteck-Instanz: Einheitsquadrat q → (m.x·q.x + m.y·q.y, m.z·q.x + m.w·q.y) + t.xy (wie GeoM),
// c: Farbe/Tönung, uv: Quelle als Anteil der Textur; uv.zw == 0 heißt Vollfarbe.
struct Inst {
  @location(0) m: vec4<f32>,
  @location(1) t: vec4<f32>,
  @location(2) c: vec4<f32>,
  @location(3) uv: vec4<f32>,
};
struct Out {
  @builtin(position) p: vec4<f32>,
  @location(0) c: vec4<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) solid: f32,
};

@vertex
fn vs(@builtin(vertex_index) v: u32, i: Inst) -> Out {
  var q = array<vec2<f32>, 6>(
    vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
    vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0));
  let k = q[v];
  let px = vec2(i.m.x * k.x + i.m.y * k.y, i.m.z * k.x + i.m.w * k.y) + i.t.xy;
  var o: Out;
  o.p = ndc(px);
  o.c = i.c;
  o.uv = i.uv.xy + k * i.uv.zw;
  o.solid = select(0.0, 1.0, i.uv.z == 0.0 && i.uv.w == 0.0);
  return o;
}

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
  let s = textureSample(tex, smp, o.uv);
  if (o.solid > 0.5) {
    return vec4(o.c.rgb * o.c.a, o.c.a);
  }
  // Tönung ist gerade (wie ColorScale bei Ebitengine): vor dem Multiplizieren vormultiplizieren.
  return s * vec4(o.c.rgb * o.c.a, o.c.a);
}

// Dreiecke (Netze und Formen): Ecke mit Ziel in logischen Pixeln, Quelle als Anteil, Farbe als Faktor.
// Formen binden eine weiße Textur und tragen vormultiplizierte Farbe.
struct TriIn {
  @location(0) pos: vec2<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) c: vec4<f32>,
};
struct TriOut {
  @builtin(position) p: vec4<f32>,
  @location(0) c: vec4<f32>,
  @location(1) uv: vec2<f32>,
};

@vertex
fn vs_tri(i: TriIn) -> TriOut {
  var o: TriOut;
  o.p = ndc(i.pos);
  o.c = i.c;
  o.uv = i.uv;
  return o;
}

@fragment
fn fs_tri(o: TriOut) -> @location(0) vec4<f32> {
  return textureSample(tex, smp, o.uv) * o.c;
}
