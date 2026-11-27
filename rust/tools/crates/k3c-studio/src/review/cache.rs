//! Stände eines Assets ([`Quelle`]) und der Render-Cache `.cache/paper/<renderKey>/`.

use std::collections::BTreeMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use serde_json::Value;

use super::cli::{Error, Renderer, Teile};
use super::git::{
    ENDUNG, STIL_PFAD, datei_in, ist_spec, lies_arbeitsstand, show, specs_im_arbeitsstand, specs_in_commit,
};
use super::hash::{lies_wert, render_key_mit, spec_hash};
use crate::kitx::sha256_hex;

/// Schützt die Dateinamen im Cache: Animation (auch `idle@spieler2`, `stand.s2`) und Variante sind Schlüssel der Spec,
/// nie Pfade.
static RE_NAME: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9.@_-]*$").ok());

pub(super) fn gueltiger_name(s: &str) -> bool {
    RE_NAME.as_ref().is_some_and(|re| re.is_match(s)) && !s.contains("..")
}

/// Die Spec ist kein lesbares JSON; im Review ein ungültiger Eintrag (Regel `schema`).
#[derive(Debug)]
pub struct SpecUnlesbar {
    pub pfad: String,
    pub grund: String,
}

impl fmt::Display for SpecUnlesbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: JSON unlesbar: {}", self.pfad, self.grund)
    }
}

/// Ein Stand eines Assets: Spec und Stil-Datei als Bytes, ihre Hashes und ob er aus der Basis stammt.
#[derive(Debug, Clone, Default)]
pub struct Quelle {
    /// Relativ zum Checkout, mit `/`.
    pub pfad: String,
    pub spec: Vec<u8>,
    pub stil: Vec<u8>,
    pub spec_hash: String,
    pub stil_hash: String,
    pub basis: bool,
    /// Specs, die die CLI neben dieser braucht (Reittiere, Regel `reittier-unbekannt`): Pfad → Inhalt.
    pub beilagen: BTreeMap<String, Vec<u8>>,
    beilagen_hash: String,
}

/// Ein Frame im Cache mit den Teil-Boxen.
#[derive(Debug, Clone, Default)]
pub struct Gerendert {
    pub datei: PathBuf,
    pub teile: Teile,
}

/// Liefert eine Datei eines Stands; `None`, wenn sie fehlt.
type Leser<'a> = &'a dyn Fn(&str) -> Result<Option<Vec<u8>>, Error>;

pub(super) fn neue_quelle(
    pfad: &str,
    spec: Vec<u8>,
    stil: Vec<u8>,
    basis: bool,
    specs: &[String],
    lies: Leser<'_>,
) -> Result<Quelle, Error> {
    let sh = spec_hash(&spec).map_err(|e| {
        Error::Unlesbar(SpecUnlesbar {
            pfad: pfad.to_owned(),
            grund: e.to_string(),
        })
    })?;
    let th = spec_hash(&stil).map_err(|e| Error::Sonst(format!("{STIL_PFAD}: {e}")))?;
    let (beilagen, beilagen_hash) = beilagen(&spec, specs, lies)?;
    Ok(Quelle {
        pfad: pfad.to_owned(),
        spec,
        stil,
        spec_hash: sh,
        stil_hash: th,
        basis,
        beilagen,
        beilagen_hash,
    })
}

