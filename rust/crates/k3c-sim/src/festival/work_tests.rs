//! Was im Frieden weiterläuft (RU62.12, AC-06): Bau und Beladen des Schiffs (Bauer bauen, der Spieler zahlt), Tag und Nacht,
//! die Lager der Vagabunden.
use k3c_data::hub::Stock;

use super::testutil::{festival_data, festival_island, tick};
use crate::island::Island;
use crate::island::testutil::DT;
use crate::state::PlayerCommand;
use crate::world::links::{dev_build_site, level_of};
use crate::world::ship::{SHIP_KIND, harbor, ship_complete};
use crate::world::vals;

/// Der Hafen der Insel mit dem Schiff: Stufe und Platz.
fn ship_at(isl: &Island) -> (usize, usize) {
    harbor(isl).expect("Hafen")
}

/// AC-06: Im Frieden schließt der Schiffsbau ab: Die ersten Abschnitte stehen (Dev), der letzte wird vom Monarchen bezahlt
/// und von einem Bauern gebaut, bis das Schiff fertig ist. Nebenbei läuft die Zeit (Tag und Nacht).
#[test]
fn schiffsbau_schliesst_im_frieden_ab() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    isl.endboss_defeated = true;
    let (wi, si) = ship_at(&isl);
    let w = &mut isl.stages[wi];
    let v = vals(w);
    let levels = v.buildings[SHIP_KIND].levels.clone();
    assert!(levels.len() >= 2, "Schiff in Abschnitten");
    dev_build_site(w, si, levels.len() as i64 - 1);
    assert!(!ship_complete(w, &w.sites[si]));
    let last = &levels[levels.len() - 1].cost;
    isl.stock = Stock {
        wood: last.wood,
        stone: last.stone,
        copper: last.copper,
        iron: last.iron,
        crystal: last.crystal,
    };
    let x = isl.stages[wi].sites[si].x;
    isl.stages[wi].players[0].gold = last.gold;
    let day0 = isl.stages[wi].cycle.day;
    let time0 = isl.stages[wi].time;
    let ticks = (levels[levels.len() - 1].build_seconds * 4.0 / DT) as usize + 2000;
    for _ in 0..ticks {
        isl.stages[wi].players[0].x = x;
        if let Some(t) = isl.stages[wi].troops.iter_mut().find(|t| t.kind == "peasant") {
            t.x = x; // ein Bauer steht am Platz und baut
        }
        let cmds = vec![
            PlayerCommand {
                pay: true,
                ..PlayerCommand::default()
            };
            isl.next_player as usize
        ];
        crate::island::step_island(&mut isl, &cmds, &data.difficulty, DT);
        let w = &isl.stages[wi];
        if ship_complete(w, &w.sites[si]) {
            break;
        }
    }
    let w = &isl.stages[wi];
    let s = &w.sites[si];
    assert!(
        ship_complete(w, s) && level_of(w, s) == levels.len() as i64,
        "Schiff fertig: {s:?}"
    );
    assert!(w.time > time0 && w.cycle.day >= day0, "Zeit läuft weiter");
    assert!(isl.stages.iter().all(|w| w.peace && w.enemies.is_empty()));
}

/// AC-06: Die Lager der Vagabunden (`camps`) schicken keine Gegner und arbeiten im Frieden weiter: Bürger kommen nach.
#[test]
fn lager_der_vagabunden_laufen_weiter() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    isl.endboss_defeated = true;
    tick(&mut isl, &data);
    let w = &mut isl.stages[0];
    assert!(!w.camps.is_empty(), "Stufe 0 hat Lager");
    w.troops.retain(|t| t.kind != "vagrant");
    for c in &mut w.camps {
        c.respawn_in = 0.1;
    }
    for _ in 0..(5.0 / DT) as usize {
        tick(&mut isl, &data);
    }
    let w = &isl.stages[0];
    assert!(w.troops.iter().any(|t| t.kind == "vagrant"), "Vagabunden kommen nach");
    assert!(w.enemies.is_empty() && w.spawn_queue.is_empty());
}
