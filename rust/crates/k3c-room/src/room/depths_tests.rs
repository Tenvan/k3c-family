//! Tests aus `depths_test.go` (B-199, B-290, B-204): ein neuer Raum spielt die ganze Insel, geladene Stände behalten ihre
//! Stufen, ein leerer Test-Raum wird sofort aufgeräumt, ein normaler nach `EMPTY_FOR_SECS`.
use k3c_sim::island::{CreateOptions, create_island};

use super::testutil::{TestManager, create_room, manager, peer, rm, wait};
use super::{EMPTY_FOR_SECS, Options, WAIT_FOR_SECS};
use crate::fehler::Fehler;

// B-199, B-290: Ein neuer Raum spielt die ganze Insel, eine Stufe je Biom (heute fünf); geladene Stände behalten ihre Stufen.
#[test]
fn neuer_raum_hat_alle_stufen() {
    let (mut m, _) = manager();
    let (code, _) = create_room(&mut m, "xbox", 1, "insel", 4, &[0]);
    let r = rm(&mut m, &code);
    assert_eq!(r.isl.stages.len(), 5, "Stufen");
    for (i, w) in r.isl.stages.iter().enumerate() {
        assert_eq!(w.biome.depth, i as i64, "Stufe {i}");
    }
    assert_eq!(r.isl.stages[r.start].biome.depth, 4, "Startstufe");
    let (p, _) = peer(2);
    let err = m
        .create("handy", p, "zu-tief", true, 5, &[0], Options::default())
        .expect_err("Startstufe 5");
    assert!(err.is(&Fehler::BadRequest), "{err}");
}

#[test]
fn gespeicherter_stand_behaelt_stufen() {
    let (mut m, _) = manager();
    let std_world = m.world("").expect("Standard-Welt");
    let isl = create_island(&m.data, &std_world, "drei", &[0, 1, 2], 1.0, CreateOptions::default()).expect("Insel");
    let save = isl.to_save("2026-10-01T12:00:00.000Z").expect("Spielstand");
    m.store
        .data
        .insert("drei".into(), serde_json::to_vec(&save).expect("JSON"));
    let (p, _) = peer(1);
    let code = m
        .create("xbox", p, "drei", false, 0, &[0], Options::default())
        .expect("create");
    assert_eq!(rm(&mut m, &code).isl.stages.len(), 3, "Stufen nach Laden");
}

fn names(m: &TestManager) -> Vec<String> {
    m.rooms().into_iter().map(|i| i.name).collect()
}

// B-204: Ein leerer Test-Raum ist nach dem nächsten Sweep weg, ein wartender Monarch hält ihn bis WaitFor offen.
#[test]
fn leerer_test_raum_schliesst_sofort() {
    let (mut m, clock) = manager();
    let (gone, _) = create_room(&mut m, "xbox", 1, "test-leer", 0, &[0]);
    let (held, _) = create_room(&mut m, "handy", 2, "test-wartet", 0, &[0]);
    m.with_room(&gone, |r, env| r.leave(env, "xbox", 1));
    m.with_room(&held, |r, env| r.drop_conn(env, "handy", 2));
    wait(&mut m, &clock, 1);
    assert_eq!(names(&m), ["test-wartet"], "Räume nach erstem Sweep");
    assert_eq!(m.store.deleted, ["test-leer"], "gelöscht");
    wait(&mut m, &clock, WAIT_FOR_SECS);
    assert!(
        m.rooms().is_empty() && m.store.deleted.len() == 2,
        "{:?} {:?}",
        names(&m),
        m.store.deleted
    );
}

#[test]
fn normaler_raum_bleibt_empty_for() {
    let (mut m, clock) = manager();
    let (code, _) = create_room(&mut m, "xbox", 1, "familie", 0, &[0]);
    m.with_room(&code, |r, env| r.leave(env, "xbox", 1));
    wait(&mut m, &clock, EMPTY_FOR_SECS - 1);
    assert_eq!(m.rooms().len(), 1, "normaler Raum vor EMPTY_FOR aufgeräumt");
    wait(&mut m, &clock, 1);
    assert!(
        m.rooms().is_empty() && m.store.deleted.is_empty(),
        "{:?}",
        m.store.deleted
    );
}
