//! FG1.2 (B-1200): Die fünf Kombinationen der nativen Regel. Der Bürger holt Bogen (Bogenschütze), Schwert (Soldat) oder
//! Werkzeuggürtel (Handwerker) selbst aus dem Regal; Soldat oder Bogenschütze plus Rüstung ergibt Elite; der Handwerker
//! nimmt nichts mehr; der Hub-HP-Bonus der Rüstkammer entfällt. Ohne native Regeln gilt das alte Verhalten (Go-Parität).

use crate::building::upgrades::{armor_bonus, armor_stage};
use crate::enemies::testutil::*;
use crate::state::World;
use crate::world::vals;

/// Ein Bürger im Dienst neben einem Regal der Art `kind` mit einem Stück `item` (`bows`, `swords`, `hammers`).
fn shelf_world(kind: &str, item: &str) -> (World, usize, usize) {
    let mut w = day_world();
    add_player(&mut w);
    let s = build_site(&mut w, kind);
    match item {
        "bows" => w.sites[s].bows = 1,
        "swords" => w.sites[s].swords = 1,
        _ => w.sites[s].hammers = 1,
    }
    let x = w.sites[s].x;
    let ti = add_peasant(&mut w, x + 2.0, "");
    (w, s, ti)
}

/// Ein Kämpfer der Art `kind` bei `x` (wie `fighter_at` der Gebäude-Tests).
fn fighter(w: &mut World, kind: &str, x: f64) -> usize {
    let ti = crate::unit::spawn_vagrant(w, x, x);
    crate::unit::jobs::make_fighter(w, ti, kind);
    let anchor = w.hub_x + (x - w.hub_x).signum();
    w.troops[ti].x = x;
    w.troops[ti].anchor_x = anchor;
    ti
}

fn run_for(w: &mut World, secs: f64, done: impl Fn(&World) -> bool) -> bool {
    run_until(w, secs, &done) < secs
}

#[test]
fn buerger_plus_bogen_wird_bogenschuetze() {
    let (mut w, _, ti) = shelf_world("bowyer", "bows");
    assert!(
        run_for(&mut w, 30.0, |w| w.troops[ti].kind == "archer"),
        "kein Bogenschütze"
    );
}

#[test]
fn buerger_plus_schwert_wird_soldat() {
    let (mut w, _, ti) = shelf_world("smithy", "swords");
    assert!(run_for(&mut w, 30.0, |w| w.troops[ti].kind == "warrior"), "kein Soldat");
}

#[test]
fn buerger_plus_guertel_wird_handwerker() {
    let (mut w, _, ti) = shelf_world("workshop", "hammers");
    assert!(run_for(&mut w, 30.0, |w| w.troops[ti].hammer), "kein Handwerker");
    assert!(w.troops[ti].serving());
}

#[test]
fn soldat_plus_ruestung_wird_elite_soldat() {
    let mut w = day_world();
    let s = build_site(&mut w, "armory");
    w.sites[s].armors = 1;
    let x = w.sites[s].x;
    let ti = fighter(&mut w, "warrior", x + 3.0);
    assert!(
        run_for(&mut w, 5.0, |w| w.troops[ti].kind == "eliteWarrior"),
        "kein Elite-Soldat"
    );
    assert_eq!(w.sites[s].armors, 0, "Rüstung verbraucht");
}

#[test]
fn bogenschuetze_plus_ruestung_wird_elite_bogenschuetze() {
    let mut w = day_world();
    let s = build_site(&mut w, "armory");
    w.sites[s].armors = 1;
    let x = w.sites[s].x;
    let ti = fighter(&mut w, "archer", x + 3.0);
    assert!(
        run_for(&mut w, 5.0, |w| w.troops[ti].kind == "eliteArcher"),
        "kein Elite-Bogenschütze"
    );
}

/// Mit zwei Spielern: Der Handwerker nimmt weder Bogen noch Schwert, ein Bürger mit Beruf (Bergmann) auch nicht.
#[test]
fn handwerker_und_berufler_nehmen_nichts() {
    let (mut w, s, ti) = shelf_world("bowyer", "bows");
    add_player(&mut w);
    w.sites[s].swords = 1;
    w.troops[ti].hammer = true;
    let mx = w.sites[s].x + 3.0;
    let miner = add_peasant(&mut w, mx, "miner");
    run_for(&mut w, 20.0, |_| false);
    assert!(
        w.troops[ti].kind == "citizen" && w.troops[ti].hammer,
        "Handwerker blieb Handwerker"
    );
    assert_eq!(w.troops[miner].kind, "citizen", "Bergmann bleibt Bürger");
    assert_eq!(w.sites[s].bows, 1, "Bogen blieb liegen");
}

