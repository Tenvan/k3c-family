//! Anzeige-Maßstab je Asset (B-948): data/paper.json › skalierung (`gruppe/id` → Faktor) vergrößert ein Asset beim
//! Laden, ohne das Papier neu zu rendern; der Fußpunkt bleibt, nur `units_per_px` wächst.
//! Vorlage: `native/k3c-game/art/scales.go`.

use std::collections::BTreeMap;

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
