//! Merkt sich das Ergebnis von `validate` je Spec, solange Spec, Stil-Datei und die Specs ihrer Reittiere unverändert
//! sind (B-706, Port von `gueltig.go`).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::UNIX_EPOCH;

use super::basis::{Asset, assets};
use super::cli::Cli;
use super::reiter::reittier_stand;
use super::{Error, Fehler};

/// Prüfungen gleichzeitig.
const PLAETZE: usize = 8;

#[derive(Debug, Clone)]
struct Eintrag {
    stand: String,
    fehler: Vec<Fehler>,
}

/// Cache der Gültigkeit je Spec-Pfad.
#[derive(Debug, Default)]
pub struct Gueltigkeit {
    cache: Mutex<BTreeMap<PathBuf, Eintrag>>,
}

pub(super) fn datei_stand(p: &Path) -> String {
    let Ok(meta) = fs::metadata(p) else {
        return "–".to_owned();
    };
    let zeit = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok());
    format!("{}/{}", meta.len(), zeit.map_or(0, |d| d.as_nanos()))
}

fn sperre<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

fn nach_id(root: &Path) -> BTreeMap<String, Asset> {
    let mut out: BTreeMap<String, Asset> = BTreeMap::new();
    for mut a in assets(root).unwrap_or_default() {
        if out.contains_key(&a.id) {
            // Mehrdeutige ID: `reittier_stand` markiert sie, statt einen Treffer zu raten.
            a.pfad = String::new();
        }
        out.insert(a.id.clone(), a);
    }
    out
}

impl Gueltigkeit {
    pub fn new() -> Gueltigkeit {
        Gueltigkeit::default()
    }

    /// Je Asset die Fehler von `validate` (leer = gültig); höchstens acht Prüfungen gleichzeitig.
    pub fn pruefe(&self, cli: &Cli, liste: &[Asset]) -> BTreeMap<String, Vec<Fehler>> {
        let stil = datei_stand(&cli.root.join("public").join("grafik").join("stil.paper.json"));
        let nach_id = nach_id(&cli.root);
        let out = Mutex::new(BTreeMap::new());
        let mut offen = Vec::new();
        for a in liste {
            let pfad = a.abs(&cli.root);
            let reittiere = reittier_stand(&cli.root, &pfad, &nach_id);
            let stand = format!("{}|{stil}{reittiere}", datei_stand(&pfad));
            match sperre(&self.cache).get(&pfad) {
                Some(e) if e.stand == stand => {
                    sperre(&out).insert(a.id.clone(), e.fehler.clone());
                }
                _ => offen.push((a.id.clone(), pfad, stand)),
            }
        }
        let naechster = AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..offen.len().min(PLAETZE) {
                scope.spawn(|| {
                    while let Some((id, pfad, stand)) = offen.get(naechster.fetch_add(1, Ordering::SeqCst)) {
                        let (fehler, sicher) = validiere_eins(cli, pfad);
                        if sicher {
                            let eintrag = Eintrag {
                                stand: stand.clone(),
                                fehler: fehler.clone(),
                            };
                            sperre(&self.cache).insert(pfad.clone(), eintrag);
                        }
                        sperre(&out).insert(id.clone(), fehler);
                    }
                });
            }
        });
        out.into_inner().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Prüft eine Spec; `false` bei Aufruffehlern (nicht cachen).
fn validiere_eins(cli: &Cli, pfad: &Path) -> (Vec<Fehler>, bool) {
    match cli.validate(pfad) {
        Ok(()) => (Vec::new(), true),
        Err(Error::Abgelehnt(fehler)) => (fehler, true),
        Err(e) => {
            let fehler = Fehler {
                feld: String::new(),
                regel: "cli".to_owned(),
                text: e.to_string(),
            };
            (vec![fehler], false)
        }
    }
}
