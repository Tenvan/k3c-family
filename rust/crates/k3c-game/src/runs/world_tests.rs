//! Port von `engine/simcli/world_test.go` (alle vier Tests, RU33.2).

use super::testutil::{DataCopy, call, call_in, report, root};
use super::{EXIT_FAILED, EXIT_OK, EXIT_USAGE};

/// B-879: level rechnet die Insel-Stufe der Standard-Welt mit ihrer Planung (Hafen in Tiefe 0).
#[test]
fn world_level() {
    let (code, out, err) = call(
        r#"{"op":"level","world":"standard","island":"island1","stage":0,"seed":"k3c"}"#,
        &["world"],
    );
    assert_eq!(code, EXIT_OK, "{err}");
    let r = report(&out);
    assert_eq!(r["biomeId"], "forest");
    assert!(!r["chunks"].as_array().expect("chunks").is_empty());
    assert!(!r["planned"].as_array().expect("planned").is_empty());
    assert!(r["warnings"].is_array());
    for bad in [
        r#"{"op":"level","world":"standard","island":"x","stage":0,"seed":"k3c"}"#,
        r#"{"op":"level","world":"standard","island":"island1","stage":99,"seed":"k3c"}"#,
        r#"{"op":"level","world":"../x","island":"island1","stage":0,"seed":"k3c"}"#,
    ] {
        assert_eq!(call(bad, &["world"]).0, EXIT_FAILED, "{bad}");
    }
    assert_eq!(call(r#"{"op":"fly"}"#, &["world"]).0, EXIT_USAGE);
}

/// `values` liefert die Basis je Datei (buildings, economy …) und die Overrides der Welt.
#[test]
fn world_values() {
    let (code, out, err) = call(r#"{"op":"values","world":"standard"}"#, &["world"]);
    assert_eq!(code, EXIT_OK, "{err}");
    let r = report(&out);
    assert!(r["base"]["economy"].is_object() && r["base"]["buildings"].is_object());
    assert!(r["overrides"].is_object() && r["children"].is_array());
}

/// `check` prüft einen Entwurf wie der Server: ein falscher Typ im Override ist ein Fehler, ein passender nicht.
#[test]
fn world_check() {
    let raw = std::fs::read_to_string(root().join("data/worlds/standard.json")).expect("standard.json");
    let draft = |ov: &str| {
        let text = format!(
            "{}, \"overrides\": {ov}}}",
            raw.trim_end_matches(['\r', '\n', ' ', '}'])
        );
        serde_json::json!({"op": "check", "world": "standard", "draft": serde_json::from_str::<serde_json::Value>(&text)
            .expect("Entwurf")})
        .to_string()
    };
    let (code, out, err) = call(&draft(r#"{"economy":{"purse":{"startGold":20}}}"#), &["world"]);
    assert!(
        code == EXIT_OK && out.contains(r#""warnings":[]"#),
        "gültiger Entwurf: Exit {code} {out} {err}"
    );
    let (code, out, _) = call(&draft(r#"{"economy":{"purse":{"startGold":"viel"}}}"#), &["world"]);
    assert!(
        code == EXIT_FAILED && out.contains("startGold"),
        "falscher Typ: Exit {code} {out}"
    );
}

/// B-981 (Review WE5.7): level setzt den Bau wie das Spiel in die tiefste Stufe der Insel, auch in Biomen ohne Feld `lair`.
#[test]
fn world_level_bau_in_tiefster_stufe() {
    let copy = DataCopy::new("kurz");
    let kurz = r#"{"id":"kurz","name":"Kurz","extends":"standard",
        "combat":{"mode":"groups","groupSize":5},
        "islands":[{"id":"island1","name":"Insel 1","depths":[0,1],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}}]}"#;
    std::fs::write(copy.0.join("worlds/kurz.json"), kurz).expect("Welt schreiben");
    for (stage, want) in [0, 1].into_iter().zip([0usize, 1]) {
        let input = format!(r#"{{"op":"level","world":"kurz","island":"island1","stage":{stage},"seed":"k3c"}}"#);
        let (code, out, err) = call_in(&copy.path(), &input, &["world"]);
        assert_eq!(code, EXIT_OK, "{err} {out}");
        let got = report(&out)["entities"]
            .as_array()
            .expect("entities")
            .iter()
            .filter(|e| e["kind"] == "lair")
            .count();
        assert_eq!(got, want, "Stufe {stage}: {got} Baue, erwartet {want}");
    }
}
