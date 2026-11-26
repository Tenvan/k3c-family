//! Port von `golden_test.go`.

use std::fs;

use super::super::anlegen::loesche;
use super::super::basis::GRAFIK;
use super::super::test_helpers::{leere_wurzel, lies, repo_root, schreibe, spec_root};
use super::*;

fn mauer() -> Asset {
    Asset {
        id: "mauer".to_owned(),
        rig: "gebaeude".to_owned(),
        gruppe: "gebaeude".to_owned(),
        pfad: format!("{GRAFIK}/gebaeude/mauer{ENDUNG}"),
    }
}

const MANIFEST: &str = "public/grafik/gebaeude/mauer/manifest.json";
const GOLDEN: &str = "public/grafik/gebaeude/golden.json";

/// B-707: Der neue renderKey steht in golden.json der Gruppe, im Format von gen:paper; andere Einträge und andere
/// Gruppen bleiben unberührt.
#[test]
fn golden_eintragen_ersetzt_nur_das_asset() {
    let w = leere_wurzel();
    let root = &w.pfad;
    let manifest =
        r#"{"id": "mauer", "renderKey": "neu", "animationen": {"stand": {"frames": 1, "datei": "stand.png"}}}"#;
    schreibe(root, MANIFEST, manifest);
    schreibe(root, "public/grafik/gebaeude/mauer/stand.png", "png");
    let alt = concat!(
        "{\n  \"mauer\": {\n    \"renderKey\": \"alt\",\n    \"dateien\": {\n      \"stand.png\": \"x\"\n    }\n  },\n",
        "  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n"
    );
    schreibe(root, GOLDEN, alt);
    let fremd = "{\n  \"monarch\": {}\n}\n";
    schreibe(root, "public/grafik/figuren/golden.json", fremd);

    golden_eintragen(root, &mauer()).unwrap();
    let hash = sha256_hex(b"png");
    let soll = [
        "{",
        "  \"mauer\": {",
        "    \"renderKey\": \"neu\",",
        "    \"dateien\": {",
        &format!("      \"stand.png\": \"{hash}\""),
        "    }",
        "  },",
        "  \"turm\": {",
        "    \"renderKey\": \"t\",",
        "    \"dateien\": {}",
        "  }",
        "}",
        "",
    ]
    .join("\n");
    assert_eq!(lies(root, GOLDEN), soll);
    assert_eq!(lies(root, "public/grafik/figuren/golden.json"), fremd);
}

/// Ohne golden.json legt das erste Asset sie an.
#[test]
fn golden_eintragen_neu() {
    let w = leere_wurzel();
    let root = &w.pfad;
    let manifest = r#"{"id": "mauer", "renderKey": "k", "animationen": {"stand": {"datei": "stand.png"}}}"#;
    schreibe(root, MANIFEST, manifest);
    schreibe(root, "public/grafik/gebaeude/mauer/stand.png", "png");
    golden_eintragen(root, &mauer()).unwrap();
    let text = lies(root, GOLDEN);
    assert!(text.contains(r#""renderKey": "k""#), "{text}");
}

/// Löschen nimmt den Eintrag aus golden.json.
#[test]
fn loesche_golden() {
    let w = spec_root();
    let root = &w.pfad;
    schreibe(root, MANIFEST, "{}");
    let turm = "  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n";
    let mauer = "  \"mauer\": {\n    \"renderKey\": \"m\",\n    \"dateien\": {}\n  },\n";
    schreibe(root, GOLDEN, format!("{{\n{mauer}{turm}"));
    loesche(root, "mauer").unwrap();
    assert_eq!(lies(root, GOLDEN), format!("{{\n{turm}"));
}

/// Jede golden.json im Repo (geschrieben von gen:paper) übersteht Lesen und Schreiben unverändert (nur gelesen).
#[test]
fn golden_format_wie_gen() {
    let grafik = repo_root().join("public").join("grafik");
    let mut geprueft = 0;
    for gruppe in fs::read_dir(grafik).unwrap().flatten() {
        let Ok(text) = fs::read_to_string(gruppe.path().join(GOLDEN_NAME)) else {
            continue;
        };
        let text = text.replace("\r\n", "\n");
        let alle = golden_lesen(text.as_bytes()).unwrap();
        assert_eq!(
            golden_text(&alle),
            text,
            "{:?}: Format weicht von gen:paper ab",
            gruppe.path()
        );
        geprueft += 1;
    }
    if geprueft == 0 {
        eprintln!("übersprungen: keine golden.json im Repo");
    }
}

/// Review RU70.10: Nur eine fehlende Datei gilt als leer; ein Verzeichnis steht für „vorhanden, nicht lesbar“.
#[test]
fn unlesbare_golden_wird_nicht_ersetzt() {
    let dir = crate::test_helpers::tmp_dir();
    let fehlt = lesen_falls_da(&dir.path().join("fehlt.json"), GOLDEN_NAME);
    assert!(fehlt.unwrap().is_none());
    assert!(lesen_falls_da(dir.path(), GOLDEN_NAME).is_err());
    assert!(golden_aendern(dir.path(), "a", None).is_err());
}

/// Neu: Auch beim Entfernen wird eine kaputte oder nicht als UTF-8 lesbare Datei nicht ersetzt.
#[test]
fn entfernen_laesst_kaputte_dateien_stehen() {
    let w = leere_wurzel();
    let root = &w.pfad;
    for kaputt in [&b"{\"turm\":"[..], &[b'[', 0xff, b']']] {
        schreibe(root, GOLDEN, kaputt);
        assert!(golden_entfernen(root, "gebaeude", "mauer").is_err());
        assert_eq!(fs::read(golden_pfad(root, "gebaeude")).unwrap(), kaputt);
        let index = gruppen_ordner(root, "gebaeude").join(INDEX_NAME);
        fs::write(&index, kaputt).unwrap();
        assert!(gruppe_entfernen(root, &mauer()).is_err());
        assert_eq!(fs::read(&index).unwrap(), kaputt);
        fs::remove_file(&index).unwrap();
    }
    fs::remove_file(golden_pfad(root, "gebaeude")).unwrap();
    assert!(!gruppe_entfernen(root, &mauer()).unwrap());
    assert!(!golden_pfad(root, "gebaeude").exists(), "ohne Datei entsteht keine");
}
