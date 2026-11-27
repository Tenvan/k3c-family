//! Der Block „Hub-Stufe → Gebäude“ der Regeln (`docs/rules/materialien-gebaeude.md` § 3) aus `data/hub.json` (Port von
//! `tools/ruletable/table`, V9; die Go-CLI `task gen:rules` bleibt). Reine Funktionen ohne Dateizugriff. Die Zahl der
//! Hub-Stufen kommt aus `levels`, es gibt keine feste Obergrenze.

use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;
use serde_json::Value;

use crate::kitx::go_quoted;

#[cfg(test)]
#[path = "table_tests.rs"]
mod tests;

/// Anfang und Ende des erzeugten Blocks im Regeltext; je genau einmal.
pub(super) const START: &str = "<!-- rules:hub-levels:start -->";
pub(super) const END: &str = "<!-- rules:hub-levels:end -->";

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Site {
    kind: String,
    offset_units: f64,
    from_depth: i64,
    needs_deeper: bool,
    hub_level: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Step {
    cost: BTreeMap<String, i64>,
}

/// Eine Hub-Stufe: Kosten direkt oder verteilt auf die Ausbauschritte (`steps`, B-592); das Material-Tor (`gate`)
/// zählt nicht zum Material der Stufe.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Level {
    cost: BTreeMap<String, i64>,
    steps: Vec<Step>,
}

impl Level {
    /// Die Kosten der Stufe samt aller Ausbauschritte.
    fn total(&self) -> BTreeMap<&str, i64> {
        let mut out: BTreeMap<&str, i64> = BTreeMap::new();
        for (key, v) in self.cost.iter().chain(self.steps.iter().flat_map(|s| &s.cost)) {
            let sum = out.entry(key).or_default();
            *sum = sum.saturating_add(*v);
        }
        out
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Placement {
    zone1_wall_units: f64,
    zone1_tower_units: f64,
    zones: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Merchant {
    buy_offset_units: f64,
    sell_offset_units: f64,
    only_depth: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Hub {
    levels: Vec<Level>,
    sites: Vec<Site>,
    island_sites: Vec<Site>,
    site_placement: Placement,
    merchant: Merchant,
}

fn kind_name(kind: &str) -> &str {
    match kind {
        "workshop" => "Werkstatt",
        "bowyer" => "Bogner",
        "stairsUp" => "Treppe hoch",
        "stairsDown" => "Treppe runter",
        "nursery" => "Baumschule",
        "healer" => "Heilplatz",
        "tavern" => "Taverne",
        "barracks" => "Kaserne",
        "smithy" => "Schmiede",
        "armory" => "Rüstkammer",
        "storage" => "Lager",
        other => other,
    }
}

fn material_name(key: &str) -> Option<&'static str> {
    Some(match key {
        "wood" => "Holz",
        "stone" => "Stein",
        "copper" => "Kupfer",
        "iron" => "Eisen",
        "crystal" => "Kristall",
        _ => return None,
    })
}

/// `null` heißt wie bei Gos `json.Unmarshal` Nullwert: Felder entfallen, ein Listen-Eintrag wird ein leeres Objekt.
fn drop_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(drop_nulls);
        }
        Value::Array(list) => {
            for v in list {
                if v.is_null() {
                    *v = Value::Object(serde_json::Map::new());
                }
                drop_nulls(v);
            }
        }
        _ => {}
    }
}

fn parse(hub_json: &[u8]) -> Result<Hub, String> {
    let failed = |e: serde_json::Error| format!("hub.json: {e}");
    let mut value: Value = serde_json::from_slice(hub_json).map_err(failed)?;
    if value.is_null() {
        return Ok(Hub::default());
    }
    drop_nulls(&mut value);
    Hub::deserialize(value).map_err(failed)
}

