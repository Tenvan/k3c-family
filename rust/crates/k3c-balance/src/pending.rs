//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::OnceLock;

use k3c_data::{Data, Source, load_all};
use k3c_level::rules::Rules;
use k3c_sim::island::{
    CreateOptions, Island, WorldDef, create_island as sim_create_island, create_island_in,
    load_world as sim_load_world, standard_world,
};
use k3c_sim::state::PlayerCommand;
use k3c_sim::world::values_room::resolve_world;
use serde::{Deserialize, Serialize};

use crate::fehler::Fehler;
use crate::hints::Hints;
use crate::run::Scenario;

/// RU31.1 (`insights.go`): die Ampel eines Ziels.
pub const LIGHT_GREEN: &str = "green";
/// RU31.1 (`insights.go`).
pub const LIGHT_YELLOW: &str = "yellow";
/// RU31.1 (`insights.go`).
pub const LIGHT_RED: &str = "red";

/// RU28.2 (`groups.go`): die bekannten Werte-Gruppen (Go: `Groups`).
pub const GROUPS: [&str; 3] = ["basis", "holz", "schiff"];

/// RU28.2 (`groups.go`): `name` ist eine bekannte Gruppe (Go: `IsGroup`).
pub fn is_group(name: &str) -> bool {
    GROUPS.contains(&name)
}

/// RU28.3 (`summary.go`, Go: `Entry` mit eingebettetem `Verdict`): ein bewertetes Ziel; nur die Felder, die
/// `Hints::levers_for` liest.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Entry {
    pub measure: String,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub value: f64,
}

/// RU28.3 (`targets.go` und `economy_measures.go`): `key` ist eine Kennzahl der Tabelle `measures`.
pub fn measure_known(key: &str) -> bool {
    matches!(
        key,
        "castleHeld"
            | "firstWallBeforeDusk1"
            | "endbossDefeated"
            | "endbossDay"
            | "caveBeforeDay6"
            | "destroyedPerWave"
            | "goldAtDawnPerPlayer"
            | "firstTowerBeforeDusk2"
            | "woodAtDawnFrom3"
            | "hubLevel2BeforeDay5"
            | "hubLevel3BeforeDay10"
            | "fightersAtDawn"
    )
}

