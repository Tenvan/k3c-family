//! Tests aus `autosave_test.go` (B-186, ST1.1): Der Raum speichert alle `SAVE_TICKS` und beim Tageswechsel; nach einem
//! gelungenen Schreiben trägt der nächste Zustand jeder Stufe das Ereignis `saved`, danach ist es aus der Welt.
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::save_testutil::{count, create_with, fast_cycle};
use super::testutil::{TestManager, join_room, manager, rm, ticks};
use super::types::{Options, SAVE_TICKS};
use crate::fehler::Fehler;

const TAKT: usize = SAVE_TICKS as usize;

// B-186/AC-01: Nach `SAVE_TICKS` Ticks liegt ein neuer Stand vor, vorher keiner; der folgende Zustand meldet saved.
#[test]
fn speichert_im_takt() {
    let (mut m, _) = manager();
    let (code, mut xi) = create_with(&mut m, "xbox", 1, "takt", true, Options::default());
    let start = m.store.saves;
    ticks(&mut m, &code, TAKT - 1);
    assert_eq!(m.store.saves, start, "vor dem Takt");
    assert_eq!(count(&mut xi, "saved").0, 0, "saved ohne Speicherung");
    ticks(&mut m, &code, 1);
    assert_eq!(m.store.saves, start + 1, "nach {TAKT} Ticks");
    ticks(&mut m, &code, 1);
    let (n, ev) = count(&mut xi, "saved");
    let ev = ev.expect("saved");
    assert!(
        n == 1 && ev["stage"] == 0,
        "saved im Zustand nach dem Speichern: {n}, {ev}"
    );
    // das Ereignis `saved` aus dem früheren Protokoll-Beispiel s2c-snapshot-gesichert.json (B-897)
    let want: serde_json::Value = serde_json::json!({ "type": "saved", "stage": 0 });
    let keys = |v: &serde_json::Value| v.as_object().map(|o| o.keys().cloned().collect::<Vec<_>>());
    assert_eq!(keys(&ev), keys(&want), "saved hat andere Schlüssel als das Beispiel");
    let r = rm(&mut m, &code);
    assert!(
        !r.isl.stages[0].events.iter().any(|e| e["type"] == "saved"),
        "saved bleibt in der Welt"
    );
    ticks(&mut m, &code, 1);
    assert_eq!(count(&mut xi, "saved").0, 0, "saved zweimal gemeldet");
}

// B-186/AC-01: Beim Tagesanbruch speichert der Raum im selben Tick.
#[test]
fn speichert_bei_tagesanbruch() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "morgen", true, Options::default());
    fast_cycle(&mut m, &code);
    for _ in 0..TAKT - 1 {
        let (day, before) = (rm(&mut m, &code).isl.stages[0].cycle.day, m.store.saves);
        ticks(&mut m, &code, 1);
        if rm(&mut m, &code).isl.stages[0].cycle.day > day {
            assert_eq!(m.store.saves, before + 1, "Tagesanbruch Tag {}", day + 1);
            return;
        }
    }
    panic!("kein Tagesanbruch vor dem Takt");
}

// B-186: Scheitert das Schreiben, meldet der Raum kein saved.
#[test]
fn kein_saved_bei_schreibfehler() {
    let (mut m, _) = manager();
    let (code, mut xi) = create_with(&mut m, "xbox", 1, "fehler", true, Options::default());
    m.store.fail = Some(Fehler::Io("gesperrt".into()));
    ticks(&mut m, &code, TAKT);
    ticks(&mut m, &code, 1);
    assert_eq!(count(&mut xi, "saved").0, 0, "saved trotz Schreibfehler");
}

fn cmd(move_x: f64, pay: bool, sprint: bool) -> PlayerCommand {
    PlayerCommand {
        move_x,
        pay,
        sprint,
        ..PlayerCommand::default()
    }
}

fn input(m: &mut TestManager, code: &str, id: &str, conn: u64, slot: i64, c: PlayerCommand) {
    let map = BTreeMap::from([(slot, c)]);
    let res = m.with_room(code, |r, _| r.input(id, conn, &map)).expect("Raum");
    assert_eq!(res, Ok(()), "Eingabe {id}");
}

/// Drei Räume mit 2, 3 und 4 Spielern und festen Eingaben (Go `setup` aus `run_test.go`).
fn setup(m: &mut TestManager) -> Vec<String> {
    let mk = |m: &mut TestManager, id: &str, conn: u64, name: &str, slots: &[i64]| {
        let (p, _) = super::testutil::peer(conn);
        m.create(id, p, name, true, 0, slots, Options::default())
            .expect("create")
    };
    let r1 = mk(m, "a", 1, "eins", &[0, 1]);
    let r2 = mk(m, "b", 2, "zwei", &[0, 1, 2]);
    let r3 = mk(m, "c", 3, "drei", &[0, 1]);
    let _di = join_room(m, "d", 4, &r3, &[0, 1]);
    input(m, &r1, "a", 1, 0, cmd(1.0, false, false));
    input(m, &r1, "a", 1, 1, cmd(-1.0, true, false));
    input(m, &r2, "b", 2, 2, cmd(1.0, false, true));
    input(m, &r3, "c", 3, 0, cmd(-1.0, false, false));
    input(m, &r3, "d", 4, 1, cmd(1.0, true, false));
    vec![r1, r2, r3]
}

/// Die Stufen des Raums als JSON (Go `world`).
fn world(m: &mut TestManager, code: &str) -> String {
    serde_json::to_string(&rm(m, code).isl.stages).expect("JSON")
}

// B-186/AC-02: Der Takt ändert den Spielverlauf nicht. Dieselben Räume mit und ohne Takt (`save_due` vor jedem Tick auf 0)
// rechnen nach mehr als zwei Takten dieselbe Welt.
#[test]
fn takt_aendert_spielverlauf_nicht() {
    let (mut with, _) = manager();
    let (mut without, _) = manager();
    let (a, b) = (setup(&mut with), setup(&mut without));
    let n = 2 * TAKT + 30;
    for (cw, co) in a.iter().zip(&b) {
        ticks(&mut with, cw, n);
        for _ in 0..n {
            rm(&mut without, co).save_due = 0;
            ticks(&mut without, co, 1);
        }
        assert!(
            with.store.saves > without.store.saves,
            "Raum {cw}: kein Takt gespeichert"
        );
        assert!(
            world(&mut with, cw) == world(&mut without, co),
            "Raum {cw}: Welt mit Takt weicht ab"
        );
    }
}
