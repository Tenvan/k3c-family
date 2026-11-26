//! Eine Spec atomar ändern (Port von `set.go`): Patch → validate → render in einen Temp-Ordner → Spec per Temp-Datei und
//! Umbenennen ersetzen → Ausgabe umbenennen. Dazu die Sperren, über die alle Schreibwege nacheinander laufen.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Condvar, LazyLock, Mutex, PoisonError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use regex::Regex;

use super::basis::{Asset, ENDUNG, assets, finde};
use super::cli::Cli;
use super::golden::gruppe_nachziehen;
use super::patch::{format as formatiere, merge_patch, schluessel};
use super::reiter::reiter_von;
use super::set_plan::pruefe_und_rendere;
use super::teile::teile_patch;
use super::{Error, FeldAenderung};
use crate::kitx::sha256_hex;

/// Der Pixel-Anteil einer Animation (Frame 0, alt gegen neu); Overlay relativ zum Checkout.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pixel {
    pub animation: String,
    pub anteil: f64,
    pub boxen: Vec<Vec<i64>>,
    pub overlay: String,
    /// Gesetzt, wenn ein Frame nicht gerendert werden konnte (z. B. Animation neu).
    pub fehler: String,
}

/// Eine angewandte Änderung.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetErgebnis {
    pub asset: Asset,
    pub felder: Vec<FeldAenderung>,
    pub animationen: Vec<String>,
    pub pngs: usize,
    pub dateien: Vec<String>,
    pub pixel: Vec<Pixel>,
    /// Assets, die dieses Asset als Reittier nutzen (Render veraltet, B-706).
    pub reiter: Vec<String>,
    /// Grund, wenn Prüfen und Rendern abbrach (nicht abgelehnt); `felder` zeigt den bekannten Stand (B-870).
    pub abbruch: String,
}

/// Gescheiterte Änderung samt dem bis dahin Bekannten (B-870).
#[derive(Debug, Clone)]
pub struct SetFehler {
    pub stand: SetErgebnis,
    pub fehler: Error,
}

impl fmt::Display for SetFehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fehler.fmt(f)
    }
}

impl std::error::Error for SetFehler {}

impl<E: Into<Error>> From<E> for Box<SetFehler> {
    fn from(e: E) -> Self {
        Box::new(SetFehler {
            stand: SetErgebnis::default(),
            fehler: e.into(),
        })
    }
}

/// Wendet einen Merge-Patch atomar auf die Spec an. Jeder Fehler vor dem Ersetzen lässt Spec und Ausgabe unberührt.
pub fn set(cli: &Cli, id: &str, patch: &str) -> Result<SetErgebnis, Box<SetFehler>> {
    let a = finde(&cli.root, id)?;
    let _sperre = sperre(&cli.root, &a.id);
    let alt = lies_text(&a.abs(&cli.root))?;
    let patch = teile_patch(&alt, patch)?;
    let text = formatiere(&merge_patch(&alt, &patch)?)?;
    let text = wie_zeilenenden(&alt, text);
    let mut r = schreibe_spec(cli, &a, &alt, &text)?;
    r.reiter = reiter_von(&cli.root, &a.id);
    Ok(r)
}

/// Erlaubte Asset-ID oder Gruppe (Schema-Muster name).
static RE_NAME: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("^[a-z][a-z0-9-]*$").ok());

pub(super) fn ist_name(s: &str) -> bool {
    RE_NAME.as_ref().is_some_and(|re| re.is_match(s))
}

pub(super) fn name_fehler(id: &str, gruppe: &str) -> Error {
    Error::Sonst(format!("id „{id}“ und gruppe „{gruppe}“: nur a-z, 0-9, - (Anfang a-z)"))
}

/// Belegte Schlüssel und das Warten darauf.
static BELEGT: (Mutex<BTreeSet<String>>, Condvar) = (Mutex::new(BTreeSet::new()), Condvar::new());

/// Freigabe beim Fallenlassen.
pub(super) struct Sperre(String);

impl Drop for Sperre {
    fn drop(&mut self) {
        let (belegt, frei) = &BELEGT;
        belegt.lock().unwrap_or_else(PoisonError::into_inner).remove(&self.0);
        frei.notify_all();
    }
}

/// Sperrt je Checkout und Schlüssel: `set`, `schreibe` und `render_eins` auf dasselbe Asset laufen nacheinander.
/// Nicht wiedereintrittsfähig.
pub(super) fn sperre(root: &Path, id: &str) -> Sperre {
    let root: PathBuf = root.components().collect();
    let key = format!("{}|{id}", root.display());
    let (belegt, frei) = &BELEGT;
    let mut menge = belegt.lock().unwrap_or_else(PoisonError::into_inner);
    while menge.contains(&key) {
        menge = frei.wait(menge).unwrap_or_else(PoisonError::into_inner);
    }
    menge.insert(key.clone());
    Sperre(key)
}

