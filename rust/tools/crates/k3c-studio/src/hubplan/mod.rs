//! Gebäude-Planer (Port von `hubplan.go`, B-923, B-462): liest `data/hub.json` und zeigt die Freischaltung als Matrix
//! Hub-Stufe × Bauplatz. Matrix und Prüfung rechnet `k3c-game sim hub`; Prüfen eines Kandidaten in `hubplan_edit.rs`,
//! Regeltabelle in `hubplan_rules.rs`, Speichern in `hubplan_save.rs`.

mod hubplan_edit;
mod hubplan_rules;
mod hubplan_save;
mod patch;
mod table;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use serde_json::value::RawValue;

pub use hubplan_rules::{HubRulesLine, HubRulesView};
pub use hubplan_save::{HubSaveResult, HubStand};

use crate::gameexe::{self, Cancel, Options};

#[cfg(test)]
#[path = "editor_tests.rs"]
pub(crate) mod editor_tests;
#[cfg(test)]
#[path = "exe_tests.rs"]
mod exe_tests;
#[cfg(test)]
#[path = "hubplan_tests.rs"]
mod tests;

/// Frist eines Aufrufs von `sim hub` (die Prüfung braucht Millisekunden).
const ENGINE_TIMEOUT: Duration = Duration::from_secs(30);

/// Die Quelle des Planers, relativ zur Repo-Wurzel.
const HUB_FILE: &str = "data/hub.json";

/// Die Engine-Seite des Planers: Eingabe ist das JSON von `k3c-game sim hub` (`{"op":"plan","raw":…}`), die Antwort
/// sein report (`None`: der Lauf nannte keinen). Ein Fehler ist die Meldung der Spiel-EXE oder ein Startfehler.
/// Blockiert.
pub type Engine = Arc<dyn Fn(Vec<u8>) -> Result<Option<Box<RawValue>>, String> + Send + Sync>;

/// Baut die Engine für den Checkout eines Aufrufs; das Studio nimmt [`exe_factory`], Tests einen Ersatz.
pub type EngineFactory = Arc<dyn Fn(&Path) -> Engine + Send + Sync>;

/// [`exe`] als [`EngineFactory`].
pub fn exe_factory() -> EngineFactory {
    Arc::new(|root| exe(root))
}

/// Die Engine über die Spiel-EXE des Checkouts `root`; ist sie nicht gebaut oder veraltet, nennt der Fehler den
/// Bau-Befehl.
pub fn exe(root: impl Into<PathBuf>) -> Engine {
    let root = root.into();
    Arc::new(move |input| {
        let options = Options {
            root: root.clone(),
            sub: "hub".to_owned(),
            input,
            ..Options::default()
        };
        call_exe(options)
    })
}

/// [`gameexe::call`] für den blockierenden Aufrufer: eigener Thread mit eigener Laufzeit, damit es aus jedem Kontext
/// geht.
fn call_exe(options: Options) -> Result<Option<Box<RawValue>>, String> {
    std::thread::scope(|scope| {
        let run = scope.spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())?;
            let deadline = async {
                tokio::time::sleep(ENGINE_TIMEOUT).await;
                Cancel::Deadline
            };
            let event = runtime.block_on(gameexe::call(options, deadline));
            event.map(|ev| ev.report).map_err(|e| e.to_string())
        });
        run.join()
            .unwrap_or_else(|_| Err("Aufruf der Spiel-EXE abgestürzt".to_owned()))
    })
}

/// Ersetzt `null` durch den Nullwert, wie Gos `json.Unmarshal`.
fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

/// Eine Zelle der Matrix (Hub-Stufe × Bauplatz): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct HubCell {
    #[serde(deserialize_with = "null_default")]
    pub available: bool,
    /// Platz zur Hub-Mitte; bei Mauerlinie und Tor je Seite (±).
    #[serde(deserialize_with = "null_default")]
    pub offset_units: f64,
    #[serde(deserialize_with = "null_default")]
    pub from_depth: i64,
    #[serde(deserialize_with = "null_default")]
    pub needs_deeper: bool,
    /// Nur in dieser Tiefe (Händler); -1 = jede Tiefe.
    #[serde(deserialize_with = "null_default")]
    pub only_depth: i64,
}

/// Ein Bauplatz der Matrix: Hub-Platz, Insel-Platz, Mauerlinie (mit Turm), Tor oder Händler.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct HubRow {
    /// `kind@offset` wie `room.PreviewSiteKey`, Baugrund der Zone 1 `zone1`.
    #[serde(deserialize_with = "null_default")]
    pub key: String,
    /// Art aus hub.json; `wall` (Baugrund der Zone 1), `merchantBuy`, `merchantSell`.
    #[serde(deserialize_with = "null_default")]
    pub kind: String,
    /// `sites`, `islandSites`, `sitePlacement`, `merchant`.
    #[serde(deserialize_with = "null_default")]
    pub source: String,
    /// Eintrag in seiner Liste (`sites`, `islandSites`, `wallUnits`); Händler 0.
    #[serde(deserialize_with = "null_default")]
    pub index: i64,
    /// Ab dieser Hub-Stufe verfügbar.
    #[serde(deserialize_with = "null_default")]
    pub hub_level: i64,
    /// Mauerlinie bzw. Tor: Linie k; sonst 0.
    #[serde(deserialize_with = "null_default")]
    pub line: i64,
    /// Je Seite (±): Mauerlinie und Tor.
    #[serde(deserialize_with = "null_default")]
    pub both_sides: bool,
    /// Mauerlinie: Turm innen; sonst 0.
    #[serde(deserialize_with = "null_default")]
    pub tower_units: f64,
    /// Streuung nach außen je Seed; 0 = fest.
    #[serde(deserialize_with = "null_default")]
    pub jitter_units: f64,
    /// Je Hub-Stufe 1 … levels.
    #[serde(deserialize_with = "null_default")]
    pub cells: Vec<HubCell>,
}

