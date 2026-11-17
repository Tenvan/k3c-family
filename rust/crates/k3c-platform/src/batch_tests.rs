use super::*;
use crate::{Affine, Canvas};

const WHITE: [f32; 4] = [1.0; 4];

fn tex(f: &mut Frame) -> TexId {
    f.texture(&[255; 16], 2, 2)
}

fn sq() -> (Rect, Rect) {
    (Rect::new(0.0, 0.0, 2.0, 2.0), Rect::new(0.0, 0.0, 1.0, 1.0))
}

#[test]
fn sortiert_nach_textur_und_behaelt_reihenfolge() {
    let mut f = Frame::new();
    let (a, b) = (tex(&mut f), tex(&mut f));
    f.begin();
    let src = sq().0;
    for (i, t) in [b, a, b, a, b].into_iter().enumerate() {
        f.sprite(t, src, Rect::new(i as f32, 0.0, 1.0, 1.0), WHITE);
    }
    f.rect(Rect::new(9.0, 0.0, 1.0, 1.0), WHITE);
    f.finish();
    let xs: Vec<f32> = f.instances().map(|i| i.t[0]).collect();
    assert_eq!(xs, [1.0, 3.0, 0.0, 2.0, 4.0, 9.0]);
    let runs: Vec<_> = f.runs().iter().map(|r| (r.tex, r.first, r.count)).collect();
    assert_eq!(runs, [(Some(a), 0, 2), (Some(b), 2, 3), (None, 5, 1)]);
}

#[test]
fn ebene_geht_vor_textur() {
    let mut f = Frame::new();
    let (a, b) = (tex(&mut f), tex(&mut f));
    f.begin();
    let (src, r) = sq();
    f.layer(2);
    f.sprite(a, src, r, WHITE);
    f.layer(1);
    f.sprite(b, src, r, WHITE);
    f.rect(r, WHITE);
    f.finish();
    let runs: Vec<_> = f.runs().iter().map(|r| (r.layer, r.tex)).collect();
    assert_eq!(runs, [(1, Some(b)), (1, None), (2, Some(a))]);
    f.begin();
    f.finish();
    assert!(f.runs().is_empty());
}

#[test]
fn ausschnitt_trennt_buendel_und_kommt_mit() {
    let mut f = Frame::new();
    let t = tex(&mut f);
    let (src, r) = sq();
    let (left, right) = (Rect::new(0.0, 0.0, 960.0, 1080.0), Rect::new(960.0, 0.0, 960.0, 1080.0));
    f.begin();
    f.clip(Some(right));
    f.sprite(t, src, r, WHITE);
    f.clip(Some(left));
    f.sprite(t, src, r, WHITE);
    f.clip(Some(right));
    f.sprite(t, src, r, WHITE);
    f.clip(None);
    f.rect(r, WHITE);
    f.finish();
    let runs: Vec<_> = f.runs().iter().map(|r| (r.clip, r.count)).collect();
    assert_eq!(runs, [(None, 1), (Some(right), 2), (Some(left), 1)]);
    f.begin();
    f.sprite(t, src, r, WHITE);
    f.finish();
    assert_eq!(f.runs()[0].clip, None, "Ausschnitt gilt nur ein Bild");
}

#[test]
fn quelle_wird_auf_textur_anteile_umgerechnet() {
    let mut f = Frame::new();
    let t = f.texture(&[0; 4 * 8 * 4], 4, 8);
    f.begin();
    f.sprite(t, Rect::new(2.0, 4.0, 2.0, 4.0), Rect::new(0.0, 0.0, 1.0, 1.0), WHITE);
    f.finish();
    assert_eq!(f.instances().next().map(|i| i.uv), Some([0.5, 0.5, 0.5, 0.5]));
}

#[test]
fn text_fasst_pixelstreifen_zu_rechtecken() {
    let mut f = Frame::new();
    f.begin();
    f.text("I", 0.0, 0.0, 7.0, WHITE);
    f.finish();
    // I: oben und unten je ein Streifen (5 breit), dazwischen fünf einzelne Pixel
    assert_eq!(f.instance_count(), 7);
    assert!(f.instances().all(|i| i.uv == [0.0; 4]));
}