/// Reittier-IDs aus `animationen[*].reittier` (sortiert, ohne Doppelte).
pub fn reittiere(spec: &[u8]) -> Vec<String> {
    let wert = lies_wert(spec).unwrap_or_default();
    let animationen = wert.get("animationen").and_then(Value::as_object);
    let mut ids: Vec<String> = animationen
        .into_iter()
        .flat_map(|a| a.values())
        .filter_map(|a| a.get("reittier").and_then(Value::as_str))
        .filter(|id| !id.is_empty())
        .map(str::to_owned)
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

/// Je Reittier `<id>.paper.json` unter den Specs des Stands; ein fehlendes bleibt fehlend (die CLI meldet es).
fn beilagen(spec: &[u8], specs: &[String], lies: Leser<'_>) -> Result<(BTreeMap<String, Vec<u8>>, String), Error> {
    let ids = reittiere(spec);
    let mut out = BTreeMap::new();
    if ids.is_empty() {
        return Ok((out, String::new()));
    }
    let mut schluessel = String::new();
    for id in ids {
        let name = format!("{id}{ENDUNG}");
        let treffer = specs.iter().find(|p| p.rsplit('/').next() == Some(&name));
        let Some(pfad) = treffer else {
            continue;
        };
        if let Some(raw) = lies(pfad)? {
            schluessel.push_str(&format!("{pfad}:{};", sha256_hex(&raw)));
            out.insert(pfad.clone(), raw);
        }
    }
    Ok((out, sha256_hex(schluessel.as_bytes())))
}

fn keine_spec(pfad: &str) -> Error {
    Error::Sonst(format!("{pfad} ist keine Spec"))
}

/// Spec und Stil-Datei aus dem Commit; fehlt die Spec dort, `None`.
pub fn quelle_basis(root: &Path, commit: &str, pfad: &str) -> Result<Option<Quelle>, Error> {
    if !ist_spec(pfad) {
        return Err(keine_spec(pfad));
    }
    let Some(spec) = show(root, commit, pfad)? else {
        return Ok(None);
    };
    let stil = show(root, commit, STIL_PFAD)?.ok_or_else(|| Error::Sonst(format!("{STIL_PFAD} fehlt in {commit}")))?;
    let specs = if reittiere(&spec).is_empty() {
        Vec::new()
    } else {
        specs_in_commit(root, commit)?
    };
    neue_quelle(pfad, spec, stil, true, &specs, &|p| show(root, commit, p)).map(Some)
}

/// Spec und Stil-Datei aus dem Checkout; fehlt die Spec, `None`.
pub fn quelle_arbeitsstand(root: &Path, pfad: &str) -> Result<Option<Quelle>, Error> {
    if !ist_spec(pfad) {
        return Err(keine_spec(pfad));
    }
    let Some(spec) = lies_arbeitsstand(root, pfad)? else {
        return Ok(None);
    };
    let stil = lies_arbeitsstand(root, STIL_PFAD)?.ok_or_else(|| Error::Sonst(format!("{STIL_PFAD} fehlt")))?;
    let specs = if reittiere(&spec).is_empty() {
        Vec::new()
    } else {
        specs_im_arbeitsstand(root)?
    };
    let lies = |p: &str| lies_arbeitsstand(root, p);
    neue_quelle(pfad, spec, stil, false, &specs, &lies).map(Some)
}

impl Quelle {
    /// `.cache/paper/<renderKey>/` (renderKey mit der Kennung des Renderers).
    pub(super) fn ordner(&self, root: &Path, r: &dyn Renderer) -> PathBuf {
        let spec = if self.beilagen_hash.is_empty() {
            self.spec_hash.clone()
        } else {
            // Ein geändertes Reittier ändert den Render des Reiters.
            sha256_hex(format!("{}+{}", self.spec_hash, self.beilagen_hash).as_bytes())
        };
        let key = render_key_mit(&spec, &self.stil_hash, &r.kennung());
        datei_in(root, ".cache/paper").join(key)
    }

    /// Legt Spec und Stil-Datei als Mini-Baum in den Cache (`<ordner>/basis|stand/public/grafik/…`): die CLI rendert
    /// so genau die gehashten Bytes, und Spec und Stil-Datei des Stands liegen zusammen.
    fn schnappschuss(&self, root: &Path, r: &dyn Renderer) -> Result<(PathBuf, PathBuf), Error> {
        let art = if self.basis { "basis" } else { "stand" };
        let baum = self.ordner(root, r).join(art);
        let spec = datei_in(&baum, &self.pfad);
        let stil = datei_in(&baum, STIL_PFAD);
        schreibe_wenn_neu(&spec, &self.spec)?;
        for (pfad, raw) in &self.beilagen {
            schreibe_wenn_neu(&datei_in(&baum, pfad), raw)?;
        }
        schreibe_wenn_neu(&stil, &self.stil)?;
        Ok((spec, stil))
    }

    /// Die Spec des Stands als Datei für die CLI (Schnappschuss im Cache).
    pub fn spec_datei(&self, root: &Path, r: &dyn Renderer) -> Result<PathBuf, Error> {
        Ok(self.schnappschuss(root, r)?.0)
    }
}

/// Rendert einen Frame in den Cache; liegt er schon dort, rendert es nicht neu.
pub fn bild(
    r: &dyn Renderer,
    root: &Path,
    q: &Quelle,
    animation: &str,
    frame: i64,
    variante: &str,
) -> Result<Gerendert, Error> {
    if !gueltiger_name(animation) || (!variante.is_empty() && !gueltiger_name(variante)) || frame < 0 {
        return Err(Error::Sonst(format!(
            "Animation „{animation}“, Frame {frame} oder Variante „{variante}“ ungültig"
        )));
    }
    let mut name = format!("{animation}-{frame}");
    if !variante.is_empty() {
        name = format!("{name}@{variante}");
    }
    let ordner = q.ordner(root, r);
    let png = ordner.join(format!("{name}.png"));
    let meta = ordner.join(format!("{name}.json"));
    if let Some(g) = aus_cache(&png, &meta) {
        return Ok(g);
    }
    let (spec, stil) = q.schnappschuss(root, r)?;
    let tmp = temp_name(&ordner, &format!("{name}.png"))?;
    // Stil des Stands, beim Vorher-Bild der Basis (B-705).
    let antwort = r
        .frame(&spec, Some(&stil), animation, frame, variante, &tmp)
        .and_then(|a| {
            uebernehme(&tmp, &png)?;
            Ok(a)
        });
    let a = match antwort {
        Ok(a) => a,
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
    };
    schreibe_atomar(&meta, &serde_json::to_vec(&a.teile)?)?;
    Ok(Gerendert {
        datei: png,
        teile: a.teile,
    })
}

fn aus_cache(png: &Path, meta: &Path) -> Option<Gerendert> {
    png.metadata().ok()?;
    let raw = std::fs::read(meta).ok()?;
    let teile: Option<Teile> = serde_json::from_slice(&raw).ok()?;
    Some(Gerendert {
        datei: png.to_path_buf(),
        teile: teile.unwrap_or_default(),
    })
}

/// Reserviert einen eindeutigen Temp-Namen im Ordner (je Aufruf, auch zwischen Threads und Prozessen); die CLI schreibt
/// dorthin, danach wird umbenannt. Die Endung bleibt die von `name`.
pub(super) fn temp_name(ordner: &Path, name: &str) -> io::Result<PathBuf> {
    let endung = Path::new(name)
        .extension()
        .map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));
    reserviere(ordner, &format!("{name}.tmp-"), &endung)
}

