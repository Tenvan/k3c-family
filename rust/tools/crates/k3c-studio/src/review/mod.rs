//! Vergleicht die Assets des Arbeitsstands mit einer Git-Basis (Port von `internal/review`; Vertrag
//! docs/standards/paper-mcp.md › Oberfläche): Basis auflösen, Specs und Stil-Datei per git lesen, kanonisch hashen
//! (Port von `hash.ts`), „Vorher“ über die Render-CLI in den Cache `.cache/paper/<renderKey>/` rendern und je Asset den
//! Review-Eintrag bilden. Alle Aufrufe blockieren (git, bun); async-Aufrufer nutzen `spawn_blocking`.

mod aufruf;
mod basis;
mod cache;
mod cli;
mod git;
mod hash;
mod pixel;
mod stil;
mod types;

#[cfg(test)]
#[path = "haertung_tests.rs"]
mod haertung_tests;
#[cfg(test)]
pub(crate) mod test_helpers;
#[cfg(test)]
#[path = "review_tests.rs"]
mod tests;

pub use aufruf::{ZEITLIMIT, aufruf, auswerten, skript};
pub use basis::{Stand, aufloesen, fehler_basis};
pub use cache::{Gerendert, Quelle, SpecUnlesbar, bild, quelle_arbeitsstand, quelle_basis, reittiere};
pub use cli::{Cli, DiffAntwort, Error, FrameAntwort, PixelAntwort, Renderer, Teile};
pub use git::{Dateien, basis_dateien, show, specs_im_arbeitsstand, specs_in_commit};
pub use hash::{RENDERER_VERSION, kanonisch, render_key, spec_hash};
pub use pixel::{overlay, pixel_anteil};
pub use stil::{animationen, erste_animation};
pub use types::{
    Asset, Ergebnis, Fehler, FeldAenderung, STATUS_GEAENDERT, STATUS_GELOESCHT, STATUS_NEU, STATUS_UNGUELTIG,
};

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

use git::{ENDUNG, STIL_PFAD, ist_spec, lies_arbeitsstand, manifest_zu};
use serde_json::Value;
use stil::{StilAenderung, vergleiche_stil};

/// Begrenzt gleichzeitige CLI-Aufrufe (bun) beim Review.
const PARALLEL: usize = 4;

/// Ordnet die Einträge: Fehler zuerst, dann Änderungen, neue und gelöschte (B-701 D-Zeile zur Reihenfolge).
fn rang(status: &str) -> u8 {
    match status {
        STATUS_GEAENDERT => 1,
        STATUS_NEU => 2,
        STATUS_GELOESCHT => 3,
        _ => 0,
    }
}

/// Gemeinsamer Zustand eines Reviews.
struct Vergleich<'a> {
    root: &'a Path,
    /// Dateien der Basis (Specs, Manifeste, Stil).
    alt: Dateien,
    stil: StilAenderung,
    stil_alt: Vec<u8>,
}

/// Vergleicht den Arbeitsstand unter `root` mit der Basis (leer = `HEAD`) und liefert je geändertem Asset einen Eintrag
/// laut Vertrag (paper-mcp.md › Oberfläche).
pub fn vergleich(root: &Path, basis: &str, r: &dyn Renderer) -> Result<Ergebnis, Error> {
    let stand = aufloesen(root, basis)?;
    let v = neuer_vergleich(root, &stand.commit)?;
    let mut pfade: BTreeSet<String> = specs_im_arbeitsstand(root)?.into_iter().collect();
    pfade.extend(v.alt.keys().filter(|p| ist_spec(p)).cloned());
    let pfade: Vec<String> = pfade.into_iter().collect();
    let mut assets = v.alle_eintraege(r, &pfade)?;
    assets.sort_by(|a, b| rang(&a.status).cmp(&rang(&b.status)).then_with(|| a.id.cmp(&b.id)));
    Ok(Ergebnis {
        basis: stand.basis,
        basis_commit: stand.commit,
        basis_stand: stand.basis_stand,
        assets,
    })
}

