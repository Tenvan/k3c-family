//! Prüfung eines fertigen Levels (Port von `engine/level/validate.go`). Die Meldungen sind wörtlich wie in Go: Tests und
//! das Spiel vergleichen sie.

use crate::biome::Biome;
use crate::level::EVENT_KINDS;
use crate::nature::with_nature;
use crate::planned_gen::{Options, fixed_objects};
use crate::rules::Rules;
use crate::types::{Layout, PlannedChunk};

/// Go: `strconv.FormatFloat(x, 'f', -1, 64)`: kürzeste Darstellung ohne Exponent, wie Rusts `{}`.
fn fmt_f(x: f64) -> String {
    format!("{x}")
}

/// Go: `Validate`, prüft die Spielbarkeits-Regeln wie `validateLevel` in TS. Leere Liste = Level ist gültig.
pub fn validate(l: &Layout, b: &Biome, rules: &Rules) -> Vec<String> {
    validate_with(l, b, &Options::default(), rules)
}

/// Go: `ValidateWith`, wie `validate`; mit eingestellter Chunk-Anzahl (`o.chunk_count`) gilt die Länge gegen die Anzahl
/// statt gegen lengthUnits (geplante-chunks.md § 7).
pub fn validate_with(l: &Layout, orig: &Biome, o: &Options, rules: &Rules) -> Vec<String> {
    let vein_errs = validate_veins(l, orig, o);
    // Ressourcen und Chunk-Gewichte nach den Natur-Werten der Stufe (B-891)
    let mut b = with_nature(orig, &l.nature);
    b.lair = o.has_lair(orig);
    b.veins.count = 0; // Adern prüft validate_veins
    let mut errs = validate_length(l, &b, o.chunk_count);
    errs.extend(vein_errs);
    // Portale, Truhen und Camps zählen als Objekte: feste Objekte geplanter Chunks stehen auch in anderen Chunk-Arten.
    if count_kind(l, "exit") != 1 {
        errs.push("Genau ein Tiefen-Eingang erwartet".into());
    }
    let n = count_entities(l, "portal");
    if n != b.portals.count {
        errs.push(format!("{} Portale erwartet, {n} gefunden", b.portals.count));
    }
    errs.extend(validate_chunks(l, &b, rules));
    errs.extend(validate_entities(l, &b));
    errs
}

/// Go: `validateLength`, prüft die Länge: mit eingestellter Anzahl genau so viele Chunks, sonst innerhalb von
/// lengthUnits.
fn validate_length(l: &Layout, b: &Biome, count: i64) -> Vec<String> {
    if count > 0 {
        if l.chunks.len() as i64 != count || l.width_units != count as f64 * l.chunk_width_units {
            return vec![format!("Level hat {} Chunks, erwartet {count}", l.chunks.len())];
        }
        return Vec::new();
    }
    let mut errs = Vec::new();
    // Toleranz: Runden auf ganze Chunks (±0.5) plus Auffüllen auf gerade Chunk-Anzahl (+1).
    let tolerance = l.chunk_width_units * 1.5;
    if l.width_units < b.length_units.min as f64 - tolerance {
        errs.push("Level zu kurz".into());
    }
    if l.width_units > b.length_units.max as f64 + tolerance {
        errs.push("Level zu lang".into());
    }
    errs
}

/// Go: `validateChunks`, prüft Lage von Ausgang und Portalen sowie die Zahl der Ereignis-Chunks.
fn validate_chunks(l: &Layout, b: &Biome, rules: &Rules) -> Vec<String> {
    let mut errs = Vec::new();
    let expected_exit = if b.exit_side == "right" {
        l.chunks.len() as i64 - 1
    } else {
        0
    };
    if let Some(c) = l.chunks.iter().find(|c| c.kind == "exit")
        && c.index != expected_exit
    {
        errs.push("Tiefen-Eingang nicht am Rand".into());
    }
    for e in &l.entities {
        let dist = (e.x - l.hub_center_units).abs();
        if e.kind == "portal" && dist < b.portals.min_distance_from_hub_units {
            errs.push(format!("Portal zu nah am Hub ({} Units)", fmt_f(dist)));
        }
    }
    errs.extend(validate_portals_outside_lines(l, rules));
    errs.extend(validate_lair(l, b));
    for kind in EVENT_KINDS {
        let (min, max) = b.event_chunks.get(kind).map_or((0, 0), |r| (r.min, r.max));
        let n = count_entities(l, kind);
        if n < min || n > max {
            errs.push(format!("{kind}: {n} (erwartet {min}-{max})"));
        }
    }
    errs
}