/// Liest eine handgepflegte Datei; kein UTF-8 wird abgelehnt statt verändert zurückgeschrieben.
pub(super) fn lies_text(pfad: &Path) -> Result<String, Error> {
    let name = pfad.file_name().unwrap_or_default().to_string_lossy();
    let raw = fs::read(pfad).map_err(|e| Error::Sonst(format!("{name}: {e}")))?;
    String::from_utf8(raw).map_err(|_| Error::Sonst(format!("{name}: kein UTF-8, nichts geschrieben")))
}

/// `neu` (mit LF) in den Zeilenenden von `alt`.
pub(super) fn wie_zeilenenden(alt: &str, neu: String) -> String {
    if alt.contains("\r\n") {
        return neu.replace("\r\n", "\n").replace('\n', "\r\n");
    }
    neu
}

/// Ersetzt die Spec von `a` atomar durch `text` (`alt` = bisheriger Inhalt, leer bei neuem Asset).
pub fn schreibe(cli: &Cli, a: &Asset, alt: &str, text: &str) -> Result<SetErgebnis, Box<SetFehler>> {
    if !ist_name(&a.id) || !ist_name(&a.gruppe) {
        return Err(name_fehler(&a.id, &a.gruppe).into());
    }
    let _sperre = sperre(&cli.root, &a.id);
    schreibe_spec(cli, a, alt, text)
}

fn schreibe_spec(cli: &Cli, a: &Asset, alt: &str, text: &str) -> Result<SetErgebnis, Box<SetFehler>> {
    let tmp = TempWeg(temp_ordner(&cli.root)?);
    let mut r = SetErgebnis {
        asset: a.clone(),
        animationen: schluessel(text, &["animationen"]),
        ..SetErgebnis::default()
    };
    match uebernimm(cli, a, &tmp.0, (alt, text), &mut r) {
        Ok(()) => Ok(r),
        Err(fehler) => Err(Box::new(SetFehler { stand: r, fehler })),
    }
}

fn uebernimm(cli: &Cli, a: &Asset, tmp: &Path, (alt, text): (&str, &str), r: &mut SetErgebnis) -> Result<(), Error> {
    let spec = tmp.join(&a.gruppe).join(format!("{}{ENDUNG}", a.id));
    spiegele_nachbarn(&cli.root, tmp, a)?;
    schreibe_datei(&spec, text.as_bytes())?;
    let mut alt_spec = None;
    if !alt.is_empty() {
        // `.alt` ist kein gültiger Gruppenname.
        let p = tmp.join(".alt").join(format!("{}{ENDUNG}", a.id));
        schreibe_datei(&p, alt.as_bytes())?;
        alt_spec = Some(p);
    }
    let render = pruefe_und_rendere(cli, a, alt_spec.as_deref(), &spec, r)?;
    ersetze(&spec, &a.abs(&cli.root))?;
    let tmp_aus = tmp.join(&a.gruppe).join(&a.id);
    if tmp_aus.metadata().is_ok()
        && let Err(e) = ersetze_ordner(cli, &tmp_aus, &a.ausgabe(&cli.root))
    {
        zurueck(&a.abs(&cli.root), alt);
        return Err(e);
    }
    gruppe_nachziehen(&cli.root, a)?;
    let vorher = format!("{}/", cli.rel(&tmp_aus));
    let nachher = format!("{}/", cli.rel(&a.ausgabe(&cli.root)));
    for d in render.dateien {
        r.pngs += usize::from(d.ends_with(".png"));
        r.dateien.push(d.replacen(&vorher, &nachher, 1));
    }
    Ok(())
}

fn mit_zusatz(p: &Path, zusatz: &str) -> PathBuf {
    let mut s = p.as_os_str().to_owned();
    s.push(zusatz);
    PathBuf::from(s)
}

/// Stellt die alte Spec wieder her (bei neuem Asset: löscht sie), wenn die Ausgabe nicht ersetzt werden konnte.
fn zurueck(spec: &Path, alt: &str) {
    if alt.is_empty() {
        let _ = fs::remove_file(spec);
        return;
    }
    let tmp = mit_zusatz(spec, ".zurueck");
    let _ = fs::write(&tmp, alt);
    let _ = fs::rename(&tmp, spec);
}

/// Kopiert die übrigen Specs in den Temp-Ordner (`<tmp>/<gruppe>/<id>.paper.json`): Die Render-CLI sucht z. B. das
/// Reittier neben der Spec und in den Nachbar-Gruppen (RD1, tools/paper/rig/reiten.ts).
fn spiegele_nachbarn(root: &Path, tmp: &Path, ohne: &Asset) -> Result<(), Error> {
    for n in assets(root)? {
        if n.id == ohne.id && n.gruppe == ohne.gruppe {
            continue;
        }
        let ziel = tmp.join(&n.gruppe).join(format!("{}{ENDUNG}", n.id));
        schreibe_datei(&ziel, &fs::read(n.abs(root))?)?;
    }
    Ok(())
}

