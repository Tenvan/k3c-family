//! Ablage der Specs (Port von `basis.go`; paper-spec.md › Dateien und Ablage): Assets listen und finden, Git-Basis.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::LazyLock;

use k3c_kit::proc::Cmd;
use regex::Regex;
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::Error;
use crate::review;

/// Ordner der Specs relativ zum Checkout.
pub const GRAFIK: &str = "public/grafik";
pub(super) const ENDUNG: &str = ".paper.json";
pub(super) const INDEX_NAME: &str = "index.paper.json";
/// Die Stil-Datei relativ zum Checkout.
pub const STIL_PFAD: &str = "public/grafik/stil.paper.json";

/// HEAD, HEAD~1, origin/main oder Commit-SHA mit 7–40 Hex-Zeichen (paper-mcp.md › 1).
static RE_BASIS: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new("^(HEAD|HEAD~1|origin/main|[0-9a-fA-F]{7,40})$").ok());

/// Lehnt eine unbekannte Basis mit den erlaubten Werten ab (Vertragstext aus `review`).
pub fn check_basis(basis: &str) -> Result<(), Error> {
    if RE_BASIS.as_ref().is_some_and(|re| re.is_match(basis)) {
        return Ok(());
    }
    Err(review::fehler_basis(basis).into())
}

/// Eine Spec unter `public/grafik/<gruppe>/<id>.paper.json`; `pfad` relativ zum Checkout mit `/`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Asset {
    pub id: String,
    pub rig: String,
    pub gruppe: String,
    pub pfad: String,
}

impl Asset {
    /// Absoluter Pfad der Spec.
    pub fn abs(&self, root: &Path) -> PathBuf {
        let mut p = root.to_path_buf();
        p.extend(self.pfad.split('/'));
        p
    }

    /// Absoluter Ausgabe-Ordner `<gruppe>/<id>/`.
    pub fn ausgabe(&self, root: &Path) -> PathBuf {
        gruppen_ordner(root, &self.gruppe).join(&self.id)
    }
}

pub(super) fn gruppen_ordner(root: &Path, gruppe: &str) -> PathBuf {
    root.join("public").join("grafik").join(gruppe)
}

/// Namen im Ordner, sortiert; ein fehlender Ordner ist leer (wie `filepath.Glob`).
fn namen(dir: &Path) -> Vec<String> {
    let Ok(eintraege) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<String> = eintraege
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .collect();
    out.sort();
    out
}

fn rig_von(spec: &Path) -> Option<String> {
    let raw = fs::read(spec).ok()?;
    let wert: Value = serde_json::from_str(&String::from_utf8_lossy(&raw)).ok()?;
    let rig = wert.get("rig")?.as_str()?;
    (!rig.is_empty()).then(|| rig.to_owned())
}

