//! Die RPC-Schicht der Asset-Oberfläche (Port von `internal/assetsui`; Vertrag docs/standards/paper-mcp.md ›
//! Oberfläche): Katalog, Spec, Stil-Datei, Prompt-Vorlagen, Review, Frames und Pixel-Diff aus dem Checkout. Vergleich,
//! Cache und Render-CLI liegen in [`crate::review`]. Alle Aufrufe blockieren (git, bun); async-Aufrufer nutzen
//! `spawn_blocking`.

mod frameohne;
mod groesse;
mod katalog;
mod stufen;
mod welt;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
#[path = "assetsui_tests.rs"]
mod tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, PoisonError};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::review::{self, Cli, FrameAntwort, Quelle, Renderer, Teile};

pub use groesse::{Groesse, GroesseBasis, Hintergrund};
pub use katalog::{AssetEintrag, PruefMemo};
pub use stufen::{Stufe, Stufen, StufenAsset};
pub use welt::{WELT_BASIS, WeltSicht};

// Typen des Vertrags; die Review-Typen kommen aus `review`.
pub use crate::review::{Asset as ReviewAsset, Ergebnis as Review, Fehler, FeldAenderung};

const STIL_DATEI: &str = "public/grafik/stil.paper.json";
const PROMPT_DATEI: &str = "data/asset-prompts.json";
/// Begrenzt gleichzeitige CLI-Aufrufe (bun) aus Katalog und Frames.
const PARALLEL: usize = 4;

/// Die Render-CLI der Oberfläche: wie [`Renderer`], dazu ein Frame ohne die Lagen einzelner Teile (B-734).
pub trait Rendern: Renderer + Send {
    /// Prüft `animation` und `ohne` nicht; das macht [`Dienst::assets_frame_ohne`].
    fn frame_ohne(
        &self,
        spec: &Path,
        animation: &str,
        frame: i64,
        ohne: &[String],
        out: &Path,
    ) -> Result<FrameAntwort, review::Error>;
}

impl Rendern for Cli {
    fn frame_ohne(
        &self,
        spec: &Path,
        animation: &str,
        frame: i64,
        ohne: &[String],
        out: &Path,
    ) -> Result<FrameAntwort, review::Error> {
        Cli::frame_ohne(self, spec, animation, frame, ohne, out)
    }
}

/// Beantwortet die Methoden für den Checkout `root`.
pub struct Dienst {
    pub root: PathBuf,
    /// Render-CLI (Vorgabe: die im Checkout).
    pub r: Arc<dyn Rendern + Sync>,
    /// Gemerkte Prüfergebnisse; die App gibt jedem Dienst dasselbe.
    pub memo: PruefMemo,
}

/// Ein gerenderter Frame als Data-URL mit den Teil-Boxen `[x, y, b, h]` in Pixeln.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Frame {
    pub bild: String,
    pub teile: Teile,
}

/// Der Pixel-Vergleich mit Overlay als Data-URL.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PixelDiff {
    #[serde(serialize_with = "go_zahl")]
    pub anteil: f64,
    pub boxen: Vec<Vec<i64>>,
    pub overlay: String,
}

/// Eine Prompt-Vorlage aus `data/asset-prompts.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Vorlage {
    pub id: String,
    pub titel: String,
    pub ort: String,
    pub text: String,
}

/// Zahl wie Go `json.Marshal`: ganze Werte ohne `.0`.
fn go_zahl<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.fract() == 0.0 && v.abs() < 9.0e15 {
        s.serialize_i64(*v as i64)
    } else {
        s.serialize_f64(*v)
    }
}

/// Ein freier Wert, wie Go ihn nach `any` liest und wieder schreibt: Schlüssel sortiert, ganze Zahlen ohne `.0`.
fn wie_go(wert: Value) -> Value {
    match wert {
        Value::Number(n) => match n.as_f64() {
            Some(f) if n.is_f64() && f.fract() == 0.0 && f.abs() < 9.0e15 => Value::from(f as i64),
            _ => Value::Number(n),
        },
        Value::Array(liste) => Value::Array(liste.into_iter().map(wie_go).collect()),
        Value::Object(map) => {
            let sortiert: BTreeMap<String, Value> = map.into_iter().collect();
            Value::Object(sortiert.into_iter().map(|(k, v)| (k, wie_go(v))).collect())
        }
        andere => andere,
    }
}

/// Datei unter `root` zu einem Pfad mit `/`.
fn datei(root: &Path, pfad: &str) -> PathBuf {
    let mut p = root.to_path_buf();
    p.extend(pfad.split('/'));
    p
}

