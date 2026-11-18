//! Spielstand der Insel lesen und prüfen (Port von `ParseIslandSave` und `validateIslandSave` aus `island_save.go`, dazu
//! aus `island_save_v3.go` und `island_save_v5.go` der Teil, den der aktuelle Stand (Version 8 und 9) braucht:
//! `decodeIsland`, `parseIslandV3`, `validateSkills`, `validateProgress`). Die älteren Versionen (`parse_island_v2`,
//! `migrate_world`) stehen seit RU10.3 in `save_v3.rs` und `save_v8.rs`.
//!
//! Geprüft wird beim Lesen nur, was ohne Welt geht (Form, Tiefen, Skill-IDs gegen die Basis); Tier-Gating und Wirtschaft
//! prüft erst `from_island_save_in` gegen den Wertesatz der Welt. Ein Ergebnis ohne `from_island_save_in` gilt nicht als
//! geprüft (B-694).

use std::collections::BTreeSet;

use serde_json::Value;

use super::options::{IslandOptions, default_options, validate_options};
use super::save::{ISLAND_SAVE_VERSION, IslandPlayerSave, IslandSave, SaveTables, island_from_v1};
use super::save_v3::parse_island_v2;
use super::save_v8::migrate_world;
use crate::state::data::has_depth;
use crate::state::save::{SAVE_VERSION, parse_save};
use crate::state::save_economy::validate_economy;
use crate::state::{Fehler, WorldValues};

fn fail(s: String) -> Fehler {
    Fehler::Spielstand(s)
}

/// Go: `ParseIslandSave` liest einen Spielstand der Version 9 oder, überführt, der Version 8 (ohne geräumte Lager), 7 (Insel
/// als Index), 6 (ohne Wirtschaft), 5 (ohne Händler), 4 (ohne Bosse und Insel), 3 (ohne Tag und Phase), 2 oder 1 (Campaign).
/// Stände bis Version 7 kommen in die Standard-Welt (`migrate_world`). Tag und Phase wirken beim Laden nicht, die Zeit kommt
/// aus `time`.
pub fn parse_island_save(raw: &str, t: &SaveTables) -> Result<IslandSave, Fehler> {
    let head: Value = serde_json::from_str(raw).map_err(|e| fail(format!("spielstand: {e}")))?;
    let Some(obj) = head.as_object() else {
        return Err(fail(
            "spielstand: json: cannot unmarshal into Go value of type struct".to_string(),
        ));
    };
    let (version, island) = (head_int(obj, "version")?, head_int(obj, "island")?); // `island`: bis Version 7 der Index
    let s = match version {
        ISLAND_SAVE_VERSION | 8 => return parse_island_v3(raw, &["world", "islandId"], t),
        SAVE_VERSION => {
            let old = parse_save(raw, &t.data.biomes, t.defaults)?;
            island_from_v1(&old, default_options(&t.data.difficulty))
        }
        2 => parse_island_v2(raw, t)?,
        3..=7 => parse_island_v3(raw, &[], t)?,
        v => {
            return Err(fail(format!(
                "spielstand: Version {v}, erwartet {SAVE_VERSION}, 2, 3, 4, 5, 6, 7, 8 oder {ISLAND_SAVE_VERSION}"
            )));
        }
    };
    migrate_world(s, island, t.standard_world)
}

/// Go liest den Kopf in ein `int`: fehlend und `null` sind 0, jeder andere Wert als eine ganze Zahl ist ein Fehler.
fn head_int(obj: &serde_json::Map<String, Value>, key: &str) -> Result<i64, Fehler> {
    let kind = match obj.get(key) {
        None | Some(Value::Null) => return Ok(0),
        Some(Value::Number(n)) => match n.as_i64() {
            Some(i) => return Ok(i),
            None => format!("number {n}"),
        },
        Some(Value::String(_)) => "string".to_string(),
        Some(Value::Bool(_)) => "bool".to_string(),
        Some(Value::Array(_)) => "array".to_string(),
        Some(Value::Object(_)) => "object".to_string(),
    };
    Err(fail(format!(
        "spielstand: json: cannot unmarshal {kind} into Go struct field .{key} of type int"
    )))
}

/// Go lässt ein Feld mit `null` auf dem Standardwert: Einträge mit `null` in Objekten fallen weg (`default` füllt sie);
/// Arrays bleiben, ihre Objekte werden ebenso gelesen.
fn strip_nulls(v: &mut Value) {
    match v {
        Value::Object(m) => {
            m.retain(|_, x| !x.is_null());
            m.values_mut().for_each(strip_nulls);
        }
        Value::Array(a) => a.iter_mut().for_each(strip_nulls),
        _ => {}
    }
}

