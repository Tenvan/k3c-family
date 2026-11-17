use std::f32::consts::FRAC_PI_2;

use super::*;

fn close(a: (f32, f32), b: (f32, f32)) -> bool {
    (a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4
}

fn same(m: &Affine, want: [f32; 6]) {
    let got = [m.a, m.b, m.c, m.d, m.tx, m.ty];
    for (g, w) in got.iter().zip(want) {
        assert!((g - w).abs() < 1e-4, "{got:?} statt {want:?}");
    }
}

/// Werte von Hand nach `ebiten.GeoM` (Rotate: a' = cos·a − sin·c …, tx' = cos·tx − sin·ty).
#[test]
fn verschieben_drehen_skalieren_wie_geom() {
    let mut m = Affine::IDENTITY;
    m.translate(-16.0, -8.0);
    same(&m, [1.0, 0.0, 0.0, 1.0, -16.0, -8.0]);
    m.rotate(FRAC_PI_2);
    same(&m, [0.0, -1.0, 1.0, 0.0, 8.0, -16.0]);
    m.scale(2.0, 3.0);
    same(&m, [0.0, -2.0, 3.0, 0.0, 16.0, -48.0]);
    m.translate(100.0, 50.0);
    same(&m, [0.0, -2.0, 3.0, 0.0, 116.0, 2.0]);
    assert!(
        close(m.apply(16.0, 8.0), (100.0, 50.0)),
        "Mittelpunkt landet auf dem Ziel"
    );
    assert!(
        close(m.apply(32.0, 8.0), (100.0, 98.0)),
        "rechts wird unten (Drehung im Uhrzeigersinn)"
    );
}

#[test]
fn concat_haengt_hinten_an() {
    let mut a = Affine::IDENTITY;
    a.scale(2.0, 2.0);
    let mut t = Affine::IDENTITY;
    t.translate(10.0, 0.0);
    let mut ab = a;
    ab.concat(&t);
    assert!(close(ab.apply(1.0, 1.0), (12.0, 2.0)), "erst skalieren, dann schieben");
    let mut ba = t;
    ba.concat(&a);
    assert!(close(ba.apply(1.0, 1.0), (22.0, 2.0)), "erst schieben, dann skalieren");
}

#[test]
fn identitaet_ist_standard() {
    assert_eq!(Affine::default(), Affine::IDENTITY);
    assert_eq!(Affine::IDENTITY.apply(3.0, 4.0), (3.0, 4.0));
}