/// Der Block samt Marken (LF-Zeilenenden, ohne Schlusszeilenumbruch) für alle `levels` der hub.json.
pub(super) fn block(hub_json: &[u8]) -> Result<String, String> {
    let hub = parse(hub_json)?;
    if hub.levels.is_empty() {
        return Err("hub.json: keine levels".to_owned());
    }
    let mut rows = vec![
        START.to_owned(),
        "| Hub-Stufe | Material | Hub-Plätze | Mauer- und Turm-Zone | Händler |".to_owned(),
        "|---|---|---|---|---|".to_owned(),
    ];
    check_levels(&hub)?;
    for (level, entry) in (1i64..).zip(&hub.levels) {
        let cells = [
            level.to_string(),
            material(level, &entry.total())?.to_owned(),
            places(&hub, level),
            line(&hub, level),
            merchant(&hub, level),
        ];
        rows.push(format!("| {} |", cells.join(" | ")));
    }
    rows.push(END.to_owned());
    Ok(rows.join("\n"))
}

/// Eine Zahl ohne überflüssige Nachkommastellen und ohne Exponent (Go `FormatFloat(v, 'f', -1, 64)`).
fn num(v: f64) -> String {
    format!("{v}")
}

fn signed(n: f64) -> String {
    if n < 0.0 {
        format!("−{}", num(-n))
    } else {
        format!("+{}", num(n))
    }
}

/// Wie in der Engine: ein fehlender Wert (0) heißt Stufe 1.
fn hub_level_of(site: &Site) -> i64 {
    site.hub_level.max(1)
}

/// Meldet Plätze, deren Stufe außerhalb von `levels` liegt; sie würden sonst still aus dem Block fallen.
fn check_levels(hub: &Hub) -> Result<(), String> {
    let count = hub.levels.len();
    for site in hub.sites.iter().chain(&hub.island_sites) {
        if usize::try_from(hub_level_of(site)).is_ok_and(|level| level > count) {
            let (kind, level) = (&site.kind, site.hub_level);
            return Err(format!(
                "hub.json: platz {kind} hat hubLevel {level}, aber nur {count} levels"
            ));
        }
    }
    Ok(())
}

/// Das Baumaterial der Stufe: das eine Material der Ausbaukosten (Gold zählt nicht), ohne Kosten Holz. Mehr als ein
/// Material oder ein unbekanntes ist ein Fehler, damit die Ausgabe eindeutig bleibt.
fn material(level: i64, cost: &BTreeMap<&str, i64>) -> Result<&'static str, String> {
    let keys: Vec<&str> = cost
        .iter()
        .filter(|&(&key, &v)| key != "gold" && v > 0)
        .map(|(&key, _)| key)
        .collect();
    let index = level - 1;
    match keys.as_slice() {
        [] => Ok("Holz"),
        [key] => material_name(key).ok_or_else(|| {
            let quoted = go_quoted(key);
            format!("hub.json: levels[{index}] hat das unbekannte material {quoted}")
        }),
        _ => Err(format!(
            "hub.json: levels[{index}] hat mehrere materialien [{}]",
            keys.join(" ")
        )),
    }
}

/// Die Plätze der Stufe nach Art gruppiert in Reihenfolge des ersten Auftretens (erst `sites`, dann `islandSites`).
fn places(hub: &Hub, level: i64) -> String {
    let mut kinds: Vec<&str> = Vec::new();
    let mut offsets: HashMap<&str, Vec<String>> = HashMap::new();
    let all = hub.sites.iter().map(|s| (s, false));
    for (site, island) in all.chain(hub.island_sites.iter().map(|s| (s, true))) {
        if hub_level_of(site) != level {
            continue;
        }
        let texts = offsets.entry(&site.kind).or_default();
        if texts.is_empty() {
            kinds.push(&site.kind);
        }
        texts.push(site_text(site, island));
    }
    if kinds.is_empty() {
        return "–".to_owned();
    }
    let parts: Vec<String> = kinds
        .iter()
        .map(|kind| format!("{} {}", kind_name(kind), offsets[kind].join(", ")))
        .collect();
    parts.join("; ")
}

