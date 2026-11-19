//! data/look.json: Look des Rust-Spiels (RU55, Standard `docs/standards/bildaufbau.md`). Nur Rust liest die Datei.
//!
//! Zwei Profile: `k2c` (Aufbau, Maßstab und Licht nach K2C-Vorbild, Standard) und `go` (Stand des Go-Spiels, damit die
//! Bildvergleiche gegen Go gültig bleiben). Das Profil wählt `K3C_LOOK`; die Tabelle wird einmal gelesen, [`current`]
//! liefert das gewählte Profil ohne Allokation. Fehlende Felder gelten als Wert des Profils `go`.
//!
//! Tests: Mit dem Feature `go-default` (Dev-Abhängigkeit von `k3c-game`) ist `go` der Standard, damit die Tests gegen Go
//! unverändert laufen; [`with_profile`] schaltet je Thread um.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::source::Source;
use crate::{Fehler, decode};

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "look.json";
/// Umgebungsvariable, die das Profil wählt.
pub const ENV: &str = "K3C_LOOK";
/// Profil nach K2C-Vorbild.
pub const K2C: &str = "k2c";
/// Profil des Go-Spiels.
pub const GO: &str = "go";

/// Datei-Aufbau `.`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Looks {
    pub notes: String,
    pub profiles: BTreeMap<String, Look>,
}

/// Ein Profil (`.profiles.*`); `Default` ist der Stand des Go-Spiels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Look {
    /// Anteil der Welt bis zur Gehlinie an der Zellenhöhe.
    pub world_share: f64,
    /// Ufer als Anteil der Zellenhöhe; 0 = `bank_px`.
    pub bank_share: f64,
    /// Ufer in Spiel-px bei 32 px je Unit (nur wenn `bank_share` 0 ist).
    pub bank_px: f64,
    /// Sichtbare Welthöhe über der Gehlinie in Units bei einer Ansicht (k2c: 5 M zu 121,5 px bei 32 px je Unit).
    pub view_units: f64,
    /// Maßstab einer Hälfte im Split-Screen gegen die ganze Ansicht (k2c 0,7, go 0,5 = nur die halbe Höhe).
    pub split_scale: f64,
    /// Mindestgröße des Monarchen in px (0 = aus); der Maßstab fällt nie darunter.
    pub min_figure_px: f64,
    /// Fluss als Spiegel der Welt (RU55.3); im Profil `go` aus (Wellen).
    pub mirror: Mirror,
}

/// Spiegel-Fluss (`.profiles.*.mirror`, Standard `docs/standards/bildaufbau.md` § 4).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Mirror {
    /// Spiegel statt Wellen (nur mit Fluss-Fassung `shader` und einem Canvas, der ihn kann).
    pub enabled: bool,
    /// Helligkeit der Spiegelung (0,65).
    pub brightness: f64,
    /// Stauchung senkrecht (1 = unverzerrt).
    pub squash: f64,
    /// Schwanken waagerecht, Amplitude in Units.
    pub wobble: f64,
    /// Anteil der Wasserfarbe in der Spiegelung.
    pub tint: f64,
    /// Verlauf zum Wasser nach unten (Anteil der Flusshöhe).
    pub depth_fade: f64,
    /// Deckkraft der Glanzstriche.
    pub glint: f64,
}

const MIRROR_OFF: Mirror = Mirror {
    enabled: false,
    brightness: 0.65,
    squash: 0.85,
    wobble: 0.07,
    tint: 0.18,
    depth_fade: 0.3,
    glint: 0.55,
};

impl Default for Mirror {
    fn default() -> Self {
        MIRROR_OFF
    }
}

/// Der Stand des Go-Spiels.
const GO_LOOK: Look = Look {
    world_share: 2.0 / 3.0,
    bank_share: 0.0,
    bank_px: 20.0,
    view_units: 420.0 / 32.0,
    split_scale: 0.5,
    min_figure_px: 0.0,
    mirror: MIRROR_OFF,
};

impl Default for Look {
    fn default() -> Self {
        GO_LOOK
    }
}

/// Höhe des berittenen Monarchen in Units (121,5 Spiel-px bei 32 px je Unit): Maß M des Standards.
pub const MONARCH_UNITS: f64 = 121.5 / 32.0;

