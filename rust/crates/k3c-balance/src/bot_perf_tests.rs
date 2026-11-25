//! Tests zu `bot_clear.rs` und `bot_material.rs` (Port von `engine/balance/bot_perf_test.go`, B-654, BAL9.2).

use k3c_data::buildings::Cost;
use k3c_sim::island::{Island, step_island};
use k3c_sim::state::site_view::site_zone;
use k3c_sim::state::{Player, PlayerCommand, ResourceNode, Site, World};

use crate::bot_abstieg::deepest_of;
use crate::bot_clear::{WantSet, near_lair};
use crate::bot_material::{amount_of, cost_of, near_any, node_for, payable_sites_x};
use crate::bots::{Bot, bot_by_name, closer};
use crate::helps::after_step;
use crate::island_world::shared;
use crate::prices::prices;
use crate::run::{MAX_TICKS_PER_DAY, Scenario, TICK_HZ, new_island};

/// B-654 (BAL9.2): `WantSet::wanted` und `node_for` liefern Tick für Tick dasselbe wie die Fassung vor B-654 (`wanted_ref`,
/// `node_for_ref`). Der Lauf ist der Meilenstein-Lauf (coop2, Welt standard) über die ersten Tage, geprüft wird jeden
/// fünften Tick in jeder Stufe mit Monarch, bevor die Bots ihre Befehle geben.
#[test]
#[ignore = "lang"]
fn bot_schnell_gleich() {
    let sc = Scenario {
        seed: "k3c-dev".into(),
        players: 2,
        bot: "coop2".into(),
        days: 3,
        world: "standard".into(),
        ..Scenario::default()
    };
    let mut isl = new_island(&sc).expect("Insel");
    let inner = bot_by_name(&sc.bot).expect("Profil coop2");
    let deepest = deepest_of(&isl);
    let descent = Bot::Descent {
        inner: Box::new(inner),
        deepest,
    };
    let bot = Bot::BossFight {
        inner: Box::new(descent),
        deepest,
    };
    let big = 1 << 20;
    let needs = [
        Cost {
            wood: 150,
            ..Cost::default()
        },
        Cost {
            stone: 40,
            iron: 10,
            ..Cost::default()
        },
        Cost {
            wood: big,
            stone: big,
            copper: big,
            iron: big,
            crystal: big,
            ..Cost::default()
        },
    ];
    let difficulty = &shared().expect("Daten").data.difficulty;
    let mut cmds = vec![PlayerCommand::default(); sc.players as usize];
    let mut checks = 0;
    for tick in 1..=sc.days * MAX_TICKS_PER_DAY {
        if isl.stages[0].cycle.day > sc.days {
            break;
        }
        for w in &isl.stages {
            if !w.players.is_empty() && tick % 5 == 0 {
                checks += same_as_ref(&isl, w, &needs, tick);
            }
            for p in &w.players {
                cmds[p.index as usize] = bot.decide(&isl, w, p);
            }
        }
        step_island(&mut isl, &cmds, difficulty, 1.0 / TICK_HZ as f64);
        after_step(&mut isl, &sc.world, &sc.helps, tick, TICK_HZ);
    }
    assert!(checks > 0, "nichts verglichen");
}

/// Vergleicht alle unbezahlten Plätze und jeden Bedarf für jeden Monarchen der Stufe; liefert die Zahl der Vergleiche.
fn same_as_ref(isl: &Island, w: &World, needs: &[Cost], tick: i64) -> i64 {
    let mut n = 0;
    let want = WantSet::new(w);
    for s in w.sites.iter().filter(|s| s.state == "unpaid") {
        let (got, reference) = (want.wanted(s), wanted_ref(w, s));
        assert_eq!(got, reference, "Tick {tick}: wanted({} @{:.1})", s.kind, s.x);
        n += 1;
    }
    for p in &w.players {
        for need in needs {
            let got = node_for(isl, w, p, need);
            let reference = node_for_ref(isl, w, p, need);
            assert!(
                same_node(got, reference),
                "Tick {tick}: node_for({need:?}) {got:?}, vorher {reference:?}"
            );
            n += 1;
        }
    }
    n
}

fn same_node(a: Option<&ResourceNode>, b: Option<&ResourceNode>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => std::ptr::eq(a, b),
        _ => false,
    }
}

/// Die Fassung von `WantSet::wanted` vor B-654 (Vergleichsbasis; Go: `wantedRef`).
fn wanted_ref(w: &World, s: &Site) -> bool {
    let Some(z) = site_zone(w, s) else {
        return true;
    };
    !w.sites.iter().any(|o| {
        !std::ptr::eq(o, s)
            && o.kind == s.kind
            && o.state != "unpaid"
            && (o.x < w.hub_x) == (s.x < w.hub_x)
            && site_zone(w, o) == Some(z)
    })
}

/// Die Fassung von `node_for` vor B-654 (Vergleichsbasis; Go: `nodeForRef`).
fn node_for_ref<'a>(isl: &Island, w: &'a World, p: &Player, need: &Cost) -> Option<&'a ResourceNode> {
    let prices = prices();
    let mut pending = Cost::default();
    for n in w.nodes.iter().filter(|n| n.marked) {
        if let Some(np) = prices.nodes.get(&n.kind) {
            crate::bot_material::add_res(&mut pending, &np.resource, np.amount);
        }
    }
    let blocked = payable_sites_x(w);
    let mut best: Option<&ResourceNode> = None;
    for n in &w.nodes {
        let Some(np) = prices.nodes.get(&n.kind) else {
            continue;
        };
        if n.marked
            || p.gold < np.mark_cost - n.paid_gold
            || near_any(n.x, &blocked)
            || near_lair(w, n.x)
            || amount_of(&isl.stock, &np.resource) + cost_of(&pending, &np.resource) >= cost_of(need, &np.resource)
        {
            continue;
        }
        if best.is_none_or(|b| closer(p.x, n.x, b.x)) {
            best = Some(n);
        }
    }
    best
}
