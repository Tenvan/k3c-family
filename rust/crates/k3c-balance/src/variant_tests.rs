//! Tests zu `variant.rs` (Port von `engine/balance/variant_test.go`, WZ28.4). Go prüft mit SHA-256 über alle Dateien, dass
//! data/ unverändert bleibt; hier ist es der Vergleich aller Dateien selbst (stärker, gleiche Aussage).

use std::path::PathBuf;

use k3c_data::Source;
use k3c_sim::island::add_island_player;

use crate::island_world::create_island;
use crate::run::Scenario;
use crate::scope::{Restrict, Scope};
use crate::summary::Summarize;
use crate::targets::default_targets;
use crate::variant::with_variants;

/// Alle Dateien unter `data/` auf der Platte.
pub(crate) fn data_on_disk() -> std::collections::BTreeMap<String, String> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../data");
    Source::Dir(dir).files().expect("data/ lesen")
}

/// Das Startgold des ersten Monarchen auf der Insel von Seed 1 (Go: `startGold`).
pub(crate) fn start_gold() -> i64 {
    let sc = Scenario {
        seed: "1".into(),
        players: 1,
        ..Scenario::default()
    };
    let mut isl = create_island(&sc).expect("Insel");
    let pi = add_island_player(&mut isl, 0);
    isl.stages[0].players[pi].gold
}

/// WZ28.4/AC-04: Eine Variante setzt den Wert im Lauf, data/ bleibt unverändert, danach gilt wieder der alte Wert.
#[test]
fn variante_aendert_wert_nicht_data() {
    let before = data_on_disk();
    let normal = start_gold();
    let varied = with_variants(&["economy.json:purse.startGold=37".to_string()], || Ok(start_gold())).expect("Lauf");
    let after = start_gold();
    assert!(
        varied == 37 && after == normal,
        "Startgold mit Variante {varied} (erwartet 37), danach {after} (erwartet {normal})"
    );
    assert!(data_on_disk() == before, "data/ wurde geändert");
}

/// Ungültige Varianten sind Fehler mit der Variante.
#[test]
fn variante_ungueltig() {
    for s in [
        "economy.json:purse.gibtsnicht=1",
        "economy.json:purse=1",
        "fehlt.json:a=1",
        "economy.json:purse.startGold=viel",
        "economy.json:purse.startGold",
    ] {
        let err = with_variants(&[s.to_string()], || Ok(())).expect_err(s);
        assert!(
            err.to_string().contains(s),
            "{s}: Fehler mit Variante erwartet, war {err}"
        );
    }
}

/// Matrix-Einschränkung: Das Szenario der Zielläufe folgt Scope; Fortschritt kommt nach jedem Lauf.
#[test]
fn scope_und_fortschritt() {
    let ts = default_targets().expect("Ziele");
    let scope = Scope {
        days: Some(1),
        players: Some(1),
        bot: "passive".into(),
        grade: "easy".into(),
        ..Scope::default()
    };
    let r = ts.restrict(&scope).expect("Scope");
    let mut lines: Vec<String> = Vec::new();
    let rep = r
        .run_targets_progress(
            2,
            None,
            Some(Box::new(|done, total| {
                lines.push(format!("{}/{}", "x".repeat(done), "x".repeat(total)))
            })),
        )
        .expect("Lauf");
    assert!(
        rep.results.len() == 2 && lines.len() == 2 && lines[1] == "xx/xx",
        "{} Läufe, Fortschritt {lines:?}",
        rep.results.len()
    );
    for res in &rep.results {
        let sc = &res.scenario;
        assert!(
            sc.days == 1 && sc.players == 1 && sc.bot == "passive" && sc.grade == "easy" && res.valid,
            "Lauf {sc:?} ({})",
            res.error
        );
    }
    assert!(
        ts.targets.first().is_none_or(|t| t.days != 1),
        "Restrict hat die Ziele selbst geändert"
    );
}

/// Ein ungültiges Szenario ist ein Fehler.
#[test]
fn scope_ungueltig() {
    let ts = default_targets().expect("Ziele");
    for s in [
        Scope {
            bot: "gibtsnicht".into(),
            ..Scope::default()
        },
        Scope {
            grade: "gibtsnicht".into(),
            ..Scope::default()
        },
        Scope {
            players: Some(5),
            ..Scope::default()
        },
    ] {
        assert!(ts.restrict(&s).is_err(), "{s:?}: Fehler erwartet");
    }
}