impl Look {
    /// Maßstab (px je Unit) einer Zelle der Höhe `cell_h` px, wenn `n` Zellen übereinanderliegen: Höhe der Welt durch
    /// `view_units`; bei mehreren Zellen mal `split_scale · n` (die natürliche Skalierung ist 1/n der Höhe), nie unter
    /// der Mindestgröße des Monarchen.
    pub fn cell_scale(&self, world_h: f64, n: usize) -> f64 {
        let f = if n >= 2 { self.split_scale * n as f64 } else { 1.0 };
        (world_h / self.view_units * f).max(self.min_figure_px / MONARCH_UNITS)
    }

    /// Sichtbare Welthöhe der kleinsten Ansicht in Units (Höhle: Decke).
    pub fn view_top(&self) -> f64 {
        self.view_units * (0.5 / self.split_scale).min(1.0)
    }
}

/// `load` liest look.json.
pub fn load(raw: &str) -> Result<Looks, Fehler> {
    decode("data/look.json", raw)
}

/// Eingebaute Profile, falls die Datei fehlt oder nicht lesbar ist (Werte wie `data/look.json`).
fn builtin() -> BTreeMap<String, &'static Look> {
    let k2c = Look {
        world_share: 0.58,
        bank_share: 0.07,
        view_units: 5.0 * MONARCH_UNITS,
        split_scale: 0.7,
        min_figure_px: 40.0,
        mirror: Mirror {
            enabled: true,
            ..MIRROR_OFF
        },
        ..GO_LOOK
    };
    BTreeMap::from([
        (K2C.to_string(), &*Box::leak(Box::new(k2c))),
        (GO.to_string(), &*Box::leak(Box::<Look>::default())),
    ])
}

/// Die Profile einmal aus `data/look.json` (Fehler: Meldung auf stderr, eingebaute Werte).
fn table() -> &'static BTreeMap<String, &'static Look> {
    static TABLE: OnceLock<BTreeMap<String, &'static Look>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let read = Source::from_env().read(FILE).and_then(|raw| load(&raw));
        match read {
            Ok(l) if l.profiles.contains_key(K2C) && l.profiles.contains_key(GO) => l
                .profiles
                .into_iter()
                .map(|(k, v)| (k, &*Box::leak(Box::new(v))))
                .collect(),
            other => {
                let why = other
                    .err()
                    .map_or_else(|| "Profil k2c oder go fehlt".to_string(), |e| e.to_string());
                eprintln!("❌ data/look.json nicht brauchbar, eingebaute Werte: {why}");
                builtin()
            }
        }
    })
}

/// Das Profil `name`, falls die Tabelle es kennt.
pub fn profile(name: &str) -> Option<&'static Look> {
    table().get(name).copied()
}

/// Name des Standard-Profils: `go` mit dem Feature `go-default` (Tests), sonst `k2c`.
pub const fn default_name() -> &'static str {
    if cfg!(feature = "go-default") { GO } else { K2C }
}

/// Wählt das Profil nach dem Wert von `K3C_LOOK`; unbekannt oder leer: Standard (unbekannt mit Meldung).
fn pick(env: &str) -> &'static Look {
    let want = env.trim().to_ascii_lowercase();
    let name = if want.is_empty() { default_name() } else { want.as_str() };
    match profile(name) {
        Some(l) => l,
        None => {
            eprintln!("🐛 K3C_LOOK={want}: unbekanntes Profil, Standard {}", default_name());
            profile(default_name()).unwrap_or(&FALLBACK)
        }
    }
}

static FALLBACK: Look = GO_LOOK;

thread_local! {
    static FORCED: Cell<Option<&'static Look>> = const { Cell::new(None) };
}

/// Das gewählte Profil (`K3C_LOOK`, einmal beim ersten Aufruf gelesen) oder das mit [`with_profile`] gesetzte.
pub fn current() -> &'static Look {
    if let Some(l) = FORCED.with(Cell::get) {
        return l;
    }
    static CURRENT: OnceLock<&'static Look> = OnceLock::new();
    CURRENT.get_or_init(|| pick(&std::env::var(ENV).unwrap_or_default()))
}

/// Führt `f` auf diesem Thread mit dem Profil `name` aus (Tests); ein unbekannter Name lässt das Profil unverändert.
pub fn with_profile<R>(name: &str, f: impl FnOnce() -> R) -> R {
    let prev = FORCED.with(Cell::get);
    if let Some(l) = profile(name) {
        FORCED.with(|c| c.set(Some(l)));
    }
    let r = f();
    FORCED.with(|c| c.set(prev));
    r
}

#[cfg(test)]
#[path = "look_tests.rs"]
mod look_tests;
