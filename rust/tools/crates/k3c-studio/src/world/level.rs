//! Engine-Seite und Level für die Chunk-Vorschau (Port von `level.go`). Level und Prüfung rechnet die Spiel-EXE.

use std::sync::Arc;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::value::RawValue;

use super::{Editor, check_id, null_default};
use crate::gameexe::{self, Cancel, Options};

/// Frist eines Aufrufs von `sim world` (Level und Prüfung brauchen Millisekunden).
const ENGINE_TIMEOUT: Duration = Duration::from_secs(30);

/// Die Engine-Seite des Editors: Eingabe ist das JSON von `k3c-game sim world` (`{"op": …}`), die Antwort sein report
/// (`None`: der Lauf nannte keinen). Ein Fehler ist die Meldung der Spiel-EXE oder ein Startfehler. Blockiert.
pub type Engine = Arc<dyn Fn(Vec<u8>) -> Result<Option<Box<RawValue>>, String> + Send + Sync>;

/// Baut die Engine für den Checkout eines Aufrufs; Studio und MCP nehmen [`exe_factory`], Tests einen Ersatz (Gos
/// `worldEngine`).
pub type EngineFactory = Arc<dyn Fn(&std::path::Path) -> Engine + Send + Sync>;

/// [`exe`] als [`EngineFactory`].
pub fn exe_factory() -> EngineFactory {
    Arc::new(|root| exe(root))
}

/// Die Engine über die Spiel-EXE des Checkouts `root`; ist sie nicht gebaut oder veraltet, nennt der Fehler den
/// Bau-Befehl.
pub fn exe(root: impl Into<std::path::PathBuf>) -> Engine {
    let root = root.into();
    Arc::new(move |input| {
        let options = Options {
            root: root.clone(),
            sub: "world".to_owned(),
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

/// Das Level wie im Spiel plus die geplanten Chunks der Stufe und die Warnungen der Spielbarkeits-Prüfung. `error` ist
/// der Hinweis für die Seite, dann fehlt das Layout.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct LevelView {
    #[serde(deserialize_with = "null_default")]
    pub seed: String,
    #[serde(deserialize_with = "null_default")]
    pub biome_id: String,
    #[serde(deserialize_with = "null_default")]
    pub width_units: f64,
    #[serde(deserialize_with = "null_default")]
    pub chunk_width_units: f64,
    #[serde(deserialize_with = "null_default")]
    pub hub_center_units: f64,
    #[serde(deserialize_with = "null_default")]
    pub chunks: Vec<Box<RawValue>>,
    #[serde(deserialize_with = "null_default")]
    pub entities: Vec<Box<RawValue>>,
    #[serde(deserialize_with = "null_default")]
    pub planned: Vec<Box<RawValue>>,
    #[serde(deserialize_with = "null_default")]
    pub warnings: Vec<String>,
    #[serde(deserialize_with = "null_default")]
    pub error: String,
    /// Natur (B-891): geltende Werte, fest gesetzte Werte, Eigenschaften und Gewichts-Gruppen des Bioms.
    pub nature: Box<RawValue>,
    pub nature_fixed: Box<RawValue>,
    pub nature_traits: Box<RawValue>,
    pub nature_weights: Box<RawValue>,
    /// Grund, warum ein Festsetzen abgelehnt wurde; das Level ist dann das alte.
    #[serde(skip_serializing_if = "String::is_empty", deserialize_with = "null_default")]
    pub rejected: String,
}

impl Default for LevelView {
    fn default() -> Self {
        let null = || RawValue::NULL.to_owned();
        LevelView {
            seed: String::new(),
            biome_id: String::new(),
            width_units: 0.0,
            chunk_width_units: 0.0,
            hub_center_units: 0.0,
            chunks: Vec::new(),
            entities: Vec::new(),
            planned: Vec::new(),
            warnings: Vec::new(),
            error: String::new(),
            nature: null(),
            nature_fixed: null(),
            nature_traits: null(),
            nature_weights: null(),
            rejected: String::new(),
        }
    }
}

/// Ersetzt `null` durch das leere Objekt bzw. die leere Liste.
fn fill(value: &mut Box<RawValue>, empty: &str) {
    if value.get() == "null"
        && let Ok(raw) = RawValue::from_string(empty.to_owned())
    {
        *value = raw;
    }
}

impl Editor {
    /// Schickt eine op an die Engine und liest den report.
    pub(super) fn call<T: DeserializeOwned + Default>(&self, request: &impl Serialize) -> Result<T, String> {
        let input = serde_json::to_vec(request).map_err(|e| e.to_string())?;
        let report = (self.engine)(input)?.ok_or_else(|| "unexpected end of JSON input".to_owned())?;
        let value = serde_json::from_str::<Option<T>>(report.get()).map_err(|e| e.to_string())?;
        Ok(value.unwrap_or_default())
    }

    /// Das Level der Insel `island` in der Stufe `stage` (Index in ihren Tiefen) der Welt `id` zum Seed, mit der
    /// gespeicherten Planung und Chunk-Anzahl der Tiefe. Der Level-Generator ist deterministisch.
    pub fn level(&self, id: &str, island: &str, stage: i64, seed: &str) -> LevelView {
        let request = json!({"op": "level", "world": id, "island": island, "stage": stage, "seed": seed});
        let result = check_id(id).and_then(|()| self.call::<LevelView>(&request));
        let mut view = result.unwrap_or_else(|error| LevelView {
            error,
            ..LevelView::default()
        });
        fill(&mut view.nature, "{}");
        fill(&mut view.nature_fixed, "{}");
        fill(&mut view.nature_traits, "[]");
        fill(&mut view.nature_weights, "[]");
        view
    }
}
