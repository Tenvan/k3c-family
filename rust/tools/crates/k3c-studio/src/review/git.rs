//! Specs, Manifeste und Stil-Datei aus dem Commit und aus dem Arbeitsstand. git läuft ohne Shell, Pfade stehen hinter
//! `--`, Revisionen dürfen nicht mit `-` beginnen.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

use regex::Regex;

use super::aufruf::{Ausgabe, Lauf, LaufFehler, ausfuehren};
use super::cli::Error;

/// Ablage der Specs (paper-spec.md › Dateien): `public/grafik/<gruppe>/<id>.paper.json`; Index und Stil sind keine.
pub(super) const GRAFIK: &str = "public/grafik";
pub(super) const STIL_PFAD: &str = "public/grafik/stil.paper.json";
pub(super) const ENDUNG: &str = ".paper.json";
const GIT_ZEIT: Duration = Duration::from_secs(30);

/// Dateien eines Stands: Pfad (relativ zum Checkout, mit `/`) → Inhalt.
pub type Dateien = BTreeMap<String, Vec<u8>>;

fn lauf(root: &Path, args: &[&str], stdin: Vec<u8>, zeit: Duration) -> Result<Ausgabe, Error> {
    let name = args.first().copied().unwrap_or_default();
    // `cat-file --batch` wie Go ohne die `-c`-Schalter und ohne eigene Umgebung.
    let batch = !stdin.is_empty();
    let mut alle: Vec<OsString> = vec!["git".into()];
    if !batch {
        alle.extend(["-c", "core.autocrlf=false", "-c", "core.quotepath=false"].map(OsString::from));
    }
    alle.extend(args.iter().map(OsString::from));
    let env: &[(&str, &str)] = &[("GIT_TERMINAL_PROMPT", "0"), ("GCM_INTERACTIVE", "never")];
    let lauf = Lauf {
        args: alle,
        dir: root,
        env: if batch { &[] } else { env },
        stdin,
        zeit,
    };
    ausfuehren(lauf).map_err(|e| match e {
        LaufFehler::Zeit => Error::Sonst(format!("git {name}: Zeitlimit überschritten")),
        LaufFehler::System(e) => Error::Sonst(format!("git {name}: {e}")),
    })
}

fn gescheitert(args: &[&str], a: &Ausgabe) -> Error {
    Error::Sonst(format!(
        "git {}: {}: {}",
        args.first().copied().unwrap_or_default(),
        a.status,
        String::from_utf8_lossy(&a.stderr).trim()
    ))
}

/// Führt git im Repo aus und liefert stdout; ein Fehler trägt stderr.
pub(super) fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, Error> {
    git_mit_zeit(root, args, GIT_ZEIT)
}

pub(super) fn git_mit_zeit(root: &Path, args: &[&str], zeit: Duration) -> Result<Vec<u8>, Error> {
    let a = lauf(root, args, Vec::new(), zeit)?;
    if !a.status.success() {
        return Err(gescheitert(args, &a));
    }
    Ok(a.stdout)
}

/// Eine Revision, die git als Schalter läse, wird abgelehnt.
fn pruefe_commit(commit: &str) -> Result<(), Error> {
    if commit.is_empty() || commit.starts_with('-') {
        return Err(Error::Sonst(format!("Commit {commit:?} ungültig")));
    }
    Ok(())
}

/// Gruppe bzw. Dateiname ohne Pfadteile (kein `..`, kein Backslash, kein Laufwerk).
static RE_SEGMENT: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9._-]*$").ok());

fn gueltiges_segment(s: &str) -> bool {
    RE_SEGMENT.as_ref().is_some_and(|re| re.is_match(s)) && !s.contains("..")
}

/// Genau `public/grafik/<gruppe>/<id>.paper.json`, ohne Gruppen-Index.
pub(super) fn ist_spec(pfad: &str) -> bool {
    let teile: Vec<&str> = pfad.split('/').collect();
    let ["public", "grafik", gruppe, datei] = teile.as_slice() else {
        return false;
    };
    let Some(id) = datei.strip_suffix(ENDUNG) else {
        return false;
    };
    id != "index" && gueltiges_segment(gruppe) && gueltiges_segment(id)
}

/// `public/grafik/<gruppe>/<id>/manifest.json` (Ausgabe eines Assets).
fn ist_manifest(pfad: &str) -> bool {
    let teile: Vec<&str> = pfad.split('/').collect();
    matches!(teile.as_slice(), ["public", "grafik", gruppe, id, "manifest.json"]
        if gueltiges_segment(gruppe) && gueltiges_segment(id))
}

/// Das Manifest der Ausgabe neben der Spec.
pub(super) fn manifest_zu(spec: &str) -> String {
    format!("{}/manifest.json", spec.strip_suffix(ENDUNG).unwrap_or(spec))
}

