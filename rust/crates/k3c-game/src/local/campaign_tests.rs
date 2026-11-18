//! Tests der Kampagne (Go: `campaign_test.go`).

use std::rc::Rc;

use k3c_room::room::Seat;
use k3c_sim::state::{SwitchState, World};

use super::campaign::Campaign;
use super::peer::StageState;
use super::testutil::{TWO_DEVICES, TempDir, start_test_game, test_session};
use crate::game::sim::Sim as _;
use crate::view::frame::Event;
use crate::view::session::Session as _;

fn event(v: serde_json::Value) -> Event {
    Event {
        data: v.as_object().cloned().unwrap_or_default(),
        ..Event::default()
    }
}

// B-974: Frame::campaign nennt Welt und Insel des laufenden Spiels; `fresh` steht genau im ersten Frame einer neuen
// Kampagne, nie nach dem Laden eines Spielstands (welten.md § 6.2: Laden ist kein Wendepunkt).
#[test]
fn campaign_fresh_only_on_new_game() {
    use crate::view::session::NewGame;
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    let data = super::worlds::game_data().expect("Daten");
    let base = k3c_sim::world::base::base().expect("Basis");
    let start = k3c_sim::island::standard_world(&data, &base.rules)
        .expect("Standard-Welt")
        .islands[0]
        .id
        .clone();
    let check = |when: &str, c: crate::view::story::Campaign, run: u64, fresh: bool| {
        let island = if run > 0 { start.as_str() } else { "" };
        assert!(
            c.run == run && c.fresh == fresh && c.island == island && c.completed.is_empty(),
            "{when}: {c:?}"
        );
    };
    check("ohne Spiel", s.step().campaign, 0, false);
    s.new_game(NewGame {
        devices: TWO_DEVICES.to_vec(),
        ..NewGame::default()
    })
    .expect("neues Spiel");
    check("erster Frame des neuen Spiels", s.step().campaign, 1, true);
    check("zweiter Frame", s.step().campaign, 1, false);
    s.save().expect("speichern");
    s.new_game(NewGame {
        name: "zwei".into(),
        devices: TWO_DEVICES.to_vec(),
        ..NewGame::default()
    })
    .expect("zweites Spiel");
    check("zweites Spiel", s.step().campaign, 2, true);
    s.load("spiel-1", &TWO_DEVICES).expect("laden");
    for _ in 0..3 {
        check("geladen", s.step().campaign, 3, false);
    }
}

// Jedes Spiel zählt die Nummer weiter und meldet seine Kampagne genau einmal als neu, sobald der Frame Stufen und Plätze
// hat. Die Welt ist bis RU19 nicht lesbar: Die Geschichte entfällt, das Spiel läuft.
#[test]
fn campaign_run_counts_games_and_fresh_once() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert_eq!(s.step().campaign.run, 0, "ohne Spiel");
    start_test_game(&mut s, "eins", &TWO_DEVICES[..1]);
    s.pause(true);
    let c = s.step().campaign;
    assert!(c.run == 1 && c.fresh, "erster Frame: {c:?}");
    let c = s.step().campaign;
    assert!(c.run == 1 && !c.fresh, "zweiter Frame: {c:?}");
    start_test_game(&mut s, "zwei", &TWO_DEVICES[..1]);
    s.pause(true);
    let c = s.step().campaign;
    assert!(c.run == 2 && c.fresh, "zweites Spiel: {c:?}");
}

// `fresh` wartet, bis der Frame Stufen und Plätze hat.
#[test]
fn campaign_fresh_waits_for_ready_frame() {
    let mut c = Campaign {
        run: 1,
        fresh: true,
        island: "a".into(),
        ..Campaign::default()
    };
    assert!(!c.frame(false).fresh && c.fresh, "noch nicht gemeldet");
    let out = c.frame(true);
    assert!(out.fresh && out.run == 1 && out.island == "a");
    assert!(!c.frame(true).fresh, "nur einmal");
}

// Insel und abgeschlossene Inseln eines geladenen Spielstands (`islandId`, `completed`); fehlt die Datei oder ist sie
// kaputt, ist es ein Fehler (die Geschichte entfällt).
#[test]
fn campaign_reads_position_from_save() {
    let dir = TempDir::new();
    let s = test_session(&dir);
    std::fs::write(
        dir.path().join("stand.json"),
        r#"{"campaignId":"x","islandId":"b","completed":["a"],"version":1}"#,
    )
    .expect("schreiben");
    let pos = s.read_position("stand").expect("lesen");
    assert_eq!(pos.island_id, "b");
    assert_eq!(pos.completed, ["a"]);
    assert!(s.read_position("fehlt").is_err());
    std::fs::write(dir.path().join("kaputt.json"), "{").expect("schreiben");
    assert!(s.read_position("kaputt").is_err());
}

// Der Stand folgt dem Inselwechsel: Die verlassene Insel gilt als abgeschlossen, das Ziel als aktuelle. Meldet der Takt den
// Wechsel, stehen die Spieler aber weiter am Wechselpunkt (der Raum hat nicht getauscht), bleibt alles.
#[test]
fn campaign_follows_island_switch() {
    let gate = World {
        island_switch: Some(SwitchState {
            open: true,
            ready: true,
            ..SwitchState::default()
        }),
        ..World::default()
    };
    let mut st = StageState::default();
    st.events = vec![
        event(serde_json::json!({"type": "night"})),
        event(serde_json::json!({"type": "islandSwitch", "target": "b"})),
    ];
    let seats = [Seat {
        slot: 0,
        monarch: 0,
        depth: 0,
        stage: 1,
    }];
    st.worlds.insert(0, Rc::new(gate.clone()));
    st.worlds.insert(1, Rc::new(World::default()));
    let mut c = Campaign {
        island: "a".into(),
        ..Campaign::default()
    };
    c.follow(&st, &seats); // Stufe 0 trägt noch die alte Insel, die Spieler stehen in der neuen
    assert!(
        c.island == "b" && c.completed == ["a"],
        "nach dem Wechsel: Insel {:?}, abgeschlossen {:?}",
        c.island,
        c.completed
    );
    c.follow(&st, &seats); // derselbe Takt noch einmal: nichts doppelt
    assert!(c.island == "b" && c.completed.len() == 1, "doppelt gefolgt");
    let mut stuck = Campaign {
        island: "a".into(),
        ..Campaign::default()
    };
    st.worlds.insert(1, Rc::new(gate));
    stuck.follow(&st, &seats);
    assert!(
        stuck.island == "a" && stuck.completed.is_empty(),
        "Wechsel nicht vollzogen: Insel {:?}",
        stuck.island
    );
}
