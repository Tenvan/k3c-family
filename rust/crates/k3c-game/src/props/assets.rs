//! Papier-Assets je Level (view.AssetSource): alles, was props im Level zeichnen kann, auch wenn es gerade nicht
//! sichtbar ist. Gebäude, Gerüste, Burg und Objekte immer; die Natur nach dem Look des Bioms (Wald: Bäume und Busch,
//! sonst Wurzeln und Höhlenbusch). Fehlt eine Datei, beendet game das Spiel mit dem Pfad (art.ErrMissing).

use super::buildings::{BUILDING_GROUP, OBJECT_GROUP};
use super::nature::{COIN_LOOK, nature_look};
use super::plan_combat::COMBAT_ASSETS;

/// `BUILDING_ASSETS`: alle Gebäude-Assets, die site_look, hub_look_of, scaffold_look, back_wall_look und entity_look
/// wählen.
pub const BUILDING_ASSETS: [&str; 25] = [
    "burg",
    "burg-ausbau",
    "lagerfeuer",
    "hub-geruest",
    "hintergrundmauer",
    "mauer",
    "turm",
    "schiff",
    "werkstatt",
    "bogner",
    "taverne",
    "heilplatz",
    "schmiede",
    "ruestkammer",
    "lager",
    "farm",
    "kaserne",
    "treppe-hoch",
    "treppe-runter",
    "hafen",
    "rekrutierungslager",
    "geruest-1",
    "geruest-3",
    "geruest-4",
    "geruest-6",
];

/// `OBJECT_ASSETS`: Objekte in jedem Biom (Felsen, Erz, Adern mit den Kristallfarben, Abbau-Schilder, Plantage,
/// Portal, Ausgang, Truhen, Stern, Ladeplätze, Fracht, Doppel-Truhe).
pub const OBJECT_ASSETS: [&str; 26] = [
    "felsen",
    "felsen-b",
    "felsen-c",
    "erzbrocken",
    "ader-stein",
    "ader-kupfer",
    "ader-eisen",
    "ader-kristall",
    "ader-kristall-violett",
    "ader-kristall-gruen",
    "schild-axt",
    "schild-spitzhacke",
    "setzling",
    "pappel-niedrig",
    "portal",
    "ausgang",
    "truhe",
    "truhe-offen",
    "stern",
    // RU62.2 Fracht am Steg und an Deck, RU62.4 Doppel-Truhe
    "ladeplatz-muenzen",
    "ladeplatz-truppen",
    "ladeplatz-material",
    "fracht-beutel",
    "fracht-kiste",
    "doppel-truhe",
    "doppel-truhe-offen",
];

/// `nature_assets`: Natur je Look (true = 1 Wald, sonst Höhle und Mine).
pub fn nature_assets(forest: bool) -> &'static [&'static str] {
    if forest {
        &["tanne", "pappel", "birke", "busch"]
    } else {
        &["wurzel", "hoehlenbusch"]
    }
}

/// `level_assets` nennt die Assets `gruppe/id` des Levels (Biom-ID), sortiert und jedes einmal.
pub fn level_assets(level: &str) -> Vec<String> {
    let mut out: Vec<String> =
        Vec::with_capacity(BUILDING_ASSETS.len() + OBJECT_ASSETS.len() + COMBAT_ASSETS.len() + 5);
    for id in BUILDING_ASSETS {
        out.push(format!("{BUILDING_GROUP}{id}"));
    }
    for id in OBJECT_ASSETS {
        out.push(format!("{OBJECT_GROUP}{id}"));
    }
    for id in nature_assets(nature_look(level) == 1) {
        out.push(format!("{OBJECT_GROUP}{id}"));
    }
    out.push(COIN_LOOK.asset.to_string());
    out.extend(COMBAT_ASSETS.iter().map(|a| a.to_string()));
    // Sortieren wie `slices.Sort` (Texte, Gleichstand ohne Folgen), danach Doppelte entfernen (`slices.Compact`).
    out.sort_unstable();
    out.dedup();
    out
}
