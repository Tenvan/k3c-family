//! Natur und Objekte (Spiegel von src/scenes/render/natureSprites.ts, plantation.ts, worldSprites.ts, mineSignView.ts):
//! Form aus der Position (gleiche Mischung wie im Browser, kein Zufall), Look aus dem Biom. Reine Auswahl.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use super::buildings::{Look, OBJECT_GROUP, stage_anim};
use crate::actors::catalog::UNIT_PX;

/// Animation aller Natur- und Objekt-Assets (Go: `idle`).
pub const IDLE: &str = "idle";

/// `COIN_LOOK` ist die Münze (Go: `coinLook`).
pub const COIN_LOOK: Look = Look::new("icons/muenze", IDLE);

/// `mix` ist die ganzzahlige Mischung der gerundeten Position (natureSprites.ts › mix, bitgleich mit Math.imul):
/// gleiche Position, gleicher Wert.
pub fn mix(x: f64) -> u32 {
    let mut h = ((x + 0.5).floor() as i64 as i32 as u32).wrapping_mul(0x9e37_79b1) ^ 0x85eb_ca6b;
    h = (h ^ (h >> 16)).wrapping_mul(0x85eb_ca6b);
    h = (h ^ (h >> 13)).wrapping_mul(0xc2b2_ae35);
    h ^ (h >> 16)
}

/// `anim_phase` ist der Startframe einer Schleife aus der Weltposition, damit Nachbarn nicht im Gleichtakt wiegen.
pub fn anim_phase(x: f64, frames: i64) -> i64 {
    if frames <= 1 {
        return 0;
    }
    i64::from(mix(x * 7.0) % frames as u32)
}

/// `nature_look` ist der Look des Bioms: 1 Wald, 2 Höhle, 3 Mine und alle übrigen; leere ID = Wald.
pub fn nature_look(biome: &str) -> i64 {
    match biome {
        "" | "forest" => 1,
        "cave" => 2,
        _ => 3,
    }
}

/// `TREE_KINDS`: Baumarten des Walds (Asset-IDs).
pub const TREE_KINDS: [&str; 3] = ["tanne", "pappel", "birke"];
/// `ROCK_KINDS`: Fels-Formen (Asset-IDs).
pub const ROCK_KINDS: [&str; 3] = ["felsen", "felsen-b", "felsen-c"];

/// `vein_kind`: Asset-ID der Ader je Knoten-Art.
fn vein_kind(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "stoneVein" => "ader-stein",
        "copperVein" => "ader-kupfer",
        "ironVein" => "ader-eisen",
        "crystalVein" => "ader-kristall",
        _ => return None,
    })
}

