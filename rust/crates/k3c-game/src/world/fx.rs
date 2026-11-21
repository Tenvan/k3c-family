//! LA1.5 (B-1179): Umgebungseffekte einzeln ein- und ausschalten, um beim Abdunkeln zu sehen, welcher Effekt das Bild
//! blass macht. Zustand ist eine Bitmaske, je Effekt ein Bit; Start aus `K3C_FX_OFF=haze,halos,…` (Komma-Liste), danach
//! per [`set`] (vorgesehen für das Cheat-Fenster). Ein Aufruf von [`on`] ist ein atomarer Lesezugriff, ohne Allokation.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

/// EFFECTS sind die schaltbaren Effekte: Kennung und Anzeigename (Reihenfolge = Bit).
pub const EFFECTS: [(&str, &str); 7] = [
    ("haze", "Dunst (nur bei Wetter-Events)"),
    ("halos", "Lichthöfe (Mond; Quellen nur mit glow.sources)"),
    ("stars", "Sterne"),
    ("sky", "Himmelsverlauf und Abendrot"),
    ("lift", "Nacht-Aufhellung"),
    ("tint", "Tönung der Tageszeit"),
    ("mirror", "Spiegelung im Fluss"),
];

/// OFF hat ein Bit je ausgeschaltetem Effekt.
static OFF: AtomicU32 = AtomicU32::new(0);
static INIT: OnceLock<()> = OnceLock::new();

fn bit(id: &str) -> u32 {
    EFFECTS.iter().position(|(i, _)| *i == id).map_or(0, |p| 1 << p)
}

fn init() {
    INIT.get_or_init(|| {
        let v = std::env::var("K3C_FX_OFF").unwrap_or_default();
        let mask = v.split(',').map(|s| bit(s.trim())).fold(0, |a, b| a | b);
        OFF.fetch_or(mask, Ordering::Relaxed);
    });
}

/// On meldet, ob der Effekt `id` eingeschaltet ist (unbekannte Kennung: an).
pub fn on(id: &str) -> bool {
    init();
    OFF.load(Ordering::Relaxed) & bit(id) == 0
}

/// Set schaltet den Effekt `id` ein oder aus (unbekannte Kennung: nichts).
pub fn set(id: &str, on: bool) {
    init();
    let b = bit(id);
    if on {
        OFF.fetch_and(!b, Ordering::Relaxed);
    } else {
        OFF.fetch_or(b, Ordering::Relaxed);
    }
}

#[cfg(test)]
#[path = "fx_tests.rs"]
mod fx_tests;
