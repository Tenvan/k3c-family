//! Tests zu `ruin.rs` (Go: `building_ruin_test.go`, `building_ruin_preview_test.go`, `building_ruin_save_test.go`, WZ14.3, WZ14.8,
//! B-468/AC-03, docs/rules/geplante-chunks.md § 8), mit 2 Spielern über `fixture_island`. Die Tests über `fixture_island`
//! (`create_world`) warten auf RU18 (Zahlplatz `new_hub_site`); Entwickler-Aktionen (`DevBuildAll`, `DevCastleFall`) und
//! die Vorschau-Funktionen kommen mit RU19, Insel-Spielstand und Zahlziel (`find_pay_target`, `pay_one_coin`) mit RU18.2.

#![allow(clippy::unimplemented)] // Rümpfe der zwei ignorierten Tests (Port offen, B-1103)

use super::repair::{repair, repair_job};
use super::ruin::{add_ruin_sites, is_ruin, kind_hub_level, line_before_ruin, ruin_site};
use super::testutil::*;
use crate::state::common::{apply_damage, destroy_site, is_dangerous};
use crate::world::vals;
use k3c_level::types::Ruin;

#[test]
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
fn ruine_wirkt_wie_gebaeude() {
    use super::barracks::troop_limit;
    use crate::island::storage::{capacity, deposit_point};
    let (mut isl, data) = fixture_island();
    let (mut isl0, _) = ruin_island(vec![]);
    let v = vals(&isl.stages[0]);
    let per = v.buildings["barracks"].troop_limit.per_building;
    let got = troop_limit(&isl.stages[0]) - troop_limit(&isl0.stages[0]);
    assert_eq!(got, per, "Kaserne als Ruine: Truppenlimit +{got}, erwartet +{per}");
    let r = isl.stage_ref(0, &data.difficulty);
    let r0 = isl0.stage_ref(0, &data.difficulty);
    let c = capacity(&isl.stages[0], &v, Some(&r)).unwrap_or(0);
    let c0 = capacity(&isl0.stages[0], &v, Some(&r0)).unwrap_or(0);
    assert_eq!(
        c - c0,
        v.economy.storage.per_storage,
        "Lager als Ruine: Kapazität +{}",
        c - c0
    );
    assert_eq!(isl.stock, isl0.stock, "Lager als Ruine ändert den Bestand");
    let w = &isl.stages[0];
    let sx = w.sites[ruin_of(w, "storage")].x;
    assert_eq!(deposit_point(w, sx), sx, "das Lager als Ruine ist keine Abgabestelle");
}

/// Hub-neutral: Hub-Stufe, Bezahlbarkeit und Hub-Stufe der Hub-Plätze und Mauerlinien sind wie ohne Ruinen; der Sieg „alles
/// gebaut“ zählt Ruinen nicht.
#[test]
fn ruine_ist_hub_neutral() {
    use crate::economy::{economy_of, site_payable};
    use crate::hub::site_hub_level;
    use crate::island::victory::all_built;
    use crate::world::lines::{level_site, line_open};
    let (mut isl, _) = fixture_island();
    let (isl0, _) = ruin_island(vec![]);
    let (w, w0) = (&isl.stages[0], &isl0.stages[0]);
    let v = vals(w);
    let (e, e0) = (economy_of(w, &v, None), economy_of(w0, &v, None));
    assert!(
        w.hub_level == 1 && e.hub_upgrade.is_some() && e.hub_upgrade == e0.hub_upgrade,
        "Hub-Stufe/Hub-Ausbau mit Ruinen"
    );
    for s in w.sites.iter().filter(|s| !s.ruin && level_site(w, s).is_none()) {
        let i0 = site_at(w0, &s.kind, s.x).unwrap_or_else(|| panic!("Platz {}@{} fehlt ohne Planung", s.kind, s.x));
        let s0 = &w0.sites[i0];
        assert!(
            site_payable(w, &v, None, s) == site_payable(w0, &v, None, s0)
                && site_hub_level(w, s) == site_hub_level(w0, s0)
                && line_open(w, s) == line_open(w0, s0),
            "Platz {}@{} verhält sich mit Ruinen anders",
            s.kind,
            s.x
        );
    }
    // Sieg „alles gebaut“: eine Ruine ist ein Bauplatz wie jeder andere, erst ihr Wiederaufbau zählt.
    for s in &mut isl.stages[0].sites {
        s.state = "built".into();
    }
    assert!(all_built(&isl), "alles gebaut inklusive Ruine muss gewinnen");
    let w = &mut isl.stages[0];
    let si = ruin_of(w, "smithy");
    let id = w.sites[si].id;
    destroy_site(w, id);
    assert!(
        !all_built(&isl),
        "eine zerstörte Ruine muss den Sieg „alles gebaut“ sperren"
    );
}