fn reserviere(ordner: &Path, vorn: &str, hinten: &str) -> io::Result<PathBuf> {
    static ZAEHLER: AtomicU64 = AtomicU64::new(0);
    std::fs::create_dir_all(ordner)?;
    loop {
        let zeit = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        let nr = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let pfad = ordner.join(format!("{vorn}{}-{zeit}-{nr}{hinten}", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&pfad) {
            Ok(_) => return Ok(pfad),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
}

/// Schreibt atomar, falls die Datei fehlt (der Inhalt ist durch den Ordnernamen festgelegt).
fn schreibe_wenn_neu(pfad: &Path, raw: &[u8]) -> io::Result<()> {
    if pfad.metadata().is_ok() {
        return Ok(());
    }
    schreibe_atomar(pfad, raw)
}

/// Schreibt per Temp-Datei und Umbenennen; ein Abbruch hinterlässt keinen halben Eintrag.
fn schreibe_atomar(pfad: &Path, raw: &[u8]) -> io::Result<()> {
    let ordner = pfad.parent().unwrap_or_else(|| Path::new("."));
    let name = pfad.file_name().unwrap_or_default().to_string_lossy();
    let tmp = reserviere(ordner, &format!("{name}.tmp-"), "")?;
    let ergebnis = std::fs::write(&tmp, raw).and_then(|()| uebernehme(&tmp, pfad));
    if ergebnis.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    ergebnis
}

/// Benennt `tmp` in `ziel` um. Unter Windows scheitert das, solange ein paralleler Aufruf `ziel` offen hält; da der
/// Inhalt im Cache durch den Schlüssel festgelegt ist, genügt dann die vorhandene Datei.
pub(super) fn uebernehme(tmp: &Path, ziel: &Path) -> io::Result<()> {
    let ergebnis = std::fs::rename(tmp, ziel);
    if ergebnis.is_err() && ziel.metadata().is_ok() {
        let _ = std::fs::remove_file(tmp);
        return Ok(());
    }
    ergebnis
}

#[cfg(test)]
#[path = "cache_tests.rs"]
mod tests;
