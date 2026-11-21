//! Fälle von `combat.go` ohne Takt: Posten, Ziele, Zahlen, Rückerstattung, Gruppe verlassen.

use super::data::{COMBAT_GENERAL, COMBAT_GROUPS, CombatMode, DEFAULT_GROUP_SIZE, combat_mode, group_size};
use super::testutil::{combat_stage, spot_for, troop, vals};
use super::{
    SpotKind, is_soldier, leave_squad, pay_combat, refund_combat, squad_active, squad_post, squad_target, troop_side,
};
use crate::state::shared::CombatSettings;
use crate::state::{World, WorldValues};

fn world_with(mode: &str) -> World {
    combat_stage(mode).0
}

#[test]
fn is_soldier_nur_krieger_und_elite() {
    let mut w = world_with(COMBAT_GROUPS);
    for (kind, want) in [
        ("warrior", true),
        ("eliteWarrior", true),
        ("archer", false),
        ("citizen", false),
    ] {
        let t = troop(&mut w, kind, 0.0, 0.0);
        assert_eq!(is_soldier(&w.troops[t]), want, "{kind}");
    }
}

#[test]
fn troop_side_nach_dem_anker() {
    let mut w = world_with(COMBAT_GROUPS);
    let (l, r) = (troop(&mut w, "warrior", 0.0, -1.0), troop(&mut w, "warrior", 0.0, 1.0));
    assert_eq!(
        (troop_side(&w, &w.troops[l]), troop_side(&w, &w.troops[r])),
        (-1.0, 1.0)
    );
}

#[test]
fn squad_target_nimmt_den_naechsten_gegner_sonst_das_portal() {
    let mut w = world_with(COMBAT_GROUPS);
    let v = vals(&w);
    let c = &v.economy.combat;
    let a = w.hub_x + 10.0;
    assert_eq!(
        squad_target(&w, &v, a, 1.0, true),
        Some(w.hub_x + 40.0),
        "Portal ohne Gegner"
    );
    assert_eq!(
        squad_target(&w, &v, a, 1.0, false),
        None,
        "Portal außerhalb der Anspähweite"
    );
    crate::enemies::testutil::spawn(&mut w, "goblin", a + c.engage_units - 1.0);
    crate::enemies::testutil::spawn(&mut w, "goblin", a + 5.0);
    assert_eq!(
        squad_target(&w, &v, a, 1.0, false),
        Some(a + 5.0),
        "der nähere Gegner zuerst"
    );
    assert_eq!(
        squad_target(&w, &v, a, -1.0, true),
        Some(w.hub_x - 40.0),
        "Gegner hinter dem Anker zählen nicht"
    );
}

#[test]
fn squad_post_ohne_gruppe_ist_der_normale_posten() {
    let mut w = world_with(COMBAT_GROUPS);
    let v = vals(&w);
    let t = troop(&mut w, "warrior", 0.0, 1.0);
    assert_eq!(squad_post(&w, &v, &w.troops[t]), None);
    w.troops[t].shield_fetch = true;
    assert_eq!(
        squad_post(&w, &v, &w.troops[t]),
        Some(w.hub_x),
        "Schild holen: zur Burg"
    );
}

#[test]
fn squad_post_gruppe_geht_vor_das_ziel() {
    let mut w = world_with(COMBAT_GROUPS);
    let v = vals(&w);
    let l = troop(&mut w, "warrior", 10.0, 10.0);
    let lid = w.troops[l].id;
    w.troops[l].leader = true;
    w.troops[l].group = lid;
    w.troops[l].sortie = 1.0;
    let m = troop(&mut w, "warrior", 9.0, 10.0);
    w.troops[m].group = lid;
    assert!(squad_active(&w, &v, &w.troops[m]));
    let reach = (v.troops["warrior"].range * v.economy.combat.stand_range_factor).max(1.0);
    assert_eq!(squad_post(&w, &v, &w.troops[m]), Some(w.hub_x + 40.0 - reach));
    w.troops[l].sortie = 0.0;
    assert!(!squad_active(&w, &v, &w.troops[m]) && squad_post(&w, &v, &w.troops[m]).is_none());
}