/// Go: `decodeIsland` prüft die Pflichtfelder (dazu `extra`, z. B. das Feld des Pools) und liest `raw`. Felder, die im Stand
/// fehlen, behalten den Wert von `defaults` (Go: der vorbelegte Zielwert; bei `options` auch je Feld).
pub fn decode_island(raw: &str, defaults: IslandOptions, extra: &[&str]) -> Result<IslandSave, Fehler> {
    let mut check: Value = serde_json::from_str(raw).map_err(|e| fail(format!("spielstand: {e}")))?;
    let obj = check.as_object_mut().ok_or_else(|| {
        fail("spielstand: json: cannot unmarshal into Go value of type map[string]json.RawMessage".to_string())
    })?;
    for f in ["campaignId", "seed", "time", "stock", "stages", "players"]
        .iter()
        .chain(extra)
    {
        if obj.get(*f).is_none_or(Value::is_null) {
            return Err(fail(format!("spielstand: Feld {f} fehlt")));
        }
    }
    let options = merge_options(obj.remove("options").as_ref(), defaults)?; // `null` ist für serde kein Objekt
    strip_nulls(&mut check);
    let mut s: IslandSave = serde_json::from_value(check).map_err(|e| fail(format!("spielstand: {e}")))?;
    s.options = options;
    Ok(s)
}

/// Go liest in den vorbelegten Wert: jedes Feld, das im JSON steht, überschreibt; `null` und fehlende Felder bleiben.
fn merge_options(v: Option<&Value>, mut o: IslandOptions) -> Result<IslandOptions, Fehler> {
    let Some(Value::Object(m)) = v else {
        return match v {
            None | Some(Value::Null) => Ok(o),
            Some(_) => Err(fail(
                "spielstand: json: cannot unmarshal into Go struct field of type sim.IslandOptions".to_string(),
            )),
        };
    };
    for (key, slot) in [
        ("grade", &mut o.grade),
        ("goal", &mut o.goal),
        ("defeat", &mut o.defeat),
    ] {
        match m.get(key) {
            None | Some(Value::Null) => {}
            Some(Value::String(s)) => *slot = s.clone(),
            Some(_) => {
                return Err(fail(format!(
                    "spielstand: json: cannot unmarshal into Go struct field IslandOptions.{key} of type string"
                )));
            }
        }
    }
    Ok(o)
}

/// Go: `parseIslandV3` liest Version 3 bis 8 und 9: Version 4 hat zusätzlich `day` und `phase`, Version 5 Bosse, Insel und
/// Zähler, Version 6 den Händler, Version 8 die Pflichtfelder `extra` (`world`, `islandId`); was fehlt, bleibt leer.
pub fn parse_island_v3(raw: &str, extra: &[&str], t: &SaveTables) -> Result<IslandSave, Fehler> {
    let mut fields = vec!["skillPool"];
    fields.extend_from_slice(extra);
    let mut s = decode_island(raw, default_options(&t.data.difficulty), &fields)?;
    s.version = ISLAND_SAVE_VERSION;
    validate_island_save(&s, None, t)?;
    Ok(s)
}

/// Go: `validateIslandSave` prüft einen Stand. Mit dem Wertesatz `v` seiner Welt (`from_island_save_in`) auch alles, was eine
/// Welt übersteuern kann (Truppenarten, Berufe, Rüstungsstufen, Tier-Gating, B-692); `v` `None` (beim Lesen, die Welt ist
/// noch nicht bekannt) lässt diese Prüfungen aus.
pub fn validate_island_save(s: &IslandSave, v: Option<&WorldValues>, t: &SaveTables) -> Result<(), Fehler> {
    validate_options(&s.options).map_err(fail)?;
    if s.stages.is_empty() {
        return Err(fail("spielstand: Stufen fehlen".to_string()));
    }
    let mut depths = BTreeSet::new();
    for h in &s.stages {
        if !has_depth(&t.data.biomes, h.depth) || depths.contains(&h.depth) {
            return Err(fail(format!("spielstand: unbekannte oder doppelte Tiefe {}", h.depth)));
        }
        depths.insert(h.depth);
        if let Some(v) = v {
            validate_economy(v, h)?;
        }
    }
    let mut seen = BTreeSet::new();
    for p in &s.players {
        if !depths.contains(&p.depth) || p.index < 0 || seen.contains(&p.index) {
            return Err(fail(format!("spielstand: Spieler {} ungültig", p.index)));
        }
        seen.insert(p.index);
    }
    validate_progress(s, t)?;
    validate_skills(s, v, t)
}

