//! Zahlziele der Kampfvariante (Port des ersten Teils von `engine/sim/combat.go`, docs/rules/kampf.md § 3): Schild an der
//! Burg (Zahlziel `shield`), Auslöser am äußersten Tor (`trigger`, je Seite) und Banner (`banner`, je Monarch an jedem
//! Tor). Zahlziele laufen über `input.pay` (economy.go › findPayTarget). Kein rng.

use serde::{Deserialize, Serialize};

use super::data::{CombatMode, combat_mode};
use super::pending::{free_warriors, leaders_of, send_groups, take_banner};
use super::squad_target;
use crate::state::common::outer_wall;
use crate::state::pending::{count_fetching, crowned};
use crate::state::{Player, Site, World, WorldValues};
use crate::unit::econ::clear_pending;

/// Die Art eines Zahlziels der Kampfvariante (Go: `CombatSpot.Kind` als Text).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpotKind {
    /// Schild an der Burg.
    #[default]
    #[serde(rename = "shield")]
    Shield,
    /// Auslöser am äußersten Tor.
    #[serde(rename = "trigger")]
    Trigger,
    /// Banner eines Monarchen.
    #[serde(rename = "banner")]
    Banner,
}

impl SpotKind {
    /// Der Text der Art wie in Go (`"shield"`, `"trigger"`, `"banner"`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shield => "shield",
            Self::Trigger => "trigger",
            Self::Banner => "banner",
        }
    }
}

/// Go: `CombatSpot`, ein Zahlziel der Kampfvariante; `side` -1 links, 1 rechts, 0 an der Burg.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct CombatSpot {
    #[serde(rename = "kind")]
    pub kind: SpotKind,
    #[serde(rename = "side")]
    pub side: i64,
    #[serde(rename = "x")]
    pub x: f64,
}

impl CombatSpot {
    /// Go: `CombatSpot.key`: der Zahl-Schlüssel des Ziels; das Banner gehört je Monarch (Anzahlungen vermischen sich
    /// nicht): `banner:<Index>:<Seite>`, sonst `<Art>:<Seite>`.
    pub fn key(&self, p: &Player) -> String {
        if self.kind == SpotKind::Banner {
            format!("banner:{}:{}", p.index, self.side)
        } else {
            format!("{}:{}", self.kind.as_str(), self.side)
        }
    }

    /// Go: `CombatSpot.NearOf`: Steht `p` in Zahl-Reichweite (+1 Unit) des Ziels? Das Spiel prüft es vor `state_for`.
    pub fn near_of(&self, v: &WorldValues, p: &Player) -> bool {
        (self.x - p.x).abs() <= v.economy.pay_range_units + 1.0
    }

    /// Go: `CombatSpot.StateFor`: der Zahlstand des Ziels für `p`; lesend, ändert die Welt nicht.
    pub fn state_for(&self, w: &World, v: &WorldValues, p: &Player) -> CombatSpotState {
        CombatSpotState {
            price: combat_cost(v, self.kind),
            paid: w.combat.paid.get(&self.key(p)).copied().unwrap_or(0),
            payable: combat_payable(w, v, p, self),
            near: self.near_of(v, p),
        }
    }
}

/// Go: `CombatSpotState`, der Zahlstand eines Zahlziels für den Monarchen `p` (Anzeige im Spiel, B-939): Preis in Münzen,
/// bereits gezahlter Betrag, ob `p` hier gerade zahlen kann (`combat_payable`) und ob `p` in Zahl-Reichweite plus einer
/// Unit Anzeige-Spielraum steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CombatSpotState {
    pub price: i64,
    pub paid: i64,
    pub payable: bool,
    pub near: bool,
}

/// Go: `CombatSpots`: die Zahlziele der Kampfvariante der Stufe (Auslöser und Banner nur an einem gebauten Tor).
pub fn combat_spots(w: &World, v: &WorldValues) -> Vec<CombatSpot> {
    let c = &v.economy.combat;
    let mut out = Vec::with_capacity(3);
    let mut kind = SpotKind::Trigger;
    if combat_mode(v) == CombatMode::General {
        kind = SpotKind::Banner;
    } else if shields_unlocked(w, v) {
        out.push(CombatSpot {
            kind: SpotKind::Shield,
            side: 0,
            x: w.hub_x + c.shield_offset_units,
        });
    }
    for side in [-1.0, 1.0] {
        if let Some(wall) = outer_wall(w, side) {
            out.push(CombatSpot {
                kind,
                side: side as i64,
                x: gate_spot_x(w, v, wall, side),
            });
        }
    }
    out
}

/// Go: `shieldsUnlocked`: Schilde (Zahlstelle und Gestell) gibt es erst ab Hub-Stufe `shieldMinHubLevel` (B-942); nur in
/// groups.
pub fn shields_unlocked(w: &World, v: &WorldValues) -> bool {
    combat_mode(v) == CombatMode::Groups && w.hub_level >= v.economy.combat.shield_min_hub_level
}

