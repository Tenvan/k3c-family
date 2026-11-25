//! Variation eines Werts aus data/ (Port von `engine/balance/vary.go`, BAL3.3): Der Pfad `datei.json:a.b.c` zeigt auf eine
//! Zahl (Listen über den Index, z. B. `buildings.json:wall.levels.0.cost.gold`). Der Wert wird nur im Speicher geändert;
//! data/*.json bleiben unberührt.
//!
//! Go ersetzt mit `sim.UseData(fsys)` die Paket-Variablen von Sim und Bot-Preisen und stellt sie danach wieder her. Rust hat
//! keinen solchen Zustand (`k3c-sim` bekommt `Data` und `Rules` als Parameter); hier hält ein Thread-Wert die Daten der
//! Variante (`with_data`), `island_world::shared` und `prices::prices` lesen ihn zuerst, und `parallel.rs` gibt ihn an die
//! Arbeiter weiter. Der Thread-Wert gilt nur für den Aufrufer und seine Läufe: Tests anderer Threads rechnen unberührt weiter.

use std::cell::Cell;

use k3c_data::Source;
use serde_json::{Number, Value};

use crate::fehler::Fehler;
use crate::island_world::{Shared, base_shared, load_shared_from};
use crate::prices::{PriceList, load_prices};

/// Die Stufen der Sensitivität in Prozent (Beschluss BAL3.1; Go: `Steps`).
pub const STEPS: [i64; 4] = [-25, -10, 10, 25];

/// Die standardmäßig variierten Werte (Beschluss BAL3.1; Go: `DefaultPaths`).
pub const DEFAULT_PATHS: [&str; 4] = [
    "economy.json:purse.startGold",
    "economy.json:dawnGoldPerPlayer",
    "buildings.json:wall.cost.gold",
    "waves.json:perExtraPlayer",
];

/// Die Dateien, die ein Lauf neu lädt (Go: `reloaded`, B-301). Eine Variation in einer anderen Datei (difficulty.json,
/// biomes/…) hätte keine Wirkung und ist ein Fehler.
pub const RELOADED: [&str; 9] = [
    "buildings.json",
    "troops.json",
    "economy.json",
    "hub.json",
    "monarch.json",
    "enemies.json",
    "waves.json",
    "ship.json",
    "goals.json",
];

/// Die Daten einer Variante, die der Lauf dieses Threads statt `data/` liest (Go: die Paket-Variablen nach `UseData`).
#[derive(Clone, Copy)]
pub struct DataCtx {
    pub shared: &'static Shared,
    pub prices: &'static PriceList,
}

thread_local! {
    static CURRENT: Cell<Option<DataCtx>> = const { Cell::new(None) };
}

/// Die Daten der Variante, in der dieser Thread gerade rechnet; `None` = `data/`.
pub fn current_data() -> Option<DataCtx> {
    CURRENT.with(Cell::get)
}

/// Setzt die Daten der Variante dieses Threads (die Arbeiter von `parallel.rs` übernehmen die des Aufrufers).
pub fn set_current_data(ctx: Option<DataCtx>) {
    CURRENT.with(|c| c.set(ctx));
}

/// Stellt beim Verlassen von `with_data` die vorigen Daten wieder her, auch bei einer Panik.
struct Restore(Option<DataCtx>);

impl Drop for Restore {
    fn drop(&mut self) {
        set_current_data(self.0);
    }
}

/// Rechnet `f` mit den Daten aus `src` (Sim und Bot-Preise) und stellt danach `data/` wieder her (Go: `withData`). Die Daten
/// einer Variante bleiben bis zum Prozessende im Speicher (`shared()` und `prices()` liefern `&'static`). Gilt nur für diesen
/// Thread und seine Läufe.
pub fn with_data<T>(src: Source, f: impl FnOnce() -> Result<T, Fehler>) -> Result<T, Fehler> {
    let prices: &'static PriceList = Box::leak(Box::new(load_prices(&src)?));
    let shared: &'static Shared = Box::leak(Box::new(load_shared_from(src)?));
    let _restore = Restore(current_data());
    set_current_data(Some(DataCtx { shared, prices }));
    f()
}

