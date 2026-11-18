//! Tests aus `worlds_values_test.go` (BAL8.2, B-681/AC-01, `docs/rules/welten.md` § 1/§ 6): Zwei Räume mit verschiedenen
//! Welten rechnen mit ihren eigenen Kosten; der Spielstand lädt mit dem Wertesatz seiner Welt; ungültige Overrides lehnen
//! die Welt ab.
use k3c_sim::economy::site_gold_left;
use k3c_sim::world::vals;

use super::testutil::{TestManager, WorldDir, peer, rm};
use super::worlds_tests::world_fixture;
use super::{Options, Room};

fn opts(world: &str) -> Options {
    Options {
        world: world.to_string(),
        ..Options::default()
    }
}

/// Das Gold, das die erste bezahlbare Mauer der Stufe 0 des Raums noch kostet (Go `wallGold`).
fn wall_gold(r: &Room) -> i64 {
    let w = &r.isl.stages[0];
    let v = vals(w);
    for s in &w.sites {
        let (gold, ok) = site_gold_left(w, &v, None, s);
        if ok && s.kind == "wall" && s.state == "unpaid" {
            return gold;
        }
    }
    panic!("keine bezahlbare Mauer in Stufe 0");
}

/// Ein Manager im Dev-Mode mit den Test-Welten (nordmeer, nordteuer, wertkaputt) als Ordner (Go `valuesFixture`).
fn values_fixture(tag: &str) -> (TestManager, WorldDir) {
    let (mut m, dir) = world_fixture(tag);
    m.worlds = Some(dir.src.clone());
    (m, dir)
}

fn create(m: &mut TestManager, id: &str, conn: u64, name: &str, fresh: bool, world: &str) -> String {
    let (p, _inbox) = peer(conn);
    m.create(id, p, name, fresh, 0, &[0], opts(world)).expect("create")
}

#[test]
fn wertesatz_zwei_raeume_eigene_kosten() {
    let (mut f, _dir) = values_fixture("kosten");
    let std = create(&mut f, "xbox", 1, "std", true, "nordmeer");
    let teuer = create(&mut f, "pc", 2, "teuer", true, "nordteuer");
    let base = wall_gold(rm(&mut f, &std));
    let got = wall_gold(rm(&mut f, &teuer));
    assert!(
        got == 9 && base != 9,
        "Mauer: nordmeer {base} Gold, nordteuer {got} Gold, erwartet Basis und 9"
    );
    {
        let (rooms, mut env) = f.split();
        rooms.get_mut(&teuer).expect("Raum").save_sync(&mut env);
    }
    let saved = f.store.data.get("teuer").cloned().expect("Spielstand");
    // frischer Manager: lädt den Stand mit seiner Welt
    let (mut g, _dir2) = values_fixture("kosten-neu");
    g.store.data.insert("teuer".into(), saved);
    let (p, _inbox) = peer(3);
    let code = g
        .create("pc", p, "teuer", false, 0, &[0], Options::default())
        .expect("laden");
    let again = rm(&mut g, &code);
    let (world, gold) = (again.isl.world_id().to_string(), wall_gold(again));
    assert!(
        world == "nordteuer" && gold == 9 && wall_gold(rm(&mut f, &std)) == base,
        "geladen: Welt {world:?}, Mauer {gold} Gold; nordmeer {base}"
    );
}

#[test]
fn wertesatz_ungueltiger_override_abgelehnt() {
    let (mut f, _dir) = values_fixture("override");
    let (p, _inbox) = peer(1);
    let err = f
        .create("xbox", p, "kaputt", true, 0, &[0], opts("wertkaputt"))
        .expect_err("abgelehnt");
    assert!(
        err.to_string().contains("data/worlds/wertkaputt.json"),
        "Fehler {err}, erwartet Meldung mit data/worlds/wertkaputt.json"
    );
}
