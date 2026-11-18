//! Rückmeldung ohne Ziel (Go: `feedback_test.go`): Schlag ins Leere und Skill ohne Ziel (S9.1a, B-321). Die Meldungen `strike`,
//! `castFailed` und `skillCast` laufen über `emit` und `unit_x` (events.go, RU19): diese Tests warten darauf. Die Tests ab
//! `own_` haben kein Go-Gegenstück und prüfen das Halten der Taste ohne Meldungen.

use super::cast_skill;
use super::testutil::*;
use crate::crown::testutil::fell;
use crate::economy::step_players;
use crate::enemies::events_of_type;
use crate::enemies::testutil::{add_player, quiet_world, spawn};
use crate::links::unit_x;
use crate::monarch::step_attack;
use crate::state::{Player, PlayerCommand, World};
use crate::world::testutil::DT;
use crate::world::vals;

/// Go: `playerStrikes`: Zahl der `strike`-Ereignisse des Spielers und das letzte `hit`.
fn player_strikes(w: &World, pid: i64) -> (usize, Option<serde_json::Value>) {
    let mut found = (0, None);
    for ev in events_of_type(w, "strike") {
        if ev["from"] == pid {
            found = (found.0 + 1, ev.get("hit").cloned());
        }
    }
    found
}

/// Go: `stepStrikes`: ein Schlag-Tick, danach die Schläge des Spielers.
fn step_strikes(w: &mut World, pi: usize) -> (usize, Option<serde_json::Value>) {
    w.events.clear();
    let v = vals(w);
    let cmd = PlayerCommand {
        attack: true,
        ..PlayerCommand::default()
    };
    step_attack(w, &v, pi, &cmd, DT);
    player_strikes(w, w.players[pi].id)
}

#[test]
fn schlag_ohne_ziel_meldet_strike_hit_false() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let (n, hit) = step_strikes(&mut w, pi);
    assert!(
        n == 1 && hit == Some(false.into()),
        "ins Leere: {n} strike, hit {hit:?}"
    );
    for _ in 0..10 {
        // Abklingzeit läuft: kein zweites Ereignis
        assert_eq!(step_strikes(&mut w, pi).0, 0, "strike während der Abklingzeit");
    }
    let mut w2 = quiet_world();
    let p2 = add_player(&mut w2);
    let px = w2.players[p2].x;
    spawn(&mut w2, "goblin", px + 1.0);
    let (n, hit) = step_strikes(&mut w2, p2);
    assert!(n == 1 && hit == Some(true.into()), "Treffer: {n} strike, hit {hit:?}");
}

#[test]
fn skill_ohne_ziel_meldet_cast_failed() {
    let mut w = quiet_world();
    let pi = add_player(&mut w);
    let v = vals(&w);
    let failed = |w: &mut World, slot: usize| {
        w.events.clear();
        cast_skill(w, &v, pi, slot, true);
        events_of_type(w, "castFailed").len()
    };
    assert_eq!(failed(&mut w, 0), 0, "leerer Slot meldet castFailed");
    w.players[pi].slots = vec!["fireball".into()];
    assert_eq!(failed(&mut w, 0), 0, "nicht gelernter Skill meldet castFailed");
    w.players[pi].skills = vec!["fireball".into()];
    w.events.clear();
    cast_skill(&mut w, &v, pi, 0, true);
    let evs = events_of_type(&w, "castFailed");
    let p = &w.players[pi];
    assert!(
        evs.len() == 1
            && evs[0]["from"] == p.id
            && evs[0]["slot"] == 0
            && evs[0]["x"] == unit_x(p.x)
            && (p.cooldowns.is_empty() || p.cooldowns[0] == 0.0),
        "ohne Ziel: {evs:?}, Cooldowns {:?}",
        p.cooldowns
    );
    w.players[pi].cooldowns = vec![5.0, 0.0, 0.0, 0.0];
    assert_eq!(failed(&mut w, 0), 0, "laufende Abklingzeit meldet castFailed");
}

/// Zahl der `castFailed`-Ereignisse des Spielers `pid` nach `ticks` Ticks mit gehaltener Taste `skill`.
fn count_failed(w: &mut World, pi: usize, skill: i64, ticks: usize) -> usize {
    let (pid, mut n) = (w.players[pi].id, 0);
    for _ in 0..ticks {
        w.events.clear();
        skills_tick(w, pi, &skill_cmd(skill));
        n += events_of_type(w, "castFailed")
            .iter()
            .filter(|ev| ev["from"] == pid)
            .count();
    }
    n
}

fn learn_fireball_only(p: &mut Player) {
    p.slots = vec!["fireball".into()];
    p.skills = vec!["fireball".into()];
}

