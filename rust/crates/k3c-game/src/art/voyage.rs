//! Werte der Schiffsfahrt beim Inselwechsel (B-977): `data/native.json › voyage`. Reine Darstellung (Fahrzeit des
//! Übergangs, Bogen, Schaukeln, Welle), keine Regel. Fehlt die Datei, der Block oder ein Wert, oder ist er
//! unzulässig, gilt sein Standardwert; das Spiel startet immer. Vorlage: `native/k3c-game/art/voyage.go`.

use std::sync::OnceLock;

use serde::Deserialize;

/// Werte der Schiffsfahrt. Längen in Asset-Pixeln des Dioramas.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoyageLook {
    /// Fahrzeit (B-977/R3).
    pub seconds: f64,
    /// Höhe des Bogens über der geraden Strecke.
    #[serde(rename = "arcPx")]
    pub arc: f64,
    /// Hub des Schiffs auf den Wellen.
    #[serde(rename = "bobPx")]
    pub bob: f64,
    /// Schaukeln je Sekunde.
    #[serde(rename = "bobHz")]
    pub bob_hz: f64,
    /// Ausschlag der vorderen Welle.
    #[serde(rename = "wavePx")]
    pub wave: f64,
    /// Schwingungen der vorderen Welle je Sekunde.
    #[serde(rename = "waveHz")]
    pub wave_hz: f64,
    /// Anteil der Strecke (0 … 1), ab dem die verlassene Insel ihre Fahne trägt.
    #[serde(rename = "flagAt")]
    pub flag_at: f64,
    /// Maßstab des Dioramas im Vollbild gegenüber dem HUD.
    pub scale: f64,
}

impl Default for VoyageLook {
    /// Standardwerte (wie data/native.json; ein Test hält beide gleich).
    fn default() -> Self {
        Self {
            seconds: 3.0,
            arc: 28.0,
            bob: 2.5,
            bob_hz: 0.7,
            wave: 14.0,
            wave_hz: 0.22,
            flag_at: 0.05,
            scale: 1.5,
        }
    }
}

/// Ergebnis von `parse_voyage`: die Werte, die unzulässigen Felder und ein JSON-Fehler.
pub type VoyageParse = (VoyageLook, Vec<&'static str>, Option<String>);

/// Liest den Block `voyage` aus dem Inhalt von native.json. Fehlende Werte behalten den Standard; Fahrzeit und
/// Maßstab müssen über 0 liegen, die übrigen dürfen 0 sein, nichts ist negativ, `flagAt` höchstens 1. Ein
/// unzulässiger Wert fällt auf seinen Standard zurück; der zweite Wert nennt die Felder. Kaputtes JSON: Standard samt
/// Fehler.
pub fn parse_voyage(raw: &[u8]) -> VoyageParse {
    #[derive(Deserialize)]
    struct File {
        voyage: Option<VoyageLook>,
    }
    let def = VoyageLook::default();
    let mut v = match serde_json::from_slice::<File>(raw) {
        Ok(f) => f.voyage.unwrap_or(def),
        Err(e) => return (def, Vec::new(), Some(e.to_string())),
    };
    let mut bad = Vec::new();
    let mut fix = |name: &'static str, got: &mut f64, std: f64, ok: bool| {
        if !ok {
            *got = std;
            bad.push(name);
        }
    };
    {
        let ok = v.seconds > 0.0;
        fix("seconds", &mut v.seconds, def.seconds, ok);
    }
    {
        let ok = v.scale > 0.0;
        fix("scale", &mut v.scale, def.scale, ok);
    }
    {
        let ok = v.arc >= 0.0;
        fix("arcPx", &mut v.arc, def.arc, ok);
    }
    {
        let ok = v.bob >= 0.0;
        fix("bobPx", &mut v.bob, def.bob, ok);
    }
    {
        let ok = v.bob_hz >= 0.0;
        fix("bobHz", &mut v.bob_hz, def.bob_hz, ok);
    }
    {
        let ok = v.wave >= 0.0;
        fix("wavePx", &mut v.wave, def.wave, ok);
    }
    {
        let ok = v.wave_hz >= 0.0;
        fix("waveHz", &mut v.wave_hz, def.wave_hz, ok);
    }
    {
        let ok = (0.0..=1.0).contains(&v.flag_at);
        fix("flagAt", &mut v.flag_at, def.flag_at, ok);
    }
    (v, bad, None)
}

/// Werte der Schiffsfahrt aus data/native.json (einmal gelesen).
pub fn voyage() -> VoyageLook {
    static V: OnceLock<VoyageLook> = OnceLock::new();
    *V.get_or_init(|| {
        let raw = match super::datafile::read("native.json") {
            Ok(r) => r,
            Err(e) => {
                k3c_platform::log::warn(
                    "💥 data/native.json nicht gelesen, Standardwerte der Schiffsfahrt",
                    &[("err", &e)],
                );
                return VoyageLook::default();
            }
        };
        let (v, bad, err) = parse_voyage(&raw);
        if let Some(e) = err {
            k3c_platform::log::warn(
                "💥 data/native.json unlesbar, Standardwerte der Schiffsfahrt",
                &[("err", &e)],
            );
        } else if !bad.is_empty() {
            let felder = bad.join(",");
            k3c_platform::log::warn(
                "💥 data/native.json › voyage: unzulässige Werte, Standard gilt",
                &[("felder", &felder)],
            );
        }
        v
    })
}
