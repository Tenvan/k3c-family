//! Inselgrößen (Go: `island_sizes_test.go`, WZ17.2, B-485/AC-02, AC-03; docs/rules/geplante-chunks.md § 7). Tests, die eine
//! Welt-Datei lesen (`parseTestWorld`, RU10.5) oder eine Insel bauen (`create_island`, RU14), tragen `ignore`; `own_*` prüft,
//! was schon läuft: `check_chunk_count(s)` auf einer von Hand gebauten `IslandDef`.

#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use std::collections::BTreeMap;

use k3c_data::island_sizes::{self, IslandSize};

use super::sizes::{check_chunk_count, check_chunk_counts, island_sizes as sizes_of};
use super::testutil::*;
use super::{CreateOptions, IslandDef, create_island};

fn size(name: &str, chunk_count: i64) -> IslandSize {
    IslandSize {
        name: name.into(),
        chunk_count,
    }
}

/// Go: `TestInselgroessenAusDenDaten`: die Tabelle in der Reihenfolge der Daten, `IslandSizes` liefert eine Kopie.
#[test]
fn inselgroessen_aus_den_daten() {
    let data = test_data();
    let want = [
        size("sehr klein", 16),
        size("klein", 24),
        size("mittel", 40),
        size("groß", 64),
        size("sehr groß", 100),
        size("mega groß", 160),
    ];
    let mut got = sizes_of(&data.island_sizes);
    assert_eq!(got, want, "Inselgrößen");
    got[0].chunk_count = 99; // Kopie: der Aufrufer ändert die Tabelle nicht
    assert_eq!(
        sizes_of(&data.island_sizes)[0].chunk_count,
        16,
        "IslandSizes liefert keine Kopie"
    );
}

