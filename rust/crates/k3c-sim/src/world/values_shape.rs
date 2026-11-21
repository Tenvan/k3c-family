//! Formtabelle der übersteuerbaren Basis-Dateien (RU14.4): ersetzt Gos `reflect.Type` aus `overrideFiles`, gegen den
//! `merge_file` ein Override prüft. Je Lese-Typ in `k3c_data` steht eine handgeschriebene Form: Felder mit ihrem
//! JSON-Namen (wie das `serde(rename)` dort; verglichen ohne Groß-/Kleinschreibung wie Gos `fieldFold`), Maps, Listen,
//! feste Listen (Go-Arrays wie `[2]int`) und die Skalare Ganzzahl, Zahl, Text, Wahrheitswert.
//!
//! Abweichungen von der Vorentscheidung: `Number` ist in `Int` und `Float` geteilt (Go meldet „Ganzzahl erwartet“ und
//! „Zahl erwartet“ getrennt), `Array` hält die feste Länge (Go: `reflect.Array`, „genau 2 Werte erwartet“), und `Any`
//! entfällt (kein übersteuerbarer Lese-Typ hat ein Feld vom Typ `any`). Ein in Go eingebettetes Feld (`VeinData` bettet
//! `Gatherable` ein, in Rust `serde(flatten)`) steht wie in Gos `t.Fields()` unter seinem Typnamen (`Gatherable`).
//!
//! Die Tabellen sind gegen die echten Basis-Dateien und die Lese-Typen getestet (`values_shape_tests.rs`).

use super::values_shape_buildings::{buildings, economy};
use super::values_shape_monarch::monarch;

