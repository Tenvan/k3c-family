//! Tests zu `site_data_test.go` (W0.1 B-206/AC-05, W11.1 B-409, WZ15.3 B-462/AC-02): Die Platz-Regeln stehen in
//! `validate_hub_data` (eine Quelle); diese Tests prüfen data/hub.json und Kopien mit gezielten Verstößen.

use k3c_data::Source;
use k3c_data::buildings::Cost;
use k3c_data::hub::{HubData, HubSite};

use super::testutil::{has_finding, hub, hub_copy, read_hub7, validate, without_kind};
use super::validate::{
    RULE_GAP, RULE_GATE, RULE_JSON, RULE_KIND, RULE_LEVELS, RULE_LINES, RULE_ONE_PER_BAU, RULE_REQUIRED, RULE_STAIRS,
    all_sites, parse_hub, required_site_kinds, validate_hub, validate_hub_data,
};
use crate::world::base::base;

fn pay_range() -> f64 {
    base().expect("Daten").data.economy.pay_range_units
}

fn site(kind: &str, offset: f64, level: i64) -> HubSite {
    HubSite {
        kind: kind.into(),
        offset_units: offset,
        hub_level: level,
        ..HubSite::default()
    }
}

#[test]
fn site_data_hub_json_gueltig() {
    let raw = Source::Embedded.read("hub.json").expect("hub.json");
    let r = validate(&parse_hub(&raw).expect("hub.json parsen"));
    assert!(r.findings.is_empty(), "data/hub.json: {:?}", r.findings);
    assert!(
        r.notes.is_empty(),
        "Hinweise {:?}, erwartet keine (die geplanten Chunks werden mitgeprüft, B-479)",
        r.notes
    );
}

/// WZ15.3/AC-03: Mehr Hub-Stufen ohne Umbau, keine feste 5.
#[test]
fn site_data_sieben_stufen() {
    let h = read_hub7();
    assert_eq!(h.levels.len(), 7, "Fixture: {} Stufen, erwartet 7", h.levels.len());
    let findings = validate(&h).findings;
    assert!(findings.is_empty(), "hub7.json: {findings:?}");
}