/// `object` ist das Bild des Objekts id (Animation `idle`). Jede ID wird einmal zu einem `Look` mit festem Text
/// (die Tabelle wächst höchstens um die Zahl der Assets), danach ohne Allokation.
pub fn object(id: &str) -> Look {
    static LOOKS: OnceLock<Mutex<BTreeMap<String, Look>>> = OnceLock::new();
    let mut map = LOOKS
        .get_or_init(|| Mutex::new(BTreeMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(l) = map.get(id) {
        return *l;
    }
    let asset: &'static str = Box::leak(format!("{OBJECT_GROUP}{id}").into_boxed_str());
    let l = Look::new(asset, IDLE);
    map.insert(id.to_string(), l);
    l
}

/// Höhen-Varianten je Baumart (BB1.3, B-1146): drei Höhen in aufsteigender Reihenfolge und die dunkle Variante für
/// die Nähe der Waldwand. Gilt im Look-Profil k2c (`look.sizes`); im Profil `go` bleibt es bei der Art selbst.
pub const TREE_VARIANTS: [(&str, [&str; 3], &str); 3] = [
    ("tanne", ["tanne", "tanne-mittel", "tanne-hoch"], "tanne-dunkel"),
    ("pappel", ["pappel-niedrig", "pappel", "pappel-hoch"], "pappel-dunkel"),
    ("birke", ["birke", "birke-mittel", "birke-hoch"], "birke-dunkel"),
];

/// Jeder fünfte Baum steht als dunkle Variante (vermutet: ersetzt die Nähe zur Waldwand, die das Spiel
/// nicht kennt; die Waldwand liegt hinter allen Bäumen).
const DARK_EVERY: u32 = 5;

/// `tree_asset`: Asset-ID eines Baums der Art `family` an der Position `x`: deterministisch aus der Position, in drei
/// Höhen, gelegentlich dunkel. Außerhalb des Profils k2c und für unbekannte Arten die Art selbst.
pub fn tree_asset(family: &'static str, x: f64) -> &'static str {
    let Some((_, heights, dark)) = TREE_VARIANTS.iter().find(|(f, _, _)| *f == family) else {
        return family;
    };
    if !k3c_data::look::current().sizes {
        return family;
    }
    let r = mix(x * 5.0);
    if r.is_multiple_of(DARK_EVERY) {
        return dark;
    }
    heights[(r / DARK_EVERY % 3) as usize]
}

/// `tree_look`: im Wald Art aus der Position (Tanne, Pappel, Birke), außerhalb des Walds Wurzeln statt Bäumen (GEB10;
/// die fünf Wurzel-Formen zeigen als Papier dasselbe Asset, B-701 D-US2-01).
pub fn tree_look(x: f64, biome: &str) -> Look {
    if nature_look(biome) != 1 {
        return object("wurzel");
    }
    let n = TREE_KINDS.len() as u32;
    object(tree_asset(TREE_KINDS[(mix(x) % (n * 3) % n) as usize], x))
}

/// `crystal_kind`: Bild der Kristall-Ader je Farbe der Gewichts-Gruppe (B-900, B-901); blau ist das ursprüngliche
/// `ader-kristall`.
fn crystal_kind(variant: &str) -> Option<&'static str> {
    Some(match variant {
        "blau" => "ader-kristall",
        "violett" => "ader-kristall-violett",
        "gruen" => "ader-kristall-gruen",
        _ => return None,
    })
}

/// `variant_look`: Bild der Variante aus dem Level (B-891): im Wald die Baumart (tanne, pappel, birke = Asset-ID), an
/// der Kristall-Ader die Farbe. `None` heißt: unbekannte Variante, Bild wie ohne Variante.
pub fn variant_look(kind: &str, variant: &str, biome: &str, x: f64) -> Option<Look> {
    if kind == "crystalVein" {
        return crystal_kind(variant).map(object);
    }
    if kind == "tree" && nature_look(biome) == 1 {
        return TREE_KINDS
            .iter()
            .find(|id| **id == variant)
            .map(|id| object(tree_asset(id, x)));
    }
    None
}

/// `rock_look`: Fels-Form aus der Position (GEB15.2).
pub fn rock_look(x: f64) -> Look {
    object(ROCK_KINDS[(mix(x) % ROCK_KINDS.len() as u32) as usize])
}

/// `node_look` ist das Bild eines Ressourcen-Knotens; unbekannte Art = keins (leerer `Look`, Platzhalter).
pub fn node_look(kind: &str, x: f64, biome: &str) -> Look {
    match kind {
        "tree" => tree_look(x, biome),
        "rock" => rock_look(x),
        "copperOre" => object("erzbrocken"),
        "bush" => bush_look(biome),
        _ => vein_kind(kind).map_or_else(Look::default, object),
    }
}

/// `bush_look`: Busch im Wald, Höhlenbusch sonst.
pub fn bush_look(biome: &str) -> Look {
    if nature_look(biome) == 1 {
        object("busch")
    } else {
        object("hoehlenbusch")
    }
}

/// `deco_roots` hängt die Versätze (Units) der Deko-Wurzeln eines Baum-Knotens an `out` an: außerhalb des Walds
/// `roots_per_node - 1` Stück, ±0,4–1,2 Units, Seite wechselnd (natureSprites.ts › decoRoots); im Wald keine.
pub fn deco_roots(out: &mut Vec<f64>, x: f64, biome: &str, roots_per_node: i64) {
    if nature_look(biome) == 1 {
        return;
    }
    for i in 0..(roots_per_node - 1).max(0) {
        let h = mix(x + (i + 1) as f64 * 7919.0);
        let side = if (h >> 4).wrapping_add(i as u32) & 1 != 0 {
            1.0
        } else {
            -1.0
        };
        out.push(side * f64::from(4 + (h >> 8) % 9) / 10.0);
    }
}

/// `mine_sign_look`: Abbau-Schild am markierten Knoten, Axt am Baum, Spitzhacke an Fels, Erz und Ader; sonst keins.
pub fn mine_sign_look(kind: &str) -> Look {
    if kind == "tree" {
        object("schild-axt")
    } else if kind == "rock" || kind == "copperOre" || vein_kind(kind).is_some() {
        object("schild-spitzhacke")
    } else {
        Look::default()
    }
}

/// `PLOT_TOLERANCE`: Abstand, bis zu dem ein Baum auf einem Plantage-Platz steht (plantation.ts).
pub const PLOT_TOLERANCE: f64 = 0.01;

/// `plot_x` ist der Ort von Platz i der Plantage um die Baumschule bei `farm_x` (plotX in engine/sim/building_plantation.go).
pub fn plot_x(farm_x: f64, i: i64, slots: i64, strip: f64) -> f64 {
    if slots < 2 {
        return farm_x;
    }
    farm_x - strip + i as f64 * 2.0 * strip / (slots - 1) as f64
}

/// `on_plot` meldet, ob x ein Platz der Plantage um `farm_x` ist.
pub fn on_plot(x: f64, farm_x: f64, slots: i64, strip: f64) -> bool {
    (0..slots).any(|i| (x - plot_x(farm_x, i, slots, strip)).abs() < PLOT_TOLERANCE)
}

/// `entity_look` ist das Bild einer festen Entität des Levels (portal, exit, harbor, recruitCamp, bush); der `f64` ist
/// ihr Versatz in Units (der Hafen-Steg steht links vom Schiffs-Platz), `None` = sie wird nicht gezeichnet.
pub fn entity_look(kind: &str, biome: &str) -> Option<(Look, f64)> {
    Some(match kind {
        "portal" => (object("portal"), 0.0),
        "exit" => (object("ausgang"), 0.0),
        "bush" => (bush_look(biome), 0.0),
        "harbor" => (Look::new("gebaeude-paper/hafen", "stand"), -HARBOR_SHIFT),
        "recruitCamp" => (
            Look::new("gebaeude-paper/rekrutierungslager", stage_anim(nature_look(biome))),
            0.0,
        ),
        _ => return None,
    })
}

/// `HARBOR_SHIFT`: Der Hafen-Steg steht so viele Kacheln links vom Schiffs-Platz (BS10).
pub const HARBOR_SHIFT: f64 = 5.5;

/// `pickup_look`: Truhe geschlossen am Boden, Stern 44 px darüber; Höhe in Units.
pub fn pickup_look(kind: &str) -> (Look, f64) {
    match kind {
        "chest" => (object("truhe"), 0.0),
        "skillPoint" => (object("stern"), 44.0 / UNIT_PX),
        _ => (Look::default(), 0.0),
    }
}

/// `chest_open_look`: geöffnete Truhe.
pub fn chest_open_look() -> Look {
    object("truhe-offen")
}

/// `seedling_look`: Setzling der Plantage.
pub fn seedling_look() -> Look {
    object("setzling")
}

/// `plant_tree_look`: gewachsener Baum der Plantage (Altbaum); im Profil k2c die mittlere Pappel (2,9 M), sonst die
/// niedrige (B-1149).
pub fn plant_tree_look() -> Look {
    object(if k3c_data::look::current().sizes {
        "pappel"
    } else {
        "pappel-niedrig"
    })
}

/// `young_tree_look`: Jungbaum der Plantage in der zweiten Hälfte der Wachszeit (nur Profil k2c, sonst `None`): die
/// kleine Pappel (etwa 1 M) zwischen Setzling (0,25 M) und Altbaum (2,9 M).
pub fn young_tree_look() -> Option<Look> {
    k3c_data::look::current().sizes.then(|| object("pappel-2"))
}
