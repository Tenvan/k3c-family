//! Liste der Spielstände mit Route, aktueller Insel, abgeschlossenen Inseln und Chronik (GS3.7, B-978/R1, AC-04).

use std::time::Duration;

use k3c_sim::island::{IslandDef, WorldDef};

use super::saves::Saves;
use super::saves_route::{ChronicleInfo, RouteIsland, route_of};
use super::testutil::{TempDir, clock};

fn island(id: &str, name: &str, etappe: i64) -> IslandDef {
    IslandDef {
        id: id.into(),
        name: name.into(),
        etappe,
        ..IslandDef::default()
    }
}

fn nordmeer() -> WorldDef {
    WorldDef {
        id: "nordmeer".into(),
        islands: vec![
            island("a", "Anfang", 0),
            island("b", "Mitte", 2),
            island("c", "Ende", 0),
        ],
        ..WorldDef::default()
    }
}

fn saves(dir: &TempDir) -> Saves {
    let mut s = Saves::new(dir.path(), clock());
    s.fs.pause = Duration::ZERO;
    s
}

#[test]
fn route_nennt_id_name_und_etappe_mit_listenplatz_als_vorgabe() {
    let want = [("a", "Anfang", 1), ("b", "Mitte", 2), ("c", "Ende", 3)].map(|(id, name, etappe)| RouteIsland {
        id: id.into(),
        name: name.into(),
        etappe,
    });
    assert_eq!(route_of(&nordmeer()), want);
}

#[test]
fn liste_nennt_route_insel_abschluesse_und_chronik() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    let stand = r#"{"version":13,"campaignId":"x","savedAt":"2026-11-24T10:00:00Z","day":2,"phase":"day","world":"nordmeer","islandId":"c","completed":["a","b"],"chronicle":[{"islandId":"a","days":4,"monarchs":2},{"islandId":"b","days":6,"monarchs":1}],"players":[{"index":0,"depth":0}]}"#;
    s.store("reise", stand.as_bytes()).expect("reise");
    let list = s.list_with_routes(&|id| (id == "nordmeer").then(nordmeer));
    let e = &list[0];
    assert_eq!(
        (e.island_id.as_str(), e.completed.as_slice()),
        ("c", ["a".to_string(), "b".to_string()].as_slice())
    );
    assert_eq!(
        e.chronicle,
        [
            ChronicleInfo {
                island: "a".into(),
                days: 4,
                monarchs: 2
            },
            ChronicleInfo {
                island: "b".into(),
                days: 6,
                monarchs: 1
            },
        ]
    );
    assert_eq!(
        e.route.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
    let json = serde_json::to_value(e).expect("JSON");
    assert_eq!(json["islandId"], "c");
    assert_eq!(
        json["route"][1],
        serde_json::json!({"id": "b", "name": "Mitte", "etappe": 2})
    );
}

#[test]
fn ohne_welt_kaputt_oder_alt_keine_route() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.store(
        "fremd",
        br#"{"version":13,"campaignId":"f","savedAt":"2026-11-24T10:00:00Z","world":"unbekannt","islandId":"a"}"#,
    )
    .expect("fremd");
    s.store(
        "alt",
        br#"{"version":4,"campaignId":"o","savedAt":"2026-10-04T22:00:00Z"}"#,
    )
    .expect("alt");
    std::fs::write(dir.path().join("kaputt.json"), "{kaputt").expect("kaputt");
    let std_world = WorldDef {
        id: "standard".into(),
        islands: vec![island("island1", "Insel 1", 0)],
        ..WorldDef::default()
    };
    let list = s.list_with_routes(&|id| (id == "standard").then(|| std_world.clone()));
    let by = |n: &str| list.iter().find(|e| e.name == n).expect("Eintrag");
    assert!(by("fremd").route.is_empty(), "unbekannte Welt: keine Vorschau");
    assert!(
        by("kaputt").route.is_empty() && !by("kaputt").error.is_empty(),
        "kaputt: nur Fehler"
    );
    assert_eq!(by("alt").route.len(), 1, "ohne Welt-Angabe gilt standard");
    assert!(
        by("alt").completed.is_empty() && by("alt").chronicle.is_empty(),
        "älterer Stand ohne Chronik"
    );
}

#[test]
fn liste_ohne_route_bleibt_wie_bisher() {
    let dir = TempDir::new();
    let mut s = saves(&dir);
    s.store(
        "neu",
        br#"{"version":13,"campaignId":"n","savedAt":"2026-11-24T10:00:00Z","islandId":"a"}"#,
    )
    .expect("neu");
    let json = serde_json::to_value(s.list()).expect("JSON");
    assert!(json[0].get("route").is_none() && json[0].get("chronicle").is_none());
}
