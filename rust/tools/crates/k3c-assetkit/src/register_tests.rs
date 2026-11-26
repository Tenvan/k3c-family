//! Port von `register_test.go`.

use super::*;
use crate::testutil::{repo, write_file};

fn choice(file: &str) -> Choice {
    Choice {
        file: file.to_string(),
        ..Choice::default()
    }
}

fn cut(file: &str, r: Rect) -> Choice {
    Choice {
        cut: Some(r),
        ..choice(file)
    }
}

/// B-298/AC-02: mehrere Zuordnungen stehen in gesetzter Reihenfolge im Register und kommen so wieder heraus.
#[test]
fn reihenfolge_im_register() {
    let dir = repo();
    let root = dir.path();
    let want = [choice("grafik/town/wagon.png"), choice("grafik/town/well.png")];
    set_choices(root, "farm", &want).unwrap();
    let reg = load(root).unwrap();
    let farm = reg.find("farm").unwrap();
    assert_eq!(farm.choices(), want);
    assert_eq!(status(root, farm), STATUS_ASSIGNED);
}

/// B-298/AC-03: ein Ausschnitt wird gespeichert und gelesen; außerhalb des Bildes wird er abgelehnt.
#[test]
fn ausschnitt() {
    let dir = repo();
    let root = dir.path();
    let full = Rect {
        x: 0,
        y: 0,
        w: 64,
        h: 48,
    };
    set_choices(root, "farm", &[cut("grafik/town/well.png", full)]).unwrap();
    let reg = load(root).unwrap();
    assert_eq!(reg.find("farm").unwrap().choices()[0].cut, Some(full));
    let outside = Rect { x: 40, w: 32, ..full };
    let err = set_choices(root, "farm", &[cut("grafik/town/well.png", outside)]).unwrap_err();
    assert!(err.to_string().contains("außerhalb"), "{err}");
}

#[test]
fn ungueltige_zuordnungen() {
    let dir = repo();
    let root = dir.path();
    for (want, file) in [
        ("fehlt", "grafik/town/gibts-nicht.png"),
        ("unter public/", "../data/sounds.json"),
        ("Art grafik", "audio/coin.ogg"),
    ] {
        let err = set_choices(root, "farm", &[choice(file)]).unwrap_err().to_string();
        assert!(err.contains(want), "{want}: {err}");
    }
    let err = set_choices(root, "nichts", &[]).unwrap_err().to_string();
    assert!(err.contains("unbekannter Asset-Slot"), "{err}");
    let sound = Choice {
        sprite: "coin".to_string(),
        ..choice("audio/coin.ogg")
    };
    set_choices(root, "sfx:coin", &[sound]).unwrap();
}

/// Ein kaputtes Register wird nie überschrieben.
#[test]
fn kaputtes_register_bleibt() {
    let dir = repo();
    let path = dir.path().join(REGISTER_PATH);
    write_file(&path, "{ kaputt");
    let err = set_choices(dir.path(), "farm", &[]).unwrap_err().to_string();
    assert!(err.contains(REGISTER_PATH), "Fehler ohne Pfad: {err}");
    assert_eq!(fs::read_to_string(&path).unwrap(), "{ kaputt");
}

#[test]
fn test_status() {
    let dir = repo();
    let reg = load(dir.path()).unwrap();
    assert_eq!(status(dir.path(), reg.find("farm").unwrap()), STATUS_GAP);
    let slot = Slot {
        id: "x".to_string(),
        kind: KIND_GRAPHIC.to_string(),
        choices: Some(vec![choice("grafik/town/weg.png")]),
        ..Slot::default()
    };
    assert_eq!(status(dir.path(), &slot), STATUS_MISSING);
}

/// Speichern behält das Format: zwei Leerzeichen, Umlaute im Klartext, Zeilenende `\n`.
#[test]
fn speichern_format() {
    let dir = repo();
    set_choices(dir.path(), "farm", &[choice("grafik/town/well.png")]).unwrap();
    let text = fs::read_to_string(dir.path().join(REGISTER_PATH)).unwrap();
    assert!(
        text.contains("\n  \"slots\"") && text.contains("Münze") && text.ends_with("}\n"),
        "{text}"
    );
}

/// Eigener Test: `mischung` bleibt (auch `null`), eine fehlende Liste `wahl` steht wie in Go als `null` in der Datei.
#[test]
fn unbekanntes_und_null_wie_go() {
    let dir = repo();
    let path = dir.path().join(REGISTER_PATH);
    write_file(
        &path,
        r#"{"mischung": null, "neu": 1, "slots": [{"id": "a", "bus": null}]}"#,
    );
    load(dir.path()).unwrap().save(dir.path()).unwrap();
    let want = concat!(
        "{\n  \"_comment\": \"\",\n  \"mischung\": null,\n  \"slots\": [\n    {\n      \"id\": \"a\",\n",
        "      \"art\": \"\",\n      \"kategorie\": \"\",\n      \"beschreibung\": \"\",\n      \"wahl\": null\n",
        "    }\n  ]\n}\n"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), want);
}

/// Eigener Test: ein Laufwerk im Pfad führt nicht aus `public/` heraus, ein fehlendes Register bleibt `NotFound`.
#[test]
fn pfade_und_fehlerarten() {
    let dir = repo();
    let outside = dir.path().join("public").join("audio").join("coin.ogg");
    let absolute = outside.to_string_lossy().replace('\\', "/");
    assert!(public_file(dir.path(), &absolute).is_err(), "{absolute}");
    assert!(public_file(dir.path(), "audio/../audio/coin.ogg").is_ok());
    let err = load(&dir.path().join("fehlt")).unwrap_err();
    assert!(
        matches!(&err, Error::Io(e) if e.kind() == io::ErrorKind::NotFound),
        "{err:?}"
    );
    assert!(err.to_string().starts_with("data/sounds.json: "), "{err}");
}
