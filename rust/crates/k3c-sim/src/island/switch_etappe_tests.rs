//! Ziele und Wahl am Hafen (Go: `island_switch_etappe_test.go`, B-961 AC-02), mit 2 Spielern. Die vier Go-Tests bauen eine
//! Insel (`create_world_with`, `step`, RU14) und tragen `ignore`. Die Tests ab `own_` prüfen Etappen, Ziele, Wahl und
//! `apply_choices` an einer Welt von Hand (`world_of`).
use super::save_tests::world_of;
use super::switch_testutil::*;
use super::testutil::*;
use super::{Island, WorldDef, choose_island, next_island, step_island_switch};

/// Go: `choose`: die Wahl des Spielers mit dem Index `index` (in der Stufe, in der er steht).
fn choose(isl: &mut Island, index: i64, id: &str) -> Result<(), String> {
    let si = isl.stage_of(index).expect("Stufe des Spielers");
    let pi = isl.stages[si]
        .players
        .iter()
        .position(|p| p.index == index)
        .expect("Platz des Spielers");
    choose_island(isl, si, pi, id)
}

#[test]
fn etappen_ziele_und_erste_linie() {
    let data = test_data();
    let (w, two, three) = etappen_welt(&data);
    let mut isl = gate_island(&data, &w, 2);
    assert_eq!(target_ids(&isl), [two.id.clone(), three.id.clone()], "Ziele");
    assert!(
        choose(&mut isl, 0, &two.id).is_err(),
        "Wahl vor dem offenen Wechselpunkt angenommen"
    );
    step_seconds(&mut isl, &data, 3.0); // ohne Wahl: erstes Ziel
    let def = isl.next_def();
    assert!(
        def.as_ref().map(|d| d.id.as_str()) == Some(two.id.as_str()) && isl.switch_ready,
        "ohne Wahl: {:?} bereit {}",
        def.map(|d| d.id),
        isl.switch_ready
    );
    let st = isl.stages[0].island_switch.clone().expect("Zustand");
    assert!(st.targets.len() == 2 && st.choice == two.id, "Zustand {st:?}");
}

#[test]
fn etappen_wahl_rest_und_sieg() {
    let data = test_data();
    let (w, two, three) = etappen_welt(&data);
    let mut isl = gate_island(&data, &w, 2);
    open_gate(&mut isl, &data);
    assert!(
        choose(&mut isl, 0, "gibtsnicht").is_err(),
        "Wahl außerhalb der Ziele angenommen"
    );
    isl.stages[0].players[0].free = true;
    assert!(
        choose(&mut isl, 0, &three.id).is_err(),
        "Wahl eines freien Spielers angenommen"
    );
    isl.stages[0].players[0].free = false;
    assert_eq!(isl.chosen_target(), two.id, "Ziel nach Ablehnung");
    // Zwei Spieler wählen im selben Takt: der höchste Spieler-Index gewinnt, unabhängig von der Aufrufreihenfolge.
    choose(&mut isl, 1, &three.id).expect("Wahl 1");
    choose(&mut isl, 0, &two.id).expect("Wahl 0");
    step_seconds(&mut isl, &data, 3.0);
    let def = isl.next_def().expect("Ziel");
    assert_eq!(def.id, three.id, "gewähltes Ziel");
    let next = next_island(&data, &isl, &def).expect("nächste Insel");
    etappen_kette(&data, &isl, next, &two);
}

/// Go: `etappenKette`: nach dem Wechsel zu Insel 3 (`next`) bleibt nur Insel 2, danach gibt es keine Ziele mehr.
fn etappen_kette(data: &k3c_data::Data, isl: &Island, mut next: Island, two: &super::IslandDef) {
    assert!(
        next.number == 2 && next.island_id == "test-island3" && next.completed == [isl.world.islands[0].id.clone()],
        "Insel {} {:?}, abgeschlossen {:?}",
        next.number,
        next.island_id,
        next.completed
    );
    assert!(
        next_island(data, isl, two).is_err(),
        "next_island baut ein nicht gewähltes Ziel"
    );
    // Auf Insel 3: Endboss besiegt, es bleibt nur Insel 2; danach keine Ziele mehr (letzte Insel, Sieg möglich).
    next.endboss_defeated = true;
    assert_eq!(target_ids(&next), [two.id.as_str()], "Ziele auf Insel 3");
    let mut last = next_island(data, &next, two).expect("letzte Insel");
    last.endboss_defeated = true;
    assert!(
        last.targets().is_empty() && last.is_last() && last.number == 1,
        "letzte Insel: Ziele {:?}, is_last {}, Platz {}",
        last.targets(),
        last.is_last(),
        last.number
    );
}

/// Der Seed hängt am Listenplatz des Ziels: gleiche Wahl, gleiche Insel; andere Wahl, andere Insel.
#[test]
fn etappen_seed_am_listenplatz() {
    let data = test_data();
    let (w, two, three) = etappen_welt(&data);
    let build = |id: &str| {
        let mut isl = gate_island(&data, &w, 2);
        isl.chosen = id.to_string();
        let def = isl.next_def().expect("Ziel");
        next_island(&data, &isl, &def).expect("nächste Insel")
    };
    let (a, b, c) = (build(&three.id), build(&three.id), build(&two.id));
    assert!(
        a.seed == b.seed && a.seed != c.seed,
        "Seeds {:?} {:?} {:?}",
        a.seed,
        b.seed,
        c.seed
    );
}

