//! Anzeige-Maßstab je Asset (B-948): data/paper.json › skalierung (`gruppe/id` → Faktor) vergrößert ein Asset beim
//! Laden, ohne das Papier neu zu rendern; der Fußpunkt bleibt, nur `units_per_px` wächst.
//! Vorlage: `native/k3c-game/art/scales.go`.

use std::collections::BTreeMap;

use k3c_data::sizes::Sizes;

use super::Sheet;
use super::library::SheetKey;

/// Liest die Faktoren aus `data/paper.json › skalierung`; ein Lesefehler steht im Log und liefert keine Faktoren
/// (Maßstab 1).
pub fn data_scales() -> BTreeMap<String, f64> {
    let parsed = super::datafile::read("paper.json").and_then(|raw| parse_scales(&raw));
    parsed.unwrap_or_else(|e| {
        k3c_platform::log::error("❌ Skalierung nicht lesbar", &[("err", &e)]);
        BTreeMap::new()
    })
}

/// Liest `skalierung` aus dem Inhalt von paper.json (fehlt der Abschnitt: keine Faktoren).
pub fn parse_scales(raw: &[u8]) -> Result<BTreeMap<String, f64>, String> {
    #[derive(serde::Deserialize)]
    struct P {
        #[serde(default, rename = "skalierung")]
        scales: BTreeMap<String, f64>,
    }
    serde_json::from_slice::<P>(raw)
        .map(|p| p.scales)
        .map_err(|e| e.to_string())
}

/// Multipliziert `units_per_px` der Sheets mit dem Faktor ihres Assets (nur Faktoren > 0).
pub fn apply_scales(sheets: &mut BTreeMap<SheetKey, Sheet>, scales: &BTreeMap<String, f64>) {
    for ((asset, _), s) in sheets.iter_mut() {
        if let Some(&f) = scales.get(asset)
            && f > 0.0
        {
            s.units_per_px *= f;
        }
    }
}

/// Gruppen, deren Größe die Stufentabelle über `units_per_px` setzt; Figuren (Truppen, Gegner, Bosse) laufen über die
/// Höhe im Katalog (`actors/sizes.rs`).
fn is_sheet_group(asset: &str) -> bool {
    matches!(asset.split_once('/'), Some(("gebaeude-paper" | "objekte", _)))
}

/// Entfernt die Faktoren der Assets, deren Größe die Stufentabelle bestimmt (sonst würde zweimal skaliert).
pub fn without_sized(scales: &BTreeMap<String, f64>, sizes: &Sizes) -> BTreeMap<String, f64> {
    let mut out = scales.clone();
    out.retain(|k, _| !(is_sheet_group(k) && sizes.assets.contains_key(k)));
    out
}

/// Setzt `units_per_px` der Gebäude- und Objekt-Sheets so, dass ihre sichtbare Höhe Stufe × Monarchenhöhe ist (LA1.1).
pub fn apply_sizes(sheets: &mut BTreeMap<SheetKey, Sheet>, sizes: &Sizes) {
    for ((asset, _), s) in sheets.iter_mut() {
        if !is_sheet_group(asset) {
            continue;
        }
        if let Some(f) = sizes.sheet_factor(asset, s.fh, s.units_per_px) {
            s.units_per_px *= f;
        }
    }
}
