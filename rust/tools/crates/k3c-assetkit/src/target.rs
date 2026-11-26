//! Zielmaß eines Grafik-Slots (B-685): Breite × Höhe in Units (1 Unit = 32 Spiel-Pixel). Der Client passt die aktive
//! Zuordnung ohne Verzerrung in dieses Maß ein; fehlt es oder ist eine Seite 0, gilt das Maß der Datei.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::gojson::{self, null_default};
use crate::register::{self, KIND_GRAPHIC, Slot};
use crate::stamp::stamp;

/// Größte Seite eines Zielmaßes (eine Hub-Breite: 100 Units).
pub const MAX_TARGET_UNITS: f64 = 100.0;

/// Zielmaß in Units.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Units {
    #[serde(serialize_with = "gojson::float", deserialize_with = "null_default")]
    pub w: f64,
    #[serde(serialize_with = "gojson::float", deserialize_with = "null_default")]
    pub h: f64,
}

/// Fehlt das Maß, ist es gültig; sonst beide Seiten endlich, nicht negativ, höchstens [`MAX_TARGET_UNITS`].
pub(crate) fn check_target(t: Option<&Units>) -> Result<()> {
    let Some(t) = t else { return Ok(()) };
    if [t.w, t.h].iter().any(|v| !(0.0..=MAX_TARGET_UNITS).contains(v)) {
        return Err(Error::Msg(format!(
            "ziel {}x{} Units ungültig (je Seite 0 bis {MAX_TARGET_UNITS})",
            go_g(t.w),
            go_g(t.h)
        )));
    }
    Ok(())
}

/// Zahl wie Go `%g`.
fn go_g(v: f64) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "+Inf" } else { "-Inf" }.to_string();
    }
    let sci = format!("{v:e}");
    let Some((mantissa, exp)) = sci.split_once('e') else {
        return sci;
    };
    let exp: i32 = exp.parse().unwrap_or(0);
    // Für die kürzeste Darstellung entscheidet Go mit der Genauigkeit 6.
    if !(-4..6).contains(&exp) {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{mantissa}e{sign}{:02}", exp.abs())
    } else {
        format!("{v}")
    }
}

/// Setzt das Zielmaß eines Grafik-Slots mit der Schreibregel von [`crate::set_choices_if`]: nur auf dem Stand `stand`,
/// sonst [`Error::Stale`] ohne Schreibzugriff. `None` oder eine Seite 0 entfernt das Zielmaß. Liefert den Slot und den
/// neuen Stand.
pub fn set_target_if(root: &Path, id: &str, t: Option<Units>, stand: &str) -> Result<(Slot, String)> {
    let _guard = register::lock();
    if stamp(root)? != stand {
        return Err(Error::Stale);
    }
    check_target(t.as_ref())?;
    let mut reg = register::load(root)?;
    let slot = reg
        .find_mut(id)
        .ok_or_else(|| Error::Msg(format!("unbekannter Asset-Slot {id:?}")))?;
    if slot.kind != KIND_GRAPHIC {
        return Err(Error::Msg(format!(
            "slot {id} ist keine Grafik, Zielmaß nur für Grafik-Slots"
        )));
    }
    slot.target = t.filter(|t| t.w != 0.0 && t.h != 0.0);
    let slot = slot.clone();
    reg.save(root)?;
    Ok((slot, stamp(root)?))
}

#[cfg(test)]
#[path = "target_tests.rs"]
mod tests;
