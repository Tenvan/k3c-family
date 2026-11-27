//! Port von `tools_hub_test.go` (B-923): Roundtrip der Tools `hub_get`, `hub_check` und `hub_save` mit der Spiel-EXE auf
//! einer Kopie von `data/` und der Regeltabelle unter `.work/`; die Dateien des Checkouts werden nur gelesen.

use std::path::Path;

use serde_json::json;

use super::super::super::tools_world::tests::World;
use super::{editor, go_g, row_line};
use crate::hubplan::editor_tests::read_repo;
use crate::hubplan::{HubCell, HubRow};

const RULES: &str = "docs/rules/materialien-gebaeude.md";

/// `None` ohne frische Spiel-EXE.
async fn world() -> Option<World> {
    let world = World::start().await?;
    std::fs::create_dir_all(world.root.join("docs/rules")).unwrap();
    std::fs::write(world.root.join(RULES), read_repo(RULES)).unwrap();
    Some(world)
}

fn hub(root: &Path) -> String {
    std::fs::read_to_string(root.join("data/hub.json")).unwrap()
}

fn stand(root: &Path) -> String {
    editor(root).plan().stand.hub
}

#[tokio::test(flavor = "multi_thread")]
async fn hub_get_nennt_matrix_pruefung_und_stand() {
    let Some(mut w) = world().await else { return };
    let (text, is_error) = w.call("hub_get", json!({})).await;
    assert!(!is_error, "hub_get: {text}");
    for want in [
        "data/hub.json · ",
        "Hub-Stufen",
        "Prüfung: ok",
        "\nStand: hub=",
        "armory@40 · sites#",
        "zone1 · sitePlacement#0 · ab 1 · ±44",
        "Regeltabelle: stimmt",
    ] {
        assert!(text.contains(want), "{want:?} fehlt:\n{text}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn hub_check_schreibt_nichts() {
    let Some(mut w) = world().await else { return };
    let before = hub(&w.root);
    let patch = json!({"patch": {"merchant": {"buyOffsetUnits": 18}}});
    let (text, is_error) = w.call("hub_check", patch).await;
    assert!(!is_error, "hub_check:\n{text}");
    for want in ["(Kandidat, nichts geschrieben)", "merchantBuy@18 ·", "Regeltabelle: "] {
        assert!(text.contains(want), "{want:?} fehlt:\n{text}");
    }
    assert_eq!(hub(&w.root), before, "hub_check hat data/hub.json geändert");
    let (text, is_error) = w.call("hub_check", json!({"patch": {"a": 1}, "candidate": "{}"})).await;
    assert!(is_error && text.contains("nicht beides"), "patch und candidate: {text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn hub_save_schreibt_eine_zeile_und_lehnt_alten_stand_ab() {
    let Some(mut w) = world().await else { return };
    let before = hub(&w.root);
    let patch = json!({"homeRadiusUnits": 13});
    let (text, is_error) = w
        .call("hub_save", json!({"patch": patch, "stand": "000000000000"}))
        .await;
    assert!(
        is_error && text.contains("seit hub_get geändert"),
        "alter Stand: {text}"
    );
    let (text, is_error) = w.call("hub_save", json!({"patch": patch})).await;
    assert!(is_error && text.contains("stand fehlt"), "ohne stand: {text}");
    let args = json!({"patch": patch, "stand": stand(&w.root)});
    let (text, is_error) = w.call("hub_save", args).await;
    assert!(
        !is_error && text.starts_with("Gespeichert: data/hub.json und Regeltabelle. REG-Änderung"),
        "{text}"
    );
    assert!(text.contains("\nCheckout: "), "{text}");
    let after = hub(&w.root);
    assert_eq!(after.matches('\n').count(), before.matches('\n').count());
    assert!(
        after.contains(r#""homeRadiusUnits": 13"#),
        "nicht format-stabil geschrieben:\n{after}"
    );
    let args = json!({"patch": patch, "stand": stand(&w.root)});
    let (text, _) = w.call("hub_save", args).await;
    assert!(text.starts_with("Unverändert"), "zweites Speichern: {text}");
    let (text, is_error) = w.call("hub_save", json!({})).await;
    assert!(
        is_error && text.contains("patch oder candidate fehlt"),
        "ohne Angabe: {text}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn hub_save_mit_befund_schreibt_nichts() {
    let Some(mut w) = world().await else { return };
    let before = hub(&w.root);
    let patch = json!({"sites": [{"kind": "mill", "offsetUnits": 1, "hubLevel": 1}]});
    let args = json!({"patch": patch, "stand": stand(&w.root)});
    let (text, is_error) = w.call("hub_save", args).await;
    assert!(
        is_error && text.contains("Befunde der Prüfung") && text.contains("\nBefund: "),
        "{text}"
    );
    assert_eq!(hub(&w.root), before, "hub.json trotz Befund geändert");
}

/// Neu: Zahlen wie Gos `%g` und die Zeile der Matrix (erste verfügbare Stufe).
#[test]
fn zahlen_und_zeile_wie_go() {
    for (value, want) in [
        (44.0, "44"),
        (-0.0, "-0"),
        (16.5, "16.5"),
        (999_999.0, "999999"),
        (1_000_000.0, "1e+06"),
        (0.0001, "0.0001"),
        (0.000_015, "1.5e-05"),
    ] {
        assert_eq!(go_g(value), want);
    }
    let row = HubRow {
        key: "armory@-40".to_owned(),
        source: "sites".to_owned(),
        index: 8,
        hub_level: 2,
        cells: vec![
            HubCell::default(),
            HubCell {
                available: true,
                offset_units: -40.0,
                from_depth: 1,
                needs_deeper: true,
                only_depth: -1,
            },
        ],
        ..HubRow::default()
    };
    let want = "armory@-40 · sites#8 · ab 2 · −40 · ab Tiefe 1 · nur mit tieferer Stufe";
    assert_eq!(row_line(&row), want);
}