#[test]
fn site_data_pflichtplatz_fehlt() {
    for kind in required_site_kinds() {
        let mut h = hub_copy();
        let i = h.sites.iter().position(|s| s.kind == kind);
        let i = i.unwrap_or_else(|| panic!("{kind} steht nicht in data/hub.json"));
        h.sites.remove(i);
        let r = validate(&h);
        assert!(
            has_finding(&r, RULE_REQUIRED, kind),
            "{kind} entfernt: Befund {RULE_REQUIRED:?} erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn site_data_optionale_art_entfernbar() {
    let required = required_site_kinds();
    let mut optional: Vec<String> = Vec::new();
    for s in all_sites(&hub_copy()) {
        if !required.contains(&s.kind.as_str()) && !optional.contains(&s.kind) {
            optional.push(s.kind.clone());
        }
    }
    for want in ["barracks", "tavern", "storage", "healer", "smithy", "armory"] {
        assert!(
            optional.iter().any(|k| k == want),
            "optionale Art {want} fehlt in data/hub.json"
        );
    }
    for kind in &optional {
        let mut h = hub_copy();
        without_kind(&mut h, kind);
        let r = validate(&h);
        assert!(
            r.findings.is_empty(),
            "{kind} entfernt: kein Befund erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn site_data_ein_platz_je_bau() {
    for (s, rule, kind) in [
        (site("healer", -140.0, 3), RULE_ONE_PER_BAU, "healer"),
        (site("workshop", -140.0, 1), RULE_REQUIRED, "workshop"),
    ] {
        let mut h = hub_copy();
        h.sites.push(s);
        let r = validate(&h);
        assert!(
            has_finding(&r, rule, kind),
            "{kind} doppelt: Befund {rule:?} erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn site_data_nur_bekannte_arten() {
    for kind in ["dragon", "wall", "tower", "gate", "castle"] {
        let mut h = hub_copy();
        h.sites.push(site(kind, -140.0, 1));
        let r = validate(&h);
        assert!(
            has_finding(&r, RULE_KIND, &format!("{kind}@")),
            "Art {kind}: Befund {RULE_KIND:?} erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn site_data_hub_stufe_im_bereich() {
    let n = hub_copy().levels.len() as i64;
    type Edit = Box<dyn Fn(&mut HubData)>;
    let cases: Vec<(&str, Edit, &str)> = vec![
        ("Stufe 0", Box::new(|h| h.sites[0].hub_level = 0), "@"),
        (
            "Stufe n+1",
            Box::new(move |h| h.island_sites[0].hub_level = n + 1),
            "storage@",
        ),
        ("ohne Stufen", Box::new(|h| h.levels.clear()), "levels"),
        (
            "letzte Stufe+",
            Box::new(move |h| {
                let last = h.sites.len() - 1;
                h.sites[last].hub_level = n + 1;
            }),
            "armory@",
        ),
    ];
    for (name, edit, site) in cases {
        let mut h = hub_copy();
        edit(&mut h);
        let r = validate(&h);
        assert!(
            has_finding(&r, RULE_LEVELS, site),
            "{name}: Befund {RULE_LEVELS:?} erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn site_data_abstand_bei_voller_streuung() {
    let mut h = hub_copy();
    h.sites[0].offset_units += 2.0 * pay_range() - 1.0; // workshop neben seine Angebote
    let r = validate(&h);
    assert!(
        has_finding(&r, RULE_GAP, "workshop"),
        "Platz zu nah: Befund {RULE_GAP:?} erwartet, bekommen {:?}",
        r.findings
    );
    // Zu nah am festen Baugrund der Zone 1 (Turm bei 36): ein Platz 1 Unit daneben.
    let mut h = hub_copy();
    without_kind(&mut h, "barracks");
    let at = h.site_placement.zone1_tower_units + 1.0;
    h.sites.push(site("barracks", at, 2));
    let r = validate(&h);
    assert!(
        has_finding(&r, RULE_GAP, "tower Z1@+1"),
        "Baugrund: Befund {RULE_GAP:?} mit Turm der Zone 1 erwartet, bekommen {:?}",
        r.findings
    );
}

#[test]
fn site_data_treppen_abstand() {
    let mut h = hub_copy();
    let i = h.sites.iter().position(|s| s.kind == "stairsDown").expect("stairsDown");
    h.sites[i].offset_units = 2.0 * pay_range(); // Abstand zur Burg reicht für Zahlziele, nicht für Treppen
    let r = validate(&h);
    assert!(
        has_finding(&r, RULE_STAIRS, "stairsDown"),
        "Treppe zu nah: Befund {RULE_STAIRS:?} erwartet, bekommen {:?}",
        r.findings
    );
}

#[test]
fn site_data_baugrund() {
    type Edit = Box<dyn Fn(&mut k3c_data::hub::SitePlacementData)>;
    let cases: Vec<(&str, Edit)> = vec![
        (
            "Turm außen vor der Mauer",
            Box::new(|p| p.zone1_tower_units = p.zone1_wall_units),
        ),
        ("Turm nicht positiv", Box::new(|p| p.zone1_tower_units = 0.0)),
        ("über dem Portal", Box::new(|p| p.outside_units = 500.0)),
    ];
    for (name, edit) in cases {
        let mut h = hub_copy();
        edit(&mut h.site_placement);
        let r = validate(&h);
        assert!(
            has_finding(&r, RULE_LINES, "sitePlacement"),
            "{name}: Befund {RULE_LINES:?} erwartet, bekommen {:?}",
            r.findings
        );
    }
}

#[test]
fn validate_hub_kaputtes_json() {
    let r = validate_hub("{");
    assert!(
        has_finding(&r, RULE_JSON, "hub.json"),
        "Befund {RULE_JSON:?} erwartet, bekommen {:?}",
        r.findings
    );
}

/// Die Prüfung ändert den Kandidaten nicht (kein globaler Zustand, keine Seiteneffekte).
#[test]
fn validate_hub_laesst_kandidat_unveraendert() {
    let h = read_hub7();
    let before = serde_json::to_string(&h).expect("json");
    validate(&h);
    assert_eq!(serde_json::to_string(&h).expect("json"), before, "Kandidat verändert");
    let embedded = serde_json::to_string(&hub()).expect("json");
    assert_ne!(
        embedded, before,
        "Fixture gleich der eingebetteten hub.json, Test sagt nichts"
    );
}

/// B-631/AC-03: Ein Material-Tor aus dem Material einer tieferen Stufe (Kupfer auf Hub-Stufe 2) ist ein Befund; die
/// eingebetteten Daten (Stein) sind gültig.
#[test]
fn site_data_material_tor_eigene_stufe() {
    let r = validate(&hub_copy());
    assert!(
        !has_finding(&r, RULE_GATE, ""),
        "eingebettete hub.json: {:?}",
        r.findings
    );
    let mut h = hub_copy();
    h.levels[1].steps[2].gate = Cost {
        copper: 20,
        ..Cost::default()
    };
    let r = validate(&h);
    assert!(
        has_finding(&r, RULE_GATE, "levels[1].steps[2].gate"),
        "Kupfer-Tor auf Hub-Stufe 2 ohne Befund: {:?}",
        r.findings
    );
}

/// Der Befund-Text bleibt wie in Go: `regel: platz: detail`.
#[test]
fn befund_text_regel_platz_detail() {
    let mut h = hub_copy();
    h.levels[1].steps[2].gate = Cost {
        copper: 20,
        ..Cost::default()
    };
    let r = validate(&h);
    let f = r.findings.iter().find(|f| f.rule == RULE_GATE).expect("Befund");
    assert_eq!(
        f.to_string(),
        "material-tor: levels[1].steps[2].gate: {Gold:0 Wood:0 Stone:0 Copper:20 Iron:0 Crystal:0} nicht aus dem Material der Stufe 2"
    );
}

/// Wie `site_data_hub_json_gueltig` und `site_data_sieben_stufen`, aber über `validate_hub` und `validate_hub_data` mit der
/// echten Standard-Welt (Go: `StandardWorld`); ihr Laden liest `story`.
#[test]
fn validate_hub_mit_standard_welt() {
    let raw = Source::Embedded.read("hub.json").expect("hub.json");
    let r = validate_hub(&raw);
    assert!(
        r.findings.is_empty() && r.notes.is_empty(),
        "data/hub.json: {:?}",
        r.findings
    );
    let r = validate_hub_data(&read_hub7());
    assert!(r.findings.is_empty(), "hub7.json: {:?}", r.findings);
}
