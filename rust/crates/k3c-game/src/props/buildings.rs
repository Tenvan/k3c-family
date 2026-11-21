//! Welche Grafik ein Gebäude zeigt (Spiegel von src/scenes/render/buildingSprites.ts, hubGrowth.ts, backWallRules.ts,
//! damageRules.ts, scaffoldView.ts, buildSignView.ts). Reine Auswahl ohne Zeichenfläche; ein leerer `Look` = keine
//! Grafik, dann zeichnet die Platzhalter-Form.

use k3c_sim::state::Site;

use super::data::building_level_count;

/// Gruppen der Papier-Assets (data/paper.json › gebaeude, objekte; Münze in icons).
pub const BUILDING_GROUP: &str = "gebaeude-paper/";
pub const OBJECT_GROUP: &str = "objekte/";

/// `Look` ist eine Grafik: Asset `gruppe/id` und Animation im Manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Look {
    pub asset: &'static str,
    pub anim: &'static str,
}

impl Look {
    pub const fn new(asset: &'static str, anim: &'static str) -> Look {
        Look { asset, anim }
    }

    pub fn ok(&self) -> bool {
        !self.asset.is_empty()
    }
}

/// `STAGE_ANIMS`: Animation der Gebäude-Stufe n (1 = `stand`, n > 1 = `stand.s<n>`), fest, damit nichts je Bild entsteht.
const STAGE_ANIMS: [&str; 6] = ["stand", "stand", "stand.s2", "stand.s3", "stand.s4", "stand.s5"];

/// `stage_anim` liefert die Animation der Stufe n (begrenzt auf 1 … 5).
pub fn stage_anim(n: i64) -> &'static str {
    STAGE_ANIMS[n.clamp(1, STAGE_ANIMS.len() as i64 - 1) as usize]
}

/// `HUB_BUILDINGS`: Hub-Gebäude → Papier-Asset; ohne eigene `levels` in den Daten folgen sie der Hub-Stufe (GEB9).
/// Asset-Pfade stehen fertig darin, damit je Bild kein String entsteht.
pub const HUB_BUILDINGS: [(&str, &str); 9] = [
    ("workshop", "gebaeude-paper/werkstatt"),
    ("bowyer", "gebaeude-paper/bogner"),
    ("armory", "gebaeude-paper/ruestkammer"),
    ("storage", "gebaeude-paper/lager"),
    ("nursery", "gebaeude-paper/farm"),
    ("barracks", "gebaeude-paper/kaserne"),
    // B-1160: Taverne, Heiler und Schmiede hatten Assets, aber keine Zuordnung und standen als farbige Rechtecke da.
    ("tavern", "gebaeude-paper/taverne"),
    ("healer", "gebaeude-paper/heilplatz"),
    ("smithy", "gebaeude-paper/schmiede"),
];

fn hub_building(kind: &str) -> Option<&'static str> {
    HUB_BUILDINGS.iter().find(|(k, _)| *k == kind).map(|(_, id)| *id)
}

/// `Staged`: Platzart mit Grafik je Stufe und ihre Stufenzahl (Mauer, Turm 1–5; Schiff: Kiel, Rumpf, Segel).
struct Staged {
    id: &'static str,
    stages: i64,
    /// Fehlende Stufe = höchste (Hub-Gebäude, Schiff); sonst Grundform.
    clamp: bool,
}

fn staged(kind: &str) -> Option<Staged> {
    match kind {
        "wall" => Some(Staged {
            id: "gebaeude-paper/mauer",
            stages: 5,
            clamp: false,
        }),
        "tower" => Some(Staged {
            id: "gebaeude-paper/turm",
            stages: 5,
            clamp: false,
        }),
        "ship" => Some(Staged {
            id: "gebaeude-paper/schiff",
            stages: 3,
            clamp: true,
        }),
        _ => None,
    }
}

/// Platzarten mit einer Grafik ohne Stufen.
fn plain_site(kind: &str) -> Option<&'static str> {
    match kind {
        "stairsUp" => Some("gebaeude-paper/treppe-hoch"),
        "stairsDown" => Some("gebaeude-paper/treppe-runter"),
        _ => None,
    }
}

/// `is_hub_building`: Hub-Gebäude ohne eigene Stufen zeigen die Grafik der Hub-Stufe (GEB9, B-569).
pub fn is_hub_building(kind: &str) -> bool {
    hub_building(kind).is_some() && building_level_count(kind) == 0
}

/// `site_stage` ist die Stufe der Grafik: Hub-Gebäude ohne Stufen je Hub-Stufe, sonst die Stufe des Platzes (0 = 1).
pub fn site_stage(kind: &str, level: i64, hub_level: i64) -> i64 {
    if is_hub_building(kind) {
        return hub_level.max(1);
    }
    level.max(1)
}

