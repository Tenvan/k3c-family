//! Port von `native/k3c-game/devtools/values.go`.
//!
//! Seite Werte des Debug-Overlays (F10, B-889): Zeilen aus dem Frame und der Welt der Zelle, ohne eigene Rechnung über
//! die Sim hinaus (Restzeit aus `seconds_to_night`, Rechenzeit aus `Frame::step_ms`). Reine Zusammenstellung, getestet.

use k3c_sim::state::{Enemy, Site, Troop, World};
use k3c_sim::world::cycle::seconds_to_night;

use crate::view::cell::Cell;
use crate::view::frame::{Frame, Seat};

/// SELECT_RANGE ist die größte Entfernung (Units) eines auswählbaren Objekts von der Stelle.
pub const SELECT_RANGE: f64 = 4.0;

/// phase_name übersetzt die Tageszeit ins Deutsche (unbekannte bleiben stehen).
fn phase_name(p: &str) -> &str {
    match p {
        "day" => "Tag",
        "dusk" => "Abend",
        "night" => "Nacht",
        "dawn" => "Morgen",
        other => other,
    }
}

/// value_lines hängt die Zeilen der Seite Werte für die Zelle `c` an `dst`.
#[allow(clippy::cast_possible_truncation)]
pub fn value_lines(dst: &mut Vec<String>, f: &Frame, c: &Cell) {
    let Some(w) = f.world(c) else {
        dst.push("Keine Welt".to_string());
        return;
    };
    let to_night = seconds_to_night(w) as i64;
    dst.push(format!(
        "Tag {} · {} · Welle {} · Nacht in {}:{:02}",
        w.cycle.day,
        phase_name(&w.cycle.phase),
        w.wave,
        to_night / 60,
        to_night % 60
    ));
    for s in &f.seats {
        if let Some(line) = player_line(f, s) {
            dst.push(line);
        }
    }
    dst.push(format!(
        "Truppen {} · Gegner {} · Bauten {} · Münzen am Boden {}",
        w.troops.len(),
        w.enemies.len(),
        built(w),
        w.coins.len()
    ));
    dst.push(format!(
        "Takt {} · {:.0}/s · {:.2} ms · Zeitraffer {}× · Seed {}",
        f.tick,
        f.dev.tps,
        f.step_ms,
        f.timescale.max(1),
        w.seed
    ));
    let (x, how) = pick_x(f, c);
    select_at(dst, w, x, how);
}

/// player_line ist die Zeile des Spielers `s`: Gold, Vorrat seiner Stufe, Position, Chunk, Stufe und Tiefe.
pub fn player_line(f: &Frame, s: &Seat) -> Option<String> {
    let st = f.stage(s.stage)?;
    let m = f.monarch(s.player)?;
    let w = st.world.as_deref()?;
    let k = &w.stock;
    Some(format!(
        "Spieler {}: Gold {} · Holz {} Stein {} Kupfer {} Eisen {} Kristall {} · x {:.1} · Chunk {} · Stufe {} (Tiefe {})",
        s.player + 1,
        m.gold,
        k.wood,
        k.stone,
        k.copper,
        k.iron,
        k.crystal,
        m.x,
        chunk_at(w, m.x),
        st.index,
        st.depth
    ))
}

/// chunk_at ist der Index des Chunks an Welt-x `x` (-1 ohne Chunks).
pub fn chunk_at(w: &World, x: f64) -> i64 {
    let mut idx = -1;
    for (i, c) in w.level.chunks.iter().enumerate() {
        if c.start_units <= x {
            idx = i64::try_from(i).unwrap_or(i64::MAX);
        }
    }
    idx
}

/// built zählt die fertigen Bauten.
pub fn built(w: &World) -> usize {
    w.sites.iter().filter(|s| s.state == "built").count()
}

/// pick_x ist die Stelle der Auswahl: die Maus-Wahl in dieser Zelle, sonst der Monarch der Zelle.
pub fn pick_x(f: &Frame, c: &Cell) -> (f64, &'static str) {
    let p = f.dev.pick;
    if p.picked && p.player == c.player {
        return (p.x, "Klick");
    }
    if let Some(m) = f.monarch(c.player) {
        return (m.x, "am Monarchen");
    }
    (f64::NAN, "")
}

