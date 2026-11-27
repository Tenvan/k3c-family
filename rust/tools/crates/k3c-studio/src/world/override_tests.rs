use serde_json::Value;

use super::super::world_tests::{STANDARD, checkout, entry, failing, fixture, read_world};
use super::super::{Editor, ValuesView};
use super::with_leaf;

fn set(e: &Editor, base: &str, path: &[&str], value: &str) -> ValuesView {
    let path: Vec<String> = path.iter().map(|p| (*p).to_owned()).collect();
    e.set_override("standard", base, &path, value)
}

// B-879/AC-02: Ein übersteuerter Wert landet in `overrides` der Welt-Datei, die übrigen Bytes bleiben.
#[test]
fn override_schreibt_welt_datei() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let v = set(e, "economy", &["purse", "startGold"], "99");
    assert!(v.error.is_empty(), "{}", v.error);
    let start = entry(&v, "economy", &["purse", "startGold"]);
    assert_eq!(start.override_value.as_ref().map(|v| v.get()), Some("99"));
    let text = read_world(e, "standard");
    let head = STANDARD.strip_suffix("\n}\n").unwrap();
    assert!(text.starts_with(head), "Kopf und Inseln nicht byte-gleich:\n{text}");
    let file: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(file["overrides"]["economy"]["purse"]["startGold"], 99);
    // Ein zweites Feld kommt dazu, das erste bleibt; Kommazahlen bleiben, wie sie eingegeben wurden.
    set(e, "economy", &["payIntervalSeconds"], "0.50");
    let text = read_world(e, "standard");
    assert!(
        text.contains(r#""startGold": 99"#) && text.contains(r#""payIntervalSeconds": 0.50"#),
        "{text}"
    );
}

