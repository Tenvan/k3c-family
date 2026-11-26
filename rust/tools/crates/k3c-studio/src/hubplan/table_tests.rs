//! Port von `tools/ruletable/table/table_test.go`.

use super::super::editor_tests::read_repo;
use super::{END, START, block, ersetzen, gegenprobe};

/// WZ15.2/AC-01: Der Block in § 3 ist genau der aus `data/hub.json` erzeugte.
#[test]
fn block_der_regeln_ist_gleich_der_json() {
    let doc = read_repo("docs/rules/materialien-gebaeude.md");
    let check = gegenprobe(&doc, read_repo("data/hub.json").as_bytes()).unwrap();
    assert!(
        check.same,
        "Block weicht ab (task gen:rules):\n--- ist\n{}\n--- soll\n{}",
        check.ist, check.soll
    );
}

#[test]
fn block_inhalt_fuer_die_heutigen_fuenf_stufen() {
    let got = block(read_repo("data/hub.json").as_bytes()).unwrap();
    for want in [
        "| 1 | Holz | Werkstatt −16; Bogner −28 | Zone 1: Baugrund Mauer ±44, Turm ±36, dazu 0…2 Plätze je Seed | Kaufen +16, Verkaufen +20 (nur Tiefe 0) |",
        "| 2 | Stein | Treppe hoch +16 (ab Tiefe 1); Treppe runter +8 (nur mit tieferer Stufe); Taverne −32; Kaserne +24; Lager −24 (je Insel) | Zone 2: 1…3 Mauern, 1…3 Türme je Seed | – |",
        "| 5 | Kristall | – | Zone 5: 1…3 Mauern, 1…3 Türme je Seed | – |",
    ] {
        assert!(got.contains(want), "Zeile fehlt: {want}\n{got}");
    }
    // Kopf und fünf Stufen.
    assert_eq!(got.matches("\n| ").count(), 5 + 1, "{got}");
}

/// WZ15.2/AC-03: Mehr Hub-Stufen ohne Umbau, keine feste 5.
#[test]
fn sieben_stufen_fixture() {
    let got = block(read_repo("testdata/hub/hub7.json").as_bytes()).unwrap();
    for want in [
        "| 6 | Stein | Schmiede +28 | Zone 6: 1…3 Mauern, 1…3 Türme je Seed | – |",
        "| 7 | Eisen | Rüstkammer +40 | Zone 7: 1…3 Mauern",
    ] {
        assert!(got.contains(want), "Zeile fehlt: {want}\n{got}");
    }
    assert_eq!(got.matches("\n| ").count(), 7 + 1);
}

#[test]
fn ersetzen_laesst_den_rest_byte_gleich() {
    let doc = format!("Kopf\n{START}\nalt\n{END}\nFuß\n");
    let new = format!("{START}\nneu\n{END}");
    let want = format!("Kopf\n{START}\nneu\n{END}\nFuß\n");
    assert_eq!(ersetzen(&doc, &new).unwrap(), want);
    let crlf = ersetzen(&doc.replace('\n', "\r\n"), &new).unwrap();
    assert_eq!(crlf, want.replace('\n', "\r\n"));
}

#[test]
fn ersetzen_fehlerfaelle() {
    for (name, doc) in [
        ("ohne Marken", "nichts".to_owned()),
        ("nur Start", format!("{START}\n")),
        ("Start doppelt", format!("{START}{END}{START}")),
        ("Ende vor Start", format!("{END}\n{START}")),
        ("Ende doppelt", format!("{START}{END}{END}")),
    ] {
        assert!(ersetzen(&doc, "x").is_err(), "{name}: Fehler erwartet");
    }
}

#[test]
fn block_fehler() {
    assert!(block(b"{").is_err(), "kaputtes JSON");
    assert!(block(br#"{"levels":[]}"#).is_err(), "ohne levels");
}

#[test]
fn block_kommazahlen_und_stufen_grenzen() {
    let doc = |site: &str, levels: &str| {
        format!(
            r#"{{"levels":{levels},"sites":[{site}],"sitePlacement":{{"zone1WallUnits":44.5,"zone1TowerUnits":36.5,"zones":1}},"merchant":{{"buyOffsetUnits":16,"sellOffsetUnits":20,"onlyDepth":0}}}}"#
        )
    };
    let run = |site: &str, levels: &str| block(doc(site, levels).as_bytes());
    let one = r#"[{"cost":{}}]"#;
    let got = run(r#"{"kind":"workshop","offsetUnits":-16.5}"#, one).unwrap();
    assert!(
        got.contains("Werkstatt −16.5") && got.contains("Baugrund Mauer ±44.5, Turm ±36.5"),
        "{got}"
    );
    // Ein fehlender hubLevel heißt Stufe 1 wie in der Engine.
    let got = run(r#"{"kind":"nursery","offsetUnits":52}"#, one).unwrap();
    assert!(got.contains("Baumschule +52"), "{got}");
    let outside = run(r#"{"kind":"nursery","offsetUnits":52,"hubLevel":3}"#, one);
    assert!(outside.is_err(), "hubLevel außerhalb von levels muss ein Fehler sein");
    let site = r#"{"kind":"nursery","offsetUnits":52,"hubLevel":1}"#;
    let two = run(site, r#"[{"cost":{}},{"cost":{"stone":1,"iron":2}}]"#);
    assert_eq!(
        two.unwrap_err(),
        "hub.json: levels[1] hat mehrere materialien [iron stone]"
    );
    let unknown = run(site, r#"[{"cost":{}},{"cost":{"mithril":1}}]"#);
    assert_eq!(
        unknown.unwrap_err(),
        r#"hub.json: levels[1] hat das unbekannte material "mithril""#
    );
    // Ausbauschritte (B-592): Material aus den Kosten der Schritte, das Material-Tor zählt nicht.
    let steps = r#"[{"steps":[]},{"steps":[{"cost":{"stone":1,"gold":1}},{"cost":{"stone":2},"gate":{"copper":3}}]}]"#;
    let got = run(site, steps).unwrap();
    assert!(got.contains("| 2 | Stein |"), "{got}");
}

#[test]
fn ersetzen_normalisiert_den_block() {
    let doc = format!("{START}\r\nalt\r\n{END}\r\n");
    let got = ersetzen(&doc, &format!("{START}\r\nneu\r\n{END}")).unwrap();
    assert_eq!(got, format!("{START}\r\nneu\r\n{END}\r\n"));
}

/// Neu: `null` heißt wie in Go Nullwert, auch als Listen-Eintrag.
#[test]
fn null_ist_nullwert() {
    let got = block(br#"{"levels":[null,{"cost":{"stone":null},"steps":null}],"sites":null,"merchant":null}"#).unwrap();
    assert!(got.contains("| 2 | Holz | – |"), "{got}");
}