/// Bauern reparieren die Ruine wie jedes beschädigte Gebäude, ohne Gold und Material.
#[test]
fn ruine_wird_repariert() {
    let (mut isl, _) = fixture_island();
    let w = &mut isl.stages[0];
    let s = ruin_of(w, "smithy");
    let ti = w
        .troops
        .iter()
        .position(|t| t.serving() && t.hammer) // repariert nur ein Handwerker (B-950)
        .expect("Ausgangslage: Bauer fehlt");
    assert!(!is_dangerous(w), "Ausgangslage: Gefahr");
    w.troops[ti].x = w.sites[s].x;
    crate::enemies::testutil::fund_repairs(w);
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
            let payable = crate::economy::site_payable(w, &v, None, &w.sites[s]);
            assert_eq!(payable, hub >= from, "{kind} auf Hub-Stufe {hub}: erwartet ab {from}");
        }
        w.hub_level = 1;
    }
    assert!(has_event(&w.events, "destroyed"), "kein Ereignis destroyed");
}

/// Wiederaufbau wie jeder Bau (bezahlen, Bauer baut): volle HP der Art, die Kennung bleibt.
#[test]
#[ignore = "Port offen (B-1103): `unit::build` und `pay_site` sind für Tests nicht sichtbar (Produktionscode, nicht pub(crate))"]
fn ruine_wiederaufbau_behaelt_kennung() {
    unimplemented!("B-1103: fixture_island, unit::build (pub(crate) machen), pay_site")
}

/// Burgfall (castleLosses): Ruinen kehren in den Ausgangszustand zurück, auch nach Zerstörung oder voller Reparatur;
/// Hub-Plätze werden leer.
#[test]
fn burgfall_setzt_ruinen_zurueck() {
    let (mut isl, data) = fixture_island();
    isl.options.defeat = "stage".into();
    let kinds = ["smithy", "barracks", "storage"];
    let w = &mut isl.stages[0];
    let ids: Vec<i64> = kinds.iter().map(|k| w.sites[ruin_of(w, k)].id).collect();
    let sid = w.sites[ruin_of(w, "smithy")].id;
    destroy_site(w, sid);
    let (bar, sto) = (ruin_of(w, "barracks"), ruin_of(w, "storage"));
    w.sites[bar].hp = vals(w).buildings["barracks"].hp;
    w.sites[sto].hp = 7.0;
    let workshop = crate::enemies::testutil::build_site(w, "workshop");
    assert_eq!(w.cycle.phase, "day", "Ausgangslage");
    w.castle.hp = 0.0;
    crate::island::step_island(
        &mut isl,
        &[Default::default(), Default::default()],
        &data.difficulty,
        crate::island::testutil::DT,
    );
    let w = &isl.stages[0];
    assert!(
        w.events.iter().any(|e| e["type"] == "castleFallen"),
        "die Burg ist nicht gefallen"
    );
    for (kind, id) in kinds.iter().zip(ids) {
        let s = &w.sites[ruin_of(w, kind)];
        assert!(
            s.id == id
                && s.state == "built"
                && s.hp == ruin_start_hp(w, kind)
                && s.max_hp == vals(w).buildings[*kind].hp,
            "Ruine {kind} nach Burgfall: {s:?}"
        );
    }
    let hub = &w.sites[workshop];
    assert!(hub.state == "unpaid" && !hub.ruin, "Hub-Platz nach Burgfall: {hub:?}");
}

/// WZ14.8 (Review-Befund S1, L2): Ausbau der Vorschau und „alles bauen“ lassen eine Ruine beschädigt (HP < MaxHP, MaxHP der
/// Art), damit Client und Studio sie mit Schaden und Trümmern zeichnen; „nicht gebaut“ in der Vorschau behält die Kennung.
#[test]
fn ruine_bleibt_nach_ausbau_beschaedigt() {
    let (mut isl, _) = fixture_island();
    let levels = crate::world::hub_of(&isl.stages[0]).levels.len() as i64;
    assert!(crate::dev::dev_build_all(&mut isl, levels), "dev_build_all abgelehnt");
    let w = &mut isl.stages[0];
    for k in ["smithy", "barracks", "storage"] {
        let si = ruin_of(w, k);
        let s = &w.sites[si];
        let b = &vals(w).buildings[k];
        let max_hp = if b.levels.is_empty() {
            b.hp
        } else {
            b.levels[(s.level - 1) as usize].hp
        }; // MaxHP der Stufe (B-569)
        let start = ruin_start_hp(w, k);
        assert!(
            s.hp == start && s.max_hp == max_hp && s.hp < s.max_hp,
            "Ruine {k} nach Ausbau: HP {}/{}",
            s.hp,
            s.max_hp
        );
    }
    let si = ruin_of(w, "smithy");
    let look = crate::world::preview::SiteLook {
        unbuilt: true,
        ..Default::default()
    };
    let r = crate::world::preview::preview_set_site(w, si, &look);
    assert!(
        r.is_ok() && w.sites[si].ruin && w.sites[si].state == "unpaid",
        "nicht gebaut in der Vorschau"
    );
}

/// Vorrang beim Bezahlen: Liegt eine Ruine in Zahl-Reichweite eines unbezahlten Bauplatzes der Mauerlinien, gehen die Münzen
/// an den Linien-Platz, nicht an Produkt oder Ausbau der Ruine; genau auf wall.x und knapp daneben.
#[test]
#[ignore = "Port offen (B-1103): `find_pay_target` und `pay_one_coin` sind für Tests nicht sichtbar (Produktionscode, nicht pub(crate))"]
fn linien_platz_vor_ruine_beim_bezahlen() {
    unimplemented!("B-1103: fixture_island, find_pay_target, pay_one_coin")
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
