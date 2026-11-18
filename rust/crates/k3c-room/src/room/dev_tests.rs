//! Tests aus `dev_test.go` (B-178, Review TR4.2): Dev-Aktionen des Raums. Ein Dev-Raum ist ein Raum im Dev-Mode mit zwei
//! Slots eines Geräts `a` (Verbindung 1). Abweichung: Gos `sim.EconomyOf(...).StockMax` liest der Test nicht, er prüft das
//! gefüllte Lager gegen `>= 200` und die Gleichheit der fünf Rohstoffe.
use k3c_sim::state::Player;

use super::testutil::{Inbox, TestManager, create_room, dev_manager, manager, peer, rm, ticks};
use super::{DEV_MAX_AMOUNT, DevAction, Options, Room};
use crate::fehler::Fehler;

/// Die Aktion `name` (die übrigen Felder leer).
pub(super) fn act(name: &str) -> DevAction {
    DevAction {
        action: name.into(),
        ..DevAction::default()
    }
}

/// Aktion mit `slot` und `amount`.
pub(super) fn act_slot(name: &str, slot: i64, amount: i64) -> DevAction {
    DevAction {
        slot: Some(slot),
        amount,
        ..act(name)
    }
}

/// Führt `a` als Gerät `id` mit der Verbindung `conn` aus.
pub(super) fn dev_as(m: &mut TestManager, code: &str, id: &str, conn: u64, a: &DevAction) -> Result<(), Fehler> {
    m.with_room(code, |r, env| r.dev(env, id, conn, a)).expect("Raum")
}

/// `dev_as` für das Gerät `a` mit der Verbindung 1.
pub(super) fn dev(m: &mut TestManager, code: &str, a: &DevAction) -> Result<(), Fehler> {
    dev_as(m, code, "a", 1, a)
}

/// Ein Dev-Raum mit zwei Slots eines Geräts (Go `devRoom`).
pub(super) fn dev_room() -> (TestManager, String, Inbox) {
    let mut m = dev_manager();
    let (code, i) = create_room(&mut m, "a", 1, "dev", 0, &[0, 1]);
    (m, code, i)
}

/// Der Insel-Spieler mit Index `idx` (Go `islPlayer`).
pub(super) fn isl_player(r: &Room, idx: i64) -> &Player {
    r.isl
        .stages
        .iter()
        .flat_map(|w| w.players.iter())
        .find(|p| p.index == idx)
        .expect("Spieler")
}

fn isl_player_mut(r: &mut Room, idx: i64) -> &mut Player {
    r.isl
        .stages
        .iter_mut()
        .flat_map(|w| w.players.iter_mut())
        .find(|p| p.index == idx)
        .expect("Spieler")
}

fn slot_index(r: &Room, slot: i64) -> i64 {
    r.devices["a"].slots[&slot] as i64
}

fn log_lines(m: &TestManager, needle: &str) -> Vec<String> {
    m.log.lines.iter().filter(|l| l.contains(needle)).cloned().collect()
}

// B-178/AC-01: dev ohne Dev-Mode ist verboten (vor jeder Feldprüfung) und steht als Warnung im Log.
#[test]
fn dev_ohne_dev_modus_verboten() {
    let (mut m, _) = manager();
    let (p, _i) = peer(1);
    let code = m
        .create("geraet-1234567890", p, "dev", true, 0, &[0], Options::default())
        .expect("create");
    for name in ["gold", "material", "timescale", "gibtsnicht"] {
        let r = dev_as(&mut m, &code, "geraet-1234567890", 1, &act(name));
        assert_eq!(r, Err(Fehler::Forbidden), "{name}: erwartet forbidden");
    }
    let warns = log_lines(&m, "Dev-Aktion abgelehnt");
    assert_eq!(warns.len(), 4, "Warnungen:\n{warns:?}");
    assert!(
        warns[0].starts_with("Warn ") && warns[0].contains("device=geraet-1 ") && warns[0].contains("aktion=gold"),
        "Warnung unvollständig: {}",
        warns[0]
    );
}

// Mit Dev-Mode: fremde Verbindung und (noch) unbekannte Aktionen ergeben bad_request.
#[test]
fn dev_mit_dev_modus_prueft_verbindung() {
    let (mut m, code, _i) = dev_room();
    assert_eq!(
        dev_as(&mut m, &code, "a", 99, &act("gold")),
        Err(Fehler::BadRequest),
        "fremde Verbindung"
    );
    assert_eq!(
        dev_as(&mut m, &code, "b", 1, &act("gold")),
        Err(Fehler::BadRequest),
        "fremdes Gerät"
    );
    assert_eq!(
        dev(&mut m, &code, &act("gibtsnicht")),
        Err(Fehler::BadRequest),
        "unbekannte Aktion"
    );
}