/// Das Matrix-Modell: Spalten = Hub-Stufen aus `levels` (beliebig viele), Zeilen = Bauplätze in der Reihenfolge
/// `sites`, `islandSites`, Mauerlinien und Tore, Händler.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct HubMatrix {
    #[serde(deserialize_with = "null_default")]
    pub levels: i64,
    #[serde(deserialize_with = "null_default")]
    pub rows: Vec<HubRow>,
}

/// Ein Befund der Prüfung mit den JSON-Namen der Oberfläche.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct HubFindingView {
    #[serde(deserialize_with = "null_default")]
    pub rule: String,
    #[serde(deserialize_with = "null_default")]
    pub site: String,
    #[serde(deserialize_with = "null_default")]
    pub detail: String,
}

/// Der geladene Stand des Planers: Pfad, Inhalt (Kandidat der Live-Vorschau), Matrix und Prüfung; `error` nennt eine
/// nicht lesbare oder ungültige Datei mit Pfad, dann ist die Matrix leer. `stand` ist der Inhalt beider Dateien beim
/// Laden (Schreibregel), `kinds` die Arten, die als Platz hinzukommen dürfen.
#[derive(Clone, Debug, Default, Serialize)]
pub struct HubPlanView {
    pub path: String,
    pub raw: String,
    pub matrix: HubMatrix,
    pub findings: Vec<HubFindingView>,
    pub notes: Vec<String>,
    pub error: String,
    pub stand: HubStand,
    pub kinds: Vec<String>,
}

/// Die Antwort von `k3c-game sim hub`.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct EngineReport {
    #[serde(deserialize_with = "null_default")]
    matrix: HubMatrix,
    #[serde(deserialize_with = "null_default")]
    findings: Vec<HubFindingView>,
    #[serde(deserialize_with = "null_default")]
    notes: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    kinds: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    parse_error: String,
}

/// Der Gebäude-Planer eines Checkouts: Repo-Wurzel und Engine-Seite. Methoden: `plan`, `check`, `rules`, `save`.
#[derive(Clone)]
pub struct Editor {
    root: PathBuf,
    engine: Engine,
}

fn empty_plan(rel: &str) -> HubPlanView {
    HubPlanView {
        path: rel.to_owned(),
        ..HubPlanView::default()
    }
}

impl Editor {
    pub fn new(root: impl Into<PathBuf>, engine: Engine) -> Editor {
        Editor {
            root: root.into(),
            engine,
        }
    }

    /// Lädt `data/hub.json` des Checkouts und liefert Matrix und Befunde.
    pub fn plan(&self) -> HubPlanView {
        self.plan_from(HUB_FILE)
    }

    /// Liest `rel` unter der Wurzel (Schrägstriche), prüft über die Engine und merkt den Stand von `rel` und der
    /// Regeltabelle.
    fn plan_from(&self, rel: &str) -> HubPlanView {
        let raw = match std::fs::read(self.root.join(rel)) {
            Ok(raw) => raw,
            Err(e) => {
                return HubPlanView {
                    error: format!("{rel} nicht lesbar: {e}"),
                    ..empty_plan(rel)
                };
            }
        };
        let mut view = self.plan_of(rel, &String::from_utf8_lossy(&raw));
        view.stand = HubStand::of(&self.root, &raw);
        view
    }

    /// Prüft den Inhalt `raw` (Datei oder Kandidat) über die Engine und baut die Matrix; ohne Stand.
    fn plan_of(&self, rel: &str, raw: &str) -> HubPlanView {
        let mut view = empty_plan(rel);
        // Auch bei Fehler: das Frontend ordnet die Antwort seinem Kandidaten zu.
        view.raw = raw.to_owned();
        let report = match self.ask(raw) {
            Ok(report) => report,
            Err(e) => {
                view.error = format!("{rel} nicht geprüft: {e}");
                return view;
            }
        };
        view.kinds = report.kinds;
        if !report.parse_error.is_empty() {
            view.error = format!("{rel} nicht lesbar als hub.json: {}", report.parse_error);
            return view;
        }
        view.matrix = report.matrix;
        view.findings = report.findings;
        view.notes = report.notes;
        view
    }

    /// Schickt `raw` an die Engine.
    fn ask(&self, raw: &str) -> Result<EngineReport, String> {
        let input = serde_json::to_vec(&json!({"op": "plan", "raw": raw})).map_err(|e| e.to_string())?;
        let report = (self.engine)(input)?.ok_or_else(|| "unexpected end of JSON input".to_owned())?;
        let value = serde_json::from_str::<Option<EngineReport>>(report.get()).map_err(|e| e.to_string())?;
        Ok(value.unwrap_or_default())
    }
}
