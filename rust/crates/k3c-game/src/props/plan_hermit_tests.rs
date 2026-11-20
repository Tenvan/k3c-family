//! Tests zum Einsiedler (RU62.11, B-1001): vor der Zahlung Hütte mit Figur und Preis, danach leere Hütte; im Hub Gerüst mit
//! Figur, dann das Bauwerk; die Zuordnung in data/paper.json passt zum Code.

use k3c_data::Source;
use k3c_sim::state::hermit_view::{HermitHut, HermitPhase, HermitState};
use k3c_sim::state::{Site, World};
use serde_json::Value;

use super::listen::SiteFlags;
use super::plan::{Item, Planner, Shape};
use super::plan_hermit::{HERMIT_ASSETS, HUT_ASSET, building_asset, figure_asset};
use super::testutil::{StubSheets, camera};

fn hermit(phase: HermitPhase, hut: bool) -> HermitState {
    HermitState {
        hermit: "swordsmith".to_string(),
        building: "dojo".to_string(),
        price: 40,
        hut: hut.then_some(HermitHut { x: 10.0, range: 2.0 }),
        phase,
        progress: 0.5,
        open: true,
        ..Default::default()
    }
}

fn assets(items: &[Item]) -> Vec<&'static str> {
    items
        .iter()
        .filter(|it| it.shape == Shape::Sprite)
        .map(|it| it.look.asset)
        .collect()
}

fn hut_plan(phase: HermitPhase) -> Planner {
    let w = World {
        hermit: Some(hermit(phase, true)),
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.hermit_hut(&w, &camera(10.0));
    p
}

fn hall_plan(phase: HermitPhase, state: &str) -> Planner {
    let s = Site {
        id: 7,
        kind: "hermitHall".to_string(),
        x: 30.0,
        state: state.to_string(),
        hp: 100.0,
        max_hp: 100.0,
        ..Default::default()
    };
    let w = World {
        hermit: Some(hermit(phase, false)),
        sites: vec![s.clone()],
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    let fl = SiteFlags {
        visible: true,
        ..Default::default()
    };
    p.hall_site(&w, &s, None, fl);
    p
}

#[test]
fn vor_der_zahlung_steht_huette_mit_figur_und_preis() {
    let p = hut_plan(HermitPhase::Hut);
    assert_eq!(assets(&p.body), vec![HUT_ASSET, "figuren/schwertschmied"]);
    let t = p.top.iter().find(|it| it.shape == Shape::Text).expect("Preis");
    assert_eq!(t.text, "40");
}

#[test]
fn nach_der_zahlung_ist_die_huette_leer() {
    for phase in [HermitPhase::Walking, HermitPhase::Built, HermitPhase::Fallen] {
        let p = hut_plan(phase);
        assert_eq!(assets(&p.body), vec![HUT_ASSET], "{phase:?}");
        assert!(p.top.is_empty(), "kein Preis bei {phase:?}");
    }
}

#[test]
fn ohne_huette_in_der_stufe_wird_nichts_gezeichnet() {
    let w = World {
        hermit: Some(hermit(HermitPhase::Hut, false)),
        ..Default::default()
    };
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.hermit_hut(&w, &camera(10.0));
    assert!(p.body.is_empty() && p.top.is_empty());
}

#[test]
fn im_bau_stehen_geruest_balken_und_figur() {
    let p = hall_plan(HermitPhase::Walking, "hermitBuilding");
    let a = assets(&p.body);
    assert!(a.contains(&"gebaeude-paper/geruest-6"), "Gerüst: {a:?}");
    assert!(a.contains(&"figuren/schwertschmied"), "Figur: {a:?}");
    assert!(!a.contains(&"gebaeude-paper/dojo"));
    assert!(p.top.iter().any(|it| it.shape == Shape::Rect), "Baubalken");
}

#[test]
fn fertig_steht_das_bauwerk_ohne_figur() {
    let p = hall_plan(HermitPhase::Built, "built");
    assert_eq!(assets(&p.body), vec!["gebaeude-paper/dojo"]);
}

#[test]
fn vor_der_zahlung_bleibt_der_hub_platz_leer() {
    let p = hall_plan(HermitPhase::Hut, "unpaid");
    assert!(p.body.is_empty() && p.top.is_empty());
}

#[test]
fn zuordnung_in_paper_json_passt_zum_code() {
    let src = Source::from_env();
    let paper: Value = serde_json::from_str(&src.read("paper.json").expect("paper.json")).expect("JSON");
    let get = |p: &str| paper.pointer(p).and_then(Value::as_str).map(str::to_string);
    assert_eq!(get("/einsiedler/huette").as_deref(), Some(HUT_ASSET));
    for (id, name) in [("swordsmith", "dojo"), ("priestess", "shrine")] {
        assert_eq!(
            get(&format!("/einsiedler/figuren/{id}")).as_deref(),
            figure_asset(id),
            "Figur {id}"
        );
        assert_eq!(
            get(&format!("/einsiedler/bauwerke/{name}")).as_deref(),
            building_asset(name),
            "Bauwerk {name}"
        );
    }
    for a in HERMIT_ASSETS {
        assert!(
            super::assets::level_assets("forest").iter().any(|x| x == a),
            "{a} in der Ladeliste"
        );
    }
}