/// `island`: aus `islandSites`, gilt je Insel, nicht je Hub.
fn site_text(site: &Site, island: bool) -> String {
    let mut text = signed(site.offset_units);
    if site.from_depth > 0 {
        text += &format!(" (ab Tiefe {})", site.from_depth);
    }
    if site.needs_deeper {
        text += " (nur mit tieferer Stufe)";
    }
    if island {
        text += " (je Insel)";
    }
    text
}

/// Die Mauer- und Turm-Plätze der Stufe (B-652): Zone n ist ab Hub-Stufe n sichtbar; Zone 1 hat den festen Baugrund
/// (Mauer, Turm), alle weiteren Plätze der Zone würfelt der Generator.
fn line(hub: &Hub, level: i64) -> String {
    let p = &hub.site_placement;
    if level > p.zones {
        return "–".to_owned();
    }
    if level == 1 {
        let (wall, tower) = (num(p.zone1_wall_units), num(p.zone1_tower_units));
        return format!("Zone 1: Baugrund Mauer ±{wall}, Turm ±{tower}, dazu 0…2 Plätze je Seed");
    }
    format!("Zone {level}: 1…3 Mauern, 1…3 Türme je Seed")
}

/// Der Händler steht in Stufe 1: er hängt in der hub.json an keiner Stufe und ist von Anfang an da.
fn merchant(hub: &Hub, level: i64) -> String {
    if level != 1 {
        return "–".to_owned();
    }
    let m = &hub.merchant;
    let (buy, sell) = (signed(m.buy_offset_units), signed(m.sell_offset_units));
    format!("Kaufen {buy}, Verkaufen {sell} (nur Tiefe {})", m.only_depth)
}

/// Tauscht den Text von der Start- bis zur End-Marke (beide inklusive) gegen `block`; der Rest bleibt byte-gleich, die
/// Zeilenenden des Blocks folgen dem Dokument. Fehlt eine Marke, kommt sie doppelt vor oder steht das Ende vor dem
/// Start, ist das ein Fehler.
pub(super) fn ersetzen(doc: &str, block: &str) -> Result<String, String> {
    let (from, to) = span(doc)?;
    let mut block = block.replace("\r\n", "\n");
    if doc.contains("\r\n") {
        block = block.replace('\n', "\r\n");
    }
    Ok(format!("{}{block}{}", &doc[..from], &doc[to..]))
}

/// Anfang der Start-Marke und Ende der End-Marke.
fn span(doc: &str) -> Result<(usize, usize), String> {
    let once = |mark: &str| {
        let mut found = doc.match_indices(mark).map(|(at, _)| at);
        found.next().filter(|_| found.next().is_none())
    };
    let (Some(from), Some(to)) = (once(START), once(END)) else {
        let (start, end) = (go_quoted(START), go_quoted(END));
        return Err(format!("marken {start} und {end} müssen je genau einmal vorkommen"));
    };
    if to < from {
        return Err("ende-marke steht vor der Start-Marke".to_owned());
    }
    Ok((from, to + END.len()))
}

/// Ergebnis von [`gegenprobe`].
pub(super) struct Gegenprobe {
    pub(super) same: bool,
    pub(super) ist: String,
    pub(super) soll: String,
}

/// Vergleicht den Block im Dokument mit dem aus `hub_json` erzeugten (Zeilenenden egal) und liefert beide Texte für die
/// Anzeige der Abweichung im Studio.
pub(super) fn gegenprobe(doc: &str, hub_json: &[u8]) -> Result<Gegenprobe, String> {
    let soll = block(hub_json)?;
    let (from, to) = span(doc)?;
    let ist = doc[from..to].replace("\r\n", "\n");
    Ok(Gegenprobe {
        same: ist == soll,
        ist,
        soll,
    })
}