/// Die Datei wird für einen Lauf neu geladen, sonst Fehler mit dem Dateinamen (Go: `checkReloaded`).
pub fn check_reloaded(file: &str) -> Result<(), String> {
    if RELOADED.contains(&file) {
        return Ok(());
    }
    Err(format!(
        "{file} wird nicht neu geladen, die Variation hätte keine Wirkung (neu geladen: {})",
        RELOADED.join(", ")
    ))
}

/// Alle Dateien aus `data/` im Speicher, den Wert unter `path` um `pct` Prozent geändert (Ganzzahlen bleiben gerundet ganz),
/// und den Wert vorher und nachher. Unbekannter Pfad oder keine Zahl: Fehler mit Pfad (Go: `variedFS`).
pub fn varied_source(path: &str, pct: i64) -> Result<(Source, f64, f64), Fehler> {
    let fail = |e: String| Fehler::Daten(format!("sensitivität: Pfad {path:?}: {e}"));
    let Some((file, keys)) = path.split_once(':').filter(|(_, k)| !k.is_empty()) else {
        return Err(fail("erwartet datei.json:a.b.c".into()));
    };
    check_reloaded(file).map_err(fail)?;
    let mut files = base_shared()?.source.files().map_err(|e| fail(e.to_string()))?;
    let raw = files
        .get(file)
        .ok_or_else(|| fail(format!("open {file}: file does not exist")))?;
    let mut root: Value = serde_json::from_str(raw).map_err(|e| fail(e.to_string()))?;
    let keys: Vec<&str> = keys.split('.').collect();
    let (before, after) = scale(&mut root, &keys, pct).map_err(fail)?;
    let text = serde_json::to_string(&root).map_err(|e| fail(e.to_string()))?;
    files.insert(file.to_string(), text);
    Ok((Source::Mem(files), before, after))
}

/// Ändert die Zahl unter `keys` in `node` um `pct` Prozent; liefert den Wert vorher und nachher (Go: `scale`).
fn scale(node: &mut Value, keys: &[&str], pct: i64) -> Result<(f64, f64), String> {
    let (last, parents) = keys.split_last().ok_or("leerer Pfad")?;
    let mut node = node;
    for k in parents {
        node = child_mut(node, k).ok_or_else(|| format!("{k:?} fehlt"))?;
    }
    let slot = child_mut(node, last).ok_or_else(|| format!("{last:?} fehlt"))?;
    let Value::Number(n) = slot else {
        return Err(format!("{last:?} ist keine Zahl"));
    };
    let before = n.as_f64().ok_or("keine Zahl")?;
    let mut after = before * (1.0 + pct as f64 / 100.0);
    if n.is_f64() {
        *slot = Value::Number(float_number(after)?);
    } else {
        after = after.round();
        *slot = Value::Number(Number::from(after as i64));
    }
    Ok((before, after))
}

/// Eine Gleitkommazahl als JSON-Zahl; ganze Werte ohne Nachkomma (wie Gos `FormatFloat(x, 'f', -1, 64)`).
fn float_number(x: f64) -> Result<Number, String> {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        return Ok(Number::from(x as i64));
    }
    Number::from_f64(x).ok_or_else(|| format!("{x} ist keine endliche Zahl"))
}

/// Das Kind `k` eines Objekts oder Arrays (Index als Zahl) zum Ändern (Go: `child`).
pub fn child_mut<'a>(node: &'a mut Value, k: &str) -> Option<&'a mut Value> {
    match node {
        Value::Object(m) => m.get_mut(k),
        Value::Array(a) => k.parse::<usize>().ok().and_then(|i| a.get_mut(i)),
        _ => None,
    }
}
