//! Sammelaufruf der Render-CLI (B-856, Port von `batch.go`): `batch <schritte.json>` führt die Schritte eines Set in
//! einem bun-Prozess aus. Jeder Schritt wird wie ein Einzelaufruf gedeutet (`review::auswerten`).

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::cli::{Cli, antwort, text};
use super::set::{TempWeg, temp_ordner};
use super::{Error, set::schreibe_datei};
use crate::review;

/// Ein Befehl im Sammelaufruf: `args` wie auf der Kommandozeile; `stopp`: scheitert er, werden alle folgenden
/// übersprungen; `braucht`: Indizes früherer Schritte, die gelingen müssen, sonst wird er übersprungen.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Schritt {
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub stopp: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub braucht: Vec<usize>,
}

/// Antwort eines Schritts: stdout (Exit 0) oder Ablehnung, Aufruffehler bzw. übersprungen.
pub type SchrittErgebnis = Result<Vec<u8>, Error>;

/// Deutet das Ergebnis eines Schritts als JSON.
pub(super) fn deute<T: DeserializeOwned>(ergebnis: &SchrittErgebnis, befehl: &str) -> Result<T, Error> {
    match ergebnis {
        Ok(raw) => antwort(befehl, raw),
        Err(e) => Err(e.clone()),
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Antwort {
    schritte: Option<Vec<SchrittAntwort>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct SchrittAntwort {
    code: Option<i32>,
    antwort: Option<Box<RawValue>>,
    stderr: String,
    uebersprungen: bool,
}

impl Cli {
    /// Führt `schritte` in einem CLI-Aufruf aus. Ein Fehler des Aufrufs selbst (bun fehlt, Zeitlimit, Exit 2) gilt
    /// für alle Schritte; sonst je Schritt ein Ergebnis.
    pub fn batch(&self, schritte: &[Schritt]) -> Result<Vec<SchrittErgebnis>, Error> {
        let tmp = TempWeg(temp_ordner(&self.root)?);
        let datei = tmp.0.join("schritte.json");
        schreibe_datei(&datei, &serde_json::to_vec(schritte)?)?;
        // Drei Einzel-Limits, begrenzt eine hängende CLI weiter.
        let raw = self.run(self.zeit * 3, &["batch".to_owned(), text(&datei)])?;
        let passt_nicht = || {
            let n = schritte.len();
            Error::Sonst(format!("Render-CLI batch: Antwort passt nicht zu {n} Schritten"))
        };
        let gelesen: Antwort = serde_json::from_slice(&raw).map_err(|_| passt_nicht())?;
        let antworten = gelesen.schritte.unwrap_or_default();
        if antworten.len() != schritte.len() {
            return Err(passt_nicht());
        }
        Ok(schritte.iter().zip(antworten).map(ergebnis).collect())
    }
}

fn ergebnis((schritt, a): (&Schritt, SchrittAntwort)) -> SchrittErgebnis {
    let Some(code) = a.code.filter(|_| !a.uebersprungen) else {
        return Err(Error::Sonst("Schritt übersprungen".to_owned()));
    };
    let befehl = schritt.args.first().map_or("", String::as_str);
    let stdout = a.antwort.map_or_else(Vec::new, |r| r.get().as_bytes().to_vec());
    Ok(review::auswerten(
        befehl,
        code,
        stdout,
        &a.stderr,
        &format!("Exit {code}"),
    )?)
}
