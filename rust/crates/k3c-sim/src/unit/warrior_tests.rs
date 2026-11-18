//! Schwert, Krieger und Verlust (Go: `unit_warrior_test.go`, W4.3a, B-014, B-122; Sprint W4 AC-04, AC-06): Schwert an der
//! Werkstatt, Krieger hinter der äußersten gebauten Sperre (Q46), Nahkampf mit den Werten aus data/troops.json.

use super::warrior::*;
use crate::enemies::testutil::*;
use crate::state::{ItemDrop, Job, World, WorldValues};

fn values(w: &World) -> std::sync::Arc<WorldValues> {
    w.vals.clone().expect("Wertesatz")
}

/// Go: `lineSite` (Teil): eine gebaute Mauer links (`side` -1) oder rechts vom Hub in `n` mal 10 Units Abstand.
fn wall_at(w: &mut World, side: f64, n: f64) -> usize {
    let x = w.hub_x + side * 20.0 * n;
    built_site(w, "wall", x)
}

#[test]
fn is_fighter_nur_kaempfer_figuren() {
    let v = test_values();
    for kind in ["archer", "warrior"] {
        assert!(is_fighter(&v.troops, kind), "{kind}");
    }
    for kind in ["vagrant", "peasant", "miner", "unbekannt"] {
        assert!(!is_fighter(&v.troops, kind), "{kind}");
    }
}

#[test]
fn schwert_zahlziel_liegt_am_angebot_der_werkstatt() {
    let mut w = bare_world();
    let v = values(&w);
    let shop = built_site(&mut w, "workshop", 300.0);
    let bowyer = built_site(&mut w, "bowyer", 400.0);
    let dx = v.buildings["workshop"]
        .offers
        .iter()
        .find(|o| o.kind == "sword")
        .expect("Schwert")
        .dx;
    assert_eq!(sword_spot(&w, &v, shop), Some(300.0 + dx));
    assert_eq!(sword_spot(&w, &v, bowyer), None, "der Bogner hat kein Schwert-Zahlziel");
    let range = v.economy.pay_range_units;
    assert_eq!(sword_at(&w, &v, None, 300.0 + dx + range - 0.01, false), Some(shop));
    assert_eq!(sword_at(&w, &v, None, 300.0 + dx + range + 0.01, false), None);
    w.sites[shop].state = "waitingWorker".into();
    assert_eq!(
        sword_at(&w, &v, None, 300.0 + dx, false),
        None,
        "nur gebaute Werkstätten"
    );
}

#[test]
fn schwert_zahlen_bis_zum_preis_dann_voll_und_erstattung() {
    let mut w = bare_world();
    let v = values(&w);
    let shop = built_site(&mut w, "workshop", 300.0);
    w.players.push(crate::state::Player::default());
    let gold = v.troops["warrior"].cost.gold;
    let x = sword_spot(&w, &v, shop).expect("Schwert");
    for n in 1..=gold {
        assert!(sword_payable(&w, &v, None, shop), "vor Münze {n}");
        w.players[0].pay_key = Some("sword".into());
        w.players[0].pay_amount = n;
        pay_sword(&mut w, &v, None, 0, shop);
    }
    assert!(!sword_payable(&w, &v, None, shop), "voll bezahlt");
    assert_eq!(sword_at(&w, &v, None, x, true), None, "nimmt keine Münzen mehr");
    assert_eq!(sword_at(&w, &v, None, x, false), Some(shop));
    assert!(
        w.players[0].pay_key.is_none() && w.players[0].pay_amount == 0,
        "Zahlung beendet"
    );
    let id = w.sites[shop].id;
    assert_eq!(refund_sword(&mut w, id, gold + 5), gold);
    assert_eq!(refund_sword(&mut w, id, 1), 0);
    assert_eq!(refund_sword(&mut w, 99_999, 1), 0, "unbekannter Platz");
    // volles Regal nimmt auch nichts mehr
    w.sites[shop].swords = v.buildings["workshop"].sword_rack;
    assert!(!sword_payable(&w, &v, None, shop));
}