/// Go: `TestInselgroessenWerdenGeprueft`: `loadIslandSizes` lehnt eine fehlerhafte Tabelle ab (seit RU3 `k3c_data`).
#[test]
fn inselgroessen_werden_geprueft() {
    let bad = [
        ("leer", r#"{"sizes":[]}"#),
        ("ungerade", r#"{"sizes":[{"name":"x","chunkCount":17}]}"#),
        (
            "doppelt",
            r#"{"sizes":[{"name":"x","chunkCount":16},{"name":"y","chunkCount":16}]}"#,
        ),
        ("ohne Name", r#"{"sizes":[{"name":"","chunkCount":16}]}"#),
        ("zu klein", r#"{"sizes":[{"name":"x","chunkCount":0}]}"#),
    ];
    for (name, raw) in bad {
        let err = island_sizes::load(raw).expect_err(name);
        assert!(err.to_string().contains("data/island-sizes.json"), "{name}: {err}");
    }
}

/// Ein Fehlerfall: `chunkCounts` der Insel und die Texte, die der Fehler nennt.
type Bad = (&'static [(&'static str, i64)], &'static [&'static str]);

fn island_with(counts: &[(&str, i64)]) -> IslandDef {
    IslandDef {
        id: "a".into(),
        depths: vec![0, 3, 4],
        chunk_counts: counts.iter().map(|(k, n)| ((*k).to_string(), *n)).collect(),
        ..IslandDef::default()
    }
}

/// Go: `TestChunkAnzahlWirdBeimLadenGeprueft` (die Prüfung selbst, ohne Welt-Datei): gültige Anzahlen gehen durch, die
/// anderen Fälle nennen Tiefe, Anzahl und die gültigen Werte.
#[test]
fn own_chunk_anzahl_wird_geprueft() {
    let data = test_data();
    let sizes = &data.island_sizes;
    assert!(check_chunk_counts(&island_with(&[("0", 24), ("4", 160)]), sizes).is_ok());
    assert!(
        check_chunk_counts(&island_with(&[]), sizes).is_ok(),
        "ohne Eintrag gilt die Länge des Bioms"
    );
    let bad: [Bad; 5] = [
        (&[("0", 18)], &["Tiefe 0", "18", "16, 24, 40, 64, 100, 160"]),
        (&[("3", 12)], &["Tiefe 3", "12", "16, 24, 40, 64, 100, 160"]),
        (&[("7", 24)], &["Tiefe \"7\""]),
        (&[("x", 24)], &["Tiefe \"x\""]),
        (&[("01", 24)], &["Tiefe \"01\""]),
    ];
    for (counts, want) in bad {
        let err = check_chunk_counts(&island_with(counts), sizes).expect_err("abgelehnt");
        for w in want {
            assert!(err.contains(w), "{counts:?}: Fehler {err:?} nennt {w:?} nicht");
        }
    }
    assert_eq!(
        check_chunk_count(sizes, 18).expect_err("18"),
        "die Chunk-Anzahl 18 ist keine Inselgröße (gültig: 16, 24, 40, 64, 100, 160)"
    );
    assert!(
        check_chunk_count(sizes, 0).is_err(),
        "0 ist keine Inselgröße, die Aufrufer lassen es vorher durch"
    );
    let d = island_with(&[("0", 24), ("4", 160)]);
    assert_eq!((d.count_at(0), d.count_at(4), d.count_at(3)), (24, 160, 0));
}

/// Go: `TestChunkAnzahlWirdBeimLadenGeprueft`: dieselben Fälle über die Welt-Datei (`parseTestWorld`).
#[test]
#[ignore = "wartet auf RU10.5"]
fn chunk_anzahl_wird_beim_laden_geprueft() {
    unimplemented!("RU10.5: parse_test_world (island_worlds_test.go)")
}

/// Go: `TestPlanungGegenChunkAnzahl` (AC-03): Die Planung wird gegen die eingestellte Anzahl geprüft (`parseTestWorld`).
#[test]
#[ignore = "wartet auf RU10.5"]
fn planung_gegen_chunk_anzahl() {
    unimplemented!("RU10.5: parse_test_world (island_worlds_test.go)")
}

/// Go: `TestInselMitChunkAnzahl`: Breite der Stufen nach Chunk-Anzahl (50 Units je Chunk bei Tiefe 0, 25 bei Tiefe 4).
#[test]
#[ignore = "wartet auf RU14"]
fn insel_mit_chunk_anzahl() {
    let data = test_data();
    let world = standard_world(&data);
    let opts = CreateOptions {
        chunk_counts: Some(BTreeMap::from([(0, 24), (4, 160)])),
        ..CreateOptions::default()
    };
    let isl = create_island(&data, &world, "groesse", &[0, 1, 4], 1.0, opts).expect("Insel");
    let plain = create_island(&data, &world, "groesse", &[1], 1.0, CreateOptions::default()).expect("Insel");
    for (i, want) in [24.0 * 50.0, plain.stages[0].width_units, 160.0 * 25.0]
        .iter()
        .enumerate()
    {
        assert_eq!(isl.stages[i].width_units, *want, "Stufe {i}: Breite");
    }
    let bad = CreateOptions {
        chunk_counts: Some(BTreeMap::from([(0, 18)])),
        ..CreateOptions::default()
    };
    let err = create_island(&data, &world, "groesse", &[0], 1.0, bad).expect_err("Anzahl 18");
    assert!(err.contains("16, 24, 40, 64, 100, 160"), "Anzahl 18 angenommen: {err}");
}

/// Go: `TestEingebetteteInselnOhneChunkAnzahl`: Die eingebetteten Daten der Standard-Welt enthalten keine Chunk-Anzahl.
#[test]
#[ignore = "wartet auf RU10.5"]
fn eingebettete_inseln_ohne_chunk_anzahl() {
    let data = test_data();
    for d in &standard_world(&data).islands {
        assert!(
            d.chunk_counts.is_empty(),
            "{}: eingebettete Daten enthalten eine Chunk-Anzahl",
            d.id
        );
    }
}
