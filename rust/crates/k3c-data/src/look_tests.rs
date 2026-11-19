//! RU55.1 (B-1043): data/look.json mit den Profilen k2c und go.

use super::*;
use crate::testutil::raw;

#[test]
fn datei_hat_beide_profile_mit_den_werten_des_standards() {
    let l = load(&raw(FILE)).expect("look.json");
    let (k2c, go) = (&l.profiles[K2C], &l.profiles[GO]);
    assert_eq!((k2c.world_share, k2c.bank_share), (0.58, 0.07));
    assert_eq!(*go, Look::default(), "Profil go ist der Stand des Go-Spiels");
}

#[test]
fn fehlende_felder_gelten_als_go() {
    let l = load(r#"{"profiles": {"x": {"worldShare": 0.5}}}"#).expect("laden");
    let x = &l.profiles["x"];
    assert_eq!(x.world_share, 0.5);
    assert_eq!((x.bank_share, x.bank_px), (0.0, 20.0));
}

#[test]
fn kaputtes_json_ist_ein_fehler() {
    assert!(load(r#"{"profiles": 3}"#).is_err());
}

#[test]
fn profil_nach_name_und_unbekannt_bleibt_standard() {
    assert_eq!(profile(K2C).map(|l| l.world_share), Some(0.58));
    assert!(profile("gibt-es-nicht").is_none());
    assert_eq!(pick("unbekannt"), profile(default_name()).expect("Standard"));
    assert_eq!(pick(" GO "), profile(GO).expect("go"));
    assert_eq!(pick(""), profile(default_name()).expect("Standard"));
}

#[test]
fn with_profile_gilt_nur_im_aufruf() {
    let before = current();
    let inside = with_profile(K2C, || current().world_share);
    assert_eq!(inside, 0.58);
    assert_eq!(current(), before);
    // unbekannter Name: Profil unverändert
    assert_eq!(with_profile("x", current), before);
}

#[test]
fn eingebaute_werte_sind_die_der_datei() {
    let l = load(&raw(FILE)).expect("look.json");
    let b = builtin();
    assert_eq!(*b[K2C], l.profiles[K2C]);
    assert_eq!(*b[GO], l.profiles[GO]);
}
