//! Tests zu `ruin.rs` (Go: `building_ruin_test.go`, `building_ruin_preview_test.go`, `building_ruin_save_test.go`, WZ14.3, WZ14.8,
//! B-468/AC-03, docs/rules/geplante-chunks.md § 8), mit 2 Spielern über `fixture_island`. Die Tests über `fixture_island`
//! (`create_world`) warten auf RU18 (Zahlplatz `new_hub_site`); Entwickler-Aktionen (`DevBuildAll`, `DevCastleFall`) und
//! die Vorschau-Funktionen kommen mit RU19, Insel-Spielstand und Zahlziel (`find_pay_target`, `pay_one_coin`) mit RU18.2.
#![allow(clippy::unimplemented)] // Rümpfe der Tests, die auf spätere Sessions warten

use super::repair::{repair, repair_job};
use super::ruin::{add_ruin_sites, is_ruin, kind_hub_level, line_before_ruin, ruin_site};
use super::testutil::*;
use crate::state::common::{apply_damage, destroy_site, is_dangerous};
use crate::world::vals;
use k3c_level::types::Ruin;

#[test]
#[ignore = "wartet auf RU18 (new_hub_site)"]
fn ruine_startet_gebaut_mit_kennung() {
    let (isl, _) = fixture_island();
    let w = &isl.stages[0];
    assert_eq!(w.level.ruins.len(), 3, "Ruinen im Level: {:?}", w.level.ruins);
    assert_eq!(
        w.sites.iter().filter(|s| s.ruin).count(),
        3,
        "Plätze mit Kennung, erwartet 3"
    );
    let v = vals(w);
    for r in &w.level.ruins {
        let si = site_at(w, &r.kind, r.x).unwrap_or_else(|| panic!("Ruine {}@{} fehlt", r.kind, r.x));
        let s = &w.sites[si];
        assert!(
            s.ruin && s.state == "built" && s.build_progress == 1.0 && s.level == 0,
            "Ruine {}@{}: {s:?}",
            r.kind,
            r.x
        );
        let base = v.buildings[&r.kind].hp;
        let want = ruin_start_hp(w, &r.kind);
        assert!(
            s.hp == want && s.max_hp == base,
            "Ruine {}: HP {}/{}, erwartet {want}/{base}",
            r.kind,
            s.hp,
            s.max_hp
        );
    }
    let snap = serde_json::to_string(&w.sites).expect("Plätze als JSON");
    assert_eq!(
        snap.matches("\"ruin\":true").count(),
        3,
        "Snapshot trägt die Kennung nicht genau dreimal: {snap}"
    );
}

/// Ohne Planung gibt es keine Ruinen und keine Kennung im Snapshot (Goldens byte-gleich).
#[test]
#[ignore = "wartet auf RU18 (new_hub_site)"]
fn ohne_planung_keine_ruine() {
    let (isl, _) = ruin_island(vec![]);
    let w = &isl.stages[0];
    let snap = serde_json::to_string(&w.sites).expect("Plätze als JSON");
    assert!(
        w.level.ruins.is_empty() && !snap.contains("\"ruin\""),
        "ohne Planung darf es keine Ruine geben"
    );
}

/// Die Ruine wirkt wie ein gebautes Gebäude ihrer Art: Kaserne → Truppenlimit, Lager → Kapazität und Abgabestelle.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, Insel-Vorrat)"]
fn ruine_wirkt_wie_gebaeude() {
    unimplemented!("RU18: Kapazität und Vorrat der Insel (Go: capacity, isl.Stock) mit IslandRef")
}

/// Hub-neutral: Hub-Stufe, Bezahlbarkeit und Hub-Stufe der Hub-Plätze und Mauerlinien sind wie ohne Ruinen; der Sieg „alles
/// gebaut“ zählt Ruinen nicht.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, site_payable, EconomyOf)"]
fn ruine_ist_hub_neutral() {
    unimplemented!("RU18: site_payable, EconomyOf(w).HubUpgrade (RU18.3)")
}