#[test]
fn squad_post_general_folgt_dem_monarchen() {
    let mut w = world_with(COMBAT_GENERAL);
    let v = vals(&w);
    w.portals.clear();
    w.players[0].banner = 1;
    w.players[0].x = w.hub_x + 30.0;
    let pid = w.players[0].id;
    let (s, a) = (troop(&mut w, "warrior", 0.0, 0.0), troop(&mut w, "archer", 0.0, 0.0));
    w.troops[s].group = pid;
    w.troops[a].group = pid;
    let f = v.economy.combat.follow_units;
    assert_eq!(
        squad_post(&w, &v, &w.troops[s]),
        Some(w.hub_x + 30.0 + f),
        "Soldaten voraus"
    );
    assert_eq!(
        squad_post(&w, &v, &w.troops[a]),
        Some(w.hub_x + 30.0 - f),
        "Schützen dahinter"
    );
}

#[test]
fn leave_squad_loest_die_truppe_und_gibt_das_schild_zurueck() {
    let mut w = world_with(COMBAT_GROUPS);
    let t = troop(&mut w, "warrior", 0.0, 0.0);
    w.troops[t].leader = true;
    w.troops[t].group = 9;
    w.troops[t].sortie = 1.0;
    w.troops[t].shield_fetch = true;
    w.troops[t].attacking = true;
    leave_squad(&mut w, t);
    let tr = &w.troops[t];
    assert!(!tr.leader && tr.group == 0 && tr.sortie == 0.0 && !tr.shield_fetch && !tr.attacking);
    assert_eq!(w.combat.shields, 1, "das Schild ging nicht verloren");
    leave_squad(&mut w, t);
    assert_eq!(w.combat.shields, 1, "ohne Schildholen kein weiteres");
}

#[test]
fn pay_combat_schild_wirkt_erst_beim_vollen_preis() {
    let mut w = world_with(COMBAT_GROUPS);
    let v = vals(&w);
    let s = spot_for(&w, SpotKind::Shield, 0).expect("Schild");
    for _ in 1..v.economy.combat.shield_cost {
        pay_combat(&mut w, &v, 0, &s);
    }
    assert_eq!(
        (w.combat.shields, w.combat.paid.get("shield:0").copied()),
        (0, Some(v.economy.combat.shield_cost - 1))
    );
    pay_combat(&mut w, &v, 0, &s);
    assert_eq!(w.combat.shields, 1);
    assert!(!w.combat.paid.contains_key("shield:0"), "der Schlüssel fällt weg");
}

#[test]
fn refund_combat_gibt_hoechstens_den_gezahlten_betrag_zurueck() {
    let mut w = world_with(COMBAT_GROUPS);
    w.combat.paid.insert("trigger:-1".into(), 3);
    assert_eq!(refund_combat(&mut w, "trigger:-1", 2), 2);
    assert_eq!(refund_combat(&mut w, "trigger:-1", 5), 1);
    assert_eq!(refund_combat(&mut w, "trigger:-1", 5), 0);
    assert_eq!(refund_combat(&mut w, "gibtEsNicht", 5), 0);
    assert_eq!(
        refund_combat(&mut w, "trigger:-1", -1),
        -1,
        "wie Go: min(amount, gezahlt), nichts zurückgebucht"
    );
    assert_eq!(w.combat.paid["trigger:-1"], 0);
}

#[test]
fn combat_settings_standardwerte() {
    let v = WorldValues {
        combat: CombatSettings {
            mode: "general".into(),
            group_size: 0,
        },
        ..WorldValues::default()
    };
    assert_eq!(combat_mode(&v), CombatMode::General);
    assert_eq!(group_size(&v), DEFAULT_GROUP_SIZE);
}
