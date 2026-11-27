//! Port der Suche-Tests aus `devices_test.go` und von `ffmpeg_find_windows_test.go`. Registry und PATH des Rechners
//! werden nie gelesen.

use std::collections::HashMap;
use std::path::Path;

use super::*;
use crate::testutil::tmp_dir;

#[derive(Default)]
struct Machine {
    env: HashMap<&'static str, &'static str>,
    on_path: Option<&'static str>,
    reg: Vec<String>,
    files: Vec<String>,
    winget: Vec<&'static str>,
}

impl Machine {
    fn find(&self) -> Result<String, Error> {
        let env = |k: &str| self.env.get(k).map_or_else(String::new, |v| (*v).to_owned());
        let look_path = |_: &str| self.on_path.map(str::to_owned);
        let reg_paths = || self.reg.clone();
        let glob = |_: &str| self.winget.iter().map(|w| (*w).to_owned()).collect();
        let is_file = |p: &str| self.files.iter().any(|f| f == p);
        let finder = Finder {
            env: &env,
            look_path: &look_path,
            reg_paths: Some(&reg_paths),
            glob: &glob,
            is_file: &is_file,
        };
        finder.find("ffmpeg")
    }
}

#[test]
fn find_ffmpeg_reihenfolge() {
    let reg = Path::new("C:").join("ff").join("bin").to_string_lossy().into_owned();
    let reg_exe = Path::new(&reg).join(exe_name()).to_string_lossy().into_owned();
    // 1. Variable hat Vorrang vor PATH.
    let m = Machine {
        env: HashMap::from([(FFMPEG_ENV, "X:/mein/ffmpeg.exe")]),
        on_path: Some("C:/pfad/ffmpeg.exe"),
        files: vec!["X:/mein/ffmpeg.exe".to_owned()],
        ..Machine::default()
    };
    assert_eq!(m.find().unwrap(), "X:/mein/ffmpeg.exe");
    // 2. PATH vor Registry.
    let m = Machine {
        on_path: Some("C:/pfad/ffmpeg.exe"),
        reg: vec![reg.clone()],
        files: vec![reg_exe.clone()],
        ..Machine::default()
    };
    assert_eq!(m.find().unwrap(), "C:/pfad/ffmpeg.exe");
    // 3. Registry-PATH, wenn der Prozess-PATH veraltet ist.
    let m = Machine {
        reg: vec![reg],
        files: vec![reg_exe.clone()],
        ..Machine::default()
    };
    assert_eq!(m.find().unwrap(), reg_exe);
    // 4. WinGet-Ordner, neueste Version zuletzt.
    let m = Machine {
        env: HashMap::from([("LOCALAPPDATA", "C:/L")]),
        winget: vec!["alt/ffmpeg.exe", "neu/ffmpeg.exe"],
        ..Machine::default()
    };
    assert_eq!(m.find().unwrap(), "neu/ffmpeg.exe");
}

#[test]
fn find_ffmpeg_fehler_nennt_orte() {
    let m = Machine {
        env: HashMap::from([(FFMPEG_ENV, "X:/weg.exe"), ("LOCALAPPDATA", "C:/L")]),
        reg: vec!["D:/a".to_owned()],
        ..Machine::default()
    };
    let error = m.find().unwrap_err();
    assert!(matches!(error, Error::NoFfmpeg(_)), "{error:?}");
    assert_eq!(
        error.to_string(),
        "ffmpeg nicht gefunden (ffmpeg installieren); geprüft: K3C_FFMPEG=X:/weg.exe (Datei fehlt), \
         PATH des Prozesses, PATH aus der Registry, WinGet-Ordner; oder K3C_FFMPEG auf die ffmpeg.exe setzen"
    );
}

#[test]
fn glob_sortiert_je_ebene() {
    let dir = tmp_dir();
    let packages = dir.path().join("Packages");
    for version in ["ffmpeg-7.1", "ffmpeg-8.0", "anderes"] {
        let bin = packages.join("Gyan.FFmpeg_x").join(version).join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::write(bin.join("ffmpeg.exe"), "").unwrap();
    }
    // Ohne Programmdatei zählt der Ordner nicht.
    std::fs::create_dir_all(packages.join("Gyan.FFmpeg_x").join("ffmpeg-9.0")).unwrap();
    let pattern = packages
        .join("Gyan.FFmpeg*")
        .join("ffmpeg-*")
        .join("bin")
        .join("ffmpeg.exe");
    let found = glob(&pattern.to_string_lossy());
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[1].contains("ffmpeg-8.0"), "{found:?}");
    assert!(matches("ffmpeg-*", "ffmpeg-") && !matches("ffmpeg-*", "ffprobe"));
}

#[cfg(windows)]
#[test]
fn expand_percent_loest_nur_einzelne_eintraege_auf() {
    let lookup = |name: &str| {
        if name == "K3C_TEST_DIR" {
            r"C:\Tools".to_owned()
        } else {
            String::new()
        }
    };
    let expand = |s: &str| expand_percent(s, &lookup);
    assert_eq!(expand(r"%K3C_TEST_DIR%\ffmpeg\bin"), r"C:\Tools\ffmpeg\bin");
    assert_eq!(expand(r"C:\a%b"), r"C:\a%b");
    assert_eq!(expand(r"C:\cost$5\bin"), r"C:\cost$5\bin");
    // Das `;` trennt vor dem Auflösen: ein einzelnes `%` wandert nicht in den Nachbareintrag.
    for dir in r"C:\a%b;C:\c%d".split(';') {
        assert_eq!(expand(dir), dir);
    }
}

/// Ohne Go-Vorbild: der Lese-Aufruf der Registry (eigener `unsafe`-Code). Der PATH des Systems nennt immer System32;
/// nichts davon wird ausgegeben.
#[cfg(windows)]
#[test]
fn registry_path_ist_lesbar() {
    let dirs = registry::path_dirs();
    let system = dirs.iter().any(|d| d.to_ascii_lowercase().contains("system32"));
    assert!(system, "{} Einträge ohne System32", dirs.len());
}