/// Liest Specs, Manifeste und Stil-Datei des Commits mit zwei git-Aufrufen (`ls-tree`, `cat-file --batch`); einzelne
/// `git show` je Datei wären unter Windows zu langsam.
pub fn basis_dateien(root: &Path, commit: &str) -> Result<Dateien, Error> {
    pruefe_commit(commit)?;
    let out = git(root, &["ls-tree", "-r", "-z", commit, "--", GRAFIK])?;
    let (mut pfade, mut shas) = (Vec::new(), Vec::new());
    for zeile in String::from_utf8_lossy(&out).split('\0') {
        let Some((kopf, pfad)) = zeile.split_once('\t') else {
            continue;
        };
        let felder: Vec<&str> = kopf.split_whitespace().collect();
        let [_, "blob", sha] = felder.as_slice() else {
            continue;
        };
        if ist_spec(pfad) || ist_manifest(pfad) || pfad == STIL_PFAD {
            pfade.push(pfad.to_owned());
            shas.push((*sha).to_owned());
        }
    }
    blobs(root, pfade, &shas)
}

/// Liest Blobs in einem Aufruf von `git cat-file --batch`.
fn blobs(root: &Path, pfade: Vec<String>, shas: &[String]) -> Result<Dateien, Error> {
    let mut dateien = Dateien::new();
    if shas.is_empty() {
        return Ok(dateien);
    }
    let args = ["cat-file", "--batch"];
    let eingabe = format!("{}\n", shas.join("\n")).into_bytes();
    let a = lauf(root, &args, eingabe, GIT_ZEIT)?;
    if !a.status.success() {
        return Err(gescheitert(&args, &a));
    }
    let mut rest = a.stdout.as_slice();
    for pfad in pfade {
        let unlesbar = |was: &str| Error::Sonst(format!("git cat-file: {was} unlesbar bei {pfad}"));
        let ende = rest.iter().position(|b| *b == b'\n');
        let ende = ende.ok_or_else(|| unlesbar("Antwort"))?;
        let kopf = String::from_utf8_lossy(&rest[..ende]).into_owned();
        let nach = &rest[ende + 1..];
        let felder: Vec<&str> = kopf.split_whitespace().collect();
        let [_, _, groesse] = felder.as_slice() else {
            return Err(unlesbar("Antwort"));
        };
        let n = groesse.parse::<usize>().ok().filter(|n| *n < nach.len());
        let n = n.ok_or_else(|| unlesbar("Größe"))?;
        dateien.insert(pfad, nach[..n].to_vec());
        rest = &nach[n + 1..];
    }
    Ok(dateien)
}

/// Spec-Pfade eines Commits (ohne Inhalt).
pub fn specs_in_commit(root: &Path, commit: &str) -> Result<Vec<String>, Error> {
    pruefe_commit(commit)?;
    let args = ["ls-tree", "-r", "-z", "--name-only", commit, "--", GRAFIK];
    let out = git(root, &args)?;
    Ok(String::from_utf8_lossy(&out)
        .split('\0')
        .filter(|p| ist_spec(p))
        .map(str::to_owned)
        .collect())
}

/// Namen im Ordner; wie Go `filepath.Glob` ohne Fehler, wenn er fehlt oder unlesbar ist.
fn namen(dir: &Path) -> Vec<String> {
    let Ok(eintraege) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    eintraege
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .collect()
}

/// Spec-Pfade im Dateisystem, auch unversionierte, sortiert.
pub fn specs_im_arbeitsstand(root: &Path) -> Result<Vec<String>, Error> {
    let basis = datei_in(root, GRAFIK);
    let mut pfade = Vec::new();
    for gruppe in namen(&basis) {
        for datei in namen(&basis.join(&gruppe)) {
            let pfad = format!("{GRAFIK}/{gruppe}/{datei}");
            if ist_spec(&pfad) {
                pfade.push(pfad);
            }
        }
    }
    pfade.sort();
    Ok(pfade)
}

/// Eine Datei aus dem Commit; fehlt sie dort, `None`.
pub fn show(root: &Path, commit: &str, pfad: &str) -> Result<Option<Vec<u8>>, Error> {
    pruefe_commit(commit)?;
    let objekt = format!("{commit}:{pfad}");
    let args = ["rev-parse", "--verify", "--quiet", &objekt];
    let a = lauf(root, &args, Vec::new(), GIT_ZEIT)?;
    // `--verify --quiet`: Exit 1 = gibt es nicht.
    if a.status.code() == Some(1) {
        return Ok(None);
    }
    if !a.status.success() {
        return Err(gescheitert(&args, &a));
    }
    git(root, &["cat-file", "blob", &objekt]).map(Some)
}

/// `base` plus ein Pfad mit `/`.
pub(super) fn datei_in(base: &Path, pfad: &str) -> PathBuf {
    let mut out = base.to_path_buf();
    out.extend(pfad.split('/'));
    out
}

/// Eine Datei relativ zum Checkout; fehlt sie, `None`.
pub(super) fn lies_arbeitsstand(root: &Path, pfad: &str) -> Result<Option<Vec<u8>>, Error> {
    match std::fs::read(datei_in(root, pfad)) {
        Ok(raw) => Ok(Some(raw)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
