//! Port von `gamedata_test.go`. Die flachen Beispiele in `testdata/gamedata/` sind Kopien der Go-Testdaten (flach, weil
//! die `.gitignore` des Repos `reports/` und `saves/` überall ignoriert).

use std::path::Path;

use super::*;
use crate::test_helpers::tmp_dir;

/// Baut einen Repo-Aufbau aus den Beispielen.
fn fixture_root() -> tempfile::TempDir {
    let root = tmp_dir();
    let copies = [
        ("report-neu.json", "reports/gamepad-2026-09-30T19-14-02-123Z.json"),
        ("report-alt.json", "reports/gamepad-2026-09-29T08-00-05-000Z.json"),
        ("report-kaputt.json", "reports/kaputt.json"),
        ("save-autosave.json", "saves/autosave.json"),
        ("save-sicherung.json", "saves/autosave-2026-09-29T10-00-00-000Z.json"),
        ("save-kaputt.json", "saves/kaputt.json"),
        ("forest.json", "data/biomes/forest.json"),
    ];
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata/gamedata");
    for (from, to) in copies {
        let target = root.path().join(to);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::copy(data.join(from), target).unwrap();
    }
    root
}

fn local(iso: &str) -> String {
    local_minute(iso).unwrap()
}

#[test]
fn reports_list_neueste_zuerst() {
    let want = [
        format!(
            "gamepad-2026-09-30T19-14-02-123Z.json · {} · Windows NT 10.0; Win64; x64; Xbox; Xbox One · 2 Controller · 58 FPS bei 4000 Sprites",
            local("2026-09-30T19:14:02.123Z")
        ),
        format!(
            "gamepad-2026-09-29T08-00-05-000Z.json · {} · Linux; Android 14 · 0 Controller",
            local("2026-09-29T08:00:05.000Z")
        ),
        "kaputt.json · nicht lesbar".to_owned(),
    ];
    assert_eq!(reports_list(fixture_root().path()), Ok(want.join("\n")));
    assert_eq!(reports_list(tmp_dir().path()), Ok("keine Berichte".to_owned()));
}

#[test]
fn report_read_verdichtet() {
    let root = fixture_root();
    let text = report_read(root.path(), "gamepad-2026-09-30T19-14-02-123Z.json").unwrap();
    for part in [
        "Controller 0: Xbox Wireless Controller (standard) · Tasten A X Y View Menu Xbox",
        "Controller 1: Xbox Wireless Controller (standard) · Tasten keine",
        "FPS min/Ø: 100 Sprites 59/60 · 4000 Sprites 41/58",
        "Vollbild: button → ok · auto → Fehler: Permissions check failed",
        "Gleichzeitig max. 2 Controller · Zurück-Navigationen 1 · Tasten-Ereignisse 3",
    ] {
        assert!(text.contains(part), "fehlt {part:?} in:\n{text}");
    }
    let err = report_read(root.path(), "kaputt.json").unwrap_err();
    assert!(err.contains("nicht lesbar"), "{err}");
}

#[test]
fn report_read_lehnt_pfade_ab() {
    let root = fixture_root();
    std::fs::write(root.path().join("geheim.json"), "{}").unwrap();
    for name in [
        "../geheim.json",
        r"..\geheim.json",
        "sub/x.json",
        r"C:\x.json",
        "/etc/x.json",
        ".versteckt.json",
        "gamepad.txt",
        "a..json",
        "",
        "x.json\n",
        "C:x.json",
        "x.json:strom",
    ] {
        assert!(report_read(root.path(), name).is_err(), "{name:?} nicht abgelehnt");
    }
}

#[test]
fn saves_list_zeilen() {
    let root = fixture_root();
    let dir = saves_dir(root.path(), "");
    let want = [
        format!(
            "autosave-2026-09-29T10-00-00-000Z · Stufe 0/0 · Tag 1 · 1 Spieler · {}",
            local("2026-09-29T10:00:00.000Z")
        ),
        // time 4000 s, Zyklus (6 + 2 + 4 + 2) min = 840 s, Morgengrauen 120 s zählt zum neuen Tag → Tag 5
        format!(
            "autosave · Stufe 2/3 · Tag 5 · 2 Spieler · {}",
            local("2026-09-30T18:02:00.000Z")
        ),
        "kaputt · nicht lesbar".to_owned(),
    ];
    assert_eq!(saves_list(root.path(), &dir), Ok(want.join("\n")));
    std::fs::remove_file(root.path().join("data/biomes/forest.json")).unwrap();
    let text = saves_list(root.path(), &dir).unwrap();
    assert!(
        text.starts_with("Tag unbekannt: ") && text.contains("autosave · Stufe 2/3 · Tag ? ·"),
        "ohne forest.json: {text}"
    );
}

#[test]
fn saves_dir_und_leerer_ordner() {
    let root = tmp_dir();
    let absolute = tmp_dir().path().join("woanders");
    let cases = [
        (String::new(), root.path().join("saves")),
        ("spiel/stand".to_owned(), root.path().join("spiel").join("stand")),
        (absolute.display().to_string(), absolute.clone()),
    ];
    for (env, want) in cases {
        assert_eq!(saves_dir(root.path(), &env), want, "saves_dir({env:?})");
    }
    let dir = saves_dir(root.path(), "");
    assert_eq!(saves_list(root.path(), &dir), Ok("keine Spielstände".to_owned()));
}
