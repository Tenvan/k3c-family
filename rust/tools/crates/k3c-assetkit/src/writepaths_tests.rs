//! WZ25.4 (B-511/AC-05): Jede Datei, die die Asset-Logik in `data/` oder `public/` schreibt, entsteht per Temp-Datei und
//! Umbenennen (bzw. Link). Eine halbe Datei darf nie dort liegen.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::thread;

use walkdir::WalkDir;

use crate::import::{import_dir, store_import};
use crate::register::KIND_SOUND;
use crate::testutil::tempdir;

/// Schreibwege ohne Atomarität nur für Ziele außerhalb der beobachteten Ordner (Datei, Grund).
const NON_ATOMIC_ALLOWED: &[(&str, &str)] = &[
    (
        "fsutil.rs",
        "Ersatz für os.CreateTemp: die Temp-Datei entsteht exklusiv neu",
    ),
    ("ticket/mod.rs", "Sperrdatei exklusiv (O_EXCL), kein Dateiinhalt"),
    ("windowstate.rs", "Fensterzustand unter dem Config-Ordner"),
];

const CALLS: &[&str] = &[
    "fs::write(",
    "File::create(",
    "File::create_new(",
    "File::options(",
    "OpenOptions",
];

/// Die Rust-Dateien ohne Tests, Schlüssel = Pfad ab `src/`.
fn sources() -> Vec<(String, String)> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    WalkDir::new(&src)
        .sort_by_file_name()
        .into_iter()
        .map(|entry| entry.expect("src lesen"))
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let rel = e
                .path()
                .strip_prefix(&src)
                .expect("unter src")
                .to_string_lossy()
                .replace('\\', "/");
            let code = rel.ends_with(".rs") && !rel.ends_with("_tests.rs") && rel != "testutil.rs";
            code.then(|| (rel, fs::read_to_string(e.path()).expect("Quelle")))
        })
        .collect()
}

#[test]
fn schreibwege_sind_atomar() {
    let files = sources();
    assert!(
        files.len() >= 10,
        "nur {} Dateien gelesen: Pfade stimmen nicht",
        files.len()
    );
    let allowed: HashSet<&str> = NON_ATOMIC_ALLOWED.iter().map(|(file, _)| *file).collect();
    let found: Vec<String> = files
        .iter()
        .filter(|(rel, _)| !allowed.contains(rel.as_str()))
        .flat_map(|(rel, src)| {
            let used = CALLS.iter().filter(|call| src.contains(*call));
            used.map(move |call| format!("{rel} ruft {call}"))
        })
        .collect();
    assert!(
        found.is_empty(),
        "schreibt nicht atomar (Temp-Datei und Umbenennen, write_atomic); außerhalb von data/ und public/ in \
         NON_ATOMIC_ALLOWED mit Grund eintragen: {found:?}"
    );
}

/// Der Import legt nie eine halbe Datei ab, hinterlässt keine Temp-Dateien und überschreibt keine vorhandene;
/// gleichzeitige Importe mit demselben Titel bekommen verschiedene Namen mit vollständigem Inhalt.
#[test]
fn import_schreibt_atomar() {
    let dir = tempdir();
    let root = dir.path();
    let data = |i: u8| vec![b'a' + i; 4096];
    let names: Vec<String> = thread::scope(|scope| {
        let handles: Vec<_> = (0..12u8)
            .map(|i| scope.spawn(move || store_import(root, KIND_SOUND, "Gleicher Titel", ".ogg", &data(i))))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("Thread").expect("store_import"))
            .collect()
    });
    let mut seen = HashSet::new();
    for (i, name) in (0u8..).zip(&names) {
        assert!(seen.insert(name), "Name {name} doppelt vergeben");
        let got = fs::read(root.join("public").join(name)).expect("lesen");
        assert!(
            got == data(i),
            "{name}: Inhalt unvollständig oder überschrieben ({} Bytes)",
            got.len()
        );
    }
    let entries: Vec<String> = fs::read_dir(root.join("public").join(import_dir(KIND_SOUND)))
        .expect("Ordner")
        .map(|e| e.expect("Eintrag").file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        !entries
            .iter()
            .any(|n| n.starts_with(".import-") || n.starts_with(".tmp")),
        "Temp-Datei bleibt liegen: {entries:?}"
    );
    assert_eq!(entries.len(), 12);
}