/// Bauern reparieren die Ruine wie jedes beschädigte Gebäude, ohne Gold und Material.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site)"]
fn ruine_wird_repariert() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    let s = ruin_of(w, "smithy");
    let ti = w
        .troops
        .iter()
        .position(|t| t.kind == "peasant" && t.hammer) // repariert nur ein Handwerker (B-950)
        .expect("Ausgangslage: Bauer fehlt");
    assert!(!is_dangerous(w), "Ausgangslage: Gefahr");
    w.troops[ti].x = w.sites[s].x;
    w.troops[ti].job = repair_job(w, ti);
    let sid = w.sites[s].id;
    assert!(
        w.troops[ti].job.as_ref().is_some_and(|j| j.site_id == sid),
        "Reparatur-Auftrag, erwartet die Schmiede-Ruine {sid}"
    );
    let stock = w.stock.clone();
    let v = vals(w);
    for _ in 0..3000 {
        if w.troops[ti].job.is_none() {
            break;
        }
        repair(w, &v, ti, DT);
    }
    let site = &w.sites[s];
    assert!(
        site.hp == site.max_hp && site.ruin && site.state == "built" && w.stock == stock,
        "nach der Reparatur: {site:?}"
    );
    assert!(has_event(&w.events, "repaired"), "kein Ereignis repaired");
}

/// Zerstört: unbezahlt mit Kennung; Wiederaufbau erst ab der Hub-Stufe der Art.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, site_payable)"]
fn ruine_zerstoert_und_wiederaufbau_ab_hub_stufe_der_art() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    let v = vals(w);
    for (kind, from) in [("smithy", 3), ("barracks", 2), ("storage", 2)] {
        // hub.json: sites bzw. islandSites
        let s = ruin_of(w, kind);
        let (id, hp) = (w.sites[s].id, w.sites[s].hp);
        apply_damage(w, id, hp + 1.0);
        let site = &w.sites[s];
        assert!(
            site.state == "unpaid"
                && site.ruin
                && site.hp == 0.0
                && site.paid_gold == 0
                && site.max_hp == v.buildings[kind].hp,
            "zerstörte Ruine {kind}: {site:?}"
        );
        for hub in 1..=crate::world::hub_of(w).levels.len() as i64 {
            w.hub_level = hub;
            let payable = super::pending::site_payable(w, &v, None, &w.sites[s]);
            assert_eq!(payable, hub >= from, "{kind} auf Hub-Stufe {hub}: erwartet ab {from}");
        }
        w.hub_level = 1;
    }
    assert!(has_event(&w.events, "destroyed"), "kein Ereignis destroyed");
}

/// Wiederaufbau wie jeder Bau (bezahlen, Bauer baut): volle HP der Art, die Kennung bleibt.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, pay_site, build)"]
fn ruine_wiederaufbau_behaelt_kennung() {
    unimplemented!("RU18: pay_site, build (RU18.2, RU18.3)")
}

/// Burgfall (castleLosses): Ruinen kehren in den Ausgangszustand zurück, auch nach Zerstörung oder voller Reparatur;
/// Hub-Plätze werden leer.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, castle_losses)"]
fn burgfall_setzt_ruinen_zurueck() {
    unimplemented!("RU18: castle_losses und step_island mit Burgfall (RU18.5)")
}

/// WZ14.8 (Review-Befund S1, L2): Ausbau der Vorschau und „alles bauen“ lassen eine Ruine beschädigt (HP < MaxHP, MaxHP der
/// Art), damit Client und Studio sie mit Schaden und Trümmern zeichnen; „nicht gebaut“ in der Vorschau behält die Kennung.
#[test]
#[ignore = "wartet auf RU19 (dev_build_all, preview_set_site) und RU18 (new_hub_site)"]
fn ruine_bleibt_nach_ausbau_beschaedigt() {
    unimplemented!("RU19: dev_build_all, preview_set_site")
}

/// Ruinen im Spielstand (kind@x, keine neue Version) und Vorrang der Mauerlinien beim Bezahlen (WZ14.3, B-468/AC-03).
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, Insel-Spielstand)"]
fn ruine_im_spielstand() {
    unimplemented!("RU18: Insel-Spielstand (to_save, parse_island_save, from_island_save_in) mit Zahlplatz")
}

/// Die Planung steht im Stand (B-489): Ändert sich die Planung der Welt danach, lädt der Stand seine Stufe so, wie er sie
/// gebaut hat; die Ruinen und ihre Plätze bleiben (vorher verlor der Stand sie, WZ24.2).
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, Insel-Spielstand)"]
fn ruine_im_spielstand_bleibt_bei_geaenderter_welt_planung() {
    unimplemented!("RU18: Insel-Spielstand (to_save, parse_island_save, from_island_save_in) mit Zahlplatz")
}

