//! Katalog (B-732): `AssetsIndex` liefert die Liste sofort (Prüfergebnis nur aus dem Memo), `AssetsCheck` prüft
//! Häppchen nach; `AssetsList` ist beides in einem Aufruf (Vertrag docs/standards/paper-mcp.md › Oberfläche).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use serde::Serialize;
use serde_json::Value;

use super::{Dienst, Fehler, PARALLEL, datei, id_von};
use crate::review::{self, Quelle};

#[cfg(test)]
#[path = "katalog_tests.rs"]
mod tests;

/// Höchste Zahl der Pfade je `AssetsCheck` (die Oberfläche schickt Häppchen zu 8).
const MAX_CHECK: usize = 64;

/// Eine Zeile des Katalogs.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct AssetEintrag {
    pub id: String,
    pub rig: String,
    pub gruppe: String,
    pub pfad: String,
    pub gueltig: bool,
    pub geaendert: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fehler: Vec<Fehler>,
    /// Noch ungeprüft (nur `AssetsIndex`, B-732); gilt bis zur Prüfung als gültig.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub offen: bool,
}

/// Prüfergebnisse je Spec-, Stil-Hash und Renderer: der Katalog prüft beim Neuladen nur Geändertes. Klone teilen den
/// Inhalt.
#[derive(Debug, Clone, Default)]
pub struct PruefMemo(Arc<Mutex<HashMap<String, Vec<Fehler>>>>);

impl PruefMemo {
    fn inhalt(&self) -> MutexGuard<'_, HashMap<String, Vec<Fehler>>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn lies(&self, key: &str) -> Option<Vec<Fehler>> {
        self.inhalt().get(key).cloned()
    }

    fn merke(&self, key: String, fehler: Vec<Fehler>) {
        self.inhalt().insert(key, fehler);
    }
}

fn regel(regel: &str, text: String) -> Vec<Fehler> {
    vec![Fehler {
        feld: String::new(),
        regel: regel.to_owned(),
        text,
    }]
}

fn setze_fehler(e: &mut AssetEintrag, fehler: Vec<Fehler>) {
    e.gueltig = fehler.is_empty();
    e.fehler = fehler;
}

/// Ein Eintrag ohne Prüfung; `fertig` = Spec unlesbar (der Fehler `schema` steht schon drin).
struct Kopf {
    eintrag: AssetEintrag,
    quelle: Quelle,
    fertig: bool,
}

/// Specs mit Änderung gegen `HEAD`; `None` ohne Commit (dann gilt alles als geändert).
type Geaendert = Option<BTreeMap<String, bool>>;

fn ist_geaendert(g: &Geaendert, pfad: &str) -> bool {
    g.as_ref().is_none_or(|g| g.get(pfad).copied().unwrap_or(false))
}

impl Dienst {
    fn specs(&self) -> Result<Vec<String>, String> {
        review::specs_im_arbeitsstand(&self.root).map_err(|e| e.to_string())
    }

    /// Der Katalog: je Spec des Arbeitsstands Rig, Gruppe, Gültigkeit und Änderung gegen `HEAD`.
    pub fn assets_list(&self) -> Result<Vec<AssetEintrag>, String> {
        let pfade = self.specs()?;
        self.pruefe_alle(&pfade, &self.geaendert())
    }

    /// Der Katalog ohne Prüfung: Einträge ohne Memo-Treffer tragen `offen` und gelten als gültig, bis `AssetsCheck` sie
    /// geprüft hat.
    pub fn assets_index(&self) -> Result<Vec<AssetEintrag>, String> {
        let pfade = self.specs()?;
        let geaendert = self.geaendert();
        pfade
            .iter()
            .map(|p| {
                let mut k = self.kopf(p, ist_geaendert(&geaendert, p))?;
                if !k.fertig {
                    match self.memo.lies(&self.memo_key(&k.quelle)) {
                        Some(f) => setze_fehler(&mut k.eintrag, f),
                        None => (k.eintrag.gueltig, k.eintrag.offen) = (true, true),
                    }
                }
                Ok(k.eintrag)
            })
            .collect()
    }