/// Ein Feld eines Objekts: JSON-Name und Form des Werts.
pub type Field = (&'static str, fn() -> Shape);

/// Die Form eines JSON-Werts im Lese-Typ (Go: `reflect.Type`, soweit `mergeValue` sie unterscheidet).
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Objekt mit festen Feldern (Go: `reflect.Struct`).
    Struct(&'static [Field]),
    /// Einträge je ID (Go: `reflect.Map`).
    Map(fn() -> Shape),
    /// Liste beliebiger Länge (Go: `reflect.Slice`).
    List(fn() -> Shape),
    /// Liste fester Länge (Go: `reflect.Array`).
    Array(usize, fn() -> Shape),
    /// Ganzzahl (Go: `int`).
    Int,
    /// Zahl (Go: `float64`).
    Float,
    /// Text (Go: `string`).
    Text,
    /// Wahrheitswert (Go: `bool`).
    Bool,
}

/// Go: `overrideFiles[name]`: die Form der Basis-Datei `name` (ohne `.json`); `None` = nicht übersteuerbar.
pub fn override_shape(name: &str) -> Option<Shape> {
    let f: fn() -> Shape = match name {
        "buildings" => || Shape::Map(buildings),
        "ship" => ship_file,
        "troops" => || Shape::Map(troop),
        "economy" => economy,
        "hub" => hub,
        "monarch" => monarch,
        "enemies" => || Shape::Map(enemy),
        "waves" => waves,
        "goals" => goals,
        "events" => events_file,
        _ => return None,
    };
    Some(f())
}

// Skalare und einfache Listen.

pub fn int() -> Shape {
    Shape::Int
}

pub fn float() -> Shape {
    Shape::Float
}

pub fn text() -> Shape {
    Shape::Text
}

pub fn boolean() -> Shape {
    Shape::Bool
}

pub fn texts() -> Shape {
    Shape::List(text)
}

/// Go: `[2]int` (Gold-Spannen, Wellen-Anzahlen).
pub fn int_pair() -> Shape {
    Shape::Array(2, int)
}

// goals.json, ship.json, events.json

fn goals() -> Shape {
    Shape::Struct(&[("Gold", int), ("Days", int)])
}

fn ship_file() -> Shape {
    Shape::Struct(&[("Stages", || Shape::List(ship_stage))])
}

fn ship_stage() -> Shape {
    Shape::Struct(&[("Cost", cost), ("HP", float), ("BuildSeconds", float)])
}

fn events_file() -> Shape {
    Shape::Struct(&[("Events", || Shape::List(night_event))])
}

const NIGHT_EVENT: &[Field] = &[
    ("ID", text),
    ("EveryNights", int),
    ("Kind", text),
    ("WolfShare", float),
    ("Elite", text),
    ("RewardGold", int),
    ("DamageFactor", float),
    ("DropFactor", float),
    ("EveryVisits", int),
    ("RewardMaterial", int),
];

fn night_event() -> Shape {
    Shape::Struct(NIGHT_EVENT)
}

// Gemeinsame Typen (buildings.json, hub.json, troops.json, ship.json).

const COST: &[Field] = &[
    ("Gold", int),
    ("Wood", int),
    ("Stone", int),
    ("Copper", int),
    ("Iron", int),
    ("Crystal", int),
];

/// Go: `Cost`.
pub fn cost() -> Shape {
    Shape::Struct(COST)
}

/// Go: `LevelData` (buildings.json › levels, hub.json › levels); `Steps` ist rekursiv.
pub fn level_data() -> Shape {
    Shape::Struct(&[
        ("Cost", cost),
        ("Gate", cost),
        ("HP", float),
        ("BuildSeconds", float),
        ("Steps", || Shape::List(level_data)),
    ])
}

// troops.json

const TROOP: &[Field] = &[
    ("HP", float),
    ("Speed", float),
    ("Damage", float),
    ("Range", float),
    ("AttacksPerSecond", float),
    ("Cost", cost),
    ("RecruitCost", cost),
    ("Professions", || Shape::Map(profession)),
    ("PostUnits", float),
    ("UpgradeFrom", text),
    ("FetchEnemyDistance", float),
];

fn troop() -> Shape {
    Shape::Struct(TROOP)
}

fn profession() -> Shape {
    Shape::Struct(&[
        ("Cost", cost),
        ("WorkFactor", float),
        ("Resources", texts),
        ("MaxPerBuilding", int),
    ])
}

// waves.json

const WAVES: &[Field] = &[
    ("Table", || Shape::List(waves_row)),
    ("SpawnSpreadSeconds", float),
    ("PerExtraPlayer", float),
    ("BossHpPerExtraPlayer", float),
    ("DepthScaling", depth_scaling),
    ("StealGold", int),
];

fn waves() -> Shape {
    Shape::Struct(WAVES)
}

fn waves_row() -> Shape {
    Shape::Struct(&[("FromWave", int), ("Standard", int_pair), ("Elite", int_pair)])
}

fn depth_scaling() -> Shape {
    Shape::Struct(&[("HP", float), ("Damage", float), ("Speed", float)])
}

// enemies.json

const ENEMY: &[Field] = &[
    ("Name", text),
    ("Tier", text),
    ("HP", float),
    ("Damage", float),
    ("Speed", float),
    ("Range", float),
    ("Gold", int_pair),
    ("Traits", texts),
    ("AttacksPerSecond", float),
    ("Aoe", || {
        Shape::Struct(&[("Radius", float), ("IntervalSeconds", float)])
    }),
    ("SwarmSize", int),
    ("Phases", || {
        Shape::Struct(&[("EverySeconds", float), ("DurationSeconds", float)])
    }),
    ("KiteDistance", float),
    ("GrabRange", float),
];

fn enemy() -> Shape {
    Shape::Struct(ENEMY)
}

// hub.json

const HUB: &[Field] = &[
    ("Levels", || Shape::List(level_data)),
    ("Sites", || Shape::List(hub_site)),
    ("IslandSites", || Shape::List(hub_site)),
    ("IslandStartStock", stock),
    ("SitePlacement", site_placement),
    ("Merchant", hub_merchant),
    ("CastleRadiusUnits", float),
    ("HomeRadiusUnits", float),
    ("StartTroops", start_troops),
    ("CastleSpareHammer", boolean),
    ("Travel", || Shape::Struct(&[("RangeUnits", float), ("Seconds", float)])),
];

fn hub() -> Shape {
    Shape::Struct(HUB)
}

fn hub_site() -> Shape {
    Shape::Struct(&[
        ("Kind", text),
        ("OffsetUnits", float),
        ("FromDepth", int),
        ("NeedsDeeper", boolean),
        ("HubLevel", int),
        ("TopOnly", boolean),
    ])
}

fn stock() -> Shape {
    Shape::Struct(&[
        ("wood", int),
        ("stone", int),
        ("copper", int),
        ("iron", int),
        ("crystal", int),
    ])
}

fn site_placement() -> Shape {
    Shape::Struct(&[
        ("Zone1WallUnits", float),
        ("Zone1TowerUnits", float),
        ("OutsideUnits", float),
    ])
}

fn hub_merchant() -> Shape {
    Shape::Struct(&[
        ("BuyOffsetUnits", float),
        ("SellOffsetUnits", float),
        ("OnlyDepth", int),
    ])
}

fn start_troops() -> Shape {
    Shape::Struct(&[("Peasant", int), ("Craftsman", int), ("Archer", int)])
}
