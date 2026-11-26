//! Port von `katalog_test.go`.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::assetsui::test_helpers::{Probe, mit, nach_id, roundtrip, test_repo};

/// Der Index ruft keinen Validate; `AssetsCheck` aller Pfade ergibt `AssetsList` (B-732/AC-02).
#[test]
fn assets_index_ohne_pruefung() {
    let Some(repo) = test_repo() else { return };
    let probe = Arc::new(Probe::zaehlend());
    let d = mit(&repo.pfad, probe.clone());
    let index = d.assets_index().unwrap();
    assert_eq!(probe.validiert.load(Ordering::SeqCst), 0, "AssetsIndex rief Validate");
    for e in &index {
        assert!(!e.id.is_empty() && !e.gruppe.is_empty(), "{e:?}");
    }
    let pfade: Vec<String> = index.iter().map(|e| e.pfad.clone()).collect();
    let mut got = d.assets_check(&pfade).unwrap();
    let want = nach_id(&roundtrip(d.assets_list().unwrap()));
    // AssetsCheck ermittelt `geaendert` nicht, das liefert der Index
    for g in &mut got {
        assert!(!g.geaendert, "{g:?}");
        g.geaendert = index.iter().find(|e| e.pfad == g.pfad).expect("im Index").geaendert;
    }
    assert_eq!(nach_id(&roundtrip(got)), want);
    // nach der Prüfung liefert der Index das Ergebnis aus dem Memo, nichts bleibt offen
    assert_eq!(nach_id(&roundtrip(d.assets_index().unwrap())), want);
}

/// Ein Pfad außerhalb des Arbeitsstands wird nicht geprüft, sein Eintrag meldet `fehlt`.
#[test]
fn assets_check_fremder_pfad() {
    let Some(repo) = test_repo() else { return };
    let probe = Arc::new(Probe::zaehlend());
    let d = mit(&repo.pfad, probe.clone());
    let got = d.assets_check(&["../../etc/passwd.paper.json".to_owned()]).unwrap();
    assert_eq!(probe.validiert.load(Ordering::SeqCst), 0);
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(!got[0].gueltig, "{got:?}");
    assert_eq!(got[0].fehler[0].regel, "fehlt");
}

/// Mehr als 64 Pfade lehnt der Aufruf ab, bevor er liest.
#[test]
fn assets_check_zu_viele() {
    let d = mit(std::path::Path::new("unbenutzt"), Arc::new(Probe::zaehlend()));
    let pfade = vec![String::new(); 65];
    assert_eq!(
        d.assets_check(&pfade).unwrap_err(),
        "höchstens 64 Pfade je Aufruf, nicht 65"
    );
}
