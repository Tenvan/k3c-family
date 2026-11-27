//! Render eines Assets und die Render-Jobs im Hintergrund (Port von `job.go`). Endet der Prozess mitten im Job, bricht
//! er ab; Gruppen-Index und `golden.json` werden nur über Temp-Datei und Umbenennen ersetzt.

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use super::Error;
use super::basis::Asset;
use super::cli::Cli;
use super::golden::gruppe_nachziehen;
use super::set::sperre;

/// Das Ergebnis eines Assets im Render-Job; `uebersprungen` nennt die erste verletzte Regel.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AssetErgebnis {
    pub id: String,
    pub pngs: usize,
    pub dateien: Vec<String>,
    pub manifest: String,
    pub uebersprungen: String,
    pub fehler: String,
}

/// Der Zustand eines Render-Jobs.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stand {
    pub id: String,
    pub n: usize,
    pub erledigt: usize,
    pub aktuell: String,
    pub fertig: bool,
    pub ergebnisse: Vec<AssetErgebnis>,
    pub dauer: Duration,
}

/// Rendert ein Asset (im Job).
pub type Renderer = Arc<dyn Fn(&Cli, &Asset) -> AssetErgebnis + Send + Sync>;

/// Rendert Assets im Hintergrund (eigener Mechanismus, nicht `task_*`); IDs `j-<n>` fortlaufend je Prozess.
pub struct Jobs {
    tabelle: Mutex<Tabelle>,
    /// Tests ersetzen es.
    pub render: Renderer,
}

#[derive(Default)]
struct Tabelle {
    n: u64,
    jobs: BTreeMap<String, Arc<Job>>,
}

struct Job {
    start: Instant,
    stand: Mutex<Stand>,
    fertig: Condvar,
}

fn halte<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Default for Jobs {
    fn default() -> Self {
        Jobs {
            tabelle: Mutex::default(),
            render: Arc::new(render_eins),
        }
    }
}

impl fmt::Debug for Jobs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Jobs").finish_non_exhaustive()
    }
}

impl Jobs {
    pub fn new() -> Jobs {
        Jobs::default()
    }

    /// Legt einen Job an und kehrt sofort zurück; gerendert wird nacheinander im Hintergrund.
    pub fn start(&self, cli: &Cli, assets: &[Asset]) -> Stand {
        let job = self.neu(assets);
        let erster = job.kopie();
        let (lauf, cli, liste) = (Arc::clone(&job), cli.clone(), assets.to_vec());
        let render = Arc::clone(&self.render);
        let thread = thread::Builder::new().name(erster.id.clone());
        if let Err(e) = thread.spawn(move || lauf.laufe(&render, &cli, &liste)) {
            job.beende(&format!("Job nicht gestartet: {e}"), assets);
            return job.kopie();
        }
        erster
    }

    fn neu(&self, assets: &[Asset]) -> Arc<Job> {
        let mut tabelle = halte(&self.tabelle);
        tabelle.n += 1;
        let stand = Stand {
            id: format!("j-{}", tabelle.n),
            n: assets.len(),
            aktuell: assets.first().map(|a| a.id.clone()).unwrap_or_default(),
            ..Stand::default()
        };
        let id = stand.id.clone();
        let job = Arc::new(Job {
            start: Instant::now(),
            stand: Mutex::new(stand),
            fertig: Condvar::new(),
        });
        tabelle.jobs.insert(id, Arc::clone(&job));
        job
    }

    fn finde(&self, id: &str) -> Result<Arc<Job>, Error> {
        let job = halte(&self.tabelle).jobs.get(id).cloned();
        job.ok_or_else(|| Error::Sonst(format!("Job „{id}“ unbekannt")))
    }

    /// Der aktuelle Stand eines Jobs.
    pub fn status(&self, id: &str) -> Result<Stand, Error> {
        Ok(self.finde(id)?.kopie())
    }