// Zurücksetzen (leer) und ein Wert gleich dem Basiswert entfernen den Override; ohne Overrides entfällt der Schlüssel.
#[test]
fn zuruecksetzen_stellt_datei_her() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let path = ["purse", "startGold"];
    set(e, "economy", &path, "99");
    let v = set(e, "economy", &path, "14");
    assert!(v.error.is_empty(), "{}", v.error);
    assert_eq!(read_world(e, "standard"), STANDARD);
    set(e, "economy", &path, "5");
    let v = set(e, "economy", &path, "");
    assert!(v.error.is_empty(), "{}", v.error);
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Ein abgelehnter Wert, ein unbekanntes Feld, eine ungültige ID und eine fehlende Engine lassen die Datei unverändert.
#[test]
fn ungueltiger_override_aendert_nichts() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let path = ["purse", "startGold"];
    let owned: Vec<String> = path.iter().map(|p| (*p).to_owned()).collect();
    let mut broken = e.clone();
    broken.engine = failing("k3c-game.exe fehlt");
    for (name, v) in [
        ("falscher Typ", set(e, "economy", &path, r#""viel""#)),
        ("kein JSON", set(e, "economy", &path, "{")),
        ("Feld unbekannt", set(e, "economy", &["gibt", "esnicht"], "1")),
        ("ID", e.set_override("../standard", "economy", &owned, "1")),
        ("ohne Engine", set(&broken, "economy", &path, "1")),
    ] {
        assert!(!v.error.is_empty(), "{name} ohne Fehler");
    }
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// B-986/AC-01: Unbekannte Datei → die Meldung nennt alle erlaubten Dateien, gelesen aus der Engine.
#[test]
fn unbekannte_datei_nennt_erlaubte() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    let v = set(e, "paper", &["x"], "1");
    assert!(v.error.contains(r#"unbekannte Datei "paper""#), "{}", v.error);
    // Die Engine nennt dieselbe Liste: Wächst sie dort, folgt die Meldung ohne Änderung des Studios.
    let draft = STANDARD.replacen("\n}\n", ",\n  \"overrides\": {\"paper\": {}}\n}\n", 1);
    let error = e.check_draft("standard", draft.as_bytes()).unwrap_err();
    let (_, list) = error.split_once("(erlaubt: ").unwrap();
    let list = list.trim().strip_suffix(')').unwrap_or(list.trim());
    assert!(
        v.error.contains(&format!("(erlaubt: {list})")),
        "Liste der Engine {list:?} fehlt in {:?}",
        v.error
    );
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// B-986/AC-02: Gesperrtes Feld → Grund und Verweis; unbekanntes Feld → Felder der Ebene (höchstens 12, danach …).
#[test]
fn gesperrt_und_unbekannt() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    for (file, path) in [("hub", "sitePlacement"), ("economy", "payRangeUnits")] {
        let v = set(e, file, &[path], "1");
        assert!(
            v.error.contains("nicht übersteuerbar") && v.error.contains("welten.md § 1.3"),
            "{file}: {}",
            v.error
        );
    }
    let v = set(e, "economy", &["purse", "gibtesnicht"], "1");
    let want = r#"unbekanntes Feld "gibtesnicht" in economy › purse (Felder: startGold"#;
    assert!(v.error.contains(want), "{}", v.error);
    let v = set(e, "buildings", &["gibtesnicht"], "1");
    assert!(
        v.error.contains("unbekanntes Feld") && v.error.contains(", …)"),
        "mehr als 12 Felder ohne …: {}",
        v.error
    );
    let v = set(e, "economy", &["purse"], "1");
    assert!(
        v.error.contains("Objekt oder eine Liste") && v.error.contains("startGold"),
        "Objekt: {}",
        v.error
    );
    assert_eq!(read_world(e, "standard"), STANDARD);
}

// Review: Auch ein tieferer Pfad unter einem gesperrten Feld nennt den Grund.
#[test]
fn tiefer_pfad_unter_gesperrtem_feld() {
    let Some(f) = fixture() else { return };
    let v = set(&f.editor, "hub", &["sitePlacement", "x", "y"], "1");
    assert!(v.error.contains("welten.md § 1.3"), "{}", v.error);
}

// Gleichzeitige Overrides auf dieselbe Datei verlieren keinen Wert (`writeMu`).
#[test]
fn gleichzeitige_overrides_verlieren_nichts() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    std::thread::scope(|scope| {
        let first = scope.spawn(|| set(e, "economy", &["purse", "startGold"], "99"));
        let second = scope.spawn(|| set(e, "economy", &["payIntervalSeconds"], "0.50"));
        for run in [first, second] {
            let v = run.join().unwrap();
            assert!(v.error.is_empty(), "{}", v.error);
        }
    });
    let text = read_world(e, "standard");
    assert!(
        text.contains(r#""startGold": 99"#) && text.contains(r#""payIntervalSeconds": 0.50"#),
        "{text}"
    );
}

// Bytes wie Gos `json.Marshal` des Baums: Schlüssel sortiert, Zahlen wie im Text, leere Objekte entfallen.
#[test]
fn blatt_setzen_wie_go() {
    let path = |parts: &[&str]| parts.iter().map(|p| (*p).to_owned()).collect::<Vec<_>>();
    let ov = with_leaf(Default::default(), "economy", &path(&["z"]), Some("1.50")).unwrap();
    let ov = with_leaf(
        ov,
        "economy",
        &path(&["a", "b"]),
        Some(r#"[{ "y": "A<", "x": 12345678901234567890 }]"#),
    )
    .unwrap();
    assert_eq!(
        ov["economy"].get(),
        r#"{"a":{"b":[{"x":12345678901234567890,"y":"A<"}]},"z":1.50}"#
    );
    let ov = with_leaf(ov, "economy", &path(&["a", "b"]), None).unwrap();
    assert_eq!(ov["economy"].get(), r#"{"z":1.50}"#);
    let ov = with_leaf(ov, "economy", &path(&["z"]), None).unwrap();
    assert!(ov.is_empty());
    assert_eq!(with_leaf(ov, "economy", &[], None).unwrap_err(), "der Pfad fehlt");
    let deep = vec!["a".to_owned(); 5000];
    let err = with_leaf(Default::default(), "economy", &deep, Some("1")).unwrap_err();
    assert_eq!(err, "der Pfad ist zu tief (höchstens 64 Ebenen)");
}

// Ein nicht lesbarer Block darf vor dem Schreiben nie als leer gelten.
#[test]
fn kaputte_overrides_bleiben_stehen() {
    let f = checkout(|_| failing("Engine darf nicht laufen"));
    let e = &f.editor;
    let broken = "{\n  \"id\": \"standard\",\n  \"overrides\": [1]\n}\n";
    std::fs::write(e.file("standard"), broken).unwrap();
    let error = e
        .draft_override("standard", "economy", &["a".to_owned()], "1", "")
        .unwrap_err();
    assert!(error.starts_with("Welt-Datei: "), "{error}");
    assert_eq!(read_world(e, "standard"), broken);
}
