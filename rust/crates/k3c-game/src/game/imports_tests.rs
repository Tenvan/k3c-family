//! Port von `native/k3c-game/game/imports_test.go`: die Importregel (B-792, `view/mod.rs`).
//!
//! Fachmodule (`world`, `props`, `actors`, `fx`, `devtools`, `hud`, `input`, `menu`, `audio`) nutzen nur `view`, `art`,
//! das eigene Modul, `k3c_sim` (lesend), `k3c_data`, `k3c_platform` und fremde Crates; nie einander, nie `game`,
//! `replay`, `still` oder `demo`. `view` kennt nur sich, `k3c_sim` und `k3c_platform` (Canvas ist das Gegenstück zu
//! `ebiten.Image`; Go-Spezifikation „nur sim“ meinte Ebitengine mit). `*_tests.rs` dürfen zusätzlich `k3c_room` und
//! `k3c_level` nutzen (Welten für Testfälle). Statt `go/parser` prüft ein Text-Scan alle Pfade `crate::<modul>`,
//! `crate::{…}`, `super::super` und `k3c_<crate>` im Quelltext ohne Kommentare und Zeichenketten.

use std::fs;
use std::path::{Path, PathBuf};

/// Die Fachmodule laut B-792; jedes hat einen Eintrag in `packages()`.
pub const FACHMODULE: [&str; 9] = [
    "actors", "audio", "devtools", "fx", "hud", "input", "menu", "props", "world",
];

/// Ordner von `src/` mit eigener Regel: Zusammenbau und Werkzeuge (dürfen alles), `art` (eigene Grafik-Schicht);
/// `runs` (Läufe im Client, RU33) ist ein Werkzeug wie `replay`; `devwin` (Cheat-Fenster, RU56) ist ein Werkzeug und nutzt `k3c_devproto` und die Dev-Typen aus `view`.
const NOT_FACH: [&str; 8] = ["game", "replay", "still", "demo", "art", "local", "devwin", "runs"];

/// `k3c_det` ist die Mathematik (Gegenstück zu Gos `math`), keine Fachschicht, und gilt wie die Standardbibliothek.
const ALLOWED_CRATES: [&str; 4] = ["k3c_sim", "k3c_data", "k3c_platform", "k3c_det"];
const TEST_CRATES: [&str; 2] = ["k3c_room", "k3c_level"];

/// Bekannte Verstöße (Dateipfad oder Ordner-Präfix, Ziel), die ein Ticket führt: Der Test von `actors` baut Rigs
/// direkt mit `k3c_rig` (Go: über `art`); `hud` liest `props::simview` und `props::pending` (der gemeinsame `SimView`
/// liegt in `props`, Go: in `view`); `props` nutzt `actors` (Figuren-Maße für Plan und Malen); ein Test von `hud` lädt Spuren aus `replay::trace`, einer von `menu` zeichnet über `still`. Neue Ausnahmen gibt es nicht;
/// ein Eintrag fällt weg, sobald der Verstoß behoben ist.
const KNOWN: [(&str, &str); 5] = [
    ("actors/draw_tests.rs", "k3c_rig"),
    ("props/", "actors"),
    ("hud/", "props"),
    ("hud/draw_tests.rs", "replay"),
    ("menu/image_tests.rs", "still"),
];

fn known(path: &str, target: &str) -> bool {
    KNOWN.iter().any(|(p, t)| path.starts_with(p) && *t == target)
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Entfernt `//`-Kommentare und Zeichenketten (durch Leerzeichen ersetzt, Zeilen bleiben).
fn strip(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let (mut in_str, mut esc, mut prev) = (false, false, ' ');
        for c in line.chars() {
            if in_str {
                match (esc, c) {
                    (true, _) => esc = false,
                    (false, '\\') => esc = true,
                    (false, '"') => in_str = false,
                    _ => {}
                }
                out.push(' ');
            } else if c == '"' {
                in_str = true;
                out.push(' ');
            } else if c == '/' && prev == '/' {
                out.pop();
                break;
            } else {
                out.push(c);
                prev = c;
            }
        }
        out.push('\n');
    }
    out
}

/// Das erste Wort von `s` (Bezeichner).
fn head(s: &str) -> &str {
    let end = s.find(|c: char| !is_ident(c)).unwrap_or(s.len());
    &s[..end]
}

/// Module hinter `crate::`: `crate::x::…` oder die obersten Einträge von `crate::{x, y::z}`.
fn crate_modules(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (at, _) in text.match_indices("crate::") {
        if text[..at].chars().next_back().is_some_and(is_ident) {
            continue; // z. B. `k3c_crate::`
        }
        let rest = &text[at + "crate::".len()..];
        if let Some(group) = rest.strip_prefix('{') {
            let (mut depth, mut start) = (0, 0);
            for (i, c) in group.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' if depth == 0 => {
                        out.push(head(group[start..i].trim()).to_string());
                        break;
                    }
                    '}' => depth -= 1,
                    ',' if depth == 0 => {
                        out.push(head(group[start..i].trim()).to_string());
                        start = i + 1;
                    }
                    _ => {}
                }
            }
        } else {
            out.push(head(rest).to_string());
        }
    }
    out.retain(|m| !m.is_empty());
    out
}

/// Fremde `k3c_*`-Crates im Text.
fn k3c_crates(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (at, _) in text.match_indices("k3c_") {
        if text[..at].chars().next_back().is_some_and(is_ident) {
            continue;
        }
        out.push(head(&text[at..]).to_string());
    }
    out
}

