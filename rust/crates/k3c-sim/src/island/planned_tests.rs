//! Planung der Insel beim Laden (Go: `island_planned_test.go`, B-461): Die Planung wird mit der Welt-Datei gelesen und
//! geprüft; eine leere Planung ist keine Planung. Beide Tests laufen: Die eingebetteten Daten der Standard-Welt prüft der
//! zweite an den Rohdaten (`standard_world` liest auch die Geschichte, RU19).
use k3c_level::level::HARBOR_KIND;

use super::IslandDef;
use super::testutil::*;

/// Go: `islandsWithPlan` liest eine Test-Welt mit einer Insel „a“ und der Planung `plan` (`parse_test_world`).
fn islands_with_plan(plan: &str) -> Result<Vec<IslandDef>, String> {
    parse_test_world(&format!(r#","plannedChunks":{plan}"#)).map(|w| w.islands)
}

/// Go: `TestInselPlanungWirdBeimLadenGeprueft`.
#[test]
fn insel_planung_wird_beim_laden_geprueft() {
    let good = r#"{"0":[{"slot":-3,"kind":"clearing","seed":"lichtung"}],"1":[{"slot":3,"kind":"cavern","objects":[{"kind":"stoneVein","at":40}]}]}"#;
    let dst = islands_with_plan(good).unwrap_or_else(|e| panic!("gültige Planung abgelehnt: {e}"));
    let plan = &dst[0].planned_chunks["0"];
    assert!(
        plan.len() == 1 && plan[0].seed == "lichtung",
        "Planung nicht geladen: {:?}",
        dst[0].planned_chunks
    );
    let bad = [
        ("Tiefe unbekannt", r#"{"7":[{"slot":1,"kind":"forest"}]}"#),
        ("Tiefe kein Wert", r#"{"x":[{"slot":1,"kind":"forest"}]}"#),
        ("Tiefe 00", r#"{"00":[{"slot":1,"kind":"forest"}]}"#),
        ("Feld unbekannt", r#"{"0":[{"slot":1,"kind":"forest","object":[]}]}"#),
        ("Platz 0", r#"{"0":[{"slot":0,"kind":"forest"}]}"#),
        ("Art unbekannt", r#"{"0":[{"slot":1,"kind":"volcano"}]}"#),
        ("Chunk der Höhle", r#"{"0":[{"slot":1,"kind":"cavern"}]}"#),
        (
            "Teil 2 reserviert",
            r#"{"0":[{"slot":1,"kind":"forest","buildings":[{"kind":"smithy"}]}]}"#,
        ),
    ];
    for (name, plan) in bad {
        match islands_with_plan(plan) {
            Err(e) if e.contains("data/worlds/test.json") => {}
            other => panic!("{name}: Ergebnis {other:?}"),
        }
    }
}

/// Go: `TestInselPlanungLeerIstKeinePlanung`.
#[test]
fn insel_planung_leer_ist_keine_planung() {
    for plan in ["{}", "null", r#"{"0":[]}"#] {
        if let Err(e) = islands_with_plan(plan) {
            panic!("{plan}: {e}");
        }
    }
    // Einzige Planung der Standard-Welt: der Hafen auf Tiefe 0 (BAS, B-637); Rohdaten der Welt-Datei.
    let data = test_data();
    for d in data.standard_world.islands.iter().flatten() {
        let plans = d.get("plannedChunks").and_then(|p| p.as_object());
        for (depth, plan) in plans.into_iter().flatten() {
            let list = plan.as_array().map_or(&[][..], |l| l.as_slice());
            assert!(
                depth == "0" && list.len() == 1 && list[0]["kind"] == HARBOR_KIND,
                "{}: eingebettete Daten enthalten außer dem Hafen eine Planung (Tiefe {depth})",
                d["id"]
            );
        }
    }
}
