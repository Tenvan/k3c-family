//! Tests zu `assets.go`. Portiert ist `TestAssetsJeLevelOhneDoppelte` (ohne den Drawer-Teil `d.Assets`, der kommt mit
//! RU16.5). `TestJederTypAusDenDatenHatEineGrafik` und `TestJedeGewaehlteGrafikLiegtVorUndIstGemeldet` brauchen
//! `node_look`, `entity_look`, `pickup_look`, `variant_look` (RU16.2) und `site_size` (RU16.3) und folgen dort.

use super::assets::level_assets;

#[test]
fn assets_je_level_ohne_doppelte() {
    let wald = level_assets("forest");
    let hoehle = level_assets("cave");
    let mut sortiert = wald.clone();
    sortiert.sort();
    sortiert.dedup();
    assert_eq!(sortiert, wald, "sortiert, jedes einmal");
    let has = |v: &[String], a: &str| v.iter().any(|x| x == a);
    assert!(
        has(&wald, "objekte/tanne") && !has(&hoehle, "objekte/tanne") && has(&hoehle, "objekte/wurzel"),
        "Natur nach dem Look des Bioms"
    );
}