/// Ohne Kandidat bleibt die Rüstung im Regal; der Hub-HP-Bonus der Rüstkammer entfällt.
#[test]
fn ruestung_im_regal_und_kein_hub_hp_bonus() {
    let mut w = day_world();
    let s = build_site(&mut w, "armory");
    w.sites[s].armors = 1;
    run_for(&mut w, 2.0, |_| false);
    assert_eq!(w.sites[s].armors, 1, "ohne Kämpfer bleibt sie im Regal");
    w.armor_level = 2;
    let v = vals(&w);
    assert!(
        armor_bonus(&w, &v) == 0.0 && armor_stage(&w, &v, None).is_none(),
        "kein Hub-HP-Bonus"
    );
    let mut legacy = v.as_ref().clone();
    legacy.native_rules = false;
    assert!(
        armor_bonus(&w, &legacy) > 0.0,
        "Go-Parität: der alte Bonus bleibt ohne native Regeln"
    );
}

/// Herstellung: Der Waffenschmied macht aus bezahltem Gold und Material ein Schwert, der Rüstungsschmied eine Rüstung.
#[test]
fn meister_stellen_schwert_und_ruestung_her() {
    let mut w = day_world();
    add_player(&mut w);
    let (smithy, armory) = (build_site(&mut w, "smithy"), build_site(&mut w, "armory"));
    let v = vals(&w);
    w.stock.wood = 500;
    w.stock.stone = 500;
    w.stock.iron = 500;
    w.stock.copper = 500;
    w.sites[smithy].sword_paid_gold = v.troops["warrior"].cost.gold;
    w.sites[armory].bow_paid_gold = crate::unit::hammer::armor_cost().gold;
    assert!(
        run_for(&mut w, 60.0, |w| w.sites[smithy].swords == 1
            && w.sites[armory].armors == 1),
        "{:?}",
        w.sites[armory]
    );
}

/// Ein Rüstungsschmied mit einer Rüstung im Regal; liefert Welt und Platz-Index.
fn armory_world() -> (World, usize) {
    let mut w = day_world();
    add_player(&mut w);
    let s = build_site(&mut w, "armory");
    w.sites[s].armors = 1;
    (w, s)
}

/// EQ1.1 AC-01: Die Rüstung wird abgeholt. Ein Soldat weit vom Gebäude bleibt zunächst Soldat und ist unterwegs; erst am
/// Gebäude wird er Elite, die Rüstung verlässt das Regal.
#[test]
fn ruestung_wird_am_gebaeude_abgeholt() {
    let (mut w, s) = armory_world();
    let sx = w.sites[s].x;
    let ti = fighter(&mut w, "warrior", sx + 30.0);
    run(&mut w, 0.2);
    assert_eq!(w.troops[ti].kind, "warrior", "weit weg: noch kein Elite");
    assert!(
        w.troops[ti].job.as_ref().is_some_and(|j| j.r#type == "fetchArmor"),
        "Auftrag fetchArmor: {:?}",
        w.troops[ti].job
    );
    assert!(
        run_for(&mut w, 60.0, |w| w.troops[ti].kind == "eliteWarrior"),
        "kommt am Gebäude an"
    );
    assert_eq!(w.sites[s].armors, 0, "Rüstung aus dem Regal");
    assert!(w.troops[ti].job.is_none());
}

/// EQ1.1 AC-02: Bei Gefahr (Gegner der Welle auf der Karte) holt niemand die Rüstung; sie bleibt im Regal.
#[test]
fn bei_gefahr_holt_niemand_die_ruestung() {
    let (mut w, s) = armory_world();
    let sx = w.sites[s].x;
    let far = w.hub_x + 40.0;
    spawn(&mut w, "goblin", far);
    let ti = fighter(&mut w, "warrior", sx + 1.0);
    run(&mut w, 5.0);
    assert_eq!(w.troops[ti].kind, "warrior", "Gefahr: kein Elite");
    assert_eq!(w.sites[s].armors, 1, "Rüstung bleibt im Regal");
    assert!(w.troops[ti].job.is_none());
}

/// EQ1.1 AC-03: Je Rüstung nur ein Kämpfer unterwegs; ohne Kandidat bleibt sie im Regal.
#[test]
fn eine_ruestung_schickt_einen_kaempfer() {
    let (mut w, s) = armory_world();
    let sx = w.sites[s].x;
    let a = fighter(&mut w, "warrior", sx + 20.0);
    let b = fighter(&mut w, "warrior", sx + 25.0);
    run(&mut w, 0.2);
    let walking = [a, b]
        .iter()
        .filter(|&&i| w.troops[i].job.as_ref().is_some_and(|j| j.r#type == "fetchArmor"))
        .count();
    assert_eq!(walking, 1, "nur einer unterwegs");
    assert!(run_for(&mut w, 60.0, |w| w.sites[s].armors == 0));
    let elites = w.troops.iter().filter(|t| t.kind == "eliteWarrior").count();
    assert_eq!(elites, 1, "nur eine Elite");
}