/// Ein Temp-Ordner, der beim Fallenlassen verschwindet.
pub(super) struct TempWeg(pub PathBuf);

impl Drop for TempWeg {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Legt `.cache/paper/tmp/<zufall>/` im Checkout an (gleiches Laufwerk, Umbenennen bleibt atomar).
pub fn temp_ordner(root: &Path) -> Result<PathBuf, Error> {
    static LAUF: AtomicU64 = AtomicU64::new(0);
    let basis = root.join(".cache").join("paper").join("tmp");
    fs::create_dir_all(&basis)?;
    loop {
        let zeit = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos());
        let saat = format!("{zeit}-{}-{}", std::process::id(), LAUF.fetch_add(1, Ordering::Relaxed));
        let mut name = sha256_hex(saat.as_bytes());
        name.truncate(12);
        let p = basis.join(name);
        match fs::create_dir(&p) {
            Ok(()) => return Ok(p),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.into()),
        }
    }
}

pub(super) fn schreibe_datei(p: &Path, b: &[u8]) -> Result<(), Error> {
    if let Some(ordner) = p.parent() {
        fs::create_dir_all(ordner)?;
    }
    Ok(fs::write(p, b)?)
}

/// Schreibt den Inhalt von `src` über eine Temp-Datei neben `ziel` und benennt sie um.
fn ersetze(src: &Path, ziel: &Path) -> Result<(), Error> {
    let b = fs::read(src)?;
    let lauf = src.parent().and_then(Path::parent).and_then(Path::file_name);
    let tmp = mit_zusatz(ziel, &format!(".tmp-{}", lauf.unwrap_or_default().to_string_lossy()));
    schreibe_datei(&tmp, &b)?;
    if let Err(e) = fs::rename(&tmp, ziel) {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    Ok(())
}

/// Benennt `src` (`<tmp>/<gruppe>/<id>`) nach `ziel` um; ein vorhandenes `ziel` wird erst nach `<tmp>/.beiseite` gelegt
/// (unter `.cache/`, nicht neben die Ausgabe, die der Server beobachtet; B-706) und danach gelöscht.
fn ersetze_ordner(cli: &Cli, src: &Path, ziel: &Path) -> Result<(), Error> {
    let umbenennen = cli.ordner_umbenennen;
    let mut beiseite = None;
    if ziel.metadata().is_ok() {
        let tmp = src.parent().and_then(Path::parent).unwrap_or(src);
        let b = tmp.join(".beiseite");
        umbenennen(ziel, &b)?;
        beiseite = Some(b);
    }
    if let Err(e) = umbenennen(src, ziel) {
        return Err(match beiseite {
            Some(b) => zurueck_legen(&b, ziel, &e),
            None => e.into(),
        });
    }
    if let Some(b) = beiseite {
        let _ = fs::remove_dir_all(b); // Aufräumen; der neue Stand liegt schon.
    }
    Ok(())
}

/// Bringt die beiseitegelegte Ausgabe zurück (drei Versuche, der Watcher kann kurz sperren). Gelingt das nicht, rettet
/// es sie nach `.cache/paper/gerettet/`, weil der Temp-Ordner danach gelöscht wird, und nennt den Pfad.
fn zurueck_legen(beiseite: &Path, ziel: &Path, ursache: &io::Error) -> Error {
    let mut letzter = String::new();
    for i in 0..3 {
        match fs::rename(beiseite, ziel) {
            Ok(()) => return Error::Sonst(ursache.to_string()),
            Err(e) => letzter = e.to_string(),
        }
        std::thread::sleep(Duration::from_millis((i + 1) * 50));
    }
    // `<paper>/tmp/<x>/.beiseite` → `<paper>/gerettet/<x>`
    let tmp = beiseite.parent().unwrap_or(beiseite);
    let paper = tmp.parent().and_then(Path::parent).unwrap_or(tmp);
    let gerettet = paper.join("gerettet").join(tmp.file_name().unwrap_or_default());
    let angelegt = fs::create_dir_all(paper.join("gerettet"));
    if angelegt.and_then(|()| fs::rename(beiseite, &gerettet)).is_ok() {
        let wo = gerettet.display();
        return Error::Sonst(format!(
            "{ursache}; alte Ausgabe liegt unter {wo} (zurückbenennen: {letzter})"
        ));
    }
    Error::Sonst(format!("{ursache}; alte Ausgabe nicht wiederhergestellt: {letzter}"))
}

/// Pfad des Overlays von `assets_set` unter `.cache/paper/mcp/`: eigener Unterordner `set/`, damit er das Overlay von
/// `assets_review` auch bei IDs mit „-“ nie trifft (B-706).
pub fn set_overlay(id: &str, anim: &str) -> String {
    format!("set/{id}-{anim}-0-diff.png")
}
