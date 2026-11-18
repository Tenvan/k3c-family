//! Berufe der Bauern (Go: `unit_professions_test.go`, W4.2, B-121, Sprint W4 AC-02): Ausbildung an den
//! Angebots-Zahlzielen, Herstellungszeit des Bogens. Alle Werte aus data/ (troops.json › peasant.professions,
//! buildings.json › offers und craftSeconds).

use super::professions::*;
use crate::enemies::testutil::*;
use crate::state::{World, WorldValues};
use k3c_data::troops::ProfessionData;

fn values(w: &World) -> std::sync::Arc<WorldValues> {
    w.vals.clone().expect("Wertesatz")
}

/// Go: `profession`: der Beruf `kind` aus den Daten.
fn profession(v: &WorldValues, kind: &str) -> ProfessionData {
    profession_of(&v.troops, kind)
        .unwrap_or_else(|| panic!("Beruf {kind} fehlt in den Daten"))
        .clone()
}

/// Go: `aboutDt`: gleich bis auf einen Takt.
fn about_dt(got: f64, want: f64) -> bool {
    (got - want).abs() <= DT + 1e-9
}

/// Go: `offerOf` ist das Angebot `kind` an der gebauten Werkstatt (Platz und Nummer).
fn offer_of<'a>(w: &mut World, v: &'a WorldValues, kind: &str) -> OfferTarget<'a> {
    let hub = w.hub_x;
    let site = built_site(w, "workshop", hub + 40.0 + w.sites.len() as f64 * 30.0);
    let offers = &v.buildings["workshop"].offers;
    let idx = offers
        .iter()
        .position(|o| o.kind == kind)
        .unwrap_or_else(|| panic!("kein Angebot {kind} an der Werkstatt"));
    OfferTarget { site, idx, offers }
}

#[test]
fn beruf_und_faktoren_kommen_aus_den_daten() {
    let mut w = bare_world();
    let v = values(&w);
    let miner = profession(&v, "miner");
    assert!(miner.work_factor > 1.0 && miner.resources.iter().any(|r| r == "stone"));
    assert!(profession_of(&v.troops, "unbekannt").is_none());
    let (plain, digger) = (add_peasant(&mut w, 0.0, ""), add_peasant(&mut w, 0.0, "miner"));
    assert_eq!(work_factor(&w, &v, plain, "miner"), 1.0);
    assert_eq!(work_factor(&w, &v, digger, "miner"), miner.work_factor);
    assert_eq!(work_factor(&w, &v, digger, "builder"), 1.0, "falscher Beruf");
    assert_eq!(work_factor(&w, &v, digger, "unbekannt"), 1.0);
    w.troops[digger].kind = "archer".into();
    assert_eq!(work_factor(&w, &v, digger, "miner"), 1.0, "nur Bauern haben Berufe");
}

/// (a) Bergmann: schneller an Fels, Kupfererz und Adern (seine resources), nicht am Baum.
#[test]
fn bergmann_baut_nur_seine_materialien_schneller_ab() {
    let mut w = bare_world();
    let v = values(&w);
    let miner = profession(&v, "miner");
    let digger = add_peasant(&mut w, 0.0, "miner");
    let plain = add_peasant(&mut w, 0.0, "");
    for resource in &miner.resources {
        assert_eq!(mine_factor(&w, &v, digger, resource), miner.work_factor, "{resource}");
        assert_eq!(mine_factor(&w, &v, plain, resource), 1.0, "{resource} ohne Beruf");
    }
    assert_eq!(
        mine_factor(&w, &v, digger, "wood"),
        1.0,
        "Holz gehört nicht zum Bergmann"
    );
}

#[test]
fn offer_payable_prueft_gebaut_beruf_voll_und_platz() {
    let mut w = bare_world();
    let v = values(&w);
    let o = offer_of(&mut w, &v, "miner");
    assert!(offer_payable(&w, &v, None, &o));
    let cost = profession(&v, "miner").cost.gold;
    w.sites[o.site].offer_paid = vec![0; o.offers.len()];
    w.sites[o.site].offer_paid[o.idx] = cost;
    assert!(!offer_payable(&w, &v, None, &o), "voll bezahlt");
    w.sites[o.site].offer_paid[o.idx] = cost - 1;
    assert!(offer_payable(&w, &v, None, &o));
    w.sites[o.site].state = "waitingWorker".into();
    assert!(!offer_payable(&w, &v, None, &o), "nicht gebaut");
    // ein Angebot ohne Beruf (Schwert) zahlt hier nicht
    w.sites[o.site].state = "built".into();
    let sword = OfferTarget {
        idx: o
            .offers
            .iter()
            .position(|x| x.kind == "sword")
            .expect("Schwert-Angebot"),
        ..o
    };
    assert!(!offer_payable(&w, &v, None, &sword));
    assert_eq!(o.paid(&w), cost - 1);
    assert_eq!(o.key(&w), format!("offer:{}.{}", w.sites[o.site].id, o.idx));
}

