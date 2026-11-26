use std::sync::atomic::Ordering;

use super::super::test_helpers::*;
use super::{bild, quelle_basis};

/// Der zweite Aufruf für dieselbe Basis rendert nicht neu; die Datei liegt unter `.cache/paper/<renderKey>/`.
#[test]
fn vorher_cache() {
    let Some(repo) = test_repo() else { return };
    let r = fake_frames();
    let commit = git_in(&repo.pfad, &["rev-parse", "--short=7", "HEAD~1"]);
    let q = quelle_basis(&repo.pfad, &commit, MONARCH)
        .expect("Quelle")
        .expect("monarch in der Basis");
    let erst = bild(&r, &repo.pfad, &q, "reiten", 0, "").expect("erster Aufruf");
    let zweit = bild(&r, &repo.pfad, &q, "reiten", 0, "").expect("zweiter Aufruf");
    assert_eq!(r.aufrufe.load(Ordering::SeqCst), 1, "Frame neu gerendert");
    let ordner = q.ordner(&repo.pfad, &r);
    assert!(erst.datei.starts_with(&ordner), "{:?}", erst.datei);
    assert_eq!(erst.datei, zweit.datei);
    assert!(erst.datei.is_file());
    assert!(zweit.teile.contains_key("kopf"), "{:?}", zweit.teile);
}