fn lies(pfad: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(pfad).map_err(|e| format!("open {}: {e}", pfad.display()))
}

/// Liest eine JSON-Datei; doppelte Schlüssel: der letzte gilt (wie Go).
fn lies_json<T: DeserializeOwned>(pfad: &Path) -> Result<T, String> {
    let wert: Value = serde_json::from_slice(&lies(pfad)?).map_err(|e| e.to_string())?;
    serde_json::from_value(wert).map_err(|e| e.to_string())
}

/// Eine Spec als Objekt ohne `$schema` (`null` bleibt `null`; alles andere als ein Objekt ist ein Fehler).
fn objekt(raw: &[u8]) -> Result<Value, String> {
    let map: Option<Map<String, Value>> = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    Ok(map.map_or(Value::Null, |mut m| {
        m.remove("$schema");
        wie_go(Value::Object(m))
    }))
}

fn data_url(pfad: &Path) -> Result<String, String> {
    Ok(format!("data:image/png;base64,{}", STANDARD.encode(lies(pfad)?)))
}

/// Gleichzeitige Renders aus der Oberfläche (je Frame ein Aufruf, je Render ein bun-Prozess).
static BELEGT: (Mutex<usize>, Condvar) = (Mutex::new(0), Condvar::new());

/// Ein belegter Render-Platz; wird beim Fallenlassen frei.
struct RenderPlatz;

impl RenderPlatz {
    fn belegen() -> RenderPlatz {
        let guard = BELEGT.0.lock().unwrap_or_else(PoisonError::into_inner);
        let mut guard = BELEGT
            .1
            .wait_while(guard, |n| *n >= PARALLEL)
            .unwrap_or_else(PoisonError::into_inner);
        *guard += 1;
        RenderPlatz
    }
}

impl Drop for RenderPlatz {
    fn drop(&mut self) {
        *BELEGT.0.lock().unwrap_or_else(PoisonError::into_inner) -= 1;
        BELEGT.1.notify_one();
    }
}

/// Der Vertragsfehler für eine unbekannte id (paper-mcp.md).
fn asset_fehlt(id: &str) -> String {
    format!("Asset „{id}“ fehlt")
}

/// Der letzte Teil eines Pfads mit `/` (Go `path.Base`).
fn basisname(pfad: &str) -> &str {
    if pfad.is_empty() {
        return ".";
    }
    let ohne = pfad.trim_end_matches('/');
    if ohne.is_empty() {
        return "/";
    }
    ohne.rsplit('/').next().unwrap_or(ohne)
}

/// Die id einer Spec aus ihrem Pfad.
fn id_von(pfad: &str) -> &str {
    let name = basisname(pfad);
    name.strip_suffix(".paper.json").unwrap_or(name)
}

/// Prüft den Animationsnamen gegen die Spec; leer = erste Animation.
fn animation(q: &Quelle, name: &str) -> Result<String, String> {
    let mut namen = review::animationen(&q.spec).map_err(|e| e.to_string())?;
    if namen.is_empty() {
        return Err(format!("Asset „{}“ hat keine Animation", id_von(&q.pfad)));
    }
    if name.is_empty() {
        return Ok(namen.swap_remove(0));
    }
    if !namen.iter().any(|n| n == name) {
        return Err(format!("Animation „{name}“ fehlt: {}", namen.join(", ")));
    }
    Ok(name.to_owned())
}

/// Lehnt einen Frame jenseits der Animation dieses Stands ab (die Basis kann kürzer sein als der Arbeitsstand).
fn pruefe_frame(q: &Quelle, anim: &str, frame: i64) -> Result<(), String> {
    #[derive(Deserialize)]
    struct Animation {
        #[serde(default)]
        frames: i64,
    }
    let spec: Value = serde_json::from_str(&String::from_utf8_lossy(&q.spec)).map_err(|e| e.to_string())?;
    let alle: Option<BTreeMap<String, Option<Animation>>> =
        serde_json::from_value(spec.get("animationen").cloned().unwrap_or_default()).map_err(|e| e.to_string())?;
    let n = alle.and_then(|mut a| a.remove(anim)).flatten().map_or(0, |a| a.frames);
    if frame >= n {
        return Err(format!("Frame {frame} fehlt: Animation „{anim}“ hat {n} Frames"));
    }
    Ok(())
}

impl Dienst {
    /// Dienst mit der Render-CLI des Checkouts und leerem Memo.
    pub fn new(root: impl Into<PathBuf>) -> Dienst {
        let root = root.into();
        Dienst {
            r: Arc::new(Cli::new(root.clone())),
            memo: PruefMemo::default(),
            root,
        }
    }