    /// Wartet bis zum Ende des Jobs, höchstens `frist` lang (`None`: ohne Grenze). Läuft die Frist ab, endet nur das
    /// Warten: Der Stand kommt mit `fertig == false` zurück und der Job läuft weiter. Blockiert den Thread.
    pub fn wait(&self, id: &str, frist: Option<Duration>) -> Result<Stand, Error> {
        let job = self.finde(id)?;
        let stand = halte(&job.stand);
        let laeuft = |s: &mut Stand| !s.fertig;
        match frist {
            Some(frist) => drop(job.fertig.wait_timeout_while(stand, frist, laeuft)),
            None => drop(job.fertig.wait_while(stand, laeuft)),
        }
        Ok(job.kopie())
    }
}

impl Job {
    fn kopie(&self) -> Stand {
        let mut s = halte(&self.stand).clone();
        if !s.fertig {
            s.dauer = self.start.elapsed();
        }
        s
    }

    fn laufe(&self, render: &Renderer, cli: &Cli, assets: &[Asset]) {
        for a in assets {
            halte(&self.stand).aktuell.clone_from(&a.id);
            let e = sicher(render, cli, a);
            let mut stand = halte(&self.stand);
            stand.ergebnisse.push(e);
            stand.erledigt += 1;
        }
        self.schliesse();
    }

    fn schliesse(&self) {
        let mut stand = halte(&self.stand);
        stand.fertig = true;
        stand.aktuell.clear();
        stand.dauer = self.start.elapsed();
        self.fertig.notify_all();
    }

    /// Ohne Thread gilt jedes Asset als gescheitert, damit niemand auf den Job wartet.
    fn beende(&self, fehler: &str, assets: &[Asset]) {
        {
            let mut stand = halte(&self.stand);
            stand.ergebnisse = assets.iter().map(|a| gescheitert(a, fehler.to_owned())).collect();
            stand.erledigt = assets.len();
        }
        self.schliesse();
    }
}

fn gescheitert(a: &Asset, fehler: String) -> AssetErgebnis {
    AssetErgebnis {
        id: a.id.clone(),
        fehler,
        ..AssetErgebnis::default()
    }
}

fn panik_text(p: &(dyn Any + Send)) -> &str {
    let text = p.downcast_ref::<&str>().copied();
    text.or_else(|| p.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("unbekannt")
}

/// Rendert ein Asset; eine Panic wird zum Fehler des Assets statt den Job abzubrechen.
fn sicher(render: &Renderer, cli: &Cli, a: &Asset) -> AssetErgebnis {
    catch_unwind(AssertUnwindSafe(|| render(cli, a)))
        .unwrap_or_else(|p| gescheitert(a, format!("Absturz: {}", panik_text(p.as_ref()))))
}

/// Rendert ein Asset; eine ungültige Spec wird mit ihrer ersten Regel übersprungen.
pub fn render_eins(cli: &Cli, a: &Asset) -> AssetErgebnis {
    let mut e = AssetErgebnis {
        id: a.id.clone(),
        ..AssetErgebnis::default()
    };
    let _sperre = sperre(&cli.root, &a.id);
    let r = match cli.render(&a.abs(&cli.root)) {
        Ok(r) => r,
        Err(Error::Abgelehnt(fehler)) => {
            e.uebersprungen = fehler.first().map(|f| f.regel.clone()).unwrap_or_default();
            return e;
        }
        Err(fehler) => {
            e.fehler = fehler.to_string();
            return e;
        }
    };
    if let Err(fehler) = gruppe_nachziehen(&cli.root, a) {
        e.fehler = fehler.to_string();
        return e;
    }
    e.pngs = r.dateien.iter().filter(|d| d.ends_with(".png")).count();
    e.manifest = r.manifeste.into_iter().next().unwrap_or_default();
    e.dateien = r.dateien;
    e
}

#[cfg(test)]
#[path = "job_tests.rs"]
mod tests;
