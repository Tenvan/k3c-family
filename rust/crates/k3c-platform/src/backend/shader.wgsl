// Eine Instanz = ein Rechteck. r: Ziel in logischen Pixeln (1920x1080), c: Farbe/Tönung,
// uv: Quelle als Anteil der Textur; uv.zw == 0 heißt Vollfarbe.
struct Inst {
  @location(0) r: vec4<f32>,
  @location(1) c: vec4<f32>,
  @location(2) uv: vec4<f32>,
};
struct Out {
  @builtin(position) p: vec4<f32>,
  @location(0) c: vec4<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) solid: f32,
};
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;

@vertex
fn vs(@builtin(vertex_index) v: u32, i: Inst) -> Out {
  var q = array<vec2<f32>, 6>(
    vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
    vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0));
  let px = i.r.xy + q[v] * i.r.zw;
  var o: Out;
  o.p = vec4(px.x / 960.0 - 1.0, 1.0 - px.y / 540.0, 0.0, 1.0);
  o.c = i.c;
  o.uv = i.uv.xy + q[v] * i.uv.zw;
  o.solid = select(0.0, 1.0, i.uv.z == 0.0 && i.uv.w == 0.0);
  return o;
}

@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
  if (o.solid > 0.5) {
    return vec4(o.c.rgb * o.c.a, o.c.a);
  }
  return textureSample(tex, smp, o.uv) * o.c;
}