// B-178/AC-02: gold fällt beim Monarchen des Slots (nicht beim anderen) und wird beim nächsten Tick aufgehoben.
#[test]
fn dev_gold() {
    let (mut m, code, _i) = dev_room();
    let r = rm(&mut m, &code);
    let (i0, i1) = (slot_index(r, 0), slot_index(r, 1));
    isl_player_mut(r, i0).gold = 0;
    isl_player_mut(r, i1).gold = 0;
    let (x0, x1) = (isl_player(r, i0).x, isl_player(r, i1).x);
    let stage = r.isl.stage_of(i0).expect("Stufe");
    let at = |r: &Room, x: f64| r.isl.stages[stage].coins.iter().filter(|c| c.x == x).count() as i64;
    let (before0, before1) = (at(r, x0), at(r, x1));
    assert_eq!(dev(&mut m, &code, &act_slot("gold", 0, 20)), Ok(()));
    let r = rm(&mut m, &code);
    assert!(
        at(r, x0) - before0 == 20 && at(r, x1) == before1,
        "Münzen bei Monarch 0: +{}, bei Monarch 1: +{}",
        at(r, x0) - before0,
        at(r, x1) - before1
    );
    ticks(&mut m, &code, 1);
    let r = rm(&mut m, &code);
    let (g0, g1) = (isl_player(r, i0).gold, isl_player(r, i1).gold);
    assert!(g0 >= 20 && g1 == 0, "Gold Monarch 0 {g0}, Monarch 1 {g1}");
}

// B-178/AC-02, AC-03: ungültige Felder ergeben bad_request.
#[test]
fn dev_ungueltig() {
    let (mut m, code, _i) = dev_room();
    let mat = |slot: Option<i64>, amount: i64, res: &str| DevAction {
        slot,
        amount,
        resource: res.into(),
        ..act("material")
    };
    let cases = [
        ("fremder Slot", act_slot("gold", 3, 10)),
        ("amount 0", act_slot("gold", 0, 0)),
        ("amount 1001", act_slot("gold", 0, 1001)),
        (
            "ohne slot",
            DevAction {
                amount: 10,
                ..act("gold")
            },
        ),
        ("unbekannter Stoff", mat(Some(0), 10, "gold")),
        ("material amount 0", mat(Some(0), 0, "wood")),
        ("material ohne slot", mat(None, 10, "wood")),
        ("material fremd Slot", mat(Some(2), 10, "wood")),
    ];
    for (name, a) in cases {
        assert_eq!(
            dev(&mut m, &code, &a),
            Err(Fehler::BadRequest),
            "{name}: erwartet bad_request"
        );
    }
}

// B-178/AC-03: material erhöht den Insel-Vorrat bis zum Lager-Maximum, der Rest wird ohne Fehler verworfen.
#[test]
fn dev_material() {
    let (mut m, code, _i) = dev_room();
    let wood = rm(&mut m, &code).isl.stock.wood;
    let a = DevAction {
        resource: "wood".into(),
        ..act_slot("material", 1, 100)
    };
    assert_eq!(dev(&mut m, &code, &a), Ok(()));
    assert_eq!(rm(&mut m, &code).isl.stock.wood, wood + 100);
    for _ in 0..3 {
        let a = DevAction {
            resource: "stone".into(),
            ..act_slot("material", 0, DEV_MAX_AMOUNT)
        };
        assert_eq!(dev(&mut m, &code, &a), Ok(()));
    }
    let r = rm(&mut m, &code);
    let limit = 300 * r.isl.stages.len() as i64; // Lager-Maximum ohne gebautes Lager
    assert_eq!(r.isl.stock.stone, limit, "Stein, erwartet Maximum");
}

// B-178/AC-06: je gelungener Dev-Aktion genau eine Info-Zeile mit Gerät, Aktion, Werten und Raum.
#[test]
fn dev_log() {
    let (mut m, code, _i) = dev_room();
    let _ = dev(&mut m, &code, &act_slot("gold", 1, 10));
    let iron = DevAction {
        resource: "iron".into(),
        ..act_slot("material", 0, 20)
    };
    let _ = dev(&mut m, &code, &iron);
    let _ = dev(&mut m, &code, &act_slot("gold", 3, 10)); // abgelehnt, keine Zeile
    let lines = log_lines(&m, "🐛 Dev-Aktion");
    assert_eq!(lines.len(), 2, "{:?}", m.log.lines);
    let room = format!("room={code}");
    let wants: [Vec<&str>; 2] = [
        vec!["Info ", "device=a ", "aktion=gold", "slot=1 ", "amount=10", &room],
        vec![
            "aktion=material",
            "slot=0 ",
            "amount=20",
            "resource=iron",
            "genommen=20",
            &room,
        ],
    ];
    for (i, want) in wants.iter().enumerate() {
        for w in want {
            assert!(lines[i].contains(w), "Zeile {i} ohne {w:?}: {}", lines[i]);
        }
    }
}

