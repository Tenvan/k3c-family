//! Render eines Assets und die Typen der Render-Jobs (Port von `job.go`; die Jobs selbst folgen in RU70.11).

use std::time::Duration;

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

/// Rendert Assets im Hintergrund (eigener Mechanismus, nicht `task_*`); IDs `j-<n>` fortlaufend je Prozess.
#[derive(Debug, Clone)]
pub struct Jobs {
    /// Rendert ein Asset; Tests ersetzen es.
    pub render: fn(&Cli, &Asset) -> AssetErgebnis,
}

impl Default for Jobs {
    fn default() -> Self {
        Jobs { render: render_eins }
    }
}

impl Jobs {
    pub fn new() -> Jobs {
        Jobs::default()
    }

    /// Legt einen Job an und kehrt sofort zurück; gerendert wird nacheinander im Hintergrund.
    #[allow(clippy::unimplemented)]
    pub fn start(&self, _cli: &Cli, _assets: &[Asset]) -> Stand {
        unimplemented!("RU70.11")
    }

    /// Der aktuelle Stand eines Jobs.
    #[allow(clippy::unimplemented)]
    pub fn status(&self, _id: &str) -> Result<Stand, Error> {
        unimplemented!("RU70.11")
    }

    /// Wartet bis zum Ende des Jobs.
    #[allow(clippy::unimplemented)]
    pub fn wait(&self, _id: &str) -> Result<Stand, Error> {
        unimplemented!("RU70.11")
    }
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