    fn renderer(&self) -> &dyn Renderer {
        &*self.r
    }

    /// Sucht die Spec einer id im Arbeitsstand, sonst (mit `commit`) in der Basis.
    fn pfad_zu(&self, id: &str, commit: &str) -> Result<String, String> {
        let name = format!("{id}.paper.json");
        let passt = |p: &String| basisname(p) == name;
        let pfade = review::specs_im_arbeitsstand(&self.root).map_err(|e| e.to_string())?;
        if let Some(p) = pfade.into_iter().find(passt) {
            return Ok(p);
        }
        if !commit.is_empty() {
            let dateien = review::basis_dateien(&self.root, commit).map_err(|e| e.to_string())?;
            if let Some(p) = dateien.into_keys().find(passt) {
                return Ok(p);
            }
        }
        Err(asset_fehlt(id))
    }

    /// Die Spec des Arbeitsstands als Objekt ohne `$schema`.
    pub fn assets_get(&self, id: &str) -> Result<Value, String> {
        let p = self.pfad_zu(id, "")?;
        objekt(&lies(&datei(&self.root, &p))?)
    }

    /// Die Stil-Datei als Objekt ohne `$schema`.
    pub fn assets_style(&self) -> Result<Value, String> {
        objekt(&lies(&datei(&self.root, STIL_DATEI))?)
    }

    /// Die Prompt-Vorlagen aus `data/asset-prompts.json`.
    pub fn assets_prompts(&self) -> Result<Option<Vec<Vorlage>>, String> {
        lies_json(&datei(&self.root, PROMPT_DATEI))
    }

    /// Vergleicht den Arbeitsstand mit der Basis; leer = `HEAD`.
    pub fn assets_review(&self, basis: &str) -> Result<Review, String> {
        review::vergleich(&self.root, basis, self.renderer()).map_err(|e| e.to_string())
    }

    /// Der Stand eines Assets: leere Basis = Arbeitsstand, sonst die aufgelöste Basis.
    fn quelle(&self, id: &str, basis: &str) -> Result<Quelle, String> {
        let commit = if basis.is_empty() {
            String::new()
        } else {
            review::aufloesen(&self.root, basis).map_err(|e| e.to_string())?.commit
        };
        let p = self.pfad_zu(id, &commit)?;
        let q = if commit.is_empty() {
            review::quelle_arbeitsstand(&self.root, &p)
        } else {
            review::quelle_basis(&self.root, &commit, &p)
        };
        q.map_err(|e| e.to_string())?.ok_or_else(|| asset_fehlt(id))
    }

    /// Ein Frame als Data-URL mit Teil-Boxen; leere Basis = Arbeitsstand.
    pub fn assets_frame(&self, id: &str, anim: &str, frame: i64, variante: &str, basis: &str) -> Result<Frame, String> {
        let q = self.quelle(id, basis)?;
        let anim = animation(&q, anim)?;
        pruefe_frame(&q, &anim, frame)?;
        let platz = RenderPlatz::belegen();
        let g = review::bild(self.renderer(), &self.root, &q, &anim, frame, variante);
        drop(platz);
        let g = g.map_err(|e| e.to_string())?;
        Ok(Frame {
            bild: data_url(&g.datei)?,
            teile: g.teile,
        })
    }

    /// Vergleicht einen Frame des Arbeitsstands mit der Basis (Pflicht), Overlay als Data-URL.
    pub fn assets_pixel_diff(&self, id: &str, anim: &str, frame: i64, basis: &str) -> Result<PixelDiff, String> {
        if basis.is_empty() {
            return Err(review::fehler_basis(basis).to_string());
        }
        let qa = self.quelle(id, basis)?;
        let qn = self.quelle(id, "")?;
        let anim = animation(&qn, anim)?;
        let fehler: Vec<String> = [&qa, &qn]
            .into_iter()
            .filter_map(|q| pruefe_frame(q, &anim, frame).err())
            .collect();
        if !fehler.is_empty() {
            return Err(fehler.join("\n"));
        }
        let platz = RenderPlatz::belegen();
        let p = review::overlay(self.renderer(), &self.root, &qa, &qn, &anim, frame);
        drop(platz);
        let p = p.map_err(|e| e.to_string())?;
        Ok(PixelDiff {
            anteil: p.anteil,
            boxen: p.boxen,
            overlay: data_url(Path::new(&p.datei))?,
        })
    }
}