#[test]
fn instanzen_und_ecken_als_bytes() {
    let mut f = Frame::new();
    f.begin();
    f.rect(Rect::new(1.0, 2.0, 3.0, 4.0), WHITE);
    f.fill_circle(0.0, 0.0, 1.0, [1.0, 0.0, 0.0, 0.5]);
    f.finish();
    let mut out = [0u8; INST_BYTES];
    f.write_instances(&mut out);
    let at = |b: &[u8], i: usize| f32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap());
    assert_eq!(
        [at(&out, 0), at(&out, 3), at(&out, 4), at(&out, 5)],
        [3.0, 4.0, 1.0, 2.0]
    );
    let mut v = vec![0u8; f.vertices().len() * VERT_BYTES];
    f.write_vertices(&mut v);
    assert_eq!([at(&v, 4), at(&v, 7)], [0.5, 0.5], "Farbe vormultipliziert");
}

#[test]
fn sprite_tf_bildet_die_quelle_mit_geom_ab() {
    let mut f = Frame::new();
    let t = f.texture(&[0; 4 * 32 * 16], 32, 16);
    let mut m = Affine::IDENTITY;
    m.scale(2.0, 2.0);
    m.translate(100.0, 50.0);
    f.begin();
    f.sprite_tf(t, Rect::new(0.0, 0.0, 32.0, 16.0), m, WHITE);
    f.finish();
    let i = f.instances().next().unwrap();
    assert_eq!(i.m, [64.0, 0.0, 0.0, 32.0], "Quelle 32×16 doppelt so groß");
    assert_eq!(&i.t[..2], &[100.0, 50.0]);
}

#[test]
fn linie_ist_ein_gedrehtes_rechteck() {
    let mut f = Frame::new();
    f.begin();
    f.stroke_line(10.0, 10.0, 10.0, 30.0, 4.0, WHITE);
    f.finish();
    let i = f.instances().next().unwrap();
    // Einheitsquadrat (q.x längs, q.y quer): Ecke (0, 0) → (12, 10), (1, 1) → (8, 30).
    let p = |qx: f32, qy: f32| (i.m[0] * qx + i.m[1] * qy + i.t[0], i.m[2] * qx + i.m[3] * qy + i.t[1]);
    assert_eq!(p(0.0, 0.0), (12.0, 10.0));
    assert_eq!(p(1.0, 1.0), (8.0, 30.0));
}

#[test]
fn netze_und_formen_mischen_sich_nach_aufruf() {
    let mut f = Frame::new();
    let t = tex(&mut f);
    let v = |x: f32, y: f32| Vertex {
        pos: [x, y],
        uv: [x, y],
        color: WHITE,
    };
    f.begin();
    f.rect(Rect::new(0.0, 0.0, 1.0, 1.0), WHITE);
    f.fill_path(&[[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0]], WHITE);
    f.rect(Rect::new(0.0, 0.0, 1.0, 1.0), WHITE);
    f.mesh(t, &[v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0)], &[0, 1, 2]);
    f.mesh(t, &[v(0.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)], &[0, 1, 2]);
    f.finish();
    let runs: Vec<_> = f.runs().iter().map(|r| (r.tex, r.kind, r.first, r.count)).collect();
    assert_eq!(
        runs,
        [
            (Some(t), Kind::Tris, 0, 6),
            (None, Kind::Quads, 0, 1),
            (None, Kind::Tris, 6, 6),
            (None, Kind::Quads, 1, 1),
        ]
    );
    assert_eq!(
        &f.indices()[..6],
        &[4, 5, 6, 7, 8, 9],
        "zwei Netze, ein Bündel, nach den Polygon-Ecken"
    );
    assert_eq!(f.vertices()[5].uv, [1.0, 0.0], "Quelle als Anteil der Textur");
}

#[test]
fn freigegebene_textur_zeichnet_nicht_mehr() {
    let mut f = Frame::new();
    let (a, b) = (tex(&mut f), tex(&mut f));
    f.free_texture(b);
    assert_eq!(
        f.take_pending().map(|p| p.id).collect::<Vec<_>>(),
        [a],
        "nie hochgeladen"
    );
    f.free_texture(a);
    f.free_texture(a);
    assert_eq!(f.take_freed().collect::<Vec<_>>(), [b, a]);
    f.begin();
    let (src, r) = sq();
    f.sprite(a, src, r, WHITE);
    f.finish();
    assert!(f.runs().is_empty());
}

#[test]
fn filter_kommt_mit_der_textur() {
    let mut f = Frame::new();
    f.texture_with(&[0; 4], 1, 1, Filter::Nearest);
    f.texture(&[0; 4], 1, 1);
    let filters: Vec<_> = f.take_pending().map(|p| p.filter).collect();
    assert_eq!(filters, [Filter::Nearest, Filter::Linear]);
}