fn neuer_vergleich<'a>(root: &'a Path, commit: &str) -> Result<Vergleich<'a>, Error> {
    let alt = basis_dateien(root, commit)?;
    let fehlt = |grund: String| Error::Sonst(format!("{STIL_PFAD} fehlt im Arbeitsstand: {grund}"));
    let stil_neu = lies_arbeitsstand(root, STIL_PFAD)
        .map_err(|e| fehlt(e.to_string()))?
        .ok_or_else(|| fehlt("<nil>".to_owned()))?;
    // Basis ohne Stil-Datei: der Stil gilt als unverändert.
    let stil_alt = alt.get(STIL_PFAD).unwrap_or(&stil_neu).clone();
    let stil = vergleiche_stil(&stil_alt, &stil_neu)?;
    Ok(Vergleich {
        root,
        alt,
        stil,
        stil_alt,
    })
}

fn rig_von(spec: &[u8]) -> String {
    let wert = hash::lies_wert(spec).unwrap_or_default();
    wert.get("rig").and_then(Value::as_str).unwrap_or_default().to_owned()
}

impl Vergleich<'_> {
    /// Bildet die Einträge parallel (höchstens [`PARALLEL`] gleichzeitig); Fehler werden gesammelt.
    fn alle_eintraege(&self, r: &dyn Renderer, pfade: &[String]) -> Result<Vec<Asset>, Error> {
        let naechster = AtomicUsize::new(0);
        let fertig = Mutex::new(Vec::with_capacity(pfade.len()));
        std::thread::scope(|scope| {
            for _ in 0..PARALLEL.min(pfade.len()) {
                scope.spawn(|| {
                    loop {
                        let i = naechster.fetch_add(1, Ordering::Relaxed);
                        let Some(pfad) = pfade.get(i) else {
                            break;
                        };
                        let eintrag = self.eintrag(r, pfad);
                        let mut fertig = fertig.lock().unwrap_or_else(PoisonError::into_inner);
                        fertig.push((i, eintrag));
                    }
                });
            }
        });
        let mut fertig = fertig.into_inner().unwrap_or_else(PoisonError::into_inner);
        fertig.sort_by_key(|(i, _)| *i);
        let (mut assets, mut fehler) = (Vec::new(), Vec::new());
        for (_, eintrag) in fertig {
            match eintrag {
                Ok(a) => assets.extend(a),
                Err(e) => fehler.push(e),
            }
        }
        match fehler.len() {
            0 => Ok(assets),
            1 => Err(fehler.remove(0)),
            _ => {
                let texte: Vec<String> = fehler.iter().map(Error::to_string).collect();
                Err(Error::Sonst(texte.join("\n")))
            }
        }
    }

    /// Der Review-Eintrag eines Spec-Pfads; `None`, wenn sich nichts geändert hat.
    fn eintrag(&self, r: &dyn Renderer, pfad: &str) -> Result<Option<Asset>, Error> {
        let name = pfad.rsplit('/').next().unwrap_or(pfad);
        let mut a = Asset {
            id: name.strip_suffix(ENDUNG).unwrap_or(name).to_owned(),
            pfad: pfad.to_owned(),
            ..Asset::default()
        };
        let qa = match self.quelle_alt(pfad) {
            // Eine kaputte Basis zählt wie eine fehlende.
            Err(Error::Unlesbar(_)) => None,
            andere => andere?,
        };
        let qn = match quelle_arbeitsstand(self.root, pfad) {
            Err(Error::Unlesbar(u)) => {
                a.status = STATUS_UNGUELTIG.to_owned();
                a.fehler = vec![Fehler {
                    feld: String::new(),
                    regel: "schema".to_owned(),
                    text: u.to_string(),
                }];
                return Ok(Some(a));
            }
            andere => andere?,
        };
        let Some(qn) = qn else {
            a.rig = qa.map_or_else(String::new, |q| rig_von(&q.spec));
            a.status = STATUS_GELOESCHT.to_owned();
            a.pixel_anteil = 1.0;
            return Ok(Some(a));
        };
        a.rig = rig_von(&qn.spec);
        let Some(qa) = qa else {
            self.neu(r, &mut a, &qn)?;
            return Ok(Some(a));
        };
        a.stil = self.stil.betrifft(&qn.spec) || self.stil.betrifft(&qa.spec);
        if qa.spec_hash == qn.spec_hash && !a.stil && self.ausgabe_gleich(pfad) {
            return Ok(None);
        }
        self.geaendert(r, &mut a, &qa, &qn)?;
        Ok(Some(a))
    }

    fn quelle_alt(&self, pfad: &str) -> Result<Option<Quelle>, Error> {
        let Some(spec) = self.alt.get(pfad) else {
            return Ok(None);
        };
        let specs: Vec<String> = self.alt.keys().filter(|p| ist_spec(p)).cloned().collect();
        let lies = |p: &str| Ok(self.alt.get(p).cloned());
        cache::neue_quelle(pfad, spec.clone(), self.stil_alt.clone(), true, &specs, &lies).map(Some)
    }

    /// Fehlt in der Basis; ungültig, wenn der Arbeitsstand eine Regel verletzt.
    fn neu(&self, r: &dyn Renderer, a: &mut Asset, qn: &Quelle) -> Result<(), Error> {
        let fehler = r.validate(&qn.spec_datei(self.root, r)?)?;
        if fehler.is_empty() {
            a.status = STATUS_NEU.to_owned();
            a.pixel_anteil = 1.0;
        } else {
            a.status = STATUS_UNGUELTIG.to_owned();
            a.fehler = fehler;
        }
        Ok(())
    }

    /// Felder und Teile aus `diff`; ungültig mit Fehlern (Anteil 0), sonst der Pixel-Anteil von Frame 0.
    fn geaendert(&self, r: &dyn Renderer, a: &mut Asset, qa: &Quelle, qn: &Quelle) -> Result<(), Error> {
        let alt = qa.spec_datei(self.root, r)?;
        let neu = qn.spec_datei(self.root, r)?;
        let d = r.diff(&alt, &neu, None)?;
        a.felder = d.felder;
        a.teile = d.teile;
        let fehler = r.validate(&neu)?;
        if !fehler.is_empty() {
            a.status = STATUS_UNGUELTIG.to_owned();
            a.fehler = fehler;
            return Ok(());
        }
        a.status = STATUS_GEAENDERT.to_owned();
        a.pixel_anteil = pixel_anteil(r, self.root, qa, qn)?;
        Ok(())
    }

    /// Vergleicht das Manifest der Ausgabe (kanonisch) zwischen Basis und Arbeitsstand.
    fn ausgabe_gleich(&self, spec: &str) -> bool {
        let m = manifest_zu(spec);
        match (self.alt.get(&m), lies_arbeitsstand(self.root, &m)) {
            (None, Ok(None)) => true,
            (Some(alt), Ok(Some(neu))) => {
                matches!((spec_hash(alt), spec_hash(&neu)), (Ok(a), Ok(n)) if a == n)
            }
            _ => false,
        }
    }
}