/// Gehaltene Skill-Taste ohne Ziel: ein `castFailed` je Tastendruck, nicht je Tick (B-378).
#[test]
fn gehaltene_skill_taste_meldet_cast_failed_einmal() {
    let mut w = quiet_world();
    let (p, q) = (add_player(&mut w), add_player(&mut w));
    learn_fireball_only(&mut w.players[p]);
    learn_fireball_only(&mut w.players[q]);
    assert_eq!(
        count_failed(&mut w, p, 1, 30),
        1,
        "30 Ticks gehalten: castFailed, erwartet 1"
    );
    count_failed(&mut w, p, 0, 1); // loslassen
    assert_eq!(count_failed(&mut w, p, 1, 30), 1, "neuer Druck: castFailed, erwartet 1");
    assert_eq!(
        count_failed(&mut w, q, 1, 30),
        1,
        "Spieler 2 unabhängig: castFailed, erwartet 1"
    );
    count_failed(&mut w, p, 0, 1);
    let px = w.players[p].x;
    let e = tough_enemy(&mut w, px + 5.0); // Ziel erscheint beim Halten: wirkt, kein castFailed
    let n = count_failed(&mut w, p, 1, 1);
    let cd = &w.players[p].cooldowns;
    assert!(
        n == 0 && !cd.is_empty() && cd[0] != 0.0 && lost(&w, e) != 0.0,
        "mit Ziel: {n} castFailed, CD {cd:?}, Schaden {}",
        lost(&w, e)
    );
}

/// B-524/AC-01: Der Tod vergisst die gehaltene Skill-Taste. Halten ohne Ziel (ein castFailed), sterben, wiederbeleben, neu
/// drücken → ein zweites castFailed; Spieler 2 hält durchgehend und meldet kein weiteres.
#[test]
fn skill_taste_nach_tod_meldet_cast_failed_neu() {
    let mut w = quiet_world();
    let (p, q) = (add_player(&mut w), add_player(&mut w));
    learn_fireball_only(&mut w.players[p]);
    learn_fireball_only(&mut w.players[q]);
    let (pid, qid) = (w.players[p].id, w.players[q].id);
    let step = |w: &mut World, skill_p: i64| {
        w.events.clear();
        let cmds = [skill_cmd(skill_p), skill_cmd(1)];
        step_players(w, &cmds, None, DT);
        let (mut np, mut nq) = (0, 0);
        for ev in events_of_type(w, "castFailed") {
            if ev["from"] == pid {
                np += 1;
            } else if ev["from"] == qid {
                nq += 1;
            }
        }
        (np, nq)
    };
    assert_eq!(step(&mut w, 1), (1, 1), "erster Druck: castFailed, erwartet je 1");
    fell(&mut w, p); // verliert die Krone mit gehaltener Taste
    for _ in 0..3 {
        assert_eq!(step(&mut w, 0), (0, 0), "ohne Krone: castFailed");
    }
    w.players[p].crown = None;
    assert_eq!(step(&mut w, 1), (1, 0), "neuer Druck nach dem Respawn: erwartet 1/0");
}

/// B-378/AC-02 (Teil): Alle Spieler halten Skill 1 ohne Ziel; es gibt höchstens je Spieler ein `castFailed`. Der Ereignis-Budget-
/// Teil von Go (`benchIsland`, `tickBytes`, `meanP99`, `events_budget_test.go`) kommt mit RU19, wenn die Ereignisse portiert
/// sind; hier läuft dieselbe Zählung auf einer Welt mit zwei Spielern.
#[test]
fn gehaltene_skill_tasten_im_ereignis_budget() {
    let mut w = quiet_world();
    let players = [add_player(&mut w), add_player(&mut w)];
    for &pi in &players {
        learn_fireball_only(&mut w.players[pi]);
    }
    let held = vec![skill_cmd(1); players.len()];
    let mut failed = 0;
    for _ in 0..1800 {
        crate::world::step(&mut w, &held, None, DT);
        failed += events_of_type(&w, "castFailed").len();
    }
    assert!(
        failed <= players.len(),
        "{failed} castFailed bei {} gehaltenen Tasten, erwartet höchstens je Spieler eines",
        players.len()
    );
}

/// Zwei Spieler halten Skill 1 ohne gelernten Skill: Es wird nichts gemeldet, aber die Taste wird je Spieler gemerkt und beim
/// Loslassen vergessen.
#[test]
fn own_gehaltene_taste_wird_je_spieler_gemerkt() {
    let mut w = quiet_world();
    let (p, q) = (add_player(&mut w), add_player(&mut w));
    skills_tick(&mut w, p, &skill_cmd(1));
    assert!(w.players[p].skill_held == 1 && w.players[q].skill_held == 0);
    skills_tick(&mut w, q, &skill_cmd(3));
    assert!(w.players[p].skill_held == 1 && w.players[q].skill_held == 3);
    skills_tick(&mut w, p, &skill_cmd(0));
    assert!(w.players[p].skill_held == 0 && w.players[q].skill_held == 3);
    assert!(
        w.events.is_empty() && w.players[p].cooldowns.is_empty(),
        "ohne gelernten Skill keine Meldung, keine Abklingzeit"
    );
}