/// (b) Ein bezahltes Schwert liegt erst nach craftSeconds.sword im Regal.
#[test]
fn schwert_nach_herstellungszeit() {
    let mut w = bare_world();
    let v = values(&w);
    let shop = built_site(&mut w, "workshop", 300.0);
    w.stock.wood = 1000;
    w.sites[shop].sword_paid_gold = v.troops["warrior"].cost.gold;
    let mut secs = 0.0;
    while w.sites[shop].swords == 0 && secs < 60.0 {
        w.time += DT;
        step_sword_craft(&mut w, &v, None, shop);
        secs += DT;
    }
    let want = v.buildings["workshop"].craft_seconds.as_ref().expect("craftSeconds")["sword"];
    assert!(
        (secs - DT - want).abs() <= DT + 1e-9,
        "Schwert nach {:.2} s, erwartet {want:.2} s",
        secs - DT
    );
    let wood = v.troops["warrior"].cost.wood;
    assert_eq!(w.stock.wood, 1000 - wood, "Holz verbraucht");
}

#[test]
fn schwert_wartet_ohne_material() {
    let mut w = bare_world();
    let v = values(&w);
    let shop = built_site(&mut w, "workshop", 300.0);
    w.sites[shop].sword_paid_gold = v.troops["warrior"].cost.gold;
    step_sword_craft(&mut w, &v, None, shop);
    if v.troops["warrior"].cost.wood > 0 {
        assert_eq!(w.sites[shop].sword_done, 0.0, "kein Holz, keine Arbeit");
        assert_eq!(
            w.sites[shop].sword_paid_gold, v.troops["warrior"].cost.gold,
            "das Gold wartet"
        );
    }
}

/// Q54: Mit gebauter Linie 2 steht keiner an Linie 1; Krieger verteilen sich auf beide Seiten.
#[test]
fn warrior_post_aeusserste_linie_und_beide_seiten() {
    let mut w = bare_world();
    let v = values(&w);
    let units = v.troops["warrior"].post_units;
    assert_eq!(
        warrior_post(&w, &v, -1.0),
        (w.hub_x - 10.0, w.hub_x - 10.0),
        "ohne Mauer"
    );
    let inner = wall_at(&mut w, -1.0, 1.0);
    let outer = wall_at(&mut w, -1.0, 2.0);
    let right = wall_at(&mut w, 1.0, 1.0);
    let (ox, ix, rx) = (w.sites[outer].x, w.sites[inner].x, w.sites[right].x);
    assert_eq!(warrior_post(&w, &v, -1.0), (ox + units, ox), "linker Posten an Linie 2");
    assert_ne!(ox, ix);
    assert_eq!(
        warrior_post(&w, &v, 1.0),
        (rx - units, rx),
        "rechter Posten hinter der rechten Mauer"
    );
}

#[test]
fn equip_hammer_beruf_und_kaempfer() {
    let mut w = bare_world();
    let v = values(&w);
    let ti = add_peasant(&mut w, 200.0, "");
    equip(&mut w, &v, ti, "hammer", 0);
    assert!(w.troops[ti].hammer);
    equip(&mut w, &v, ti, "miner", 7);
    assert_eq!((w.troops[ti].profession.as_str(), w.troops[ti].work_site), ("miner", 7));
    assert_eq!(w.troops[ti].kind, "peasant");
}

#[test]
fn fetching_zaehlt_andere_bauern_mit_gleichem_auftrag() {
    let mut w = bare_world();
    let (a, b, c) = (
        add_peasant(&mut w, 0.0, ""),
        add_peasant(&mut w, 0.0, ""),
        add_peasant(&mut w, 0.0, ""),
    );
    w.troops[b].job = Some(Job {
        r#type: "fetchBow".into(),
        site_id: 5,
        ..Job::default()
    });
    w.troops[c].job = Some(Job {
        r#type: "pickup".into(),
        drop_id: 5,
        ..Job::default()
    });
    assert_eq!(fetching(&w, a, "fetchBow", 5), 1);
    assert_eq!(fetching(&w, b, "fetchBow", 5), 0, "sich selbst nicht");
    assert_eq!(fetching(&w, a, "pickup", 5), 1);
    assert_eq!(fetching(&w, a, "fetchSword", 5), 0);
    assert_eq!(fetching(&w, a, "fetchBow", 6), 0);
}

