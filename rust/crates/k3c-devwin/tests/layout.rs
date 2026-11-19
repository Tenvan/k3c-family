//! Layout: 1, 2 und 4 Spalten ohne Überlappung, Scrollbereich, Mindestbreite (RU56.3, AC-03).
#![allow(clippy::unwrap_used, clippy::indexing_slicing)] // Testcode

use k3c_devwin::layout::{Control, Group, Kind, Layout, MIN_BUTTON_W, layout};

fn groups(n: usize) -> Vec<Group> {
    (0..n)
        .map(|i| {
            let mut controls = vec![Control::new("l", Kind::Label, "Titel")];
            for b in 0..5 {
                controls.push(Control::new(&format!("b{b}"), Kind::Button, "Knopf"));
            }
            controls.push(Control::new("t", Kind::Toggle, "Schalter"));
            controls.push(Control::new("s", Kind::Select, "Auswahl").with_options(vec!["a".into()]));
            controls.push(Control::new("e", Kind::Input, "Eingabe"));
            Group {
                title: format!("Gruppe {i}"),
                controls,
            }
        })
        .collect()
}

fn check(l: &Layout, gs: &[Group], width: i32) {
    assert_eq!(l.groups.len(), gs.len());
    for (i, a) in l.groups.iter().enumerate() {
        assert!(a.x >= 0 && a.x + a.w <= width, "Gruppe {i} ragt aus dem Fenster");
        for b in &l.groups[i + 1..] {
            assert!(!a.overlaps(b), "Gruppen überlappen");
        }
    }
    let total: usize = gs.iter().map(|g| g.controls.len()).sum();
    assert_eq!(l.controls.len(), total);
    for (i, p) in l.controls.iter().enumerate() {
        assert!(l.groups[p.group].contains(&p.rect), "Control außerhalb seiner Gruppe");
        for q in &l.controls[i + 1..] {
            assert!(!p.rect.overlaps(&q.rect), "Controls überlappen");
        }
        let c = &gs[p.group].controls[p.control];
        assert!(p.rect.w >= c.min_w, "Mindestbreite verletzt");
    }
}

#[test]
fn spalten_nach_breite() {
    let gs = groups(6);
    for (w, cols) in [(420, 1), (720, 2), (1600, 4)] {
        let l = layout(w, 10_000, &gs);
        assert_eq!(l.columns, cols, "Breite {w}");
        assert!(!l.scroll);
        check(&l, &gs, w);
    }
}

#[test]
fn hoher_inhalt_scrollt_und_haelt_die_breite() {
    let gs = groups(6);
    let l = layout(420, 480, &gs);
    assert!(l.scroll);
    assert!(l.content_height > 480);
    check(&l, &gs, 420 - 16);
    let tall = layout(420, l.content_height, &gs);
    assert!(!tall.scroll, "passt der Inhalt genau, gibt es keine Scrollleiste");
}

#[test]
fn schaltflaechen_stehen_in_reihen_ab_mindestbreite() {
    let gs = groups(1);
    let l = layout(420, 10_000, &gs);
    let first_row_y = l.controls.iter().find(|p| p.control == 1).unwrap().rect.y;
    let in_row = l
        .controls
        .iter()
        .filter(|p| p.group == 0 && p.rect.y == first_row_y)
        .count();
    assert_eq!(in_row, 3, "420 dip fassen drei Schaltflächen je Reihe");
    assert!(
        l.controls
            .iter()
            .all(|p| p.rect.w >= MIN_BUTTON_W || gs[0].controls[p.control].min_w == 0)
    );
}

#[test]
fn leer_und_winzig_ohne_panic() {
    assert_eq!(layout(420, 480, &[]).content_height, 0);
    let l = layout(10, 10, &groups(2));
    assert_eq!(l.columns, 1);
}
