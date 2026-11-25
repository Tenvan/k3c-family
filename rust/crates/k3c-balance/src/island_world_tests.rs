//! Tests zu `island_world.rs` und `scope.rs` (Port von `engine/balance/island_world_test.go`, BS11).

use k3c_data::balance_targets::{FILE, load_targets};
use k3c_sim::island::add_island_player;

use crate::island_world::{create_in, load_world, shared, worlds};
use crate::run::{Matrix, Scenario, new_island, run};
use crate::scope::{Restrict, Scope};

fn scenario(world: &str, players: i64) -> Scenario {
    Scenario {
        seed: "1".into(),
        players,
        days: 1,
        world: world.into(),
        ..Scenario::default()
    }
}

/// BS11.1/AC-01 (B-642/AC-01): Mit Welt hat die Insel alle Tiefen der ersten Insel, ohne Welt wie bisher eine Stufe.
#[test]
fn welt_baut_alle_tiefen() {
    let isl = new_island(&scenario("standard", 2)).expect("Insel");
    let want = &shared().expect("Daten").standard.islands[0].depths;
    assert!(
        isl.stages.len() == 5 && isl.stages.len() == want.len(),
        "Stufen {}, Tiefen {want:?}",
        isl.stages.len()
    );
    for (i, w) in isl.stages.iter().enumerate() {
        assert_eq!(w.biome.depth, want[i], "Tiefe der Stufe {i}");
    }
    assert_eq!(isl.stages[0].players.len(), 2, "Monarchen auf Stufe 0");
    let old = new_island(&scenario("", 2)).expect("ohne Welt");
    assert_eq!(old.stages.len(), 1, "ohne Welt eine Stufe");
    assert!(
        new_island(&scenario("gibtsnicht", 1)).is_err(),
        "unbekannte Welt muss ein Fehler sein"
    );
}

/// Die Welt `standard` steht in der Liste der Welten.
#[test]
fn welten_nennt_standard() {
    assert!(worlds().expect("Welten").iter().any(|w| w == "standard"));
}

/// Ein Lauf ohne Welt schreibt keine Insel-Felder; mit Welt stehen die Stufen im Ergebnis.
#[test]
#[ignore = "wartet auf RU28"]
fn insel_lauf_metriken() {
    // `Metrics.stages` mit Ankunft je Stufe und die Abwesenheit der Insel-Felder im JSON braucht den Sammler (RU28.1).
}

/// BS11.1/AC-02: Der Sammler führt Ankunft, Bosse, Endboss und Burgfall je Stufe nach festem Ereignisverlauf.
#[test]
#[ignore = "wartet auf RU28"]
fn sammler_insel_ereignisse() {
    // `Collector::observe_island` und `island_event` (metrics_island.go) kommen mit RU28.1.
}

/// BS11.4: `Scope.world` lässt alle Ziele die ganze Insel spielen; jede andere Einschränkung ohne Welt entfernt die Insel-Ziele.
#[test]
fn scope_welt() {
    let raw = shared().expect("Daten").source.read(FILE).expect("Datei der Ziele");
    let ts = load_targets(&raw).expect("Ziele");
    let w = ts
        .restrict(&Scope {
            days: Some(40),
            bot: "coop2".into(),
            world: "standard".into(),
            ..Scope::default()
        })
        .expect("Umfang mit Welt");
    assert_eq!(w.targets.len(), ts.targets.len(), "Zahl der Ziele");
    for g in &w.targets {
        assert!(
            g.world == "standard" && g.days == 40 && g.bot == "coop2",
            "Ziel {:?}: {g:?}",
            g.name
        );
    }
    let one = ts
        .restrict(&Scope {
            days: Some(40),
            ..Scope::default()
        })
        .expect("Umfang ohne Welt");
    for g in &one.targets {
        assert!(g.world.is_empty(), "Ziel {:?} behält die Welt", g.name);
    }
}

/// BS11.5: Der Insel-Lauf ist deterministisch: gleiche Matrix, gleiche Bytes.
#[test]
fn insel_lauf_deterministisch() {
    let m = Matrix {
        seeds: vec!["k3c-dev".into()],
        players: vec![2],
        bots: vec!["coop2".into()],
        depths: vec![0],
        days: 1,
        world: "standard".into(),
        ..Matrix::default()
    };
    let a = run(&m).expect("Lauf").json().expect("JSON");
    let b = run(&m).expect("Lauf").json().expect("JSON");
    assert_eq!(a, b, "zweiter Insel-Lauf liefert andere Bytes");
}

/// B-692: Der Balancing-Lauf löst `extends` und `overrides` der Welt auf, Eltern aus data/worlds.
#[test]
fn welt_mit_overrides_im_lauf() {
    let mut w = load_world("standard").expect("Welt");
    w.id = "teuer".into();
    w.extends = "standard".into();
    w.overrides.insert(
        "economy".into(),
        serde_json::json!({"purse": {"startGold": 27, "maxGold": 40}}),
    );
    let mut isl = create_in(&w, "1").expect("Insel");
    let pi = add_island_player(&mut isl, 0);
    assert_eq!(isl.stages[0].players[pi].gold, 27, "Startgold aus den Overrides");
    w.extends = "fehlt".into();
    let err = create_in(&w, "1").expect_err("unbekannte Eltern-Welt");
    assert!(err.to_string().contains("extends"), "Fehler {err}");
}
