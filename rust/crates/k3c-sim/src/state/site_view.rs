//! Platz-Stufe für das Protokoll (Port von `engine/sim/site_view.go`): ab welcher Hub-Stufe ein Bauplatz bezahlbar ist.
//! Nur lesend, aus den bestehenden Regeln abgelesen (`site_hub_level`, `level_site`); nichts gespeichert, Welt-JSON und
//! Golden bleiben gleich.

use super::pending::site_hub_level;
use super::{Site, World};
use crate::world::lines::level_site;

/// Go: `SiteHubLevel`: die Hub-Stufe, ab der ein Platz bezahlbar ist: Mauer und Turm der Zone n ab n, alle übrigen Plätze
/// nach data/hub.json (`site_hub_level`). Weitere Bedingungen (Freiraum von Bäumen und Felsen) zählen nicht, sie sind
/// keine Stufe.
pub fn site_hub_level_of(w: &World, s: &Site) -> i64 {
    match level_site(w, s) {
        Some(l) => l.zone.max(1),
        None => site_hub_level(w, s),
    }
}

/// Go: `SiteZone`: die Zone n (von innen, ab 1) eines Mauer- oder Turm-Platzes; `None` für alle übrigen Plätze.
pub fn site_zone(w: &World, s: &Site) -> Option<i64> {
    level_site(w, s).map(|l| l.zone)
}

/// Go: `SiteVisible`: Steht ein Platz im Zustand für den Client? Plätze bis zur nächsten Hub-Stufe (Platz-Stufe ≤
/// Hub-Stufe + 1); bezahlte, wartende und gebaute Plätze immer.
pub fn site_visible(w: &World, s: &Site) -> bool {
    if s.state != "unpaid" || s.paid_gold > 0 {
        return true;
    }
    site_hub_level_of(w, s) <= w.hub_level + 1
}