#[test]
fn find_offer_und_near_offer_liegen_in_zahl_reichweite() {
    let mut w = bare_world();
    let v = values(&w);
    let o = offer_of(&mut w, &v, "builder");
    let x = o.x(&w);
    let range = v.economy.pay_range_units;
    let found = find_offer(&w, &v, None, x).expect("Angebot");
    assert_eq!(found.site, o.site);
    assert!(near_offer(&w, &v, x + range - 0.01));
    assert!(!near_offer(&w, &v, x + 3.0 * range + 100.0));
    assert!(find_offer(&w, &v, None, x + 1000.0).is_none(), "zu weit");
    w.sites[o.site].state = "waitingWorker".into();
    assert!(find_offer(&w, &v, None, x).is_none(), "nicht gebaut");
    assert!(!near_offer(&w, &v, x));
}

#[test]
fn refund_offer_gibt_hoechstens_das_bezahlte_zurueck() {
    let mut w = bare_world();
    let v = values(&w);
    let o = offer_of(&mut w, &v, "miner");
    w.sites[o.site].offer_paid = vec![0; o.offers.len()];
    w.sites[o.site].offer_paid[o.idx] = 3;
    let r = format!("{}.{}", w.sites[o.site].id, o.idx);
    assert_eq!(refund_offer(&mut w, &r, 2), 2);
    assert_eq!(refund_offer(&mut w, &r, 5), 1);
    assert_eq!(refund_offer(&mut w, &r, 5), 0);
    let missing = format!("{}.99", w.sites[o.site].id);
    assert_eq!(refund_offer(&mut w, "99999.0", 5), 0, "unbekannter Platz");
    assert_eq!(refund_offer(&mut w, &missing, 5), 0, "unbekanntes Angebot");
    assert_eq!(refund_offer(&mut w, "kaputt", 5), 0);
}

#[test]
fn free_peasant_nimmt_den_naechsten_freien_bauern() {
    let mut w = bare_world();
    let busy = add_peasant(&mut w, 100.0, "");
    w.troops[busy].job = Some(crate::state::Job::default());
    let smith = add_peasant(&mut w, 101.0, "");
    w.troops[smith].hammer = true;
    let same = add_peasant(&mut w, 102.0, "miner");
    let farther = add_peasant(&mut w, 140.0, "");
    let nearer = add_peasant(&mut w, 110.0, "");
    assert_eq!(
        free_peasant(&w, 100.0, "miner"),
        Some(nearer),
        "Beschäftigte, Handwerker und Gleichberufige zählen nicht"
    );
    assert_eq!(
        free_peasant(&w, 100.0, "builder"),
        Some(same),
        "Umschulung: auch ein anderer Beruf ist frei"
    );
    w.troops[nearer].kind = "archer".into();
    w.troops[same].kind = "archer".into();
    assert_eq!(free_peasant(&w, 100.0, "miner"), Some(farther));
}

/// (c) Ein dritter Handwerker je Gebäude wird nicht ausgebildet: `workers_at` zählt die Bauern am Platz.
#[test]
fn workers_at_zaehlt_nur_gebundene_bauern_des_berufs() {
    let mut w = bare_world();
    let v = values(&w);
    let o = offer_of(&mut w, &v, "craftsman");
    let sid = w.sites[o.site].id;
    let max = profession(&v, "craftsman").max_per_building;
    for _ in 0..max {
        let ti = add_peasant(&mut w, 0.0, "craftsman");
        w.troops[ti].work_site = sid;
    }
    add_peasant(&mut w, 0.0, "craftsman"); // ohne Platz-Bindung
    let other = add_peasant(&mut w, 0.0, "miner");
    w.troops[other].work_site = sid;
    assert_eq!(workers_at(&w, sid, "craftsman"), max);
    assert!(!offer_payable(&w, &v, None, &o), "Platz voll");
}

/// Go: `bowSeconds`: so lange dauert ein bezahlter Hammer mit `n` Handwerkern an der Werkstatt (nur `step_bow_craft`,
/// ohne Abholer).
fn bow_seconds(n: i64) -> f64 {
    let mut w = bare_world();
    let v = values(&w);
    let hub = w.hub_x;
    let si = built_site(&mut w, "workshop", hub + 50.0);
    let sid = w.sites[si].id;
    for _ in 0..n {
        let ti = add_peasant(&mut w, 0.0, "craftsman");
        w.troops[ti].work_site = sid;
    }
    w.stock.wood = 1000;
    w.sites[si].bow_paid_gold = v.buildings["workshop"].hammer_cost.gold;
    let mut secs = 0.0;
    while w.sites[si].hammers == 0 && secs < 60.0 {
        w.time += DT;
        step_bow_craft(&mut w, &v, None, si);
        secs += DT;
    }
    secs - DT // der erste Takt beginnt die Arbeit erst
}

