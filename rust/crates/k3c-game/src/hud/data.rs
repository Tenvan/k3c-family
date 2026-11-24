//! Port von `native/k3c-game/hud/data.go`.
//!
//! Werte aus data/ für die Anzeige: Skill-Katalog in der Reihenfolge der Datei (monarch.json › skills; wie
//! Object.entries im Browser), Namen der Linien, Gold-Maximum, Bogen-Regal, Namen der Gebäude und Phasenzahl der
//! Endbosse. Einmal beim Laden gelesen; ein Lesefehler lässt die Werte leer (die Anzeige zeigt dann weniger, nie
//! falsch).
//!
//! Abweichung von Go: Die Paket-Variablen (`skillLines`, `skillCatalog`, `purseMaxGold`, `buildingInfo`,
//! `endBossPhases`) sind Funktionen über `OnceLock`; `orderedJSON` ersetzt `k3c_data::ordered::Ordered`.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use k3c_data::Source;
use k3c_data::monarch::MonarchDataLines;
use k3c_data::ordered::Ordered;
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// SkillInfo ist ein Eintrag aus monarch.json › skills.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct SkillInfo {
    /// kommt aus dem Schlüssel, nicht aus dem Wert
    #[serde(skip)]
    pub id: String,
    #[serde(rename = "Line", alias = "line")]
    pub line: String,
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "Tier", alias = "tier")]
    pub tier: i64,
}

/// LineInfo ist eine Linie aus monarch.json › lines (Reihenfolge der Datei).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineInfo {
    pub id: String,
    pub name: String,
}

/// BuildingInfo ist ein Gebäude aus buildings.json (Name, Bogen- und Hammer-Regal).
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct BuildingInfo {
    #[serde(rename = "Name", alias = "name")]
    pub name: String,
    #[serde(rename = "BowRack", alias = "bowRack")]
    pub bow_rack: i64,
    #[serde(rename = "HammerRack", alias = "hammerRack")]
    pub hammer_rack: i64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MonarchFile {
    #[serde(rename = "Lines", alias = "lines")]
    lines: Ordered<MonarchDataLines>,
    #[serde(rename = "Skills", alias = "skills")]
    skills: Ordered<SkillInfo>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct EconomyFile {
    #[serde(rename = "Purse", alias = "purse")]
    purse: PurseFile,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct PurseFile {
    #[serde(rename = "MaxGold", alias = "maxGold")]
    max_gold: i64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct BossesFile {
    #[serde(rename = "endBosses")]
    end_bosses: Vec<EndBoss>,
    bosses: Vec<EndBoss>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct EndBoss {
    #[serde(rename = "ID", alias = "id")]
    id: String,
    #[serde(rename = "Phases", alias = "phases")]
    phases: Vec<serde::de::IgnoredAny>,
    name: String,
}

/// skill_lines sind die Linien aus monarch.json › lines in Dateireihenfolge.
pub fn skill_lines() -> &'static [LineInfo] {
    &monarch().0
}

/// skill_catalog ist der Skill-Katalog aus monarch.json › skills in Dateireihenfolge.
pub fn skill_catalog() -> &'static [SkillInfo] {
    &monarch().1
}

/// purse_max_gold ist das Gold-Maximum aus economy.json › purse.maxGold.
pub fn purse_max_gold() -> i64 {
    static M: OnceLock<i64> = OnceLock::new();
    *M.get_or_init(|| read_json::<EconomyFile>("economy.json").purse.max_gold)
}

/// building_info sind Name, Bogen- und Hammer-Regal je Gebäude aus buildings.json.
pub fn building_info() -> &'static BTreeMap<String, BuildingInfo> {
    static B: OnceLock<BTreeMap<String, BuildingInfo>> = OnceLock::new();
    B.get_or_init(|| read_json("buildings.json"))
}

/// end_boss_phases ist die Phasenzahl je Endboss (bosses.json › endBosses).
pub fn end_boss_phases() -> &'static BTreeMap<String, i64> {
    static P: OnceLock<BTreeMap<String, i64>> = OnceLock::new();
    P.get_or_init(read_boss_phases)
}

/// boss_names sind die Namen aller Bosse aus bosses.json (Mini- und Endbosse), Rückfall für Namen ohne Text (B-993).
pub fn boss_names() -> &'static BTreeMap<String, String> {
    static N: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    N.get_or_init(|| {
        let b: BossesFile = read_json("bosses.json");
        b.bosses
            .into_iter()
            .chain(b.end_bosses)
            .filter(|e| !e.name.is_empty())
            .map(|e| (e.id, e.name))
            .collect()
    })
}

/// read_json liest eine Datei aus data/; Fehler landen im Log, das Ergebnis bleibt dann leer.
fn read_json<T: DeserializeOwned + Default>(name: &str) -> T {
    let res = Source::from_env()
        .read(name)
        .map_err(|e| e.to_string())
        .and_then(|raw| serde_json::from_str::<T>(&raw).map_err(|e| e.to_string()));
    match res {
        Ok(v) => v,
        Err(e) => {
            k3c_platform::log::error("💥 HUD-Werte nicht lesbar", &[("datei", &name), ("err", &e)]);
            T::default()
        }
    }
}

/// monarch liest Linien und Skills aus monarch.json (einmal).
fn monarch() -> &'static (Vec<LineInfo>, Vec<SkillInfo>) {
    static M: OnceLock<(Vec<LineInfo>, Vec<SkillInfo>)> = OnceLock::new();
    M.get_or_init(read_monarch)
}

/// read_monarch liest Linien und Skills aus monarch.json.
fn read_monarch() -> (Vec<LineInfo>, Vec<SkillInfo>) {
    let m: MonarchFile = read_json("monarch.json");
    let lines = m
        .lines
        .0
        .into_iter()
        .map(|e| LineInfo {
            id: e.key,
            name: e.value.name,
        })
        .collect();
    let skills = m
        .skills
        .0
        .into_iter()
        .map(|e| SkillInfo { id: e.key, ..e.value })
        .collect();
    (lines, skills)
}

/// read_boss_phases liest die Phasenzahl je Endboss aus bosses.json.
fn read_boss_phases() -> BTreeMap<String, i64> {
    let b: BossesFile = read_json("bosses.json");
    let mut out = BTreeMap::new();
    for e in b.end_bosses {
        out.insert(e.id, e.phases.len() as i64);
    }
    out
}

/// line_index ist die Stelle der Linie in monarch.json › lines (unbekannt: hinter allen).
pub fn line_index(line: &str) -> usize {
    for (i, l) in skill_lines().iter().enumerate() {
        if l.id == line {
            return i;
        }
    }
    skill_lines().len()
}

/// line_name ist der Name der Linie aus den Daten, sonst ihre ID.
pub fn line_name(line: &str) -> String {
    for l in skill_lines() {
        if l.id == line && !l.name.is_empty() {
            return l.name.clone();
        }
    }
    line.to_string()
}