/// Meldet, ob das Modul `pkg` (Ordnername) den Pfad `m` aus `crate::m` nutzen darf.
fn module_allowed(pkg: &str, m: &str) -> bool {
    m == pkg || (pkg != "view" && (m == "view" || m == "art"))
}

fn crate_allowed(pkg: &str, c: &str, test: bool) -> bool {
    let base = ALLOWED_CRATES.contains(&c);
    let view_ok = pkg != "view" || c == "k3c_sim" || c == "k3c_platform";
    (base && view_ok) || (test && TEST_CRATES.contains(&c))
}

/// Die verbotenen Importe des Moduls `pkg` aus den Quellen `files` (Dateiname, Inhalt), sortiert.
pub fn violations(pkg: &str, files: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    for (name, src) in files {
        let test = name.ends_with("_tests.rs");
        let text = strip(src);
        for m in crate_modules(&text) {
            if !module_allowed(pkg, &m) && !known(&format!("{pkg}/{name}"), &m) {
                out.push(format!("{pkg}/{name} importiert crate::{m}"));
            }
        }
        if !test && text.contains("super::super") {
            out.push(format!("{pkg}/{name} nutzt super::super"));
        }
        for c in k3c_crates(&text) {
            if !known(&format!("{pkg}/{name}"), &c) && !crate_allowed(pkg, &c, test) {
                out.push(format!("{pkg}/{name} importiert {c}"));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Alle `.rs`-Dateien unter `dir` (Name relativ zu `dir`, Inhalt).
fn read_module(dir: &Path, rel: &str, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let (p, n) = (e.path(), e.file_name().to_string_lossy().to_string());
        let name = if rel.is_empty() { n } else { format!("{rel}/{n}") };
        if p.is_dir() {
            read_module(&p, &name, out);
        } else if p.extension().is_some_and(|x| x == "rs")
            && let Ok(src) = fs::read_to_string(&p)
        {
            out.push((name, src));
        }
    }
}

/// Die Ordner von `src/`, für die eine Regel gilt: alle Fachmodule und `view`; ein neues Modul fällt von selbst darunter.
fn checked_modules() -> Vec<String> {
    let mut out: Vec<String> = fs::read_dir(src_dir())
        .map(|d| d.flatten().filter(|e| e.path().is_dir()).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| !NOT_FACH.contains(&n.as_str()))
        .collect();
    out.sort();
    out
}

#[test]
fn import_rule() {
    let pkgs = checked_modules();
    for want in FACHMODULE.iter().chain(&["view"]) {
        assert!(pkgs.iter().any(|p| p == want), "Modul {want} fehlt");
    }
    let mut all = Vec::new();
    for pkg in &pkgs {
        let mut files = Vec::new();
        read_module(&src_dir().join(pkg), "", &mut files);
        assert!(!files.is_empty(), "{pkg}: keine Quelldateien gelesen");
        all.extend(violations(pkg, &files));
    }
    assert!(all.is_empty(), "🚫 Importregel: {}", all.join("; "));
}

fn source(lines: &[&str]) -> Vec<(String, String)> {
    vec![("x.rs".to_string(), lines.join("\n"))]
}

/// Der Test schlägt bei einem künstlichen Querimport an: verbotene Importe werden gemeldet, erlaubte nicht.
#[test]
fn import_rule_catches_cross_import() {
    let forbidden = [
        "use crate::actors::Actors;",
        "use crate::game::Part;",
        "use crate::replay::blend;",
        "use crate::{view::frame::Frame, hud::Hud};",
        "use crate::{\n    view::cell::Cell,\n    fx::Fx,\n};",
        "use k3c_room::Room;",
        "use k3c_rig::Rig;",
        "let x = k3c_level::build();",
    ];
    for src in forbidden {
        let bad = violations("world", &source(&[src]));
        assert_eq!(bad.len(), 1, "Querimport nicht erkannt: {src}: {bad:?}");
    }
    let ok = [
        "use std::rc::Rc;",
        "use serde_json::json;",
        "use crate::view::cell::Cell;",
        "use crate::art::Library;",
        "use crate::world::camera::Cameras;",
        "use crate::{art, view};",
        "use k3c_sim::state::World;",
        "use k3c_data::paper;",
        "use k3c_platform::Canvas;",
        "use super::cell::Cell; // crate::hud steht im Kommentar",
        "let s = \"crate::hud und k3c_room im Text\";",
    ];
    for src in ok {
        let bad = violations("world", &source(&[src]));
        assert!(bad.is_empty(), "erlaubter Import gemeldet: {src}: {bad:?}");
    }
}

/// `view` und Tests haben engere bzw. weitere Regeln.
#[test]
fn import_rule_view_and_tests() {
    assert_eq!(
        violations("view", &source(&["use crate::art::Library;"])).len(),
        1,
        "view kennt art nicht"
    );
    assert!(
        violations(
            "view",
            &source(&["use k3c_platform::Canvas;", "use k3c_sim::state::World;"])
        )
        .is_empty()
    );
    assert_eq!(
        violations("view", &source(&["use k3c_data::x;"])).len(),
        1,
        "view kennt k3c_data nicht"
    );
    assert_eq!(
        violations("world", &source(&["use super::super::x;"])).len(),
        1,
        "super::super im Quelltext"
    );
    let tests = vec![(
        "x_tests.rs".to_string(),
        "use k3c_level::build;\nuse super::super::x;".to_string(),
    )];
    assert!(
        violations("world", &tests).is_empty(),
        "Test darf k3c_level und super::super nutzen"
    );
}