/// `site_look` ist die Grafik eines gebauten Platzes in Stufe stage; Baustellen und unbekannte Arten haben keine.
/// Mauer und Turm über Stufe 5 zeigen die Grundform, Hub-Gebäude und Schiff die höchste Stufe (B-458).
pub fn site_look(kind: &str, state: &str, stage: i64) -> Look {
    if state != "built" {
        return Look::default();
    }
    if let Some(id) = plain_site(kind) {
        return Look::new(id, "stand");
    }
    if let Some(id) = hub_building(kind) {
        return Look::new(id, stage_anim(stage.clamp(1, 5)));
    }
    let Some(s) = staged(kind) else {
        return Look::default();
    };
    let stage = if s.clamp {
        stage.clamp(1, s.stages)
    } else if stage < 1 || stage > s.stages {
        1
    } else {
        stage
    };
    Look::new(s.id, stage_anim(stage))
}

/// `scaffold_class`: Größenklasse des Gerüsts je Platzart = Breite des Gebäudes in Kacheln (B-586).
fn scaffold_class(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "wall" => "gebaeude-paper/geruest-1",
        "tower" => "gebaeude-paper/geruest-3",
        "nursery" | "stairsUp" | "stairsDown" => "gebaeude-paper/geruest-4",
        // B-1147: Das Schiff hat im Profil k2c ein eigenes Gerüst in Schiffsgröße (kein Größensprung beim Fertigstellen).
        "ship" if k3c_data::look::current().sizes => "gebaeude-paper/geruest-schiff",
        "workshop" | "bowyer" | "storage" | "barracks" | "tavern" | "healer" | "smithy" | "armory" | "ship" => {
            "gebaeude-paper/geruest-6"
        }
        _ => return None,
    })
}

/// `scaffold_look` ist das Gerüst der Platzart (unbekannte Art: Klasse 6).
pub fn scaffold_look(kind: &str) -> Look {
    Look::new(scaffold_class(kind).unwrap_or("gebaeude-paper/geruest-6"), "stand")
}

/// `Marker` ist, was am ungebauten Platz steht: Gerüst im Bau, sonst Bauschild bei bezahlbarem Platz, nie beides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    None,
    Sign,
    Scaffold,
}

/// `site_marker`: bezahlt, aber nicht fertig = Gerüst; ungebaut und bezahlbar = Bauschild (B-584, B-586).
pub fn site_marker(state: &str, payable: bool) -> Marker {
    if state == "waitingMaterial" || state == "waitingWorker" {
        Marker::Scaffold
    } else if state != "built" && payable {
        Marker::Sign
    } else {
        Marker::None
    }
}

/// `sign_variant`: Bauschild nach Art des Platzes (0 Siedlung, 1 Verteidigung, 2 Treppen).
pub fn sign_variant(kind: &str) -> i64 {
    match kind {
        "wall" | "tower" => 1,
        "stairsUp" | "stairsDown" => 2,
        _ => 0,
    }
}

/// `HubLook` ist das Bild der Burg: Asset und Animation, Überlage der Ausbauschritte und das Gerüst davor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HubLook {
    pub main: Look,
    pub overlay: Look,
    pub scaffold: bool,
}

/// `hub_look_of` wählt den Look aus Hub-Stufe (1–5) und Ausbauschritt (1–3, GEB17.6): Hub-Stufe 1 Lagerfeuer, Zelt,
/// Holzhütte (lagerfeuer 1–3), ab Stufe 2 die Burg der Stufe und in Schritt 2 und 3 ihr Schmuck als Überlage
/// (burg-ausbau 2–3, D-MG6-05). Das Gerüst steht bei jedem waitingWorker (B-572); im Bau noch kein Schmuck.
pub fn hub_look_of(hub_level: i64, hub_step: i64, upgrade_state: &str) -> HubLook {
    let level = hub_level.clamp(1, 5);
    let step = hub_step.clamp(1, 3);
    let mut out = HubLook {
        scaffold: upgrade_state == "waitingWorker",
        ..Default::default()
    };
    if level == 1 {
        out.main = Look::new("gebaeude-paper/lagerfeuer", stage_anim(step));
        return out;
    }
    out.main = Look::new("gebaeude-paper/burg", stage_anim(level));
    if step > 1 && !out.scaffold {
        out.overlay = Look::new("gebaeude-paper/burg-ausbau", stage_anim(step));
    }
    out
}

/// `HUB_SCAFFOLD` ist das Baugerüst vor der Burg.
pub const HUB_SCAFFOLD: Look = Look::new("gebaeude-paper/hub-geruest", "stand");

/// `BackWall` ist ein Abschnitt Hintergrundmauer von from bis to (Units) in der Stufe der äußeren Mauer (B-442).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackWall {
    pub id: i64,
    pub from: f64,
    pub to: f64,
    pub stage: i64,
}

