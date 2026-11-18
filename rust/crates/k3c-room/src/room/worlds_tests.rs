//! Tests aus `worlds_test.go` und `worlds_values_test.go` (WZ18.4, B-488/AC-03; BAL8.2, B-681/AC-01; `docs/rules/welten.md`
//! § 1/5/6): Der Raum legt neue Inseln in der gewählten Welt an (Wahl nur im Dev-Mode, sonst nur standard), lädt Spielstände
//! mit ihrer gespeicherten Welt und lehnt unbekannte oder ungültige Welten ab. Gos `m.Worlds` (`MapFS`) ist ein Ordner unter
//! `.work/` (`WorldDir`) und gehört dem Manager (`m.worlds`); einige Tests rufen `create_in_from` mit der Quelle direkt.
use k3c_sim::island::{CreateOptions, add_island_player, create_island_in, next_island};

use super::Options;
use super::testutil::{TestManager, WorldDir, manager, peer};
use crate::fehler::Fehler;

/// Eine Test-Welt mit zwei Inseln (nicht aus `data/worlds/`).
pub(super) const NORDMEER_RAW: &str = r#"{"id":"nordmeer","name":"Nordmeer","islands":[{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},{"id":"eisinsel","name":"Eisinsel","depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15}}]}"#;

/// Erbt von `nordmeer` (`extends` aus dem Ordner der Welten) und macht die Mauer teurer.
const NORDTEUER_RAW: &str = r#"{"id":"nordteuer","name":"Nordmeer teuer","extends":"nordmeer","overrides":{"buildings":{"wall":{"cost":{"gold":9}}}},"islands":[{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}}]}"#;

const WERTKAPUTT_RAW: &str = r#"{"id":"wertkaputt","name":"Kaputt","overrides":{"buildings":{"wall":{"cost":{"gold":"viel"}}}},"islands":[{"id":"island1","name":"Insel 1","depths":[0]}]}"#;