/// Vorrang beim Bezahlen: Liegt eine Ruine in Zahl-Reichweite eines unbezahlten Bauplatzes der Mauerlinien, gehen die Münzen
/// an den Linien-Platz, nicht an Produkt oder Ausbau der Ruine; genau auf wall.x und knapp daneben.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, find_pay_target, pay_one_coin)"]
fn linien_platz_vor_ruine_beim_bezahlen() {
    unimplemented!("RU18: find_pay_target, pay_one_coin (RU18.2)")
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): `is_ruin` kennt nur Ruinen der Planung, `ruin_site` baut
/// den Ausgangszustand mit dem HP-Anteil der Art, `add_ruin_sites` legt je Ruine einen Platz an und zieht je eine ID.
#[test]
fn ruine_ausgangszustand_aus_der_planung() {
    let mut w = bare_world();
    w.level.ruins = vec![
        Ruin {
            kind: "smithy".into(),
            x: 300.0,
        },
        Ruin {
            kind: "storage".into(),
            x: 340.0,
        },
    ];
    assert!(is_ruin(&w, "smithy", 300.0) && is_ruin(&w, "storage", 340.0));
    assert!(
        !is_ruin(&w, "smithy", 301.0) && !is_ruin(&w, "storage", 300.0),
        "kind@x muss passen"
    );
    let id_before = w.next_id;
    add_ruin_sites(&mut w);
    assert_eq!(w.sites.len(), 2);
    assert_eq!(w.next_id, id_before + 2, "eine ID je Ruine");
    let v = vals(&w);
    for (s, r) in w.sites.iter().zip(w.level.ruins.iter()) {
        assert!(
            s.ruin && s.state == "built" && s.build_progress == 1.0 && s.level == 0,
            "{s:?}"
        );
        assert_eq!(s.max_hp, v.buildings[&r.kind].hp);
        assert_eq!(s.hp, (0.25 * s.max_hp).ceil(), "Start-HP der Ruine");
        assert!(s.hp < s.max_hp);
    }
    let extra = ruin_site(&mut w, "smithy", 300.0);
    assert!(extra.ruin && extra.id == id_before + 2);
}

/// Eigener Test: Die Hub-Stufe einer Art ist die kleinste ihrer Hub-Plätze (sites, islandSites), sonst 1; ein zerstörter
/// Ruinen-Platz behält die Kennung (`empty_site`, `destroy_site`).
#[test]
fn kind_hub_level_und_zerstoerte_ruine() {
    let mut w = bare_world();
    assert_eq!(kind_hub_level(&w, "gibtEsNicht"), 1);
    let h = crate::world::hub_of(&w).clone();
    for kind in ["smithy", "barracks", "storage"] {
        let want = h
            .sites
            .iter()
            .chain(h.island_sites.iter())
            .filter(|s| s.kind == kind)
            .map(|s| s.hub_level)
            .min();
        assert_eq!(kind_hub_level(&w, kind), want.unwrap_or(1).max(1), "{kind}");
    }
    w.level.ruins = vec![Ruin {
        kind: "smithy".into(),
        x: 300.0,
    }];
    add_ruin_sites(&mut w);
    let id = w.sites[0].id;
    destroy_site(&mut w, id);
    let s = &w.sites[0];
    assert!(s.state == "unpaid" && s.ruin && s.hp == 0.0, "zerstörte Ruine: {s:?}");
}

/// Eigener Test: `line_before_ruin` lässt ein Ziel, das keine Ruine ist, und eine Ruine ohne Linien-Platz unverändert.
#[test]
fn line_before_ruin_ohne_linien_platz_bleibt_ziel() {
    let mut w = bare_world();
    w.level.ruins = vec![Ruin {
        kind: "workshop".into(),
        x: 300.0,
    }];
    add_ruin_sites(&mut w);
    let plain = built_site(&mut w, "bowyer", 310.0);
    let v = vals(&w);
    assert_eq!(line_before_ruin(&w, &v, None, None, 300.0), None);
    assert_eq!(
        line_before_ruin(&w, &v, None, Some(plain), 300.0),
        Some(plain),
        "keine Ruine"
    );
    assert_eq!(
        line_before_ruin(&w, &v, None, Some(0), 300.0),
        Some(0),
        "Ruine ohne Linien-Platz in Reichweite"
    );
}