/// `back_walls` hängt die Hintergrundmauern zwischen je zwei benachbarten gebauten Mauern derselben Seite an out an:
/// links, dann rechts, je von innen nach außen. `walls` ist ein Puffer für die Mauern einer Seite (Indizes in
/// `sites`, damit der Puffer über die Bilder erhalten bleibt und je Bild nichts angelegt wird).
pub fn back_walls(mut out: Vec<BackWall>, walls: &mut Vec<usize>, sites: &[Site], hub_x: f64) -> Vec<BackWall> {
    // Innerste Mauer je Seite (links, rechts) für den Burgbereich (B-1175)
    let mut innermost: [Option<usize>; 2] = [None; 2];
    for (side_i, side) in [-1.0, 1.0].into_iter().enumerate() {
        walls.clear();
        for (i, s) in sites.iter().enumerate() {
            if s.kind == "wall" && sign(s.x - hub_x) == side {
                walls.push(i);
            }
        }
        by_distance(walls, sites, hub_x);
        innermost[side_i] = walls.first().copied();
        for i in 1..walls.len() {
            let (inner, ex) = (&sites[walls[i - 1]], &sites[walls[i]]);
            if inner.state == "built" && ex.state == "built" {
                out.push(BackWall {
                    id: ex.id,
                    from: inner.x.min(ex.x),
                    to: inner.x.max(ex.x),
                    stage: ex.level.max(1),
                });
            }
        }
    }
    // B-1175: Im Burgbereich schließt eine Mauer die Stelle zwischen den innersten Mauern beider Seiten (Profil k2c).
    if let [Some(l), Some(r)] = innermost
        && k3c_data::look::current().sizes
        && sites[l].state == "built"
        && sites[r].state == "built"
    {
        out.push(BackWall {
            id: sites[r].id,
            from: sites[l].x.min(sites[r].x),
            to: sites[l].x.max(sites[r].x),
            stage: sites[l].level.min(sites[r].level).max(1),
        });
    }
    out
}

/// `by_distance` sortiert die Mauern (Indizes in `sites`) einer Seite stabil nach Abstand zur Hub-Mitte (Einfügen,
/// wenige Einträge, ohne Abschluss je Bild). Wie in Go ein Einfügen-Sort, der bei Gleichstand nicht tauscht: stabil.
pub fn by_distance(walls: &mut [usize], sites: &[Site], hub_x: f64) {
    let d = |i: usize| (sites[i].x - hub_x).abs();
    for i in 1..walls.len() {
        let mut j = i;
        while j > 0 && d(walls[j]) < d(walls[j - 1]) {
            walls.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn sign(v: f64) -> f64 {
    if v < 0.0 {
        -1.0
    } else if v > 0.0 {
        1.0
    } else {
        0.0
    }
}

/// `wall_height`: Anzeigehöhe der Mauer je Ausbaustufe als Anteil der vollen Höhe (WL1.4, B-1213): Zaun (Stufe 1) halb
/// so hoch, Kristall (Stufe 5) voll, dazwischen gleichmäßig 0,5 · 0,625 · 0,75 · 0,875 · 1,0. Nur die Darstellung, die
/// Kollision bleibt. Stufen außerhalb 1 bis 5 zeigen die volle Höhe.
pub fn wall_height(stage: i64) -> f64 {
    match stage {
        1..=5 => 0.5 + 0.125 * (stage - 1) as f64,
        _ => 1.0,
    }
}

/// `back_wall_look` ist die Hintergrundmauer der Stufe 1–5; andere Stufen haben keine Grafik.
pub fn back_wall_look(stage: i64) -> Look {
    if !(1..=5).contains(&stage) {
        return Look::default();
    }
    Look::new("gebaeude-paper/hintergrundmauer", stage_anim(stage))
}

/// `damage_stage`: 0 unbeschädigt, 1 leicht (≥ 2/3 der HP), 2 mittel (≥ 1/3), 3 schwer; nicht gebaut oder ohne MaxHP 0.
pub fn damage_stage(state: &str, hp: f64, max_hp: f64) -> i64 {
    if state != "built" || max_hp <= 0.0 || hp >= max_hp {
        return 0;
    }
    let share = hp / max_hp;
    if share >= 2.0 / 3.0 {
        1
    } else if share >= 1.0 / 3.0 {
        2
    } else {
        3
    }
}

/// `DAMAGE_SHADE` ist die Abdunklung je Schadensstufe (Anteil Schwarz, damageView.ts › SHADE).
pub const DAMAGE_SHADE: [f32; 4] = [0.0, 0.12, 0.25, 0.4];

/// `show_rubble`: Trümmer nur an einer gebauten Ruine unter der Hälfte ihrer HP.
pub fn show_rubble(ruin: bool, state: &str, hp: f64, max_hp: f64) -> bool {
    ruin && state == "built" && max_hp > 0.0 && hp / max_hp < 0.5
}