/// Linear wie heute: ohne Etappen zwei Inseln, ein Ziel.
#[test]
fn etappen_linear_ohne_feld() {
    let data = test_data();
    let (w, two) = test_islands(&data);
    let isl = gate_island(&data, &w, 2);
    assert_eq!(target_ids(&isl), [two.id], "Ziele");
}

/// Eine Welt von Hand mit den Etappen 1, 2, 2 (Go: `etappenWelt`).
fn hand_etappen() -> WorldDef {
    let mut w = world_of(&["island1", "two", "three"]);
    w.islands[1].etappe = 2;
    w.islands[2].etappe = 2;
    w
}

fn hand_island(w: WorldDef, number: i64) -> Island {
    Island {
        world: w,
        number,
        ..Island::default()
    }
}

#[test]
fn own_ziele_der_etappen_und_erste_linie() {
    let mut isl = hand_island(hand_etappen(), 0);
    assert_eq!(
        target_ids(&isl),
        ["two", "three"],
        "die Etappe 1 hat nur Insel 1: die nächste Etappe öffnet"
    );
    isl.endboss_defeated = true;
    assert_eq!(target_ids(&isl), ["two", "three"], "Ziele in Listenreihenfolge");
    let t = isl.targets();
    assert_eq!((t[0].etappe, t[1].etappe), (2, 2));
    assert_eq!(isl.chosen_target(), "two", "ohne Wahl das erste Ziel");
    assert_eq!(isl.next_def().map(|d| d.id), Some("two".to_string()));
    isl.chosen = "three".into();
    assert_eq!(isl.chosen_target(), "three");
    assert_eq!(isl.chosen_index(), Some(2));
}

#[test]
fn own_etappenkette_bis_zur_letzten_insel() {
    // Insel 3 (Etappe 2): Insel 1 ist abgeschlossen, bleibt nur Insel 2; danach keine Ziele mehr.
    let mut on3 = hand_island(hand_etappen(), 2);
    on3.completed = vec!["island1".into()];
    on3.endboss_defeated = true;
    assert_eq!(target_ids(&on3), ["two"]);
    let mut last = hand_island(hand_etappen(), 1);
    last.completed = vec!["island1".into(), "three".into()];
    last.endboss_defeated = true;
    assert!(last.targets().is_empty() && last.is_last());
    assert!(last.next_def().is_none() && last.chosen_target().is_empty());
}

#[test]
fn own_linear_ohne_etappen_feld_und_unbekannte_insel() {
    let mut isl = hand_island(world_of(&["a", "b", "c"]), 0);
    isl.endboss_defeated = true;
    assert_eq!(target_ids(&isl), ["b"], "Etappe = Listenplatz + 1: nur die nächste");
    assert_eq!(isl.world.etappe_of(2), 3);
    assert!(
        hand_island(world_of(&["a"]), 5).target_indexes().is_empty(),
        "Listenplatz außerhalb: keine Ziele"
    );
    assert!(hand_island(world_of(&["a"]), -1).target_indexes().is_empty());
}

#[test]
fn own_targets_from_ist_eine_reine_funktion_der_wendepunkte() {
    let w = hand_etappen();
    assert_eq!(w.targets_from(0, |_| false), [1, 2], "alle offen: die nächste Etappe");
    assert_eq!(w.targets_from(1, |_| false), [2], "gleiche Etappe zuerst");
    assert_eq!(
        w.targets_from(1, |i| i == 2),
        [] as [i64; 0],
        "Etappe 2 fertig, keine Etappe 3"
    );
    assert_eq!(w.targets_from(1, |i| i == 0), [2]);
}

#[test]
fn own_wahl_vor_dem_offenen_wechselpunkt_wird_abgelehnt() {
    let mut isl = hand_island(hand_etappen(), 0);
    isl.endboss_defeated = true;
    let err = choose_island(&mut isl, 0, 0, "two").expect_err("Wechselpunkt zu");
    assert_eq!(err, "insel: Wechselpunkt nicht offen");
    isl.gate_open = true;
    isl.switch_ready = true;
    assert!(
        choose_island(&mut isl, 0, 0, "two").is_err(),
        "nach dem Wechsel keine Wahl mehr"
    );
    assert!(isl.pending_choice.is_empty());
}

#[test]
fn own_apply_choices_der_hoechste_spieler_index_gewinnt() {
    let mut isl = hand_island(hand_etappen(), 0);
    isl.apply_choices();
    assert!(isl.chosen.is_empty(), "ohne Wahl bleibt es leer");
    isl.pending_choice.insert(1, "three".into());
    isl.pending_choice.insert(0, "two".into());
    isl.apply_choices();
    assert_eq!(isl.chosen, "three", "unabhängig von der Einfügereihenfolge");
    assert!(isl.pending_choice.is_empty(), "die Wahlen sind übernommen");
    step_island_switch(&mut isl, 1.0); // ohne Endboss: nichts mehr
    assert_eq!(isl.chosen, "three");
}