/// Pick ist das nächstgelegene Objekt der Auswahl.
enum Picked<'a> {
    Nothing,
    Coin(i64, f64),
    Site(&'a Site),
    Monarch(&'a k3c_sim::state::Player),
    Troop(&'a Troop),
    Enemy(&'a Enemy),
}

/// select_at hängt die Beschreibung des Objekts nächst `x` an `dst` (höchstens `SELECT_RANGE` entfernt; Gegner,
/// Truppen, Bauten, Monarchen, Münzen; bei Gleichstand gewinnt das später geprüfte).
pub fn select_at<'a>(dst: &mut Vec<String>, w: &'a World, x: f64, how: &str) {
    let mut best = SELECT_RANGE;
    let mut pick = Picked::Nothing;
    let consider = |ox: f64, p: Picked<'a>, best: &mut f64, pick: &mut Picked<'a>| {
        let dist = (ox - x).abs();
        if dist <= *best {
            *best = dist;
            *pick = p;
        }
    };
    for e in &w.coins {
        consider(e.x, Picked::Coin(e.id, e.x), &mut best, &mut pick);
    }
    for s in &w.sites {
        consider(s.x, Picked::Site(s), &mut best, &mut pick);
    }
    for p in &w.players {
        consider(p.x, Picked::Monarch(p), &mut best, &mut pick);
    }
    for t in &w.troops {
        consider(t.x, Picked::Troop(t), &mut best, &mut pick);
    }
    for e in &w.enemies {
        consider(e.x, Picked::Enemy(e), &mut best, &mut pick);
    }
    let start = dst.len();
    match pick {
        Picked::Nothing => dst.push("Auswahl: –".to_string()),
        Picked::Coin(id, cx) => dst.push(format!("Münze #{id} bei x {cx:.1}")),
        Picked::Site(s) => site_lines(dst, s),
        Picked::Monarch(p) => dst.push(format!(
            "Monarch {} (#{}) · Gold {} · Krone {}",
            p.index + 1,
            p.id,
            p.gold,
            p.crown.is_none()
        )),
        Picked::Troop(t) => troop_lines(dst, t),
        Picked::Enemy(e) => enemy_lines(dst, e),
    }
    if let Some(first) = dst.get_mut(start).filter(|l| !how.is_empty() && !l.ends_with('–')) {
        *first = format!("Auswahl ({how}): {first}");
    }
}

fn site_lines(dst: &mut Vec<String>, s: &Site) {
    dst.push(format!("Bau #{} {}", s.id, s.kind));
    dst.push(format!(
        "Zustand {} · Fortschritt {:.0} % · LP {:.0}/{:.0} · bezahlt {}",
        s.state,
        s.build_progress * 100.0,
        s.hp,
        s.max_hp,
        s.paid_gold
    ));
}

fn troop_lines(dst: &mut Vec<String>, t: &Troop) {
    let mut job = "frei".to_string();
    if let Some(j) = &t.job {
        job.clone_from(&j.r#type);
        if !j.resource.is_empty() {
            job.push(' ');
            job.push_str(&j.resource);
        }
    }
    let mut kind = t.kind.clone();
    if !t.profession.is_empty() {
        kind = format!("{kind} ({})", t.profession);
    }
    dst.push(format!("Truppe #{} {} · LP {:.0}/{:.0}", t.id, kind, t.hp, t.max_hp));
    dst.push(format!(
        "Ziel x {:.1} · Anker x {:.1} · KI: {}",
        t.target_x, t.anchor_x, job
    ));
}

fn enemy_lines(dst: &mut Vec<String>, e: &Enemy) {
    dst.push(format!("Gegner #{} {} · LP {:.0}/{:.0}", e.id, e.kind, e.hp, e.max_hp));
    dst.push(format!(
        "Zustand {} · Ziel {} · KI: {}",
        enemy_state(e),
        enemy_target(e),
        enemy_decision(e)
    ));
}

fn enemy_state(e: &Enemy) -> &'static str {
    if e.fleeing {
        "flieht"
    } else if e.stun > 0.0 {
        "betäubt"
    } else if e.slow > 0.0 && e.slow_for > 0.0 {
        "verlangsamt"
    } else {
        "aktiv"
    }
}

fn enemy_target(e: &Enemy) -> String {
    if e.taunt_for > 0.0 {
        return format!("Spieler #{} (Spott)", e.taunt_id);
    }
    if e.fleeing {
        return format!("Portal x {:.1}", e.home_x);
    }
    "Burg".to_string()
}

fn enemy_decision(e: &Enemy) -> String {
    if e.fleeing {
        "zurück zum Portal".to_string()
    } else if e.stun > 0.0 {
        format!("wartet {:.1} s", e.stun)
    } else if e.cooldown > 0.0 {
        format!("nächster Angriff in {:.1} s", e.cooldown)
    } else {
        "angreifen bzw. vorrücken".to_string()
    }
}
