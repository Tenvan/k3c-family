//! Port von `native/k3c-game/hud/icons_test.go`.

use std::collections::BTreeMap;

use serde_json::Value;

use super::icons::*;
use super::purse::PURSE_ASSET;
use super::testutil::hud;
use super::values::{RESOURCE_ORDER, StockKey};
use k3c_data::hub::Stock;

/// paper_groups liefert je Asset-ID aus data/paper.json die Gruppe (alle Ebenen durchsucht).
fn paper_groups() -> BTreeMap<String, String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/paper.json");
    let raw = std::fs::read_to_string(path).expect("data/paper.json");
    let v: Value = serde_json::from_str(&raw).expect("paper.json ist JSON");
    fn walk(x: &Value, out: &mut BTreeMap<String, String>) {
        let Some(m) = x.as_object() else {
            return;
        };
        for (k, sub) in m {
            if let Some(g) = sub.get("gruppe").and_then(Value::as_str) {
                out.insert(k.clone(), g.to_string());
            }
            walk(sub, out);
        }
    }
    let mut out = BTreeMap::new();
    walk(&v, &mut out);
    out
}

/// AC-01 (B-890): Jeder Rohstoff der Sim (Felder von `Stock`) hat ein Icon; jedes Icon des HUD steht in
/// data/paper.json in der Gruppe icons und wird als Asset gemeldet.
#[test]
fn every_resource_has_icon() {
    // Alle fünf Felder von Stock, mit Wert, damit auch `omitempty`-Felder in der JSON-Sicht stehen.
    let Stock {
        wood,
        stone,
        copper,
        iron,
        crystal,
    } = Stock::default();
    let all = Stock {
        wood: wood + 1,
        stone: stone + 1,
        copper: copper + 1,
        iron: iron + 1,
        crystal: crystal + 1,
    };
    let json = serde_json::to_value(&all).expect("Stock als JSON");
    let tags: Vec<&String> = json.as_object().expect("Objekt").keys().collect();
    assert_eq!(
        tags.len(),
        RESOURCE_ORDER.len(),
        "Stock hat {} Rohstoffe, HUD kennt {}",
        tags.len(),
        RESOURCE_ORDER.len()
    );
    for r in RESOURCE_ORDER {
        assert!(tags.iter().any(|t| *t == r), "Rohstoff {r} fehlt in Stock");
        assert!(!resource_icon(r).is_empty(), "Rohstoff {r}: kein Icon");
    }
    for (i, (r, _)) in RESOURCE_ICONS.iter().enumerate() {
        assert_eq!(RESOURCE_ORDER[i], *r, "Reihenfolge");
    }
    let groups = paper_groups();
    let mut icons: Vec<String> = vec![GOLD_ICON.to_string(), PURSE_ASSET.to_string()];
    icons.extend(RESOURCE_ICONS.iter().map(|(_, a)| (*a).to_string()));
    icons.extend(ITEM_ICONS.iter().map(|(_, a)| (*a).to_string())); // EQ1.2: Gegenstände im Regal
    let assets = hud().hud_assets();
    for a in &icons {
        let (group, id) = a.split_once('/').unwrap_or((a, ""));
        let g = groups.get(id).map_or("", String::as_str);
        assert!(
            g == group && assets.contains(a),
            "{a}: Gruppe {g:?} in paper.json, gemeldet {}",
            assets.contains(a)
        );
    }
    assert!(assets.is_sorted() && assets.len() == icons.len(), "Assets {assets:?}");
}

fn key(primary: &str, stock: [i64; 5], max: i64, points: i64) -> StockKey {
    StockKey {
        lang: "de".to_string(),
        primary: primary.to_string(),
        stock,
        max,
        points,
    }
}

fn part(res: &str, text: &str, full: bool) -> StockPart {
    StockPart {
        res: res.to_string(),
        text: text.to_string(),
        full,
    }
}

/// AC-02 (B-890): Die Vorrats-Anzeige hat je Rohstoff mit Vorrat (oder dem Primärrohstoff) Icon und Zahl statt Namen,
/// mit Maximum „n/max“ und voll markiert; Skill-Punkte stehen als Text dahinter.
#[test]
fn stock_parts_zeigt_icon_und_zahl() {
    let mut got = Vec::new();
    stock_parts(&mut got, &key("stone", [12, 0, 0, 3, 0], 12, 2));
    assert!(
        got.len() == 4
            && got[0] == part("wood", "12/12", true)
            && got[1].res == "stone"
            && got[1].text == "0/12"
            && got[2] == part("iron", "3/12", false)
            && got[3].res.is_empty()
            && !got[3].text.is_empty(),
        "Teile {got:?}"
    );
    stock_parts(&mut got, &key("stone", [12, 0, 0, 3, 0], 0, 0));
    assert!(
        got.len() == 3 && got[0].text == "12" && !got[0].full,
        "ohne Maximum: {got:?}"
    );
    for p in &got {
        assert!(
            !p.text.contains("Holz") && !p.text.contains("Stein"),
            "Name statt Icon: {p:?}"
        );
    }
}