/// Go: `validatePortalsOutsideLines`, meldet Portale, die nicht außerhalb der Verteidigungszone liegen
/// (`sitePlacement.outsideUnits`, Q49, Q57).
fn validate_portals_outside_lines(l: &Layout, rules: &Rules) -> Vec<String> {
    let mut errs = Vec::new();
    for e in &l.entities {
        let dx = (e.x - l.hub_center_units).abs();
        if e.kind == "portal" && dx <= rules.site_placement.outside_units {
            errs.push(format!("Portal innerhalb der Verteidigungszone ({} Units)", fmt_f(dx)));
        }
    }
    errs
}

/// Go: `validateLair`: Biome mit `lair` haben genau einen Bau, an der inneren Kante des Ausgangs-Chunks und mit dem
/// Hub-Abstand eines Portals; alle anderen keinen.
fn validate_lair(l: &Layout, b: &Biome) -> Vec<String> {
    let want = i64::from(b.lair);
    let n = count_entities(l, "lair");
    if n != want {
        return vec![format!("{want} Bau erwartet, {n} gefunden")];
    }
    let edge = if b.exit_side == "right" {
        l.width_units - l.chunk_width_units
    } else {
        l.chunk_width_units
    };
    for e in &l.entities {
        let near_hub = (e.x - l.hub_center_units).abs() < b.portals.min_distance_from_hub_units;
        if e.kind == "lair" && (e.x != edge || near_hub) {
            return vec![format!("Bau nicht vor dem Ausgang ({} Units)", fmt_f(e.x))];
        }
    }
    Vec::new()
}

fn count_kind(l: &Layout, kind: &str) -> i64 {
    l.chunks.iter().filter(|c| c.kind == kind).count() as i64
}

/// Go: `countEntities`.
pub(crate) fn count_entities(l: &Layout, kind: &str) -> i64 {
    l.entities.iter().filter(|e| e.kind == kind).count() as i64
}

fn validate_entities(l: &Layout, b: &Biome) -> Vec<String> {
    let mut errs = Vec::new();
    let (mut skill_points, mut castles, mut outside) = (0, 0, false);
    for e in &l.entities {
        match e.kind.as_str() {
            "skillPoint" => skill_points += 1,
            "castle" => castles += 1,
            _ => {}
        }
        outside = outside || e.x < 0.0 || e.x > l.width_units;
    }
    if skill_points < b.skill_points.min || skill_points > b.skill_points.max {
        errs.push(format!("skillPoints: {skill_points}"));
    }
    if castles != 1 {
        errs.push("Genau eine Burg erwartet".into());
    }
    if outside {
        errs.push("Entity außerhalb des Levels".into());
    }
    if b.veins.count > 0 && count_entities(l, &b.veins.kind) != b.veins.count {
        errs.push(format!("falsche Anzahl Adern: {}", count_entities(l, &b.veins.kind)));
    }
    errs
}

/// Go: `fixedVeins`, zählt die festen Adern der geplanten Chunks.
fn fixed_veins(b: &Biome, plan: &[PlannedChunk], cw: f64) -> i64 {
    let mut n = 0;
    for c in plan {
        n += fixed_objects(c, cw)
            .iter()
            .flat_map(|l| l.iter())
            .filter(|o| o.kind == b.veins.kind)
            .count() as i64;
    }
    n
}

/// Go: `validateVeins`, prüft die Zahl der Adern (B-114, B-891): höchstens das Ziel (Natur-Wert, mindestens die festen
/// Adern geplanter Chunks) und mindestens das Ziel oder die Zahl des Bioms, je nachdem was kleiner ist. Mehr Adern als
/// im Biom kann der Generator nur setzen, solange freie Chunks bleiben; die Zahl des Bioms hat immer Platz
/// (`check_room`).
fn validate_veins(l: &Layout, b: &Biome, o: &Options) -> Vec<String> {
    if b.veins.count == 0 {
        return Vec::new();
    }
    let fixed = fixed_veins(b, &o.planned, l.chunk_width_units);
    let target = i64::max(with_nature(b, &l.nature).veins.count, fixed);
    let floor = i64::min(target, i64::max(b.veins.count, fixed));
    let n = count_entities(l, &b.veins.kind);
    if n > target || n < floor {
        return vec![format!("falsche Anzahl Adern: {n}")];
    }
    Vec::new()
}

#[cfg(test)]
#[path = "validate_tests.rs"]
mod validate_tests;