/// Go: `validateProgress` prüft die Felder der Version 5: bekannte Bosse ohne Doppel, Zähler nicht negativ. Welt und Insel
/// prüft `from_island_save_in` gegen die Welt (`island_in`).
pub fn validate_progress(s: &IslandSave, t: &SaveTables) -> Result<(), Fehler> {
    if s.gold_collected < 0 {
        return Err(fail(format!("spielstand: Münzzähler {} negativ", s.gold_collected)));
    }
    let b = &t.data.bosses;
    for (i, id) in s.defeated_bosses.iter().enumerate() {
        let known = b.bosses.iter().chain(&b.end_bosses).any(|x| &x.id == id);
        if !known || s.defeated_bosses[..i].contains(id) {
            return Err(fail(format!(
                "spielstand: besiegter Boss {id:?} unbekannt oder doppelt"
            )));
        }
    }
    if s.merchant_visits < 0 || s.merchant.as_ref().is_some_and(|m| m.hp <= 0.0 || m.hp > m.max_hp) {
        return Err(fail(format!(
            "spielstand: Händler-Besuche {} oder Händler-HP ungültig",
            s.merchant_visits
        )));
    }
    for p in &s.players {
        if p.skill_base < 0 || p.skill_base > p.skills.len() as i64 {
            return Err(fail(format!(
                "spielstand: Spieler {}: Skill-Basis {} bei {} Skills",
                p.index,
                p.skill_base,
                p.skills.len()
            )));
        }
    }
    Ok(())
}

/// Go: `validateSkills` prüft Pool und Verteilung: bekannte Skills, jeder einmal, nicht mehr als der Pool, Tier-Gating in
/// Lernreihenfolge und Slots nur mit gelernten aktiven Skills; Tier-Gating nur mit dem Wertesatz `v` der Welt.
pub fn validate_skills(s: &IslandSave, v: Option<&WorldValues>, t: &SaveTables) -> Result<(), Fehler> {
    if s.skill_pool < 0 {
        return Err(fail(format!("spielstand: Pool {} negativ", s.skill_pool)));
    }
    // Der Katalog der Prüfung: der der Welt, ohne Welt die Basis (die Skill-IDs sind in jeder Welt gleich).
    let catalog = v.unwrap_or(t.defaults);
    for p in &s.players {
        if p.skills.len() as i64 - p.skill_base > s.skill_pool {
            // mitgebrachte Skills zählen nicht (skill_base)
            return Err(fail(format!(
                "spielstand: Spieler {} hat {} Skills, Pool {}",
                p.index,
                p.skills.len(),
                s.skill_pool
            )));
        }
        validate_learned(catalog, v.is_some(), p)?;
        validate_slots(catalog, p)?;
    }
    Ok(())
}

fn skill_of<'a>(v: &'a WorldValues, id: &str) -> Option<&'a k3c_data::monarch::SkillData> {
    v.skills.index.get(id).and_then(|&i| v.skills.list.get(i))
}

/// Go: `tierNeed`: gelernte Skills der Linie, die Tier `tier` verlangt (`points` = monarch.json › tierPoints).
fn tier_need(points: &[i64], tier: i64) -> i64 {
    match usize::try_from(tier - 1).ok().and_then(|i| points.get(i)) {
        Some(&n) if tier >= 1 => n,
        _ => i64::MAX,
    }
}

fn validate_learned(v: &WorldValues, gating: bool, p: &IslandPlayerSave) -> Result<(), Fehler> {
    for (i, id) in p.skills.iter().enumerate() {
        let Some(sk) = skill_of(v, id) else {
            return Err(fail(format!("spielstand: Spieler {}: Skill {id:?} unbekannt", p.index)));
        };
        if p.skills[..i].contains(id) {
            return Err(fail(format!("spielstand: Spieler {}: Skill {id:?} doppelt", p.index)));
        }
        if gating {
            // Go: `lineCount(v, &Player{Skills: p.Skills[:i]}, sk.Line)`
            let lines = p.skills[..i]
                .iter()
                .filter(|l| skill_of(v, l).is_some_and(|x| x.line == sk.line))
                .count() as i64;
            if lines < tier_need(&v.monarch.tier_points, sk.tier) {
                return Err(fail(format!(
                    "spielstand: Spieler {}: Skill {id:?} verletzt das Tier-Gating",
                    p.index
                )));
            }
        }
    }
    Ok(())
}

fn validate_slots(v: &WorldValues, p: &IslandPlayerSave) -> Result<(), Fehler> {
    if p.slots.len() > 4 {
        return Err(fail(format!(
            "spielstand: Spieler {} hat {} Slots",
            p.index,
            p.slots.len()
        )));
    }
    for (i, id) in p.slots.iter().enumerate() {
        if id.is_empty() {
            continue;
        }
        let ok = skill_of(v, id).is_some_and(|sk| sk.kind == "active")
            && p.skills.contains(id)
            && !p.slots[..i].contains(id);
        if !ok {
            return Err(fail(format!(
                "spielstand: Spieler {}: Slot {} ({id:?}) passt nicht zu den gelernten aktiven Skills",
                p.index,
                i + 1
            )));
        }
    }
    Ok(())
}
