//! Golden der Planung (`testdata/planung/`, README dort): je Fall `vorher/` unter `.work/` aufbauen, ein Werkzeug
//! aufrufen, Antwort und Dateibaum wie der Go-Läufer (`golden_test.go`, `golden_lader_test.go`) vergleichen.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use k3c_workbench::planning::{self, Filter};
use serde::Deserialize;
use serde_json::Value;

/// Werkzeuge, die der Port schon kann; RU69.11 ergänzt.
const AKTIV: &[&str] = &[
    "plan_get",
    "plan_list",
    "plan_set",
    "plan_section",
    "plan_sections",
    "plan_create",
    "plan_delete",
    "plan_index",
];
const FAELLE_GESAMT: usize = 174;
const FAELLE_AKTIV: usize = 125;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Neuaufbau {
    dry: Option<bool>,
}

#[derive(Deserialize)]
struct Aufruf {
    werkzeug: String,
    #[serde(default)]
    args: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Id {
    id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Felder {
    id: String,
    fields: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Abschnitt {
    id: String,
    section: String,
    text: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Abschnitte {
    id: String,
    sections: BTreeMap<String, String>,
}

fn repo() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4);
    root.expect("Repo-Wurzel").to_owned()
}

fn golden_dir() -> PathBuf {
    repo().join("testdata").join("planung")
}

fn temp_root() -> tempfile::TempDir {
    let base = std::env::var_os("K3C_TMP").filter(|v| !v.is_empty()).map_or_else(
        || repo().join(".work").join("tmp"),
        |v| PathBuf::from(v).components().collect(),
    );
    fs::create_dir_all(&base).expect("Temp-Basis anlegen");
    tempfile::Builder::new()
        .prefix("golden-")
        .tempdir_in(base)
        .expect("Temp-Ordner anlegen")
}

/// Fall-Ordner (ohne die mit `_` beginnenden Hilfsordner), nach Name, mit ihrem Aufruf.
fn faelle() -> Vec<(String, Aufruf)> {
    let mut namen: Vec<String> = fs::read_dir(golden_dir())
        .expect("testdata/planung lesbar")
        .map(|e| e.expect("Eintrag"))
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('_'))
        .collect();
    namen.sort();
    namen
        .into_iter()
        .map(|name| {
            let text = fs::read_to_string(golden_dir().join(&name).join("aufruf.json")).expect("aufruf.json");
            let aufruf: Aufruf = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{name}: aufruf.json: {e}"));
            assert!(!aufruf.werkzeug.is_empty(), "{name}: werkzeug fehlt");
            (name, aufruf)
        })
        .collect()
}

/// Alle Dateien unter `dir` als Pfad ab `dir` (mit `/`).
fn dateien(dir: &Path) -> BTreeMap<String, PathBuf> {
    let mut out = BTreeMap::new();
    let mut offen = vec![dir.to_owned()];
    while let Some(ordner) = offen.pop() {
        let Ok(eintraege) = fs::read_dir(&ordner) else {
            continue;
        };
        for eintrag in eintraege {
            let pfad = eintrag.expect("Eintrag").path();
            if pfad.is_dir() {
                offen.push(pfad);
            } else {
                let rel = pfad.strip_prefix(dir).expect("unter dir");
                out.insert(rel.to_string_lossy().replace('\\', "/"), pfad);
            }
        }
    }
    out
}

fn kopiere_baum(von: &Path, nach: &Path) {
    for (rel, quelle) in dateien(von) {
        let ziel = nach.join(rel);
        fs::create_dir_all(ziel.parent().expect("Elternordner")).expect("Ordner anlegen");
        fs::copy(quelle, ziel).expect("Datei kopieren");
    }
}

/// `vorher/` plus die festen Vorlagen; Git kennt keine leeren Ordner, darum entstehen die Sprint-Ordner hier.
fn baue_vorher(root: &Path, fall: &str) {
    kopiere_baum(&golden_dir().join(fall).join("vorher"), root);
    kopiere_baum(&golden_dir().join("_vorlagen"), &root.join("docs").join("vorlagen"));
    for state in planning::store::STATES {
        fs::create_dir_all(root.join("docs").join("sprints").join(state)).expect("Sprint-Ordner");
    }
}

/// LF statt CRLF, das heutige Datum der Zeile „Erstellt“ und der Pfad der Wurzel durch Platzhalter.
fn maskiere(text: &str, root: &Path) -> String {
    let wurzel = root.to_string_lossy().into_owned();
    let heute = chrono::Local::now().format("%Y-%m-%d");
    text.replace("\r\n", "\n")
        .replace(&wurzel, "<WURZEL>")
        .replace(&wurzel.replace('\\', "/"), "<WURZEL>")
        .replace(&format!("**Erstellt:** {heute}"), "**Erstellt:** <HEUTE>")
}

/// Alle Dateien unter `dir` als Pfad → maskierter Text; docs/vorlagen/ gehört nicht zum Vergleich.
fn lese_baum(dir: &Path, root: &Path) -> BTreeMap<String, String> {
    dateien(dir)
        .into_iter()
        .filter(|(rel, _)| !rel.starts_with("docs/vorlagen/"))
        .map(|(rel, pfad)| {
            let bytes = fs::read(pfad).expect("Datei lesen");
            (rel, maskiere(&String::from_utf8_lossy(&bytes), root))
        })
        .collect()
}

/// Die erste Zeile, in der zwei Texte sich unterscheiden.
fn erste_abweichung(soll: &str, ist: &str) -> String {
    let (w, g): (Vec<&str>, Vec<&str>) = (soll.split('\n').collect(), ist.split('\n').collect());
    (0..w.len().max(g.len())).find(|&i| w.get(i) != g.get(i)).map_or_else(
        || "gleich".to_owned(),
        |i| {
            let zeile = |v: &[&str]| v.get(i).copied().unwrap_or_default().to_owned();
            format!(
                "Zeile {}:\n  erwartet: {:?}\n  erhalten: {:?}",
                i + 1,
                zeile(&w),
                zeile(&g)
            )
        },
    )
}

/// Ruft das Werkzeug eines Falls auf der Wurzel auf; Argumente streng wie im Go-Läufer.
fn fuehre_aus(root: &Path, aufruf: &Aufruf) -> Result<String, String> {
    let args = || match &aufruf.args {
        Value::Null => Value::Object(serde_json::Map::new()),
        other => other.clone(),
    };
    let falsch = |e: serde_json::Error| format!("aufruf.json args: {e}");
    match aufruf.werkzeug.as_str() {
        "plan_get" => {
            let a: Id = serde_json::from_value(args()).map_err(falsch)?;
            planning::get(root, &a.id)
        }
        "plan_list" => {
            let a: Filter = serde_json::from_value(args()).map_err(falsch)?;
            planning::list(root, &a)
        }
        "plan_set" => {
            let a: Felder = serde_json::from_value(args()).map_err(falsch)?;
            planning::set(root, &a.id, &a.fields)
        }
        "plan_section" => {
            let a: Abschnitt = serde_json::from_value(args()).map_err(falsch)?;
            planning::section(root, &a.id, &a.section, &a.text)
        }
        "plan_sections" => {
            let a: Abschnitte = serde_json::from_value(args()).map_err(falsch)?;
            planning::sections(root, &a.id, &a.sections)
        }
        "plan_create" => {
            let a: planning::NewDoc = serde_json::from_value(args()).map_err(falsch)?;
            planning::create(root, &a)
        }
        "plan_delete" => {
            let a: Id = serde_json::from_value(args()).map_err(falsch)?;
            planning::delete(root, &a.id)
        }
        "plan_index" => {
            let a: Neuaufbau = serde_json::from_value(args()).map_err(falsch)?;
            // dry ist wie im Tool standardmäßig an.
            planning::rebuild_index(root, a.dry.unwrap_or(true))
        }
        other => Err(format!("aufruf.json: unbekanntes werkzeug {other:?}")),
    }
}

/// Erste Abweichung eines Falls (Antwort, dann Dateibaum).
fn pruefe(fall: &str, aufruf: &Aufruf) -> Option<String> {
    let tmp = temp_root();
    let root = tmp.path();
    baue_vorher(root, fall);
    let antwort = match fuehre_aus(root, aufruf) {
        Ok(text) => maskiere(&text, root),
        Err(e) => format!("FEHLER: {}", maskiere(&e, root)),
    };
    let soll = fs::read_to_string(golden_dir().join(fall).join("antwort.txt")).expect("antwort.txt");
    let soll = soll.replace("\r\n", "\n");
    if soll != antwort {
        return Some(format!(
            "{fall}: antwort.txt weicht ab, {}",
            erste_abweichung(&soll, &antwort)
        ));
    }
    let ist = lese_baum(root, root);
    let soll = lese_baum(&golden_dir().join(fall).join("nachher"), root);
    for (rel, w) in &soll {
        match ist.get(rel) {
            None => return Some(format!("{fall}: Datei fehlt: {rel}")),
            Some(g) if g != w => {
                return Some(format!("{fall}: Datei {rel} weicht ab, {}", erste_abweichung(w, g)));
            }
            Some(_) => {}
        }
    }
    ist.keys()
        .find(|rel| !soll.contains_key(*rel))
        .map(|rel| format!("{fall}: Datei zu viel: {rel}"))
}

#[test]
fn golden_der_aktiven_werkzeuge() {
    let aktiv: Vec<_> = faelle()
        .into_iter()
        .filter(|(_, a)| AKTIV.contains(&a.werkzeug.as_str()))
        .collect();
    assert_eq!(aktiv.len(), FAELLE_AKTIV, "Zahl der Fälle von {AKTIV:?}");
    let fehler: Vec<String> = aktiv.iter().filter_map(|(fall, a)| pruefe(fall, a)).collect();
    assert!(
        fehler.is_empty(),
        "{} von {} Fällen weichen ab:\n{}",
        fehler.len(),
        aktiv.len(),
        fehler.join("\n")
    );
}

#[test]
fn golden_zaehlt_uebersprungene_faelle() {
    let alle = faelle();
    let uebersprungen = alle
        .iter()
        .filter(|(_, a)| !AKTIV.contains(&a.werkzeug.as_str()))
        .count();
    // Direkt auf stderr: die Ausgabe erscheint auch ohne `--nocapture`.
    writeln!(
        std::io::stderr(),
        "planung_golden: {} Fälle, {} ausgeführt, {uebersprungen} übersprungen",
        alle.len(),
        alle.len() - uebersprungen
    )
    .expect("stderr");
    assert_eq!(alle.len(), FAELLE_GESAMT);
    assert_eq!(uebersprungen, FAELLE_GESAMT - FAELLE_AKTIV);
}