/// Alle Specs, sortiert nach ID; Gruppen-Indizes und die Stil-Datei zählen nicht.
pub fn assets(root: &Path) -> Result<Vec<Asset>, Error> {
    let grafik = root.join("public").join("grafik");
    let mut out = Vec::new();
    for gruppe in namen(&grafik) {
        for name in namen(&grafik.join(&gruppe)) {
            let Some(id) = name.strip_suffix(ENDUNG).filter(|_| name != INDEX_NAME) else {
                continue;
            };
            let rig = rig_von(&grafik.join(&gruppe).join(&name));
            out.push(Asset {
                id: id.to_owned(),
                rig: rig.unwrap_or_else(|| "?".to_owned()),
                gruppe: gruppe.clone(),
                pfad: format!("{GRAFIK}/{gruppe}/{name}"),
            });
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

/// Die ID ohne Gruppe kommt in mehreren Gruppen vor; `treffer` sind `<gruppe>/<id>`.
#[derive(Debug, Clone, PartialEq)]
pub struct MehrdeutigFehler {
    pub id: String,
    pub treffer: Vec<String>,
}

impl fmt::Display for MehrdeutigFehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mehrdeutig: {} (mit <gruppe>/<id> wählen)", self.treffer.join(", "))
    }
}

/// Sucht ein Asset über `<gruppe>/<id>` oder die bloße ID. Eine bloße ID in mehreren Gruppen wird mit der Trefferliste
/// abgelehnt, eine unbekannte Gruppe mit den vorhandenen Gruppen.
pub fn finde(root: &Path, id: &str) -> Result<Asset, Error> {
    let liste = assets(root)?;
    let (gruppe, kurz) = match id.split_once('/') {
        Some((g, k)) => (Some(g), k),
        None => (None, id),
    };
    let gruppen: BTreeSet<&str> = liste.iter().map(|a| a.gruppe.as_str()).collect();
    if let Some(g) = gruppe
        && !gruppen.contains(g)
    {
        let vorhanden: Vec<&str> = gruppen.into_iter().collect();
        return Err(Error::Sonst(format!(
            "Gruppe „{g}“ unbekannt, vorhanden: {}",
            vorhanden.join(", ")
        )));
    }
    let mut treffer: Vec<&Asset> = liste
        .iter()
        .filter(|a| a.id == kurz && gruppe.is_none_or(|g| a.gruppe == g))
        .collect();
    match treffer.len() {
        0 => Err(Error::Sonst(format!("Asset „{id}“ fehlt"))),
        1 => Ok(treffer.remove(0).clone()),
        _ => Err(Error::Mehrdeutig(MehrdeutigFehler {
            id: id.to_owned(),
            treffer: treffer.iter().map(|a| format!("{}/{}", a.gruppe, a.id)).collect(),
        })),
    }
}

/// Die IDs, deren Spec oder Ausgabe gegen HEAD geändert ist (auch unversioniert oder gelöscht).
pub fn geaendert(root: &Path) -> Result<BTreeMap<String, bool>, Error> {
    let out = git_status(root)?;
    let mut ids = BTreeMap::new();
    for zeile in String::from_utf8_lossy(&out).split('\n') {
        let Some(rest) = zeile.get(3..).filter(|r| !r.is_empty()) else {
            continue;
        };
        for pfad in rest.split(" -> ") {
            if let Some(id) = asset_von_pfad(pfad.trim_matches('"')) {
                ids.insert(id.to_owned(), true);
            }
        }
    }
    Ok(ids)
}

/// `public/grafik/<gruppe>/<id>.paper.json` oder `public/grafik/<gruppe>/<id>/…` → id.
fn asset_von_pfad(p: &str) -> Option<&str> {
    let rest = p.strip_prefix(GRAFIK)?.strip_prefix('/')?;
    let teile: Vec<&str> = rest.split('/').collect();
    let id = match teile.as_slice() {
        [_] | [] => return None,
        [_, datei] if *datei == INDEX_NAME => return None,
        [_, datei] => datei.strip_suffix(ENDUNG)?,
        [_, id, ..] => id,
    };
    (!id.is_empty()).then_some(id)
}

async fn lies(pipe: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    if let Some(mut pipe) = pipe {
        pipe.read_to_end(&mut out).await?;
    }
    Ok(out)
}

/// `git status` der Grafik im Checkout `root`, ohne Shell und höchstens [`review::ZEITLIMIT`] lang.
fn git_status(root: &Path) -> Result<Vec<u8>, Error> {
    let fehler = |text: &str| Error::Sonst(format!("git status: {text}"));
    let args: Vec<OsString> = [
        "git",
        "-c",
        "core.quotePath=false",
        "status",
        "--porcelain",
        "-uall",
        "--",
        GRAFIK,
    ]
    .map(OsString::from)
    .to_vec();
    let lauf = async {
        let mut cmd = Cmd::new(&args);
        let command = cmd.command_mut();
        command.current_dir(root).stdout(Stdio::piped()).stderr(Stdio::piped());
        // Geerbte Variablen (etwa aus einem Git-Hook) lenkten git vom Checkout weg.
        for name in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR"] {
            command.env_remove(name);
        }
        let mut proc = cmd.start()?;
        let child = proc.child_mut();
        let (stdout, stderr) = (child.stdout.take(), child.stderr.take());
        let alles = async { tokio::join!(proc.wait(), lies(stdout), lies(stderr)) };
        let (status, stdout, stderr) = tokio::time::timeout(review::ZEITLIMIT, alles).await?;
        Ok::<_, io::Error>((status?, stdout?, stderr?))
    };
    // Eigener Thread mit eigener Laufzeit, damit auch Aufrufer aus `spawn_blocking` ihn nutzen können.
    let ergebnis = std::thread::scope(|scope| {
        let thread = scope.spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?
                .block_on(lauf)
        });
        thread
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("Prozesslauf abgebrochen")))
    });
    let (status, stdout, stderr) = ergebnis.map_err(|e| fehler(&e.to_string()))?;
    if !status.success() {
        return Err(fehler(String::from_utf8_lossy(&stderr).trim()));
    }
    Ok(stdout)
}

#[cfg(test)]
#[path = "basis_gruppe_tests.rs"]
mod gruppe_tests;
#[cfg(test)]
#[path = "basis_tests.rs"]
mod tests;