/// Je Spec des Arbeitsstands, ob sie sich gegenüber der Basis geändert hat (Spec, betroffener Stil oder Ausgabe), ohne
/// die CLI zu rufen; Grundlage von `geaendert` im Katalog.
pub fn geaendert(root: &Path, basis: &str) -> Result<BTreeMap<String, bool>, Error> {
    let stand = aufloesen(root, basis)?;
    let v = neuer_vergleich(root, &stand.commit)?;
    let mut out = BTreeMap::new();
    for p in specs_im_arbeitsstand(root)? {
        let (qa, qn) = (v.quelle_alt(&p), quelle_arbeitsstand(root, &p));
        if matches!(qa, Err(Error::Unlesbar(_))) || matches!(qn, Err(Error::Unlesbar(_))) {
            out.insert(p, true);
            continue;
        }
        let (qa, qn) = (qa?, qn?);
        // Zwischen Auflisten und Lesen gelöscht: wie Go (leerer Hash) als geändert.
        let geaendert = match (qa, qn) {
            (Some(qa), Some(qn)) => qa.spec_hash != qn.spec_hash || v.stil.betrifft(&qn.spec) || !v.ausgabe_gleich(&p),
            _ => true,
        };
        out.insert(p, geaendert);
    }
    Ok(out)
}
