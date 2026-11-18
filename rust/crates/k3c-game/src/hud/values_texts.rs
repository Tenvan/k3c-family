//! Zweiter Teil von `values.go` (ab `merchantText`): Händler, Beruf, Kampf, Reise und Ort. `values.rs` führt die
//! Namen wieder zusammen.

use super::texts::{Arg, name_of, tf, tr};
use super::values::{pick, res_name};
use k3c_sim::state::Travel;

/// merchant_text: Händler mit Angebot und Abreisetag, laufender Kauf in der langen Form.
pub fn merchant_text(lang: &str, resource: &str, leaves: i64, paid: i64, compact: bool) -> String {
    let res: Arg = res_name(lang, resource).into();
    if compact {
        return tf(lang, "merchant.short", &[("res", res)]);
    }
    if paid > 0 {
        let kv = [("res", res), ("day", leaves.into()), ("paid", paid.into())];
        return tf(lang, "merchant.paid", &kv);
    }
    tf(lang, "merchant.text", &[("res", res), ("day", leaves.into())])
}

/// profession_text ist der Name eines Berufs (bekannte Berufe, sonst neutral; kein Beruf "").
pub fn profession_text(lang: &str, profession: &str, compact: bool) -> String {
    if profession.is_empty() {
        return String::new();
    }
    let keys = profession_keys(profession);
    tr(lang, if compact { keys.1 } else { keys.0 }).to_string()
}

/// profession_keys: Text-Schlüssel je Beruf, lang und kurz (fest, damit je Bild kein Schlüssel entsteht).
fn profession_keys(profession: &str) -> (&'static str, &'static str) {
    match profession {
        "miner" => ("prof.miner", "prof.minerShort"),
        "builder" => ("prof.builder", "prof.builderShort"),
        "craftsman" => ("prof.craftsman", "prof.craftsmanShort"),
        _ => ("prof.unknown", "prof.unknownShort"),
    }
}

/// WaveKey: Welle und Zahl der Gegner (laufend und wartend).
#[derive(Clone, Debug, PartialEq)]
pub struct WaveKey {
    pub lang: String,
    pub wave: i64,
    pub n: i64,
}

/// wave_text ist die Kampf-Zeile; ohne Gegner "".
pub fn wave_text(k: &WaveKey) -> String {
    if k.n <= 0 {
        return String::new();
    }
    tf(&k.lang, "hud.wave", &[("wave", k.wave.into()), ("n", k.n.into())])
}

/// TravelKey: Reise zur Tiefe über Treppe (`up`) mit Fortschritt in Zehnteln; `on` false = keine Reise.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TravelKey {
    pub lang: String,
    pub on: bool,
    pub up: bool,
    pub depth: i64,
    pub tenth: i64,
}

/// new_travel_key liest den Stufenwechsel der Welt.
pub fn new_travel_key(lang: &str, t: Option<&Travel>) -> TravelKey {
    let Some(t) = t else {
        return TravelKey {
            lang: lang.to_string(),
            ..TravelKey::default()
        };
    };
    TravelKey {
        lang: lang.to_string(),
        on: true,
        up: t.via == "stairsUp",
        depth: t.to_depth,
        tenth: (t.progress.clamp(0.0, 1.0) * 10.0).ceil() as i64,
    }
}

/// travel_text: Ziel der Reise und ein Balken aus zehn Kästchen.
pub fn travel_text(k: &TravelKey) -> String {
    if !k.on {
        return String::new();
    }
    let key = pick(k.up, "hud.travelUp", "hud.travelDown");
    let s = tf(&k.lang, key, &[("depth", k.depth.into())]);
    let tenth = k.tenth.clamp(0, 10) as usize;
    format!("{s}  {}{}", "■".repeat(tenth), "□".repeat(10 - tenth))
}

/// PlaceKey: Spieler und Biom der Stufe.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaceKey {
    pub lang: String,
    pub id: String,
    pub name: String,
    pub player: i64,
}

/// place_text ist der Ort des Spielers: „P1 · Oberwelt (Wald)“.
pub fn place_text(k: &PlaceKey) -> String {
    let stage = name_of(&k.lang, "biome", &k.id, &k.name);
    tf(
        &k.lang,
        "hud.place",
        &[("p", (k.player + 1).into()), ("stage", stage.into())],
    )
}
