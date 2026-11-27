//! `plan_waves` (B-783, Go `waves.go`): Ebene aus den Abhängigkeiten, dann je Ebene in Rang-Reihenfolge (Rang, Prio,
//! Nummer) in die erste Welle, in der kein Mitglied denselben Bereich (Präfix) oder dieselbe Sperre hat und eine Lane
//! frei ist. Zwei Lanes: `zweit` (Second Worker, autonom) und `haupt` (Hauptrechner). Sessions in Arbeit stehen in
//! Welle 1 und belegen ihre Lane. Regeln: docs/arbeitsweise.md › Parallelität und Wellen.

use std::collections::HashSet;
use std::path::Path;

use serde::Deserialize;

use super::list::matches;
use super::store_resolve::session_parts;
use super::tables::first_of;
use super::waves_nodes::{Node, WaveData, load_wave_data};
use super::waves_spec::{area_overlap, shared_lock};

/// Second Worker: autonomer Zuarbeiter, führt seine Sessions selbst aus.
const LANE_SECOND: &str = "zweit";
/// Hauptrechner: Planung, Ralf-Queue und alles, was der Second Worker nicht übernimmt.
const LANE_MAIN: &str = "haupt";
/// Ohne Angabe nur die Welle, die jetzt startbar ist.
const DEFAULT_WAVES: i64 = 1;

/// Eingabe von [`waves`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WavesFilter {
    /// Zahl der gezeigten Wellen, ab Welle 1 (Standard 1)
    pub wave_count: i64,
    /// nur Knoten dieses Projekts zeigen, z. B. WZG (Lanes und Kollisionen gelten für alle)
    pub project: String,
    /// nur Knoten dieser Domäne zeigen (Lanes und Kollisionen gelten für alle)
    pub domain: String,
}

/// Die Arbeitswellen aus den lokalen Dateien.
pub fn waves(root: &Path, f: &WavesFilter) -> Result<String, String> {
    let count = if f.wave_count == 0 { DEFAULT_WAVES } else { f.wave_count };
    let Ok(count @ 1..) = usize::try_from(count) else {
        return Err(format!("wave_count {} ungültig: mindestens 1", f.wave_count));
    };
    let mut w = load_wave_data(root)?;
    let waves = assign(&mut w);
    Ok(waves_text(&w, &waves, count, f))
}

/// Verteilt die Knoten auf Wellen und Lanes; Welle 1 beginnt mit den Sessions in Arbeit. Je Welle die Knoten-Plätze.
fn assign(w: &mut WaveData) -> Vec<Vec<usize>> {
    let mut waves = vec![Vec::new()];
    let mut queue = Vec::new();
    for i in 0..w.nodes.len() {
        if w.nodes[i].running {
            let lane = free_lane(&w.nodes, &waves[0], &w.nodes[i]);
            let node = &mut w.nodes[i];
            (node.level, node.wave, node.lane) = (1, 1, lane);
            waves[0].push(i);
        } else if w.nodes[i].why.is_empty() && level(w, i, &mut HashSet::new()) > 0 {
            queue.push(i);
        }
    }
    // Ebene, Rang des Projekts, Prio und Reihenfolge der Planung (der Platz macht die Ordnung total).
    queue.sort_by_key(|&i| (w.nodes[i].level, w.nodes[i].rank, w.nodes[i].prio, i));
    for i in queue {
        place(w, &mut waves, i);
    }
    waves
}

/// 1 ohne offene Abhängigkeit, sonst eins mehr als die höchste Ebene der Abhängigkeiten; 0 = in keiner Welle (`why`
/// sagt, warum).
fn level(w: &mut WaveData, i: usize, seen: &mut HashSet<usize>) -> usize {
    if w.nodes[i].running || w.nodes[i].level > 0 {
        return w.nodes[i].level.max(1);
    }
    if !w.nodes[i].why.is_empty() {
        return 0;
    }
    if !seen.insert(i) {
        "Abhängigkeiten im Kreis".clone_into(&mut w.nodes[i].why);
        return 0;
    }
    let mut lvl = 1;
    for dep in dep_nodes(w, i) {
        let below = level(w, dep, seen);
        if below == 0 {
            if w.nodes[i].why.is_empty() {
                w.nodes[i].why = format!("wartet auf {} (nicht in einer Welle)", w.nodes[dep].id);
            }
            return 0;
        }
        lvl = lvl.max(below + 1);
    }
    if !w.nodes[i].why.is_empty() {
        return 0;
    }
    w.nodes[i].level = lvl;
    lvl
}