impl Hints {
    /// RU28.2 (`groups.go`): verlangt in der Datei der Richtwerte eine bekannte Gruppe an jedem Hebel (Go: `checkGroups`).
    pub fn check_groups(&self) -> Result<(), String> {
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                if !is_group(&lv.group) {
                    return Err(format!(
                        "hebel {key}, pfad {}: group {:?}, erlaubt: {}",
                        lv.path,
                        lv.group,
                        GROUPS.join(", ")
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Die Daten, die Go als Paket-Variablen hält (`data.Files`, Regeln, Standard-Welt), einmal gelesen.
pub struct Shared {
    pub source: Source,
    pub data: Data,
    pub rules: Rules,
    pub standard: WorldDef,
}

static SHARED: OnceLock<Result<Shared, Fehler>> = OnceLock::new();

fn load_shared() -> Result<Shared, Fehler> {
    let wrap = |e: String| Fehler::Daten(format!("balance: {e}"));
    let source = Source::from_env();
    let data = load_all(&source).map_err(|e| wrap(e.to_string()))?;
    let hub = source.read("hub.json").map_err(|e| wrap(e.to_string()))?;
    let rules = Rules::from_data(&data, &hub).map_err(|e| wrap(e.to_string()))?;
    let standard = standard_world(&data, &rules).map_err(wrap)?;
    Ok(Shared {
        source,
        data,
        rules,
        standard,
    })
}

/// RU25.5 (`island_world.go`, `replay.go`): die geteilten Daten; der Fehler beim Lesen bleibt für alle Aufrufer derselbe.
pub fn shared() -> Result<&'static Shared, Fehler> {
    SHARED.get_or_init(load_shared).as_ref().map_err(Clone::clone)
}

/// RU25.5 (`island_world.go`): lädt eine Welt (Go: `loadWorld`); `standard` ist die eingebettete Standard-Welt.
pub fn load_world(id: &str) -> Result<WorldDef, Fehler> {
    let sh = shared()?;
    if id == "standard" {
        return Ok(sh.standard.clone());
    }
    sim_load_world(&sh.source, id, &sh.data, &sh.rules).map_err(Fehler::Sim)
}

/// RU25.5 (`island_world.go`): baut die Insel des Szenarios: ohne Welt eine Stufe der Tiefe, mit Welt alle Tiefen der
/// ersten Insel (Go: `createIsland` und `createIn`).
pub fn create_island(sc: &Scenario) -> Result<Island, Fehler> {
    let sh = shared()?;
    if sc.world.is_empty() {
        return sim_create_island(
            &sh.data,
            &sh.standard,
            &sc.seed,
            &[sc.depth],
            1.0,
            CreateOptions::default(),
        )
        .map_err(Fehler::Sim);
    }
    let world = load_world(&sc.world)?;
    if world.islands.is_empty() {
        return Err(Fehler::Sim(format!("welt {:?} hat keine Insel", sc.world)));
    }
    let lookup = |id: &str| load_world(id).map_err(|e| e.to_string());
    let world = resolve_world(&world, &sh.source, &sh.data, &sh.rules, &lookup).map_err(Fehler::Sim)?;
    let first = &world.islands[0];
    let opts = first.create_options(&sh.data.biomes);
    create_island_in(&sh.data, &world, &sc.seed, &first.depths, 1.0, opts).map_err(Fehler::Sim)
}

/// RU25.5 (`replay.go`): die Version des Replay-Formats.
pub const REPLAY_VERSION: i64 = 1;

/// RU25.5 (`replay.go`): ein Befehl, der `n` Ticks hintereinander gilt.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub n: i64,
    pub command: PlayerCommand,
}

/// RU25.5 (`replay.go`): ein Lauf als Szenario, Datenstand-Hash und Eingaben je Tick (nur die Felder, die `run.rs` setzt).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Replay {
    pub version: i64,
    pub scenario: Scenario,
    pub data_hash: String,
    pub ticks: i64,
    pub end_hash: String,
    pub castle_fall_tick: Option<i64>,
    pub inputs: Vec<Vec<Span>>,
}

impl Replay {
    /// Eine leere Aufnahme des Szenarios (Go: `&Replay{Version: ReplayVersion, Scenario: sc, DataHash: dataHash}`).
    pub fn new(sc: &Scenario) -> Self {
        Replay {
            version: REPLAY_VERSION,
            scenario: sc.clone(),
            data_hash: data_hash(),
            ..Replay::default()
        }
    }

    /// Hängt die Befehle eines Ticks an die Lauflängen an (Go: `record`).
    pub fn record(&mut self, cmds: &[PlayerCommand]) {
        if self.inputs.is_empty() {
            self.inputs = vec![Vec::new(); cmds.len()];
        }
        for (spans, c) in self.inputs.iter_mut().zip(cmds) {
            match spans.last_mut() {
                Some(last) if last.command == *c => last.n += 1,
                _ => spans.push(Span {
                    n: 1,
                    command: c.clone(),
                }),
            }
        }
        self.ticks += 1;
    }
}

/// RU25.5 (`replay.go`): der Hash der Daten (Go: `dataHash`); hier leer.
pub fn data_hash() -> String {
    String::new()
}

/// RU25.5 (`replay.go`): der Hash des Inselzustands (Go: SHA-256 des JSON). Hier ein Hash der Debug-Ausgabe, der für den
/// Vergleich zweier Läufe im selben Prozess genügt.
pub fn end_hash(isl: &Island) -> String {
    let mut h = DefaultHasher::new();
    h.write(format!("{isl:?}").as_bytes());
    format!("{:016x}", h.finish())
}

/// RU28.1 (`metrics.go`, Go: `StageMetrics`): eine Stufe des Laufs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageMetrics {
    pub arrival_day: Option<i64>,
}

/// RU28.1 (`metrics.go`, Go: `BossMetrics`): ein besiegter Boss.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BossMetrics {
    pub boss: String,
    pub day: i64,
}

/// RU28.1 (`metrics.go`, Go: `DawnMetrics`): der Stand zum Tagesbeginn.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DawnMetrics {
    pub day: i64,
    pub gold: Vec<i64>,
    pub hub_level: i64,
    pub fighters: i64,
}

/// RU28.1 (`metrics.go`): die Kennzahlen eines Laufs; nur die Felder, die `run.rs`, `progress.rs` und ihre Tests lesen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lost: String,
    pub first_wall_tick: Option<i64>,
    pub castle_fall_tick: Option<i64>,
    pub endboss_day: Option<i64>,
    pub stages: Vec<StageMetrics>,
    pub bosses: Vec<BossMetrics>,
    pub dawns: Vec<DawnMetrics>,
}

/// RU28.1 (`metrics.go`, Go: `collector`): sammelt die Kennzahlen eines Laufs; hier ohne Messung.
pub struct Collector;

impl Collector {
    /// Go: `newCollector(isl, threshold)`.
    pub fn new(_isl: &Island, _threshold: i64) -> Self {
        Collector
    }

    /// Go: `observe(tick)` (der Sammler hält die Insel in Go; hier kommt sie als Parameter).
    pub fn observe(&mut self, _isl: &Island, _tick: i64) {}

    /// Go: `result()`.
    pub fn result(&mut self, _isl: &Island) -> Metrics {
        Metrics::default()
    }
}
