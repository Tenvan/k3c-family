//! Jede Log-Meldung der Rust-Crates beginnt mit einem Emoji aus `k3c_platform::log::EMOJIS` (Liste aus `CLAUDE.md` ›
//! Logging): geprüft werden alle Aufrufe `log::debug("…`, `log::info("…`, `log::warn("…`, `log::error("…` in
//! `rust/crates/*/src/**` und `examples/**`.

use std::fs;
use std::path::{Path, PathBuf};

use k3c_platform::log::{EMOJIS, starts_with_emoji};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Erste Zeichenkette nach jedem `log::<stufe>(` in `text` (nur Literale, die direkt folgen).
fn messages(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for lvl in ["debug", "info", "warn", "error"] {
        let pat = format!("log::{lvl}(");
        for (i, _) in text.match_indices(&pat) {
            let rest = text[i + pat.len()..].trim_start();
            if let Some(lit) = rest.strip_prefix('"') {
                out.push(lit.split('"').next().unwrap_or_default().to_string());
            }
        }
    }
    out
}

#[test]
fn jede_log_meldung_beginnt_mit_emoji_aus_der_liste() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut files = Vec::new();
    for c in fs::read_dir(&crates).unwrap().flatten() {
        rust_files(&c.path().join("src"), &mut files);
        rust_files(&c.path().join("examples"), &mut files);
    }
    let mut checked = 0;
    let mut bad = Vec::new();
    for f in &files {
        let text = fs::read_to_string(f).unwrap();
        for m in messages(&text) {
            checked += 1;
            if !starts_with_emoji(&m) {
                bad.push(format!("{}: {m:?}", f.display()));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "Meldungen ohne Emoji aus {EMOJIS:?}:\n{}",
        bad.join("\n")
    );
    assert!(checked >= 5, "zu wenige Meldungen gefunden: {checked}");
}

#[test]
fn erkennung_trennt_gute_und_schlechte_meldungen() {
    let src = r#"log::warn("🤒 Gerät weg", &[]); log::info( "Start ohne Emoji", &[]); log::error("⚠\u{FE0F} x", &[]);"#;
    let m = messages(src);
    assert_eq!(m.len(), 3);
    assert_eq!(m.iter().filter(|m| starts_with_emoji(m)).count(), 1, "{m:?}");
    assert!(
        !starts_with_emoji("⚠\u{FE0F} x"),
        "kein U+FE0F, kein Emoji außerhalb der Liste"
    );
    assert_eq!(EMOJIS.len(), 30);
}
