//! Fehlerfälle der Welt-Datei (Go: `TestWeltFehlerNennenOrt` in `island_worlds_test.go`, AC-02): Jeder Fall wird abgelehnt;
//! die Meldung nennt Datei (Welt), Insel, Tiefe und Platz, soweit vorhanden. Go hat eine Tabelle; hier stehen die Fälle in
//! drei Gruppen (Funktionen höchstens 60 Zeilen). Ein Fall ist `(Name, Text der Datei, Anfang der Meldung nach dem
//! Dateipfad, weitere Teile)`.
use super::switch_testutil::{test_island, test_world};
use super::testutil::*;

const FILE: &str = "data/worlds/test.json: ";
const SCALING: &str = r#""depthScaling":{"hp":1,"damage":1,"speed":1}"#;

type Case = (&'static str, String, &'static str, &'static [&'static str]);

fn island(fields: &str) -> String {
    test_world(&test_island("a", fields))
}

fn with_islands(head: &str, a: &str) -> String {
    format!(r#"{{{head},"islands":[{a}]}}"#)
}

/// Fälle der Datei und der Liste `islands`.
#[rustfmt::skip]
fn world_cases() -> Vec<Case> {
    let a = test_island("a", "");
    vec![
        ("kein JSON", r#"{"id":"#.into(), "kein gültiges JSON", &[]),
        ("ohne Insel", test_world(""), "keine Insel", &[]),
        ("doppelte Insel", test_world(&format!("{a},{a}")), "Insel a: doppelt", &[]),
        ("Insel-ID ungültig", test_world(&test_island("Insel 2", "")), "Insel \"Insel 2\": id ungültig", &[]),
        ("Feld der Welt", with_islands(r#""id":"test","name":"Test","size":3"#, &a), "unbekanntes Feld \"size\"", &[]),
        ("Pflicht Welt-Name", with_islands(r#""id":"test""#, &a), "name fehlt (Pflichtfeld)", &[]),
        ("Welt-Name leer", with_islands(r#""id":"test","name":"""#, &a), "name", &[]),
        ("Pflicht Inseln", r#"{"id":"test","name":"Test"}"#.into(), "islands fehlt (Pflichtfeld)", &[]),
        ("ID passt nicht", with_islands(r#""id":"nord","name":"Test""#, &a), "id \"nord\" passt nicht zum Dateinamen", &[]),
        ("Pflicht Welt-ID", with_islands(r#""name":"Test""#, &a), "id fehlt (Pflichtfeld)", &[]),
        ("Daten nach Ende", format!("{}}}", test_world(&a)), "kein gültiges JSON", &[]),
    ]
}

/// Fälle der Prüfung einer Insel (Name, Tiefen, Endboss, Faktoren, Planung, Chunk-Anzahl).
#[rustfmt::skip]
fn island_cases() -> Vec<Case> {
    vec![
        ("Name leer", island(r#","name":"""#), "Insel a: name", &[]),
        ("ohne Tiefen", island(r#","depths":[]"#), "Insel a: ohne Tiefen", &[]),
        ("Tiefe doppelt", island(r#","depths":[0,0,4]"#), "Insel a: doppelte Tiefe 0", &[]),
        ("unbekannte Tiefe", island(r#","depths":[4,9]"#), "Insel a: unbekannte Tiefe 9", &[]),
        ("Tiefe vor Endboss", island(r#","depths":[0,7]"#), "Insel a: unbekannte Tiefe 7", &[]),
        ("Endboss unbekannt", island(r#","endBoss":"x""#), "Insel a: endBoss \"x\" unbekannt (erlaubt: ", &["crystalHeartWarden"]),
        ("Endboss Miniboss", island(r#","endBoss":"goblinLeader""#), "Insel a: endBoss \"goblinLeader\" ist ein Miniboss", &[]),
        ("Faktor 0", island(r#","depthScaling":{"hp":0,"damage":1,"speed":1}"#), "Insel a: depthScaling braucht Faktoren > 0", &[]),
        ("Faktor negativ", island(r#","depthScaling":{"hp":1,"damage":-1,"speed":1}"#), "Insel a: depthScaling braucht Faktoren > 0", &[]),
        ("Plan mit Platz", island(r#","plannedChunks":{"0":[{"slot":9,"kind":"forest"}]}"#), "Insel a: plannedChunks Tiefe 0", &["Platz +9"]),
        ("Plan Tiefe fremd", island(r#","plannedChunks":{"7":[]}"#), "Insel a: plannedChunks: Tiefe \"7\"", &[]),
        ("Chunk-Anzahl", island(r#","chunkCounts":{"0":18}"#), "Insel a: chunkCounts Tiefe 0", &["18", "16, 24, 40, 64, 100, 160"]),
    ]
}

/// Fälle des strengen Lesens (unbekannte, doppelte und fehlende Felder).
#[rustfmt::skip]
fn strict_cases() -> Vec<Case> {
    let tail = format!(r#""depths":[4],{SCALING}"#);
    vec![
        ("Feld der Insel", island(r#","size":3"#), "Insel a: unbekanntes Feld \"size\"", &[]),
        ("Feld der Faktoren", island(r#","depthScaling":{"hp":1,"damage":1,"speed":1,"armor":1}"#), "Insel a: depthScaling: unbekanntes Feld \"armor\"", &[]),
        ("Feld der Planung", island(r#","plannedChunks":{"0":[{"slot":1,"kind":"forest","object":[]}]}"#), "", &["object"]),
        ("Pflicht Faktor", test_world(r#"{"id":"a","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1}}"#), "Insel a: depthScaling.speed fehlt (Pflichtfeld)", &[]),
        ("Pflicht Tiefen", test_world(&format!(r#"{{"id":"a","name":"Insel",{SCALING}}}"#)), "Insel a: depths fehlt (Pflichtfeld)", &[]),
        ("Pflicht Insel-ID", test_world(&format!(r#"{{"name":"Insel",{tail}}}"#)), "Insel Nr. 1: id fehlt (Pflichtfeld)", &[]),
        ("Feld groß", island(r#","ID":"b""#), "Insel a: unbekanntes Feld \"ID\"", &[]),
        ("Faktor doppelt", test_world(r#"{"id":"a","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1,"speed":1,"hp":0}}"#), "Insel a: depthScaling: doppeltes Feld \"hp\"", &[]),
        ("Insel-Feld doppelt", test_world(&format!(r#"{{"id":"a","id":"b","name":"Insel",{tail}}}"#)), "", &["doppeltes Feld \"id\""]),
    ]
}

/// Go: `TestWeltFehlerNennenOrt` (AC-02).
#[test]
fn welt_fehler_nennen_ort() {
    let cases = [world_cases(), island_cases(), strict_cases()].concat();
    assert_eq!(cases.len(), 32, "Go prüft 32 Fälle");
    for (name, raw, head, rest) in cases {
        match parse_world_in(&raw, "test") {
            Ok(w) => panic!("{name}: kein Fehler ({w:?})"),
            Err(e) => {
                let first = format!("{FILE}{head}");
                assert!(e.contains(&first), "{name}: Fehler {e:?} nennt {first:?} nicht");
                for w in rest {
                    assert!(e.contains(w), "{name}: Fehler {e:?} nennt {w:?} nicht");
                }
            }
        }
    }
}