/// Ein Manager im Dev-Mode und der Ordner der Test-Welten (Go `worldFixture`).
pub(super) fn world_fixture(tag: &str) -> (TestManager, WorldDir) {
    let (mut m, _) = manager();
    m.dev = true;
    let files = [
        ("nordmeer", NORDMEER_RAW),
        ("nordteuer", NORDTEUER_RAW),
        ("wertkaputt", WERTKAPUTT_RAW),
        ("kaputt", r#"{"id":"kaputt","name":"Kaputt","islands":[],"size":3}"#),
    ];
    (m, WorldDir::new(tag, &files))
}

fn opts(world: &str) -> Options {
    Options {
        world: world.to_string(),
        ..Options::default()
    }
}

#[test]
fn welt_wahl_im_dev_mode() {
    let (m, dir) = world_fixture("wahl");
    let isl = m
        .create_in_from(Some(&dir.src), "nord", 4, &opts("nordmeer"))
        .expect("nordmeer");
    assert_eq!(
        (
            isl.world_id(),
            isl.island_id(),
            isl.stages.len(),
            isl.stages[1].biome.depth
        ),
        ("nordmeer", "island1", 2, 4)
    );
    let s = isl.to_save("2026-10-19T20:00:00Z").expect("Spielstand");
    assert_eq!((s.world.as_str(), s.island_id.as_str()), ("nordmeer", "island1"));
}

#[test]
fn welt_unbekannte_startstufe_ist_bad_request() {
    let (m, dir) = world_fixture("stufe");
    assert_eq!(
        m.create_in_from(Some(&dir.src), "nord", 3, &opts("nordmeer")).err(),
        Some(Fehler::BadRequest)
    );
}

#[test]
fn welt_standard_ohne_angabe() {
    for world in ["", "standard"] {
        let (m, _) = manager(); // ohne Dev-Mode und ohne Welten-Ordner
        let isl = m.create_in("std", 0, &opts(world)).expect("standard");
        assert_eq!((isl.world_id(), isl.stages.len()), ("standard", 5), "Welt {world:?}");
    }
}

// Die Standard-Welt kommt auch im Dev-Mode aus der Einbettung, nie von der Platte (welten.md § 6).
#[test]
fn welt_standard_im_dev_mode_eingebettet() {
    let (m, _) = world_fixture("std");
    let dir = WorldDir::new("std-kaputt", &[("standard", "{")]);
    let isl = m
        .create_in_from(Some(&dir.src), "std", 0, &opts("standard"))
        .expect("standard");
    assert_eq!((isl.world_id(), isl.stages.len()), ("standard", 5));
}

/// `Manager::create` scheitert mit der Welt: kein Raum, kein Spielstand, das Log meldet 🚫.
fn rejected(m: &mut TestManager, world: &str, want: &Fehler, msg: &str) {
    let (p, _inbox) = peer(1);
    let err = m
        .create("xbox", p, "nord", true, 0, &[0], opts(world))
        .expect_err("abgelehnt");
    assert!(
        err.is(want) && err.to_string().contains(msg),
        "Welt {world:?}: {err}, erwartet {want} mit {msg:?}"
    );
    assert!(
        m.rooms().is_empty() && !m.store.data.contains_key("nord"),
        "Welt {world:?}: Raum oder Spielstand angelegt"
    );
    assert!(
        m.log.lines.iter().any(|l| l.contains("🚫 Beitritt abgelehnt")),
        "Log ohne Ablehnung {msg:?}"
    );
}

#[test]
fn welt_wahl_ohne_dev_mode_abgelehnt() {
    let (mut m, _) = world_fixture("ohne-dev");
    m.dev = false;
    rejected(
        &mut m,
        "nordmeer",
        &Fehler::Forbidden,
        r#"Welt "nordmeer" nur im Dev-Mode"#,
    );
}

#[test]
fn welt_ohne_ordner_unbekannt() {
    // Dev-Mode, aber kein Ordner der Welten (`worlds_source` ist leer)
    let (mut m, _) = world_fixture("ohne-ordner");
    rejected(
        &mut m,
        "nordmeer",
        &Fehler::WorldUnknown,
        r#"Welt "nordmeer" unbekannt"#,
    );
}

#[test]
fn welt_unbekannt_oder_ungueltig_abgelehnt() {
    let cases = [
        ("suedsee", r#"Welt "suedsee" unbekannt"#),
        ("../saves/nord", r#"Welt-ID "../saves/nord" ungültig"#),
        ("kaputt", r#"data/worlds/kaputt.json: unbekanntes Feld "size""#),
        ("Nordmeer", r#"Welt-ID "Nordmeer" ungültig"#),
        ("nordmeer/../x", "ungültig"),
    ];
    for (world, msg) in cases {
        let (m, dir) = world_fixture("ungueltig");
        let err = m
            .create_in_from(Some(&dir.src), "nord", 0, &opts(world))
            .expect_err("abgelehnt");
        assert!(
            err.is(&Fehler::WorldUnknown) && err.to_string().contains(msg),
            "Welt {world:?}: {err}, erwartet {msg:?}"
        );
    }
}

/// Ein Spielstand auf Insel 2 der Test-Welt nordmeer (Go `nordmeerSave`).
fn nordmeer_save(m: &TestManager, dir: &WorldDir) -> Vec<u8> {
    let w = m.world_from(Some(&dir.src), "nordmeer").expect("nordmeer");
    let mut isl = create_island_in(&m.data, &w, "nord", &[0, 4], 1.0, CreateOptions::default()).expect("Insel");
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    let next = next_island(&m.data, &isl, &w.islands[1]).expect("nächste Insel");
    serde_json::to_vec(&next.to_save("2026-10-19T20:00:00Z").expect("Spielstand")).expect("JSON")
}

#[test]
fn welt_laden_mit_gespeicherter_welt() {
    let (mut m, dir) = world_fixture("laden");
    m.worlds = Some(dir.src.clone());
    let save = nordmeer_save(&m, &dir);
    m.store.data.insert("nord".into(), save);
    let (p, _inbox) = peer(1);
    let code = m
        .create("xbox", p, "nord", false, 0, &[0, 1], Options::default())
        .expect("create");
    let isl = &m.room(&code).expect("Raum").isl;
    assert_eq!(
        (isl.world_id(), isl.island_id(), isl.number),
        ("nordmeer", "eisinsel", 1),
        "Welt, Insel, Nummer"
    );
}

// Ohne Dev-Mode, Datei fehlt, Datei ungültig: `spielstand: …`, Stand und Raumliste unverändert.
#[test]
fn welt_laden_abgelehnt_stand_unveraendert() {
    let (m0, dir0) = world_fixture("abgelehnt-save");
    let save = nordmeer_save(&m0, &dir0);
    let cases: [(&str, bool, Option<WorldDir>, Fehler, &str); 3] = [
        (
            "ohne Dev-Mode",
            false,
            None,
            Fehler::Forbidden,
            r#"spielstand: Welt "nordmeer" nur im Dev-Mode"#,
        ),
        (
            "Datei fehlt",
            true,
            Some(WorldDir::new("abgelehnt-fehlt", &[])),
            Fehler::WorldUnknown,
            r#"spielstand: Welt "nordmeer" unbekannt"#,
        ),
        (
            "Datei ungültig",
            true,
            Some(WorldDir::new("abgelehnt-kaputt", &[("nordmeer", "{")])),
            Fehler::WorldUnknown,
            "data/worlds/nordmeer.json: kein gültiges JSON",
        ),
    ];
    for (name, dev, dir, want, msg) in cases {
        let (mut m, _) = manager();
        m.dev = dev;
        m.worlds = dir.as_ref().map(|d| d.src.clone());
        m.store.data.insert("nord".into(), save.clone());
        let (p, _inbox) = peer(1);
        let err = m
            .create("xbox", p, "nord", false, 0, &[0], Options::default())
            .expect_err("abgelehnt");
        assert!(
            err.is(&want) && err.to_string().contains(msg),
            "{name}: {err}, erwartet {msg:?}"
        );
        assert!(
            m.store.data.get("nord") == Some(&save)
                && m.rooms().is_empty()
                && m.log.lines.iter().any(|l| l.contains("🚫")),
            "{name}: Stand geändert, Raum angelegt oder kein 🚫 im Log"
        );
    }
}

// Die Weltliste: standard zuerst, im Dev-Mode dazu jede gültige Datei (ungültige fehlen), sonst nur standard.
#[test]
fn weltliste_standard_zuerst_dann_gueltige_dateien() {
    let (mut m, dir) = world_fixture("liste");
    let ids = |m: &TestManager| m.world_list().into_iter().map(|w| w.id).collect::<Vec<_>>();
    assert_eq!(ids(&m), ["standard"], "ohne Ordner");
    m.worlds = Some(dir.src.clone());
    // wertkaputt (ohne depthScaling) und kaputt (unbekanntes Feld) sind ungültig und fehlen
    assert_eq!(ids(&m), ["standard", "nordmeer", "nordteuer"]);
    assert_eq!(m.world_list()[1].name, "Nordmeer");
    m.dev = false;
    assert_eq!(ids(&m), ["standard"], "ohne Dev-Mode");
}

// Ein Spielstand, der kein JSON ist, scheitert mit der Meldung der Sim (`spielstand: …`).
#[test]
fn laden_ungueltiger_spielstand_nennt_spielstand() {
    let (m, _) = manager();
    let err = m.load_in(b"{").expect_err("ungültig");
    assert!(err.to_string().starts_with("spielstand:"), "{err}");
}

// AC-01: `Manager::create` mit der Standard-Welt und zwei Peers rechnet 10 Takte. Die Standard-Welt braucht
// `decode_world_story` (RU19), das Anlegen von Spielern `preset_for` und `new_hub_site` (RU18).
#[test]
fn create_standard_welt_zwei_peers_rechnet_10_takte() {
    let (mut m, _) = manager();
    let (x, mut xi) = peer(1);
    let (h, mut hi) = peer(2);
    let code = m
        .create("xbox", x, "std", true, 0, &[0], Options::default())
        .expect("create");
    m.join("handy", h, &code, &[0]).expect("join");
    for _ in 0..10 {
        assert!(m.safe_tick(&code));
    }
    assert_eq!(m.room(&code).map(|r| r.tick), Some(10));
    assert!(
        xi.has("state 10") && hi.has("state 10"),
        "{:?} {:?}",
        xi.log(),
        hi.log()
    );
}
