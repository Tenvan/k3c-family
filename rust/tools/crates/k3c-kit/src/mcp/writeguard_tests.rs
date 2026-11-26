//! Port von `mcpkit/writeguard_test.go`, dazu `inside_root`.

use super::*;
use crate::mcp::inside_root;
use crate::testutil::tmp_dir;

fn tree(dirs: &[&str]) -> (tempfile::TempDir, PathBuf) {
    let tmp = tmp_dir();
    let root = canon(tmp.path()).join("repo");
    for d in dirs {
        fs::create_dir_all(join(&root, d)).unwrap();
    }
    (tmp, root)
}

fn guard(allow: &[&str]) -> WriteGuard {
    WriteGuard {
        name: "Test".to_owned(),
        allow: allow.iter().map(|a| (*a).to_owned()).collect(),
        hint: String::new(),
    }
}

// TestWriteGuard (WZ63.7): nur die genannten Ordner, nicht zu umgehen mit „..“ oder absoluten Pfaden.
#[test]
fn write_guard() {
    let (_tmp, root) = tree(&["data/worlds", "engine/sim", "reports"]);
    let g = guard(&["data/worlds", "reports"]);
    let abs = root.join("reports").join("ratings.json");
    let mut ok = vec![
        "data/worlds/standard.json",
        "data/worlds/neu/x.json",
        "reports/simtest-1/simtest.md",
        abs.to_str().unwrap(),
    ];
    if cfg!(windows) {
        ok.push(r"data\worlds\standard.json");
        ok.push("DATA/Worlds/standard.json");
    }
    for p in ok {
        assert_eq!(g.check(&root, p), Ok(()), "{p} abgelehnt");
    }
    let outside = root.parent().unwrap().join("fremd.json");
    for p in [
        "data/troops.json",
        "data/worldsx/a.json",
        "engine/sim/x.go",
        "rust/a.rs",
        "data/worlds/../troops.json",
        r"data\worlds\..\troops.json",
        "../fremd.json",
        outside.to_str().unwrap(),
        "",
        ".",
    ] {
        assert!(g.check(&root, p).is_err(), "{p} erlaubt");
    }
    assert!(g.check("", "data/worlds/a.json").is_err(), "ohne Wurzel erlaubt");
}

#[test]
fn write_guard_meldungen_wie_go() {
    let (_tmp, root) = tree(&["data"]);
    let mut g = guard(&["data/worlds", "reports"]);
    assert_eq!(
        g.check(&root, "data/troops.json").unwrap_err(),
        "Test darf \"data/troops.json\" nicht schreiben (nicht in data/worlds/, reports/)"
    );
    assert_eq!(
        g.check(&root, ".").unwrap_err(),
        "Test darf \".\" nicht schreiben (nicht in data/worlds/, reports/)"
    );
    g.hint = "so geht es".to_owned();
    assert_eq!(
        g.check(&root, "a/../b").unwrap_err(),
        "Test darf \"a/../b\" nicht schreiben (Pfad leer oder mit „..“); so geht es"
    );
    assert_eq!(
        g.check("", "x").unwrap_err(),
        "Test: Repo-Wurzel fehlt, Schreiben abgelehnt"
    );
}

#[cfg(windows)]
fn junction(target: &Path, link: &Path) -> bool {
    std::process::Command::new("cmd")
        .args(["/d", "/c", "mklink", "/J"])
        .args([link, target])
        .output()
        .is_ok_and(|out| out.status.success())
}

#[cfg(windows)]
fn symlink(target: &Path, link: &Path) -> bool {
    std::os::windows::fs::symlink_dir(target, link).is_ok()
}

#[cfg(not(windows))]
fn symlink(target: &Path, link: &Path) -> bool {
    std::os::unix::fs::symlink(target, link).is_ok()
}

/// Ein Link unter data/worlds auf engine/ darf den Schutz nicht umgehen.
fn link_rejected(kind: &str, make: fn(&Path, &Path) -> bool) {
    let (_tmp, root) = tree(&["data/worlds", "engine"]);
    let worlds = join(&root, "data/worlds");
    let link = worlds.join("link");
    if !make(&root.join("engine"), &link) {
        eprintln!("{kind} nicht anlegbar (Windows ohne Entwicklermodus oder Adminrechte), übersprungen");
        return;
    }
    let g = guard(&["data/worlds"]);
    let abs = link.join("x.go");
    for p in ["data/worlds/link/x.go", abs.to_str().unwrap(), "data/worlds/link"] {
        let err = g.check(&root, p).expect_err(p);
        assert!(err.starts_with("Test darf"), "Meldung ohne Namen: {err}");
        assert!(err.contains("Weg führt über den Link data/worlds/link"), "{err}");
    }
}