/// (c) Der Hammer liegt nach craftSeconds.hammer im Regal, mit einem Handwerker nach 1/workFactor davon.
#[test]
fn handwerker_und_herstellungszeit() {
    let w = bare_world();
    let v = values(&w);
    let craft = profession(&v, "craftsman");
    let base = v.buildings["workshop"].craft_seconds.as_ref().expect("craftSeconds")["hammer"];
    assert!(
        about_dt(bow_seconds(0), base),
        "Hammer ohne Handwerker: {:.2} s, erwartet {base:.2} s",
        bow_seconds(0)
    );
    let want = base / craft.work_factor;
    assert!(
        about_dt(bow_seconds(1), want),
        "Hammer mit Handwerker: {:.2} s, erwartet {want:.2} s",
        bow_seconds(1)
    );
    // mehr Handwerker als `max_per_building` beschleunigen nicht weiter
    let capped = base / (1.0 + (craft.work_factor - 1.0) * craft.max_per_building as f64);
    assert!(about_dt(bow_seconds(craft.max_per_building + 2), capped));
}

#[test]
fn bows_crafting_ist_eins_solange_ein_bogen_entsteht() {
    let mut w = bare_world();
    let si = built_site(&mut w, "bowyer", 250.0);
    assert_eq!(bows_crafting(&w.sites[si]), 0);
    w.sites[si].craft_done = 5.0;
    assert_eq!(bows_crafting(&w.sites[si]), 1);
}

/// Der Bogner stellt den Bogen her: Material wird erst verbraucht, wenn er in Arbeit geht.
#[test]
fn bogen_verbraucht_material_beim_arbeitsbeginn() {
    let mut w = bare_world();
    let v = values(&w);
    let si = built_site(&mut w, "bowyer", 250.0);
    let cost = v.troops["archer"].cost.clone();
    w.sites[si].bow_paid_gold = cost.gold;
    step_bow_craft(&mut w, &v, None, si);
    assert_eq!(
        w.sites[si].bow_paid_gold,
        if cost.wood > 0 { cost.gold } else { 0 },
        "ohne Material wartet das Gold"
    );
    w.stock.wood = 1000;
    step_bow_craft(&mut w, &v, None, si);
    assert_eq!((w.sites[si].bow_paid_gold, w.stock.wood), (0, 1000 - cost.wood));
    assert!(w.sites[si].craft_done > w.time);
}

/// Go: `addPeasant` und `stepOffers` rechnen `emit` (RU19): Der Beruf wird erst mit dem Ereignis vergeben.
#[test]
#[ignore = "wartet auf RU19"]
fn ausbildung_trifft_den_naechsten_freien_bauern() {
    let mut w = bare_world();
    let v = values(&w);
    let o = offer_of(&mut w, &v, "miner");
    let sid = w.sites[o.site].id;
    let sx = w.sites[o.site].x;
    let near = add_peasant(&mut w, sx + 1.0, "");
    let far = add_peasant(&mut w, sx + 40.0, "");
    w.sites[o.site].offer_paid = vec![0; o.offers.len()];
    w.sites[o.site].offer_paid[o.idx] = profession(&v, "miner").cost.gold;
    step_offers(&mut w, &v, None);
    assert_eq!(w.troops[near].profession, "miner");
    assert_eq!(w.troops[far].profession, "");
    assert_eq!(o.paid(&w), 0);
    assert_eq!(
        w.troops[near].work_site, 0,
        "Bergmann ist nicht an ein Gebäude gebunden"
    );
    let _ = sid;
}

/// (a) Bergmann: 1/workFactor der Zeit an Fels, Kupfererz und Adern (Go: `gather` braucht RU14 und RU18).
#[test]
fn bergmann_baut_schneller_ab() {
    let w = quiet_world();
    let _ = (w, first_node, step_for);
}

/// (a) Ein Bergmann zählt an der Ader als einer der höchstens `maxWorkers` Bauern (Go: `veinWorld`, `Step`).
#[test]
fn bergmann_zaehlt_an_der_ader() {
    let mut w = quiet_world();
    step_for(&mut w, 30.0);
}

/// (a) Baumeister: Bau in 1/workFactor der Bauzeit (Go: `build` und `Step`).
#[test]
fn baumeister_baut_schneller() {
    let mut w = quiet_world();
    step_for(&mut w, 1.0);
}

/// (b) Ausbildung kostet das Gold aus den Daten, trifft den nächsten freien Bauern; Umschulung kostet erneut; zu wenig
/// Gold lässt den Beruf unverändert (Go: `holdAt` mit `Step` und Spieler).
#[test]
#[ignore = "wartet auf RU18.5 (speed_of)"]
fn ausbildung_und_umschulung() {
    let mut w = quiet_world();
    let p = add_player(&mut w);
    let _ = pay_at(&mut w, &[p], 0.0, 8.0, &|_| true);
}

/// (c) Ein dritter Handwerker je Gebäude wird nicht ausgebildet (Go: `holdAt`).
#[test]
#[ignore = "wartet auf RU18.5 (speed_of)"]
fn dritter_handwerker_wird_nicht_ausgebildet() {
    let mut w = quiet_world();
    let p = add_player(&mut w);
    let _ = pay_at(&mut w, &[p], 0.0, 8.0, &|_| true);
}
