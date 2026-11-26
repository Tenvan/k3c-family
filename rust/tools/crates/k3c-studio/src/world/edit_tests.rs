use std::sync::Arc;

use super::super::Editor;
use super::super::world_tests::{STANDARD, checkout, failing, fixture, read_world};
use super::write_checked;

const HARBOR: &str = r#""plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}"#;

/// Namen aller Dateien im Welten-Ordner, sortiert.
fn world_dir(e: &Editor) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(e.root.join("data").join("worlds"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// B-954/AC-01: Anlegen, Setzen, Chunks und Prüfen; die Datei bleibt gültig und format-stabil.
#[test]
fn anlegen_setzen_chunks_pruefen() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("germania", "Germania", "standard").unwrap();
    e.create("germania", "Nochmal", "").unwrap_err();
    let patch = r#"{"notes":"Römerzeit","combat":{"mode":"groups","groupSize":6},"islands":[
        {"id":"island1","name":"Wald","biomes":["forest","cave","mine","ironhold","crystal"],"depthScaling":{"hp":2}},
        {"id":"island2","name":"Moor"}]}"#;
    // Insel 1 ist nicht die letzte: ohne Hafen lehnt die Prüfung ab, die Datei bleibt.
    let before = read_world(e, "germania");
    let error = e.set("germania", patch).unwrap_err();
    assert!(error.contains("Hafen"), "{error}");
    assert_eq!(read_world(e, "germania"), before);
    let with_harbor = format!(r#""depthScaling":{{"hp":2}},{HARBOR}"#);
    let patch = patch.replacen(r#""depthScaling":{"hp":2}"#, &with_harbor, 1);
    e.set("germania", &patch).unwrap();
    let text = read_world(e, "germania");
    for want in [
        r#""notes": "Römerzeit""#,
        r#""combat": { "mode": "groups", "groupSize": 6 }"#,
        r#""depths": [0, 1, 2, 3, 4]"#,
        r#""depthScaling": { "hp": 2, "damage": 1.3, "speed": 1.1 }"#,
        r#"{ "id": "island2", "name": "Moor""#,
    ] {
        assert!(text.contains(want), "fehlt {want:?}:\n{text}");
    }
    assert!(text.find(r#""notes""#) < text.find(r#""islands""#), "{text}");
    // Dieselbe Änderung nochmal: Datei byte-gleich.
    e.set("germania", &patch).unwrap();
    assert_eq!(read_world(e, "germania"), text);
    chunks(e);
}

fn chunks(e: &Editor) {
    let harbor = r#"[{"slot":4,"kind":"harbor"}]"#;
    e.set_chunks("germania", "island2", 1, Some(harbor), Some(16)).unwrap();
    let got = read_world(e, "germania");
    assert!(
        got.contains(r#""plannedChunks": { "1": [ { "slot": 4, "kind": "harbor" } ] }"#)
            && got.contains(r#""chunkCounts": { "1": 16 }"#),
        "Chunks fehlen:\n{got}"
    );
    let error = e.set_chunks("germania", "island2", 7, Some("[]"), None).unwrap_err();
    assert_eq!(
        error,
        r#"die Tiefe 7 gehört nicht zur Insel "island2" (Tiefen: [0 1 2 3 4])"#
    );
    e.set_chunks("germania", "island2", 1, Some("null"), Some(0)).unwrap();
    let got = read_world(e, "germania");
    assert!(
        !got.contains("chunkCounts") && !got.contains(r#""1": ["#),
        "Chunks nicht entfernt:\n{got}"
    );
    assert!(e.check("germania").unwrap().unwrap_or_default().is_empty());
    assert_eq!(e.raw("germania").unwrap(), got);
    let d = e.load("germania");
    assert!(d.error.is_empty() && d.islands.len() == 2, "{d:?}");
    assert_eq!(
        (d.islands[0].biomes[1].as_str(), d.extends.as_str()),
        ("cave", "standard")
    );
}

#[test]
fn set_lehnt_ungueltiges_ab() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    for (name, patch) in [
        ("unbekanntes Feld", r#"{"farbe":"rot"}"#),
        ("kein Objekt", "[1]"),
        (
            "unbekanntes Biom",
            r#"{"islands":[{"id":"island1","biomes":["lava"]}]}"#,
        ),
        ("Insel ohne id", r#"{"islands":[{"name":"x"}]}"#),
        ("Kampf", r#"{"combat":{"mode":"chaos"}}"#),
    ] {
        assert!(e.set("standard", patch).is_err(), "{name} ohne Fehler");
    }
    assert!(e.create("../x", "X", "").is_err(), "ungültige ID angelegt");
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Die Texte der Insel-Prüfung wie in Go; sie fallen vor dem Aufruf der Engine.
#[test]
fn insel_fehlertexte_ohne_engine() {
    let f = checkout(|_| failing("Engine darf nicht laufen"));
    let e = &f.editor;
    for (patch, want) in [
        (
            r#"{"islands":{}}"#,
            "islands im patch muss eine Liste von Objekten sein",
        ),
        (r#"{"islands":[null]}"#, "islands[0]: id fehlt oder ist ungültig"),
        (
            r#"{"islands":[{"id":"island1","farbe":1}]}"#,
            "islands[0]: unbekanntes Feld \"farbe\" (erlaubt: id, biomes, name, etappe, depths, depthScaling, \
             plannedChunks, chunkCounts, endBoss, nature, natureByDepth)",
        ),
        (
            r#"{"islands":[{"id":"island1","story":{}}]}"#,
            "islands[0]: story ändert world_story_set, nicht world_set",
        ),
        (
            r#"{"islands":[{"id":"island1","biomes":["forest"],"depths":[0]}]}"#,
            "islands[0]: depths oder biomes, nicht beides",
        ),
        (
            r#"{"islands":[{"id":"island1","depthScaling":3}]}"#,
            "islands[0]: depthScaling muss ein Objekt sein (hp, damage, speed)",
        ),
        (
            r#"{"islands":[{"id":"neu","name":""}]}"#,
            "islands[0]: name muss ein Text sein",
        ),
    ] {
        assert_eq!(e.set("standard", patch).unwrap_err(), want, "{patch}");
    }
    let chunks = |chunks, count| e.set_chunks("standard", "island1", 0, chunks, count).unwrap_err();
    assert_eq!(chunks(None, None), "chunks oder count fehlt");
    assert_eq!(chunks(None, Some(-1)), "count darf nicht negativ sein");
    let error = e.set_chunks("standard", "weg", 0, None, Some(1)).unwrap_err();
    assert_eq!(error, r#"die Insel "weg" gibt es nicht"#);
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Review: Eine zwischen Lesen und Schreiben geänderte Datei wird nicht überschrieben; Anlegen ist exklusiv.
#[test]
fn write_checked_verliert_keine_aenderung() {
    let f = checkout(|_| failing("keine Engine"));
    let e = &f.editor;
    let path = e.file("standard");
    let raw = std::fs::read(&path).unwrap();
    let changed = [raw.as_slice(), b" "].concat();
    std::fs::write(&path, &changed).unwrap();
    let files = world_dir(e);
    let error = write_checked(&path, Some(&raw), b"{}", &e.log).unwrap_err();
    assert_eq!(error, "die Welt wurde inzwischen geändert, neu lesen");
    let error = write_checked(&path, None, b"{}", &e.log).unwrap_err();
    assert_eq!(error, "standard.json gibt es schon (ändern mit world_set)");
    assert_eq!(std::fs::read(&path).unwrap(), changed);
    assert_eq!(world_dir(e), files, "Temp-Reste");
}

#[test]
fn commit_erkennt_zwischenzeitliche_aenderung() {
    // Die Engine nimmt an, aber die Datei ändert sich währenddessen.
    let f = checkout(|root| {
        let path = root.join("data").join("worlds").join("standard.json");
        Arc::new(move |_| {
            std::fs::write(&path, "{\"id\": \"standard\"}\n").map_err(|e| e.to_string())?;
            Ok(Some(serde_json::value::RawValue::from_string("{}".to_owned()).unwrap()))
        })
    });
    let e = &f.editor;
    let files = world_dir(e);
    let error = e.set("standard", r#"{"notes":"neu"}"#).unwrap_err();
    assert_eq!(error, "die Welt wurde inzwischen geändert, neu lesen");
    assert_eq!(read_world(e, "standard"), "{\"id\": \"standard\"}\n");
    assert_eq!(world_dir(e), files, "Temp-Reste");
}

#[test]
fn abgelehnte_welt_entsteht_nicht() {
    let f = checkout(|_| failing("abgelehnt"));
    let e = &f.editor;
    let files = world_dir(e);
    assert_eq!(e.create("neu", "Neu", "").unwrap_err(), "abgelehnt");
    let view = e.create_view("neu", "Neu", "");
    assert_eq!(view.error, "abgelehnt");
    assert_eq!(view.warnings, Some(Vec::new()));
    assert_eq!(world_dir(e), files);
}

// B-985/AC-01: Ein Patch setzt den Hafen der bisher letzten Insel und hängt die nächste an; die Welt ist nie nur halb.
#[test]
fn insel_mit_hafen_in_einem_aufruf_anhaengen() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("hanse", "Hanse", "standard").unwrap();
    let patch = r#"{"islands":[
        {"id":"island1","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},
        {"id":"island2","name":"Visby","depths":[0,4],"plannedChunks":{"0":[{"slot":-5,"kind":"harbor"}]},"chunkCounts":{"0":16}},
        {"id":"island3","name":"Reval"}]}"#;
    e.set("hanse", patch).unwrap();
    let text = read_world(e, "hanse");
    for want in [
        r#""id": "island3""#,
        r#""id": "island2", "name": "Visby""#,
        r#""chunkCounts": { "0": 16 }"#,
        r#""kind": "harbor""#,
    ] {
        assert!(text.contains(want), "fehlt {want:?}:\n{text}");
    }
    e.check("hanse").unwrap();
}

// B-985/AC-02: Ohne Hafen lehnt die Prüfung ab; die Meldung nennt Insel und Feld plannedChunks, die Datei bleibt.
#[test]
fn anhaengen_ohne_hafen_nennt_insel_und_feld() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    e.create("hanse", "Hanse", "standard").unwrap();
    let before = read_world(e, "hanse");
    let error = e
        .set("hanse", r#"{"islands":[{"id":"island2","name":"Visby"}]}"#)
        .unwrap_err();
    for want in ["island1", "kein Hafen", "plannedChunks", "world_set"] {
        assert!(error.contains(want), "Meldung ohne {want:?}: {error}");
    }
    assert_eq!(read_world(e, "hanse"), before);
    // Ungültiger Chunk-Plan: die Engine nennt den Grund, die Datei bleibt.
    let patch =
        r#"{"islands":[{"id":"island1","plannedChunks":{"0":[{"slot":-7,"kind":"gibtesnicht"}]}},{"id":"island2"}]}"#;
    assert!(e.set("hanse", patch).is_err());
    assert_eq!(read_world(e, "hanse"), before);
}