// TestWriteGuardLink/symlink
#[test]
fn write_guard_link_symlink() {
    link_rejected("symlink", symlink);
}

// TestWriteGuardLink/junction
#[cfg(windows)]
#[test]
fn write_guard_link_junction() {
    let (_tmp, root) = tree(&[]);
    fs::create_dir_all(&root).unwrap();
    assert!(junction(&root, &root.join("selbst")), "mklink /J braucht keine Rechte");
    link_rejected("junction", junction);
}

/// Auch eine Junction, deren Ziel erlaubt wäre oder die erst ins erlaubte Verzeichnis führt, wird abgelehnt.
#[cfg(windows)]
#[test]
fn write_guard_junction_mit_erlaubtem_ziel() {
    let (_tmp, root) = tree(&["data/worlds/echt", "engine"]);
    let worlds = join(&root, "data/worlds");
    assert!(junction(&worlds.join("echt"), &worlds.join("innen")));
    assert!(junction(&worlds, &root.join("engine").join("hinein")));
    let g = guard(&["data/worlds"]);
    assert!(g.check(&root, "data/worlds/innen/a.json").is_err());
    assert!(g.check(&root, "engine/hinein/a.json").is_err());
}

/// Review RU66.3: Was `std` woanders hinschreiben würde als die Prüfung annimmt, wird abgelehnt.
#[cfg(windows)]
#[test]
fn mehrdeutige_windows_pfade_abgelehnt() {
    let (_tmp, root) = tree(&["data/worlds"]);
    let g = guard(&["data/worlds"]);
    for path in [
        r"\data\worlds\x.json",
        "/data/worlds/x.json",
        "C:data/worlds/x.json",
        "data/worlds/C:x.json",
    ] {
        assert!(g.check(&root, path).is_err(), "{path} erlaubt");
        assert!(inside_root(&root, path).is_err(), "{path} gefunden");
    }
    assert!(g.check(&root, "data/worlds/x.json").is_ok());
}

#[test]
fn inside_root_nur_unterhalb_der_wurzel() {
    let (_tmp, root) = tree(&["saves"]);
    fs::write(root.join("saves").join("a.json"), "{}").unwrap();
    let outside = root.parent().unwrap().join("fremd.json");
    fs::write(&outside, "{}").unwrap();
    let want = root.join("saves").join("a.json");
    assert_eq!(inside_root(&root, "saves/a.json"), Ok(want.clone()));
    assert_eq!(inside_root(&root, want.to_str().unwrap()), Ok(want));
    assert_eq!(
        inside_root(&root, "../fremd.json").unwrap_err(),
        "path \"../fremd.json\" abgelehnt: nur Dateien unterhalb der Repo-Wurzel"
    );
    assert!(inside_root(&root, outside.to_str().unwrap()).is_err());
    assert_eq!(
        inside_root(&root, "").unwrap_err(),
        "path fehlt: Datei relativ zur Repo-Wurzel"
    );
    let missing = inside_root(&root, "fehlt.json").unwrap_err();
    assert!(missing.starts_with("fehlt.json nicht gefunden: "), "{missing}");
    let no_root = inside_root(root.join("weg"), "a").unwrap_err();
    assert!(no_root.starts_with("Repo-Wurzel "), "{no_root}");
}

#[cfg(windows)]
#[test]
fn inside_root_junction_nach_draussen() {
    let (_tmp, root) = tree(&["saves"]);
    let outside = root.parent().unwrap().join("draussen");
    fs::create_dir_all(&outside).unwrap();
    fs::write(outside.join("a.json"), "{}").unwrap();
    assert!(junction(&outside, &root.join("saves").join("raus")));
    assert!(inside_root(&root, "saves/raus/a.json").is_err());
    assert!(inside_root(&root, "saves/raus").is_err());
}
