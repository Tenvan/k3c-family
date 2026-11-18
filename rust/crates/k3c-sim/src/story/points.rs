//! Wendepunkte der Geschichte (Port von `engine/sim/story_points.go`, B-965/R6, docs/rules/welten.md § 6.2): reine Funktionen,
//! die zu einem Moment der Kampagne die Zeilen nennen. Ohne Zustand, ohne Zufall, ohne neue Ereignisse; das Spiel ruft sie bei
//! den vorhandenen Ereignissen (`islandSwitch`, `bossSpawned`, `bossDefeated`) und blendet das Ergebnis ein. Das Laden eines
//! Spielstands ruft sie nicht. Die Zeilen gelten für alle Spieler der Insel gleich. Ergebnis: alle Zeilen in Reihenfolge
//! (Einträge hintereinander), leer = keine Zeile.
//!
//! Gos Methoden an `WorldDef` und `Island` sind freie Funktionen: `story_*(w: &WorldDef, …)` und `island_story_*(isl: &Island)`.

use crate::island::{Island, WorldDef};

/// Go: `StoryStart`: Neue Kampagne, `start` der Welt, danach `arrival` der Startinsel (erste Insel der Welt).
pub fn story_start(w: &WorldDef) -> Vec<String> {
    match w.islands.first() {
        None => lines(&w.story.start).to_vec(),
        Some(d) => concat(&[lines(&w.story.start), lines(&d.story.arrival)]),
    }
}

/// Go: `StoryArrival`: Ankunft nach einem Inselwechsel, `arrival` der Insel `island_id`.
pub fn story_arrival(w: &WorldDef, island_id: &str) -> Vec<String> {
    island_index(w, island_id).map_or_else(Vec::new, |i| lines(&w.islands[i].story.arrival).to_vec())
}

/// Go: `StoryBoss`: Endboss der Insel `island_id` ausgelöst (`bossSpawned` des Endbosses), `boss` der Insel. Miniboss: keine
/// Zeile.
pub fn story_boss(w: &WorldDef, island_id: &str) -> Vec<String> {
    island_index(w, island_id).map_or_else(Vec::new, |i| lines(&w.islands[i].story.boss).to_vec())
}

/// Go: `StoryDefeated`: Endboss der Insel `island_id` besiegt (`bossDefeated` des Endbosses). `completed` sind die IDs der
/// früher abgeschlossenen Inseln (`Island::completed`); die Insel `island_id` zählt schon mit. Ergebnis: `done` der Insel;
/// schließt der Sieg die Etappe ab, danach `etappen[<Nr>]`; ist es die letzte Insel der Welt, statt dessen `end`.
pub fn story_defeated(w: &WorldDef, island_id: &str, completed: &[String]) -> Vec<String> {
    let Some(cur) = island_index(w, island_id) else {
        return Vec::new();
    };
    let done = done_fn(w, cur, completed);
    let out = lines(&w.islands[cur].story.done);
    if w.targets_from(cur as i64, &done).is_empty() {
        return concat(&[out, lines(&w.story.end)]);
    }
    if etappe_done(w, cur, &done) {
        let key = w.etappe_of(cur as i64).to_string();
        let etappe = w.story.etappen.get(&key).map_or(&[][..], |v| v.as_slice());
        return concat(&[out, etappe]);
    }
    out.to_vec()
}

/// Go: `StoryVictory`: Sieg einer Variante ohne Endboss (z. B. Tage überleben) auf der Insel `island_id`. Nur auf der letzten
/// Insel der Welt gilt es: `done` der Insel, danach `end`; auf früheren Inseln bleibt es beim Endboss, dann keine Zeile.
pub fn story_victory(w: &WorldDef, island_id: &str, completed: &[String]) -> Vec<String> {
    let Some(cur) = island_index(w, island_id) else {
        return Vec::new();
    };
    let done = done_fn(w, cur, completed);
    if !w.targets_from(cur as i64, &done).is_empty() {
        return Vec::new();
    }
    concat(&[lines(&w.islands[cur].story.done), lines(&w.story.end)])
}

/// Go: `Island.StoryArrival`: Zeilen der aktuellen Insel bei der Ankunft (Welt der Insel, ID der Insel).
pub fn island_story_arrival(isl: &Island) -> Vec<String> {
    story_arrival(&isl.world, &isl.island_id)
}

/// Go: `Island.StoryBoss`: Zeilen der aktuellen Insel, wenn ihr Endboss ausgelöst wird.
pub fn island_story_boss(isl: &Island) -> Vec<String> {
    story_boss(&isl.world, &isl.island_id)
}

/// Go: `Island.StoryDefeated`: Zeilen der aktuellen Insel nach dem Sieg über ihren Endboss (nach den früher
/// abgeschlossenen Inseln).
pub fn island_story_defeated(isl: &Island) -> Vec<String> {
    story_defeated(&isl.world, &isl.island_id, &isl.completed)
}

/// Go: `Island.StoryVictory`: Zeilen der aktuellen Insel beim Sieg einer Variante ohne Endboss.
pub fn island_story_victory(isl: &Island) -> Vec<String> {
    story_victory(&isl.world, &isl.island_id, &isl.completed)
}

/// Ein fehlender Eintrag (`nil`) hat keine Zeilen.
fn lines(entry: &Option<Vec<String>>) -> &[String] {
    entry.as_deref().unwrap_or(&[])
}

/// Go: `islandIndex`: der Listenplatz der Insel `id`, `None` ohne Treffer.
fn island_index(w: &WorldDef, id: &str) -> Option<usize> {
    w.islands.iter().position(|d| d.id == id)
}

/// Gos `done`-Funktion: die Insel `cur` und alle in `completed` genannten gelten als abgeschlossen.
fn done_fn<'a>(w: &'a WorldDef, cur: usize, completed: &'a [String]) -> impl Fn(i64) -> bool + 'a {
    move |i| i as usize == cur || w.islands.get(i as usize).is_some_and(|d| completed.contains(&d.id))
}

/// Go: `etappeDone`: Alle Inseln der Etappe der Insel `cur` gelten als abgeschlossen.
fn etappe_done(w: &WorldDef, cur: usize, done: &impl Fn(i64) -> bool) -> bool {
    let e = w.etappe_of(cur as i64);
    (0..w.islands.len() as i64).all(|i| w.etappe_of(i) != e || done(i))
}

/// Go: `concat`: hängt Einträge zu einer frischen Liste zusammen.
fn concat(parts: &[&[String]]) -> Vec<String> {
    parts.iter().flat_map(|p| p.iter().cloned()).collect()
}