    /// Prüft die Specs unter `pfade` (Pfade wie in `AssetsIndex`); ein Pfad außerhalb des Arbeitsstands wird nicht
    /// geprüft, sein Eintrag trägt den Fehler `fehlt`. `geaendert` bleibt false: der Vergleich mit `HEAD` kostet je
    /// Aufruf fast eine halbe Sekunde und steht schon im Index.
    pub fn assets_check(&self, pfade: &[String]) -> Result<Vec<AssetEintrag>, String> {
        if pfade.len() > MAX_CHECK {
            return Err(format!("höchstens {MAX_CHECK} Pfade je Aufruf, nicht {}", pfade.len()));
        }
        let bekannt: BTreeSet<String> = self.specs()?.into_iter().collect();
        let sortiert: BTreeSet<&String> = pfade.iter().collect();
        let (da, fehlt): (Vec<&String>, Vec<&String>) = sortiert.into_iter().partition(|p| bekannt.contains(*p));
        let da: Vec<String> = da.into_iter().cloned().collect();
        let mut liste = self.pruefe_alle(&da, &Some(BTreeMap::new()))?;
        liste.extend(fehlt.into_iter().map(|p| AssetEintrag {
            id: id_von(p).to_owned(),
            pfad: p.clone(),
            fehler: regel("fehlt", "Spec fehlt im Arbeitsstand".to_owned()),
            ..AssetEintrag::default()
        }));
        Ok(liste)
    }

    fn geaendert(&self) -> Geaendert {
        review::geaendert(&self.root, "HEAD").ok()
    }

    /// Bildet die Einträge, höchstens [`PARALLEL`] gleichzeitig; Fehler werden gesammelt.
    fn pruefe_alle(&self, pfade: &[String], geaendert: &Geaendert) -> Result<Vec<AssetEintrag>, String> {
        let naechster = AtomicUsize::new(0);
        let fertig = Mutex::new(Vec::with_capacity(pfade.len()));
        std::thread::scope(|scope| {
            for _ in 0..PARALLEL.min(pfade.len()) {
                scope.spawn(|| {
                    loop {
                        let i = naechster.fetch_add(1, Ordering::Relaxed);
                        let Some(p) = pfade.get(i) else {
                            break;
                        };
                        let e = self.eintrag(p, ist_geaendert(geaendert, p));
                        fertig.lock().unwrap_or_else(PoisonError::into_inner).push((i, e));
                    }
                });
            }
        });
        let mut fertig = fertig.into_inner().unwrap_or_else(PoisonError::into_inner);
        fertig.sort_by_key(|(i, _)| *i);
        let (mut liste, mut fehler) = (Vec::new(), Vec::new());
        for (_, e) in fertig {
            match e {
                Ok(e) => liste.push(e),
                Err(e) => fehler.push(e),
            }
        }
        if fehler.is_empty() {
            Ok(liste)
        } else {
            Err(fehler.join("\n"))
        }
    }

    fn kopf(&self, pfad: &str, geaendert: bool) -> Result<Kopf, String> {
        let mut k = Kopf {
            eintrag: AssetEintrag {
                id: id_von(pfad).to_owned(),
                gruppe: pfad.split('/').nth(2).unwrap_or_default().to_owned(),
                pfad: pfad.to_owned(),
                geaendert,
                ..AssetEintrag::default()
            },
            quelle: Quelle::default(),
            fertig: false,
        };
        match review::quelle_arbeitsstand(&self.root, pfad) {
            // Zwischen Auflisten und Lesen gelöscht: wie Go leere Quelle, die CLI meldet es.
            Ok(q) => k.quelle = q.unwrap_or_default(),
            Err(review::Error::Unlesbar(u)) => {
                k.eintrag.fehler = regel("schema", u.to_string());
                k.fertig = true;
                return Ok(k);
            }
            Err(e) => return Err(e.to_string()),
        }
        let spec: Value = serde_json::from_slice(&k.quelle.spec).unwrap_or_default();
        spec.get("rig")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .clone_into(&mut k.eintrag.rig);
        Ok(k)
    }

    fn memo_key(&self, q: &Quelle) -> String {
        format!("{}:{}:{}", q.spec_hash, q.stil_hash, self.r.kennung())
    }

    fn eintrag(&self, pfad: &str, geaendert: bool) -> Result<AssetEintrag, String> {
        let Kopf {
            mut eintrag,
            quelle,
            fertig,
        } = self.kopf(pfad, geaendert)?;
        if fertig {
            return Ok(eintrag);
        }
        let key = self.memo_key(&quelle);
        if let Some(f) = self.memo.lies(&key) {
            setze_fehler(&mut eintrag, f);
            return Ok(eintrag);
        }
        match self.r.validate(&datei(&self.root, pfad)) {
            Ok(f) => {
                self.memo.merke(key, f.clone());
                setze_fehler(&mut eintrag, f);
            }
            // Ein Absturz der CLI trifft nur diesen Eintrag, nicht den Katalog.
            Err(e) => eintrag.fehler = regel("cli", e.to_string()),
        }
        Ok(eintrag)
    }
}
