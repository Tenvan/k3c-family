//! Anzeige-Maßstab je Asset (B-948): data/paper.json › skalierung (`gruppe/id` → Faktor) vergrößert ein Asset beim
//! Laden, ohne das Papier neu zu rendern; der Fußpunkt bleibt, nur `units_per_px` wächst.
//! Vorlage: `native/k3c-game/art/scales.go`.

use std::collections::BTreeMap;

use super::Sheet;
use super::library::SheetKey;

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