#[test]
fn drop_by_id_fetches_fighter_weapon_und_shelf_stocked() {
    let mut w = bare_world();
    let v = values(&w);
    assert!(!shelf_stocked(&w));
    let shop = built_site(&mut w, "workshop", 300.0);
    assert!(!shelf_stocked(&w), "leeres Regal");
    w.sites[shop].hammers = 1;
    assert!(shelf_stocked(&w));
    w.sites[shop].state = "waitingWorker".into();
    assert!(!shelf_stocked(&w), "nicht gebaut");
    let id = w.new_id();
    w.drops.push(ItemDrop {
        id,
        kind: "warrior".into(),
        x: 10.0,
        work_site: 0,
    });
    let id2 = w.new_id();
    w.drops.push(ItemDrop {
        id: id2,
        kind: "miner".into(),
        x: 20.0,
        work_site: 0,
    });
    assert_eq!(drop_by_id(&w, id2), Some(1));
    assert_eq!(drop_by_id(&w, 0), None);
    let pick = |drop_id| Job {
        r#type: "pickup".into(),
        drop_id,
        ..Job::default()
    };
    assert!(fetches_fighter_weapon(&w, &v, &pick(id)));
    assert!(
        !fetches_fighter_weapon(&w, &v, &pick(id2)),
        "Beruf ist keine Kämpfer-Waffe"
    );
    assert!(!fetches_fighter_weapon(&w, &v, &pick(0)));
    for (typ, want) in [("fetchBow", true), ("fetchSword", true), ("fetchHammer", false)] {
        let j = Job {
            r#type: typ.into(),
            ..Job::default()
        };
        assert_eq!(fetches_fighter_weapon(&w, &v, &j), want, "{typ}");
    }
}

/// Der Bauer läuft zur Ausrüstung am Boden, nimmt sie und bekommt den Beruf zurück.
#[test]
fn fetch_weapon_holt_ausruestung_vom_boden() {
    let mut w = bare_world();
    let v = values(&w);
    let ti = add_peasant(&mut w, 100.0, "");
    let id = w.new_id();
    w.drops.push(ItemDrop {
        id,
        kind: "miner".into(),
        x: 103.0,
        work_site: 0,
    });
    w.troops[ti].job = Some(Job {
        r#type: "pickup".into(),
        drop_id: id,
        ..Job::default()
    });
    for _ in 0..300 {
        if w.troops[ti].job.is_none() {
            break;
        }
        fetch_weapon(&mut w, &v, ti, DT);
    }
    assert!(w.troops[ti].job.is_none() && w.drops.is_empty());
    assert_eq!(w.troops[ti].profession, "miner");
}

#[test]
fn fetch_weapon_holt_hammer_aus_dem_regal() {
    let mut w = bare_world();
    let v = values(&w);
    let shop = built_site(&mut w, "workshop", 104.0);
    w.sites[shop].hammers = 1;
    let (ti, sid) = (add_peasant(&mut w, 100.0, ""), w.sites[shop].id);
    w.troops[ti].job = Some(Job {
        r#type: "fetchHammer".into(),
        site_id: sid,
        ..Job::default()
    });
    for _ in 0..300 {
        if w.troops[ti].job.is_none() {
            break;
        }
        fetch_weapon(&mut w, &v, ti, DT);
    }
    assert!(w.troops[ti].hammer && w.sites[shop].hammers == 0 && w.troops[ti].job.is_none());
}

#[test]
fn fetch_weapon_gibt_auf_wenn_das_ziel_fehlt_oder_ein_gegner_naht() {
    let mut w = bare_world();
    let v = values(&w);
    let ti = add_peasant(&mut w, 100.0, "");
    w.troops[ti].job = Some(Job {
        r#type: "pickup".into(),
        drop_id: 42,
        ..Job::default()
    });
    fetch_weapon(&mut w, &v, ti, DT);
    assert!(w.troops[ti].job.is_none(), "der Drop ist weg");
    let shop = built_site(&mut w, "workshop", 130.0);
    w.sites[shop].hammers = 1;
    w.troops[ti].job = Some(Job {
        r#type: "fetchHammer".into(),
        site_id: w.sites[shop].id,
        ..Job::default()
    });
    spawn(&mut w, "goblin", 100.0 + v.troops["peasant"].fetch_enemy_distance - 0.5);
    fetch_weapon(&mut w, &v, ti, DT);
    assert!(
        w.troops[ti].job.is_none() && w.troops[ti].x == 100.0,
        "Gegner in Hörweite: nicht holen"
    );
}