/// Go: `ShieldRack`: Das Gestell neben der Burg ist sichtbar (Schilde sind baubar) und trägt `stack` gebaute, noch nicht
/// abgeholte Schilde (höchstens `shieldStackMax`); `x` ist die Zahlstelle des Schilds (B-942, Anzeige). Ergebnis
/// `(x, stack, visible)`.
pub fn shield_rack(w: &World, v: &WorldValues) -> (f64, i64, bool) {
    if !shields_unlocked(w, v) {
        return (0.0, 0, false);
    }
    let c = &v.economy.combat;
    (
        w.hub_x + c.shield_offset_units,
        w.combat.shields.min(c.shield_stack_max),
        true,
    )
}

/// Go: `gateSpotX`: Ort von Auslöser und Banner, `gateOutUnits` außerhalb (vor) dem äußersten Tor der Seite; liegt dort ein
/// Bauplatz näher als `spotClearUnits`, rückt das Ziel hinter ihn nach außen (B-941).
pub fn gate_spot_x(w: &World, v: &WorldValues, wall: &Site, side: f64) -> f64 {
    let c = &v.economy.combat;
    let mut x = wall.x + side * c.gate_out_units;
    for _ in &w.sites {
        let mut moved = false;
        for s in &w.sites {
            if s.id != wall.id && !s.ruin && (s.x - x).abs() < c.spot_clear_units && (s.x - wall.x) * side >= 0.0 {
                x = s.x + side * c.spot_clear_units;
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }
    x.min(w.width_units).max(0.0) // am Levelrand bleibt das Ziel erreichbar (Review KS7)
}

/// Go: `combatSpotNear`: das Zahlziel in Zahl-Reichweite von `x`, auch wenn es gerade nichts nimmt.
pub fn combat_spot_near(w: &World, v: &WorldValues, x: f64) -> bool {
    combat_spots(w, v)
        .iter()
        .any(|s| (s.x - x).abs() <= v.economy.pay_range_units)
}

/// Go: `combatTarget`: das Zahlziel, das für den Monarchen `p` gerade Münzen nimmt, sonst `None`.
pub fn combat_target(w: &World, v: &WorldValues, p: &Player) -> Option<CombatSpot> {
    let mut best: Option<CombatSpot> = None;
    for s in combat_spots(w, v) {
        if (s.x - p.x).abs() > v.economy.pay_range_units || !combat_payable(w, v, p, &s) {
            continue;
        }
        if best.is_none_or(|b| (s.x - p.x).abs() < (b.x - p.x).abs()) {
            best = Some(s);
        }
    }
    best
}

/// Go: `combatPayable`: Schild nur, solange ein freier Soldat da ist; Auslöser nur mit einer Gruppe der Seite; Banner nur
/// ohne Banner.
pub fn combat_payable(w: &World, v: &WorldValues, p: &Player, s: &CombatSpot) -> bool {
    match s.kind {
        SpotKind::Shield => {
            w.combat.shields < v.economy.combat.shield_stack_max
                && w.combat.shields + count_fetching(w) < free_warriors(w).len() as i64
        }
        SpotKind::Trigger => trigger_useful(w, v, s),
        SpotKind::Banner => p.banner == 0 && crowned(p),
    }
}

/// Go: `triggerUseful`: Eine Gruppe der Seite steht am Tor und hat ein Ziel (gegen Gegner oder Portal voraus).
pub fn trigger_useful(w: &World, v: &WorldValues, s: &CombatSpot) -> bool {
    leaders_of(w, s.side as f64)
        .into_iter()
        .any(|li| w.troops[li].sortie == 0.0 && squad_target(w, v, s.x, s.side as f64, true).is_some())
}

/// Go: `combatCost`: der Preis des Zahlziels dieser Art in Münzen.
pub fn combat_cost(v: &WorldValues, kind: SpotKind) -> i64 {
    let c = &v.economy.combat;
    match kind {
        SpotKind::Shield => c.shield_cost,
        SpotKind::Trigger => c.trigger_cost,
        SpotKind::Banner => c.banner_cost,
    }
}

/// Go: `payCombat`: nimmt eine Münze am Zahlziel `s` von Monarch `pi`; ist der Preis voll, wirkt es (Schild vergeben,
/// Gruppen los, Banner aufnehmen).
pub fn pay_combat(w: &mut World, v: &WorldValues, pi: usize, s: &CombatSpot) {
    let key = s.key(&w.players[pi]);
    let paid = w.combat.paid.entry(key.clone()).or_insert(0);
    *paid += 1;
    if *paid < combat_cost(v, s.kind) {
        return;
    }
    w.combat.paid.remove(&key);
    clear_pending(&mut w.players[pi]);
    match s.kind {
        SpotKind::Shield => w.combat.shields += 1,
        SpotKind::Trigger => send_groups(w, s.side as f64),
        SpotKind::Banner => take_banner(w, pi, s.side),
    }
}

/// Go: `refundCombat`: zieht bis zu `amount` Münzen vom Zahlziel `key` (`"trigger:-1"`) zurück und liefert die Anzahl.
pub fn refund_combat(w: &mut World, key: &str, amount: i64) -> i64 {
    let back = amount.min(w.combat.paid.get(key).copied().unwrap_or(0));
    if back > 0
        && let Some(paid) = w.combat.paid.get_mut(key)
    {
        *paid -= back;
    }
    back
}
