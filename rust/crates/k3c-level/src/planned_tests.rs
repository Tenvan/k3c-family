use super::{geometry_of, validate_plan};
use crate::biome::Biome;
use crate::testutil::{biome, rules};
use crate::types::{PlannedChunk, PlannedObject};

fn p(slot: i64, kind: &str, objects: &[PlannedObject]) -> PlannedChunk {
    PlannedChunk {
        slot,
        kind: kind.into(),
        objects: objects.to_vec(),
        ..PlannedChunk::default()
    }
}

fn obj(kind: &str, at: f64) -> PlannedObject {
    PlannedObject { kind: kind.into(), at }
}

fn plain(kind: &str, slots: &[i64]) -> Vec<PlannedChunk> {
    slots.iter().map(|&s| p(s, kind, &[])).collect()
}

fn plan_ok(b: &Biome, plan: &[PlannedChunk]) -> Result<(), String> {
    validate_plan(b, plan, rules()).map_err(|e| e.to_string())
}

#[test]
#[ignore = "wartet auf RU6.3"]
fn validate_plan_gueltig() {
    let (forest, crystal) = (biome("forest"), biome("crystal"));
    let seed = PlannedChunk {
        slot: -3,
        kind: "clearing".into(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let cases: Vec<(&str, &Biome, Vec<PlannedChunk>)> = vec![
        ("keine Planung", &forest, vec![]),
        ("Chunk-Seed", &forest, vec![seed]),
        ("Portal in der Mitte", &forest, vec![p(4, "portal", &[])]),
        (
            "Portal genau am Mindestabstand",
            &forest,
            vec![p(3, "portal", &[obj("portal", 0.0)])],
        ),
        (
            "Portal in 25er-Biom bei 10",
            &crystal,
            vec![p(4, "portal", &[obj("portal", 10.0)])],
        ),
        (
            "feste Objekte",
            &forest,
            vec![p(
                2,
                "clearing",
                &[obj("tree", 10.0), obj("bush", 30.0), obj("chest", 45.0)],
            )],
        ),
    ];
    for (name, b, plan) in cases {
        if let Err(e) = plan_ok(b, &plan) {
            panic!("{name}: {e}");
        }
    }
}

type Case = (&'static str, Biome, Vec<PlannedChunk>, &'static str);

fn case(name: &'static str, b: &Biome, plan: Vec<PlannedChunk>, want: &'static str) -> Case {
    (name, b.clone(), plan, want)
}

fn one(kind: &str, o: PlannedObject) -> Vec<PlannedChunk> {
    vec![p(1, kind, &[o])]
}

/// Chunk-Art, Platz und Länge.
fn cases_chunk(forest: &Biome, mine: &Biome) -> Vec<Case> {
    let beyond = "jenseits der kürzesten Länge";
    vec![
        case("Platz 0", forest, plain("forest", &[0]), "keinen Platz 0"),
        case("Hub", forest, vec![p(1, "hub", &[])], "nicht planbar"),
        case("Ausgang", forest, vec![p(1, "exit", &[])], "nicht planbar"),
        case("Rand", forest, vec![p(1, "edge", &[])], "nicht planbar"),
        case("zu weit rechts", forest, plain("forest", &[8]), beyond),
        case("zu weit links", forest, plain("forest", &[-8]), beyond),
        case("Mine kürzer", mine, vec![p(5, "shaft", &[])], beyond),
        case("doppelter Platz", forest, plain("forest", &[2, 2]), "doppelt geplant"),
        case(
            "unbekannte Chunk-Art",
            forest,
            vec![p(1, "volcano", &[])],
            "unbekannte Chunk-Art",
        ),
    ]
}

/// Objekt-Art und Lage.
fn cases_objects(forest: &Biome, crystal: &Biome) -> Vec<Case> {
    let (unknown, border) = ("unbekannte Objekt-Art", "Abstand zur Chunk-Grenze");
    let double = vec![p(1, "forest", &[obj("tree", 10.0), obj("tree", 10.0)])];
    let across = vec![
        p(3, "recruitCamp", &[obj("recruitCamp", 50.0)]),
        p(4, "recruitCamp", &[obj("recruitCamp", 0.0)]),
    ];
    vec![
        case(
            "unbekannte Objekt-Art",
            forest,
            one("forest", obj("diamond", 10.0)),
            unknown,
        ),
        case(
            "kein Skill-Punkt",
            forest,
            one("forest", obj("skillPoint", 10.0)),
            unknown,
        ),
        case(
            "Wald ohne Adern",
            forest,
            one("forest", obj("stoneVein", 10.0)),
            unknown,
        ),
        case("kind@x doppelt", forest, double, "doppelt"),
        case("zu nah an der Grenze", forest, one("forest", obj("tree", 1.0)), border),
        case(
            "zu nah an der Grenze 2",
            forest,
            one("forest", obj("chest", 49.0)),
            border,
        ),
        case("Portal zu nah am Hub", forest, vec![p(2, "portal", &[])], "mindestens"),
        case(
            "Portal in 25er-Biom",
            crystal,
            vec![p(4, "portal", &[obj("portal", 9.0)])],
            "mindestens",
        ),
        case(
            "Portal Platz 3 in 25er",
            crystal,
            vec![p(3, "portal", &[])],
            "mindestens",
        ),
        case(
            "Portal hinter dem Chunk",
            forest,
            one("portal", obj("portal", 100.0)),
            "außerhalb des Chunks",
        ),
        case("kind@x über Chunks", forest, across, "kind@x"),
    ]
}

/// Summen und Platz.
fn cases_totals(forest: &Biome, mine: &Biome) -> Vec<Case> {
    let chest = |slot| p(slot, "chest", &[]);
    let camp = |slot| p(slot, "recruitCamp", &[]);
    let copper = |slot| p(slot, "shaft", &[obj("copperVein", 10.0)]);
    let portals = vec![p(3, "portal", &[]), p(-3, "portal", &[]), p(4, "portal", &[])];
    let big_plan = plain("forest", &[1, -1, 2, -2, 3, -3, 4, -4]);
    let no_portal_room = plain("forest", &[3, -3, 4, -4, 5, -5, 6, -6, 7, -7]);
    vec![
        case("zu viele Portale", forest, portals, "feste Portale"),
        case(
            "zu viele Truhen",
            forest,
            vec![chest(1), chest(-1), chest(2), chest(-2)],
            "feste chest",
        ),
        case(
            "zu viele Camps",
            forest,
            vec![camp(1), camp(-1), camp(2)],
            "feste recruitCamp",
        ),
        case(
            "zu viele Adern",
            mine,
            vec![copper(3), copper(4), copper(-3)],
            "feste Adern",
        ),
        case("kein Platz mehr", forest, big_plan, "kein Platz mehr"),
        case("kein Platz für Portale", forest, no_portal_room, "weitere Portale"),
        case("Mine ohne Platz", mine, plain("shaft", &[1, -1]), "kein Platz mehr"),
    ]
}

#[test]
#[ignore = "wartet auf RU6.3"]
fn validate_plan_lehnt_ab() {
    let (forest, crystal, mine) = (biome("forest"), biome("crystal"), biome("mine"));
    let mut cases = cases_chunk(&forest, &mine);
    cases.extend(cases_objects(&forest, &crystal));
    cases.extend(cases_totals(&forest, &mine));
    for (name, b, plan, want) in cases {
        let err = plan_ok(&b, &plan);
        assert!(
            err.as_ref().is_err_and(|e| e.contains(want)),
            "{name}: Fehler {err:?}, erwartet mit {want:?}"
        );
    }
}

#[test]
fn validate_plan_nennt_den_platz() {
    let err = plan_ok(&biome("forest"), &[p(5, "volcano", &[])]);
    assert!(
        err.as_ref().is_err_and(|e| e.contains("Platz +5")),
        "Fehler {err:?}, erwartet mit Platz +5"
    );
}

#[test]
#[ignore = "wartet auf RU6.3"]
fn validate_plan_mine_mit_freiem_platz() {
    if let Err(e) = plan_ok(&biome("mine"), &plain("shaft", &[1])) {
        panic!("ein geplanter Chunk in der Mine lässt Platz: {e}");
    }
}

#[test]
fn leere_felder_sind_keine_planung() {
    for raw in ["null", "[]"] {
        let plan: Option<Vec<PlannedChunk>> = serde_json::from_str(raw).expect("JSON");
        let plan = plan.unwrap_or_default();
        if let Err(e) = plan_ok(&biome("forest"), &plan) {
            panic!("{raw}: {e}");
        }
    }
}

#[test]
fn platz_grenzen_je_biom() {
    let want = [("forest", 7), ("cave", 5), ("mine", 4), ("ironhold", 7), ("crystal", 9)];
    for (id, n) in want {
        let got = geometry_of(&biome(id), 0, rules().hub_sites.clone()).usable;
        assert_eq!(got, n, "{id}: planbare Plätze je Seite");
    }
}

#[test]
fn planned_chunk_liest_streng() {
    let strict = |raw: &str| serde_json::from_str::<PlannedChunk>(raw);
    assert!(
        strict(r#"{"slot":1,"kind":"forest","object":[]}"#).is_err(),
        "unbekanntes Feld object muss abgelehnt werden"
    );
    assert!(
        strict(r#"{"slot":1,"kind":"forest","objects":[{"kind":"tree","att":3}]}"#).is_err(),
        "unbekanntes Feld in einem Objekt muss abgelehnt werden"
    );
    let ok = strict(r#"{"slot":1,"kind":"forest","seed":"x","objects":[{"kind":"tree","at":3}]}"#);
    assert!(ok.is_ok(), "gültiges Chunk-Format: {ok:?}");
}