/// Zwei Krieger verteilen sich auf beide Seiten und stehen hinter der äußersten Mauer; der Gegner an der Mauer nimmt
/// den Schaden aus den Daten (Go: `Step` mit Kampfvariante und Begleitschutz, `escortPost`/`squadPost` sind RU15).
#[test]
#[ignore = "wartet auf RU15"]
fn krieger_steht_hinter_der_aeussersten_mauer_und_schlaegt() {
    let mut w = bare_world();
    let v = values(&w);
    let wall = wall_at(&mut w, -1.0, 2.0);
    let hub = w.hub_x;
    let ti = add_peasant(&mut w, hub, "");
    crate::unit::jobs::make_fighter(&mut w, ti, "warrior");
    for _ in 0..2400 {
        super::warrior::step_warrior(&mut w, &v, None, ti, DT);
    }
    let want = w.sites[wall].x + v.troops["warrior"].post_units;
    assert!(
        (w.troops[ti].x - want).abs() <= super::ARRIVE,
        "Krieger bei {}, erwartet {want}",
        w.troops[ti].x
    );
    let wx = w.sites[wall].x;
    let e = spawn(&mut w, "skeleton", wx - 8.0);
    let hp = w.enemies[e].hp;
    for _ in 0..600 {
        super::warrior::step_warrior(&mut w, &v, None, ti, DT);
        if w.enemies[e].hp < hp {
            break;
        }
    }
    assert_eq!(hp - w.enemies[e].hp, v.troops["warrior"].damage);
}

/// Der Bauer nimmt das Schwert aus dem Regal und wird Krieger (Go: `dayWorld`, `runUntil`).
#[test]
#[ignore = "wartet auf RU18.5 (step_passives)"]
fn warrior_nimmt_schwert_und_kaempft_hinter_der_mauer() {
    let mut w = day_world();
    let workshop = build_site(&mut w, "workshop");
    w.sites[workshop].swords = 1;
    run(&mut w, 30.0);
}

/// (e) Zwei Spieler zahlen gleichzeitig: einer am Bogen (Bogner), einer am Schwert-Zahlziel der Werkstatt.
#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn zwei_spieler_zahlen_bogen_und_schwert() {
    let mut w = quiet_world();
    let ps = [add_player(&mut w), add_player(&mut w)];
    let (s, shop) = (build_site(&mut w, "bowyer"), build_site(&mut w, "workshop"));
    let v = crate::world::vals(&w);
    let spot = sword_spot(&w, &v, shop).expect("Werkstatt ohne Schwert-Zahlziel");
    let (g1, g2) = (w.players[ps[0]].gold, w.players[ps[1]].gold);
    let cmds = [
        crate::state::PlayerCommand {
            pay: true,
            ..Default::default()
        },
        crate::state::PlayerCommand {
            pay: true,
            ..Default::default()
        },
    ];
    let mut tm = 0.0;
    while tm < 15.0 {
        w.players[ps[0]].x = w.sites[s].x;
        w.players[ps[1]].x = spot;
        step(&mut w, Some(&cmds), DT);
        tm += DT;
    }
    let (bow, sword) = (v.troops["archer"].cost.gold, v.troops["warrior"].cost.gold);
    let (paid_bow, paid_sword) = (w.sites[s].bow_paid_gold, w.sites[shop].sword_paid_gold);
    let (d1, d2) = (g1 - w.players[ps[0]].gold, g2 - w.players[ps[1]].gold);
    assert!(
        paid_bow == bow && paid_sword == sword && d1 == bow && d2 == sword,
        "Bogen {paid_bow}/{bow} (Spieler 1 zahlte {d1}), Schwert {paid_sword}/{sword} (Spieler 2 zahlte {d2})"
    );
}
