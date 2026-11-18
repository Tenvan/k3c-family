//! Hilfen der Tests dieser Gruppe (Go: `merchantDays`, `merchantWorld`, `tradeWorld`, `merchantArrives` aus `merchant_test.go`
//! und `merchant_raid_target_test.go`, dazu `toDawn` aus `building_tavern_test.go` und `steps` aus `crown_helpers_test.go`).

use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_det::golden;

use super::{merchant_dawn, merchant_x};
use crate::enemies::testutil::{DT, add_player, step};
use crate::island::Island;
use crate::state::island_ref::IslandRef;
use crate::state::{Merchant, PlayerCommand, World};
use crate::world::base::base;
use crate::world::cycle::{cycle_at, cycle_phases};
use crate::world::testutil::new_world;
use crate::world::{Options, step as world_step};

/// Go: `merchantWorld`: eine Stufe der Tiefe `depth` ohne Truppen, Camps, Truhen und Portale.
pub fn merchant_world(depth: i64) -> World {
    let mut w = new_world(depth, "haendler", Options::default());
    w.troops.clear();
    w.camps.clear();
    w.pickups.clear();
    w.portals.clear();
    w
}

/// Go: `toDawn` rechnet bis zum nächsten Tagesanbruch (Beginn des Morgengrauens).
pub fn to_dawn(w: &mut World) {
    let dn = base().expect("eingebettete Daten").day_night.clone();
    let (phases, length) = cycle_phases(&dn);
    let now = w.time * w.cycle_speed;
    let mut target = (now / length).floor() * length + phases[3].start;
    if target <= now + 2.0 * DT * w.cycle_speed {
        target += length;
    }
    w.time = target / w.cycle_speed - 2.0 * DT;
    w.cycle = cycle_at(&dn, w.time * w.cycle_speed);
    for _ in 0..10 {
        step(w, None, DT);
        if w.events
            .iter()
            .any(|e| e.get("type").and_then(|t| t.as_str()) == Some("dawn"))
        {
            return;
        }
    }
    panic!("kein dawn");
}

/// Go: `merchantDays`: tickt `n` dawns weiter und liefert je Tag, ob der Händler da ist (Schlüssel = Tag).
pub fn merchant_days(w: &mut World, n: i64) -> BTreeMap<i64, bool> {
    let mut present = BTreeMap::new();
    for _ in 0..n {
        to_dawn(w);
        present.insert(w.cycle.day, w.merchant.is_some());
    }
    present
}

/// Go: `tradeWorld`: Händler mit Material `res`, Spieler 0 am Zahlziel `side0`, Spieler 1 (falls `side1` nicht leer) an `side1`.
pub fn trade_world(res: &str, side0: &str, side1: &str) -> (World, Vec<usize>) {
    let mut w = merchant_world(0);
    w.merchant = Some(Merchant {
        resource: res.to_string(),
        leaves: 99,
        ..Merchant::default()
    });
    let mut ps = vec![add_player(&mut w)];
    let x = merchant_x(&w, side0);
    w.players[ps[0]].x = x;
    if !side1.is_empty() {
        ps.push(add_player(&mut w));
        let x = merchant_x(&w, side1);
        w.players[ps[1]].x = x;
    }
    (w, ps)
}

/// Go: `steps`: `seconds` Sekunden lang mit denselben Eingaben.
pub fn steps(w: &mut World, cmds: &[PlayerCommand], seconds: f64) {
    for _ in 0..(seconds / DT) as i64 {
        world_step(w, cmds, None, DT);
    }
}

/// Go: `merchantArrives`: stellt die Stufe auf den dawn von Tag `day` und lässt den Händler nach Rhythmus kommen oder gehen.
pub fn merchant_arrives(w: &mut World, isl: Option<&mut IslandRef>, day: i64) {
    w.cycle = crate::state::CycleInfo {
        phase: "day".into(),
        day,
        ..Default::default()
    };
    merchant_dawn(w, isl);
}

/// Go: `merchantArrives` für die Stufe `i` einer Insel: Der Bezug zur Insel kommt aus `stage_ref` und geht danach zurück.
pub fn island_arrives(isl: &mut Island, data: &Data, i: usize, day: i64) {
    let mut r = isl.stage_ref(i, &data.difficulty);
    merchant_arrives(&mut isl.stages[i], Some(&mut r), day);
    isl.put_stage_ref(r);
}

/// Go: `readFixture`: `testdata/saves/v<version>/familie.json`.
pub fn read_fixture(version: i64) -> String {
    let path = golden::testdata().join(format!("saves/v{version}/familie.json"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}
