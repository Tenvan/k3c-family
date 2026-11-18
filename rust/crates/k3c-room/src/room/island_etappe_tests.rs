//! Test aus `island_etappe_test.go` (B-961, AC-03): Etappen im Raum. Die Wahl des Ziels läuft über `Room::choose_island`
//! (`skills.rs`, Gos `Room.ChooseIsland`). Der Spielstand trägt Wahl und abgeschlossene Inseln. Welt `etappen`:
//! Insel `ia` (Etappe 1), danach `ib` und `ic` (Etappe 2, frei wählbar), 2 Spieler.
use super::Options;
use super::island_switch_tests::board_harbor;
use super::testutil::{TestManager, WorldDir, dev_manager, join_room, peer, rm, ticks};
use crate::fehler::Fehler;

const ETAPPEN_RAW: &str = r#"{"id":"etappen","name":"Etappen","islands":[{"id":"ia","name":"A","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},{"id":"ib","name":"B","etappe":2,"depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},{"id":"ic","name":"C","etappe":2,"depths":[0,4],"depthScaling":{"hp":1.7,"damage":1.5,"speed":1.2},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}}]}"#;

fn etappen_manager(tag: &str) -> (TestManager, WorldDir) {
    let dir = WorldDir::new(tag, &[("etappen", ETAPPEN_RAW)]);
    let mut m = dev_manager();
    m.worlds = Some(dir.src.clone());
    (m, dir)
}

/// Wählt für den Monarchen im Slot 0 des Geräts `id` (Verbindung `conn`) das Ziel `target` (Gos `Room.ChooseIsland`).
fn choose(m: &mut TestManager, code: &str, id: &str, conn: u64, target: &str) -> Result<(), Fehler> {
    m.with_room(code, |r, env| r.choose_island(env, id, conn, 0, target))
        .expect("Raum")
}

fn json(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).expect("JSON")
}

#[test]
fn etappen_wahl_speichern_laden_und_wechsel() {
    let (mut m, _dir) = etappen_manager("etap");
    let (p, _x) = peer(1);
    let code = m
        .create(
            "xbox",
            p,
            "etap",
            true,
            4,
            &[0],
            Options {
                world: "etappen".into(),
                ..Options::default()
            },
        )
        .expect("create");
    let _h = join_room(&mut m, "handy", 2, &code, &[0]);
    board_harbor(&mut rm(&mut m, &code).isl);
    rm(&mut m, &code).isl.endboss_defeated = true;
    ticks(&mut m, &code, 1); // Wechselpunkt offen
    assert!(
        choose(&mut m, &code, "xbox", 1, "unbekannt").is_err(),
        "Wahl außerhalb der Ziele angenommen"
    );
    assert_eq!(choose(&mut m, &code, "handy", 2, "ic"), Ok(()));
    ticks(&mut m, &code, 1);
    let r = rm(&mut m, &code);
    assert!(
        r.isl.number == 0 && r.isl.chosen_target() == "ic",
        "Ziel {:?} (Nummer {})",
        r.isl.chosen_target(),
        r.isl.number
    );
    m.with_room(&code, |r, env| r.save_now(env))
        .expect("Raum")
        .expect("speichern");
    let s = json(&m.store.data["etap"]);
    assert!(
        s["chosen"] == "ic"
            && s.get("completed")
                .is_none_or(|c| c.as_array().is_some_and(Vec::is_empty)),
        "Stand vor dem Wechsel: {s}"
    );
    let pre = m.store.data["etap"].clone();
    let post = wechsel_und_speichern(&mut m, &code);
    etappen_laden(&pre, &post);
}

/// Rechnet bis zum Wechsel zu `ic`, prüft Nummer und Stand danach und liefert den gespeicherten Stand.
fn wechsel_und_speichern(m: &mut TestManager, code: &str) -> Vec<u8> {
    for _ in 0..200 {
        if rm(m, code).isl.number != 0 {
            break;
        }
        ticks(m, code, 1);
    }
    let r = rm(m, code);
    let got = format!("{} {} {}", r.isl.number != 0, r.isl.island_id(), r.isl.number);
    assert_eq!(got, "true ic 2", "Insel getauscht/ID/Nummer");
    m.with_room(code, |r, env| r.save_now(env))
        .expect("Raum")
        .expect("speichern");
    let post = m.store.data["etap"].clone();
    let s = json(&post);
    assert!(
        s["islandId"] == "ic" && s["completed"] == serde_json::json!(["ia"]) && s.get("chosen").is_none_or(|c| c == ""),
        "Stand nach dem Wechsel: {s}"
    );
    post
}

/// Der Stand vor dem Wechsel (`pre`) behält die Wahl, der nach dem Wechsel (`post`) die abgeschlossenen Inseln.
fn etappen_laden(pre: &[u8], post: &[u8]) {
    let (mut m1, _d1) = etappen_manager("etap-pre");
    m1.store.data.insert("etap".into(), pre.to_vec());
    let (p, _) = peer(1);
    let c1 = m1
        .create("xbox", p, "etap", false, 0, &[0, 1], Options::default())
        .expect("laden vor dem Wechsel");
    assert_eq!(rm(&mut m1, &c1).isl.chosen, "ic", "Wahl nach dem Laden");
    let (mut m2, _d2) = etappen_manager("etap-post");
    m2.store.data.insert("etap".into(), post.to_vec());
    let (p, _) = peer(2);
    let c2 = m2
        .create("xbox", p, "etap", false, 0, &[0, 1], Options::default())
        .expect("laden nach dem Wechsel");
    let r2 = rm(&mut m2, &c2);
    r2.isl.endboss_defeated = true;
    let targets = r2.isl.targets();
    assert!(
        r2.isl.island_id() == "ic" && r2.isl.completed.len() == 1 && targets.len() == 1 && targets[0].id == "ib",
        "geladen: Insel {:?}, abgeschlossen {:?}, Ziele {:?}",
        r2.isl.island_id(),
        r2.isl.completed,
        targets
    );
}