// Dev-Aktionen für Lager, Baukosten, Ausbau und Tageszeit (nur mit Dev-Mode, Felder werden geprüft).
#[test]
fn dev_lager_kosten_ausbau() {
    let (mut m, code, _i) = dev_room();
    let on = |name: &str, v: bool| DevAction {
        on: Some(v),
        ..act(name)
    };
    assert_eq!(
        dev(&mut m, &code, &act("cheapBuild")),
        Err(Fehler::BadRequest),
        "cheapBuild ohne on"
    );
    let b6 = DevAction {
        level: 6,
        ..act("buildAll")
    };
    assert_eq!(dev(&mut m, &code, &b6), Err(Fehler::BadRequest), "buildAll Stufe 6");
    let b2 = DevAction {
        level: 2,
        ..act("buildAll")
    };
    for a in [
        act("maxStock"),
        act("maxStock"),
        on("cheapBuild", true),
        b2,
        act("fillStock"),
    ] {
        assert_eq!(dev(&mut m, &code, &a), Ok(()), "{}", a.action);
    }
    let r = rm(&mut m, &code);
    assert_eq!(
        isl_player(r, 0).gold,
        30 + 200,
        "Gold nach fillStock, erwartet Maximum 230"
    );
    let s = r.summary();
    assert!(
        s.dev_stock_plus == 200 && s.dev_purse_bonus == 200 && s.dev_cheap,
        "Summary {s:?}"
    );
    let st = &r.isl.stock;
    let all = [st.wood, st.stone, st.copper, st.iron, st.crystal];
    assert!(
        all[0] >= 200 && all.iter().all(|&v| v == all[0]) && r.isl.stages[0].hub_level == 2,
        "Vorrat {all:?}, Hub-Stufe {}",
        r.isl.stages[0].hub_level
    );
    assert_eq!(dev(&mut m, &code, &on("cheapBuild", false)), Ok(()));
    assert!(!rm(&mut m, &code).summary().dev_cheap, "cheapBuild bleibt an");
}

// Nächste Tageszeit und Zeitraffer-Auto: Der Zeitraffer endet im ersten Tick nach dem Tageszeitwechsel, einmalig.
#[test]
fn dev_zeitraffer_endet_mit_tageszeit() {
    let (mut m, code, _i) = dev_room();
    let yes = DevAction {
        on: Some(true),
        ..act("timescaleAuto")
    };
    let ts = |f: i64| DevAction {
        factor: f,
        ..act("timescale")
    };
    assert_eq!(dev(&mut m, &code, &ts(4)), Ok(()));
    assert_eq!(dev(&mut m, &code, &yes), Ok(()));
    ticks(&mut m, &code, 1);
    let r = rm(&mut m, &code);
    assert!(
        r.scale() == 4 && r.summary().dev_auto_reset,
        "Zeitraffer endet ohne Tageszeitwechsel"
    );
    assert_eq!(dev(&mut m, &code, &act("nextPhase")), Ok(()));
    ticks(&mut m, &code, 1);
    let r = rm(&mut m, &code);
    assert!(
        r.scale() == 1 && !r.summary().dev_auto_reset,
        "Faktor {}, Auto {} nach Tageszeitwechsel",
        r.scale(),
        r.summary().dev_auto_reset
    );
    assert_eq!(dev(&mut m, &code, &ts(2)), Ok(()));
    assert_eq!(dev(&mut m, &code, &act("nextPhase")), Ok(()));
    ticks(&mut m, &code, 1);
    assert_eq!(rm(&mut m, &code).scale(), 2, "Zeitraffer ohne Auto beendet");
}

// Review TR4.2: Ein Beobachter löst keine Dev-Aktion aus, auch keine raumweite (Grad, Phase, Zeitraffer). Beobachter
// entstehen erst mit RU24.4 (`observe`); der Test setzt die Beobachter-Eigenschaft des Geräts selbst.
#[test]
fn dev_beobachter_abgelehnt() {
    let (mut m, code, _i) = dev_room();
    let (p, _o) = peer(2);
    let mut_room = rm(&mut m, &code);
    mut_room
        .devices
        .insert("o".into(), super::Device::new(p, Default::default()));
    if let Some(d) = mut_room.devices.get_mut("o") {
        d.observer = true;
    }
    let a = DevAction {
        grade: "ultra".into(),
        ..act("grade")
    };
    assert_eq!(
        dev_as(&mut m, &code, "o", 2, &a),
        Err(Fehler::BadRequest),
        "Beobachter dev grade"
    );
    assert_ne!(
        rm(&mut m, &code).isl.options.grade,
        "ultra",
        "Grad durch Beobachter geändert"
    );
}
