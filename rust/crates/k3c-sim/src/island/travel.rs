//! Einzelwechsel der Stufe (Port von `engine/sim/island_travel.go`, Entscheidung 003, B-100): Jeder Spieler wechselt allein,
//! indem er `hub.travel.seconds` im Bereich `hub.travel.range_units` eines Reisepunkts seiner Stufe steht (Tiefen-Eingang
//! nach unten, gebaute Treppen). Die anderen Spieler bleiben stehen. Tote und freie Monarchen (B-059) wechseln nicht selbst.
//!
//! Go hält `*Player` in den Listen der Stufen und verschiebt sie; in Rust sind es Werte: `remove_player` nimmt den Spieler
//! aus der Liste der alten Stufe, `insert_player` setzt ihn in die neue (Reihenfolge nach Spieler-Index).

use serde_json::Value;

use super::Island;
use crate::economy::refund_pending;
use crate::state::links::{TravelPoint, crowned, return_banner, travel_points, values};
use crate::state::{Event, IslandRef, Player};

/// Go: `islandTravel`, der Reisefortschritt eines Spielers an einem Punkt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandTravel {
    pub x: f64,
    pub progress: f64,
}

/// Go: `islandMove`, ein fertiger Wechsel: der Spieler (Index), seine Stufe und die Zieltiefe.
struct IslandMove {
    index: i64,
    from: usize,
    to_depth: i64,
}

/// Go: `pointNear`, der erste Reisepunkt in Reichweite `range_units` von `x` (Reihenfolge wie `travel_points`).
pub fn point_near(points: &[TravelPoint], x: f64, range_units: f64) -> Option<&TravelPoint> {
    points.iter().find(|p| (x - p.x).abs() <= range_units)
}

/// Go: `stepIslandTravel` rechnet den Reisefortschritt aller Spieler und führt fertige Wechsel aus (nach dem Tick der
/// Stufen). Es läuft über die Stufen- und Spielerlisten, nie über die Map `travel`.
pub fn step_island_travel(isl: &mut Island, dt: f64) {
    let (range, seconds) = {
        let t = &isl.values().hub.travel;
        (t.range_units, t.seconds)
    };
    let mut moves: Vec<IslandMove> = Vec::new();
    for (si, w) in isl.stages.iter().enumerate() {
        let points = travel_points(w);
        for p in &w.players {
            let near = if crowned(p) && !p.free {
                point_near(&points, p.x, range)
            } else {
                None
            };
            let Some(pt) = near else {
                isl.travel.remove(&p.index);
                continue;
            };
            let st = isl.travel.entry(p.index).or_default();
            if st.x != pt.x {
                *st = IslandTravel { x: pt.x, progress: 0.0 };
            }
            st.progress += dt / seconds;
            if st.progress >= 1.0 {
                moves.push(IslandMove {
                    index: p.index,
                    from: si,
                    to_depth: pt.to_depth,
                });
            }
        }
    }
    for m in moves {
        isl.travel.remove(&m.index);
        move_to_stage(isl, &m);
    }
}

/// Go: `moveToStage` setzt einen Spieler in die Zielstufe: neue ID, Gold und Index bleiben, Position an der Burg. Hat die
/// Insel die Zielstufe nicht, bleibt der Spieler stehen.
fn move_to_stage(isl: &mut Island, m: &IslandMove) {
    let Some(to) = isl.stage_by_depth(m.to_depth) else {
        return;
    };
    let Some(pos) = isl.stages[m.from].players.iter().position(|p| p.index == m.index) else {
        return;
    };
    // bezahlte, nicht fertige Münzen bleiben in der alten Stufe liegen; `site_payable` liest vom Bezug nur die Dev-Preise
    let iref = IslandRef {
        cheap_build: isl.dev.cheap_build,
        stock_bonus: isl.dev.stock_bonus,
        stage: m.from,
        stages: isl.stages.len() as i64,
        storages_elsewhere: isl.storages_except(m.from),
        ..IslandRef::default()
    };
    let v = values(&isl.stages[m.from]);
    refund_pending(&mut isl.stages[m.from], &v, Some(&iref), pos);
    if isl.stages[m.from].players[pos].banner != 0 {
        return_banner(&mut isl.stages[m.from], pos); // das Banner bleibt am Tor der alten Stufe
    }
    let Some(mut q) = remove_player(&mut isl.stages[m.from].players, m.index) else {
        return;
    };
    let target = &mut isl.stages[to];
    q.id = target.new_id(); // IDs sind nur innerhalb einer Welt eindeutig
    q.x = target.hub_x + 3.0;
    if q.index % 2 == 0 {
        q.x = target.hub_x - 3.0;
    }
    q.vx = 0.0;
    q.pay_cooldown = 0.5;
    q.paying = false;
    q.pay_key = None;
    q.pay_amount = 0;
    let player = q.index;
    insert_player(&mut target.players, q);
    let mut ev = Event::new();
    ev.insert("type".to_string(), Value::from("arrived"));
    ev.insert("depth".to_string(), Value::from(m.to_depth));
    ev.insert("name".to_string(), Value::from(target.biome.name.clone()));
    ev.insert("player".to_string(), Value::from(player));
    target.events.push(ev);
}

/// Go: `removePlayer` nimmt den Spieler mit dem Index `index` aus der Liste (die übrigen behalten ihre Reihenfolge);
/// `None`, wenn er nicht in der Liste steht.
pub fn remove_player(players: &mut Vec<Player>, index: i64) -> Option<Player> {
    let pos = players.iter().position(|p| p.index == index)?;
    Some(players.remove(pos))
}

/// Go: `insertPlayer` fügt `p` nach aufsteigendem Spielerindex ein (stabile Reihenfolge: nach Spielern mit gleichem
/// Index); liefert die Position von `p`.
pub fn insert_player(players: &mut Vec<Player>, p: Player) -> usize {
    let mut i = players.len();
    while i > 0 && players[i - 1].index > p.index {
        i -= 1;
    }
    players.insert(i, p);
    i
}

impl Island {
    /// Go: `stageByDepth`, der Index der Stufe mit dieser Tiefe; `None` (Go: -1), wenn die Insel sie nicht hat.
    pub fn stage_by_depth(&self, depth: i64) -> Option<usize> {
        self.stages.iter().position(|w| w.biome.depth == depth)
    }

    /// Go: `StageOf`, die Stufe eines Spielers der Insel nach Index; `None` (Go: -1), wenn es ihn nicht gibt.
    pub fn stage_of(&self, index: i64) -> Option<usize> {
        self.stages
            .iter()
            .position(|w| w.players.iter().any(|p| p.index == index))
    }

    /// Go: `Players`, alle Spieler der Insel nach Index sortiert (Stufen in Reihenfolge, bei gleichem Index die frühere
    /// Stufe zuerst).
    pub fn players(&self) -> Vec<&Player> {
        let mut out: Vec<&Player> = self.stages.iter().flat_map(|w| w.players.iter()).collect();
        out.sort_by_key(|p| p.index); // stabil, wie die Einfüge-Regel von `insert_player`
        out
    }
}
