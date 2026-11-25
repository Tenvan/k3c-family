//! Kronen-Rückholung (Port von `engine/balance/crown_bot.go`, B-931).

use k3c_sim::state::{Player, PlayerCommand, World};

use crate::bots::go_to;

/// So weit läuft ein Bot mit Krone zur Krone eines Mitspielers.
const CROWN_RESCUE_UNITS: f64 = 40.0;

/// Ein Bot ohne Krone läuft zur nächsten liegenden Krone (eigene oder fremde) und hebt sie so auf; trägt ein Bot eine
/// fremde Krone, bringt er sie zum Besitzer. Sonst gilt sein Profil (`cmd`).
pub fn retrieve_crown(w: &World, p: &Player, cmd: PlayerCommand) -> PlayerCommand {
    for q in &w.players {
        if let Some(c) = &q.crown
            && c.state == "player"
            && c.carrier == p.id
        {
            return go_to(p, q.x, false); // Mitspieler trägt: zum Besitzer
        }
    }
    let mut target: Option<f64> = None;
    for c in w.players.iter().filter_map(|q| q.crown.as_ref()) {
        if c.state == "ground" && target.is_none_or(|t| (c.x - p.x).abs() < (t - p.x).abs()) {
            target = Some(c.x);
        }
    }
    match target {
        None if p.crown.is_some() => PlayerCommand::default(),
        None => cmd,
        // Ein Bot mit Krone rettet nur nahe fremde Kronen.
        Some(x) if p.crown.is_some() && (x - p.x).abs() > CROWN_RESCUE_UNITS => cmd,
        Some(x) => go_to(p, x, false),
    }
}