/// Die Knoten, nach denen `i` kommt; offene Abhängigkeiten ohne Knoten setzen `why`.
fn dep_nodes(w: &mut WaveData, i: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for id in w.nodes[i].deps.clone() {
        let target = w.dep_target(&id);
        if target.unknown {
            let node = &mut w.nodes[i];
            node.why = format!("unbekannte ID {id} in „Abhängig von“");
            node.fault = true;
            return Vec::new();
        }
        if !target.done && !target.wait.is_empty() {
            w.nodes[i].why = format!("wartet auf {}", target.wait);
            return Vec::new();
        }
        let own = &w.nodes[i].sprint;
        for d in target.nodes {
            let other_sprint = own.is_empty() || w.nodes[d].sprint != *own || session_parts(&id).is_some();
            if d != i && other_sprint && !out.contains(&d) {
                out.push(d);
            }
        }
    }
    out
}

/// Setzt `i` in die erste Welle ab der nach seinen Abhängigkeiten frühesten, mit freier Lane und ohne Kollision.
fn place(w: &mut WaveData, waves: &mut Vec<Vec<usize>>, i: usize) {
    let mut start = 1;
    for d in dep_nodes(w, i) {
        if w.nodes[d].wave + 1 > start {
            start = w.nodes[d].wave + 1;
            w.nodes[i].note = format!("nach {}", w.nodes[d].id);
        }
    }
    // Eine leere Welle nimmt jeden Knoten, die Suche endet also spätestens hinter der letzten.
    let mut at = start;
    loop {
        while waves.len() < at {
            waves.push(Vec::new());
        }
        match fit(&w.nodes, &waves[at - 1], &w.nodes[i]) {
            Ok(lane) => {
                (w.nodes[i].wave, w.nodes[i].lane) = (at, lane);
                waves[at - 1].push(i);
                return;
            }
            Err(why) if at == start => {
                let note = &mut w.nodes[i].note;
                *note = if note.is_empty() { why } else { format!("{note}; {why}") };
            }
            Err(_) => {}
        }
        at += 1;
    }
}

/// Die Lanes, die `n` in einer Welle nehmen darf. `Lane: zweit` gehört dem Second Worker, jeder andere Wert dem
/// Hauptrechner; Domäne SIM nur `zweit` (Sim-Sperre für den Hauptrechner, Ausnahme Second Worker).
fn lanes_for(n: &Node) -> &'static [&'static str] {
    if n.want_lane == LANE_SECOND || n.domain == "SIM" {
        &[LANE_SECOND]
    } else if n.want_lane.is_empty() {
        &[LANE_SECOND, LANE_MAIN]
    } else {
        &[LANE_MAIN]
    }
}

/// Die erste Lane von `n`, die in der Welle frei ist; leer, wenn keine frei ist.
fn free_lane(nodes: &[Node], wave: &[usize], n: &Node) -> &'static str {
    lanes_for(n)
        .iter()
        .copied()
        .find(|lane| !wave.iter().any(|&m| nodes[m].lane == *lane))
        .unwrap_or("")
}

/// Die Lane für `n` in der Welle, oder der Grund, warum `n` nicht hineinpasst.
fn fit(nodes: &[Node], wave: &[usize], n: &Node) -> Result<&'static str, String> {
    for &m in wave {
        if let Some(why) = collision(&nodes[m], n) {
            return Err(format!("kollidiert mit {}: {why}", nodes[m].id));
        }
    }
    let lane = free_lane(nodes, wave, n);
    if !lane.is_empty() {
        return Ok(lane);
    }
    match lanes_for(n) {
        [only] => Err(format!("Lane {only} belegt")),
        _ => Err("Lanes belegt".to_owned()),
    }
}

