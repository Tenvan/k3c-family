//! Port von `engine/sim/portal_test.go` (B-927, docs/rules/kampf.md § 2). Go rechnet mit `CreateIsland` und `Step`; hier
//! laufen die Schritte direkt über `step_portals`. Tests mit `emit` (RU19), `Step` (RU14) oder Inseln (RU10) tragen
//! `ignore`.

use super::testutil::{PORTAL_HP, attacker, portal_stage};
use super::{apply_portals, portal_in_reach, safe, standing_portal_near, step_portals, sync_portals};
use crate::state::save::HubSave;

const DT: f64 = 1.0 / 30.0;

#[test]
fn portal_hp_aus_den_daten() {
    let (mut w, _) = portal_stage();
    sync_portals(&mut w);
    assert_eq!(w.portal_hp, vec![PORTAL_HP, PORTAL_HP]);
    assert_eq!(w.portal_defense.len(), 2);
    assert!(w.biome.portals.defense.size > 0);
}

#[test]
fn nur_truppen_im_angriff_beschaedigen_portale() {
    let (mut w, v) = portal_stage();
    let ti = attacker(&mut w, &v, 0);
    w.troops[ti].attacking = false;
    sync_portals(&mut w);
    w.portal_defense = vec![999.0, 999.0]; // kein Verteidiger-Takt in diesem Test (der braucht `emit`, RU19)
    for _ in 0..90 {
        step_portals(&mut w, &v, None, DT);
    }
    assert_eq!(w.portal_hp[0], PORTAL_HP, "Portal ohne Angreifer beschädigt");
    w.troops[ti].attacking = true;
    for _ in 0..90 {
        step_portals(&mut w, &v, None, DT);
    }
    assert!(
        w.portal_hp[0] < PORTAL_HP && w.portal_hp[1] == PORTAL_HP,
        "{:?}",
        w.portal_hp
    );
}

#[test]
fn portal_in_reach_nimmt_das_naechste_in_schlagweite() {
    let (mut w, v) = portal_stage();
    let ti = attacker(&mut w, &v, 1);
    assert_eq!(portal_in_reach(&w, &v, ti), Some(1));
    w.troops[ti].x = w.hub_x;
    assert_eq!(portal_in_reach(&w, &v, ti), None, "die Mitte liegt außer Reichweite");
}

#[test]
fn sicher_erst_wenn_alle_portale_zerstoert_sind() {
    let (mut w, _) = portal_stage();
    assert!(!safe(&w));
    w.portals.clear();
    assert!(!safe(&w), "ohne Portale von Anfang an ist die Stufe nicht „zerstört“");
    w.portals_down.push(1.0);
    assert!(safe(&w));
}

#[test]
fn standing_portal_near_liefert_das_naechste_stehende() {
    let (w, _) = portal_stage();
    assert_eq!(standing_portal_near(&w, w.hub_x - 100.0), w.portals[0]);
    assert_eq!(standing_portal_near(&w, w.hub_x + 3.0), w.portals[1]);
}

#[test]
fn zerstoerte_portale_bleiben_nach_dem_laden_weg() {
    let (mut w, _) = portal_stage();
    let (down, left) = (w.portals[0], w.portals[1]);
    let h = HubSave {
        portals_down: vec![down],
        ..HubSave::default()
    };
    apply_portals(&mut w, &h);
    assert_eq!((w.portals.clone(), w.portals_down.clone()), (vec![left], vec![down]));
    assert_eq!(w.portal_hp, vec![PORTAL_HP], "HP gleich lang wie die Portale");
    assert!(!safe(&w));
}

#[test]
#[ignore = "wartet auf RU19 (emit: portalDefense) und RU18 (Verteidiger-Wellen)"]
fn angegriffenes_portal_spawnt_verteidiger() {}

#[test]
#[ignore = "wartet auf RU19 (emit: portalDestroyed, stageSafe)"]
fn letztes_portal_macht_stufe_sicher() {}

#[test]
#[ignore = "wartet auf RU10 (Island, Speichern und Laden)"]
fn sichere_stufe_bleibt_nach_dem_laden() {}

#[test]
#[ignore = "wartet auf RU14 (Step)"]
fn dieb_ohne_portal_laesst_beute_fallen() {}

#[test]
#[ignore = "wartet auf RU10 und RU14 (CreateIsland, Step)"]
fn portal_simulation_ist_deterministisch() {}

#[test]
#[ignore = "wartet auf RU10 (Island, Miniboss-Punkt)"]
fn letztes_portal_ohne_doppel_punkt_bei_miniboss() {}