/// Derselbe Sprint (B-838: Sessions eines Sprints laufen nacheinander, ein Sprint-Branch), gleiche Sperre oder
/// überlappender Bereich (wie `clash` in `plan_next`).
fn collision(a: &Node, b: &Node) -> Option<String> {
    if !a.sprint.is_empty() && a.sprint == b.sprint {
        return Some(format!("Sprint {}", a.sprint));
    }
    if let Some(lock) = shared_lock(&a.locks, &b.locks).filter(|l| !l.is_empty()) {
        return Some(format!("Sperre {lock}"));
    }
    area_overlap(&a.area, &b.area).map(|pair| format!("Bereich {pair}"))
}

/// Kopf, die ersten `count` Wellen mit Lanes und Grund der Zurückstellung, dann die Knoten ohne Welle.
fn waves_text(w: &WaveData, waves: &[Vec<usize>], count: usize, f: &WavesFilter) -> String {
    let domain = f.domain.to_uppercase();
    let show = |n: &Node| {
        matches(&f.project, &n.project) && (f.domain.is_empty() || n.domain.to_uppercase().contains(&domain))
    };
    let running = waves.iter().flatten().filter(|&&i| w.nodes[i].running).count();
    let placed = waves.iter().map(Vec::len).sum::<usize>() - running;
    let mut title = format!(
        "Arbeitswellen: {running} in Arbeit, {placed} eingeplant in {} Wellen ({} gezeigt), {} in keiner Welle · \
         Lanes: {LANE_SECOND} (Second Worker), {LANE_MAIN} (Hauptrechner)",
        waves.len(),
        count.min(waves.len()),
        w.nodes.len() - placed - running
    );
    if !f.project.is_empty() || !f.domain.is_empty() {
        title.push_str(" · Anzeige gefiltert (Lanes und Kollisionen gelten für alle)");
    }
    let mut out = vec![title];
    for (i, wave) in waves.iter().take(count).enumerate() {
        let lines: Vec<String> = wave
            .iter()
            .map(|&n| &w.nodes[n])
            .filter(|n| show(n))
            .map(|n| node_line(n, i == 0))
            .collect();
        out.push(if i == 0 {
            format!("── Welle 1, jetzt ({} Knoten) ──", wave.len())
        } else {
            format!("── Welle {} ({} Knoten) ──", i + 1, wave.len())
        });
        out.extend(first_lines(lines, "keine"));
    }
    let rest: Vec<String> = w
        .nodes
        .iter()
        .filter(|n| n.wave == 0 && show(n))
        .map(blocked_line)
        .collect();
    out.push("── In keiner Welle ──".to_owned());
    out.extend(first_lines(rest, "keine"));
    out.join("\n")
}

pub(super) fn first_lines(lines: Vec<String>, empty: &str) -> Vec<String> {
    if lines.is_empty() {
        vec![empty.to_owned()]
    } else {
        lines
    }
}

/// `- ID · Art · Titel (Domäne, Projekt P) · Lane: x · Sperre y — Grund`
fn node_line(n: &Node, first: bool) -> String {
    let kind = if n.running { "in Arbeit" } else { n.kind };
    let mut s = format!(
        "- {} · {kind} · {} ({}, Projekt {}",
        n.id,
        n.title,
        first_of(&[&n.domain, "?"]),
        first_of(&[&n.project, "–"])
    );
    if !n.sprint.is_empty() {
        s = format!("{s}, Sprint {}", n.sprint);
    }
    s.push(')');
    if !n.lane.is_empty() {
        s = format!("{s} · Lane: {}", n.lane);
    }
    if !n.locks.is_empty() {
        s = format!("{s} · Sperre {}", n.locks.join(", "));
    }
    if !first && !n.note.is_empty() {
        s = format!("{s} — {}", n.note);
    }
    s
}

fn blocked_line(n: &Node) -> String {
    let prefix = if n.fault { "- Fehler: " } else { "- " };
    format!("{prefix}{} · {} · {} — {}", n.id, n.kind, n.title, n.why)
}
