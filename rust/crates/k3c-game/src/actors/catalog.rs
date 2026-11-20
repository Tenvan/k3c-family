//! Port von `native/k3c-game/actors/catalog.go`.
//!
//! Katalog der Figuren: welche Spielrolle welches Papier-Asset zeigt, wie hoch, wie weit über dem Boden und ob als Rig
//! oder Sheet (data/paper.json, data/monarch.json, data/bosses.json). Reine Datei-Logik ohne Grafik.

use std::collections::BTreeMap;

use k3c_data::{Fehler, Source};
use serde::Deserialize;

use super::levels::by_depth;

/// unitPx sind Spiel-Pixel je Unit (UNIT_PX des Browser-Clients); Höhen in data/paper.json stehen in Spiel-Pixeln.
pub const UNIT_PX: f64 = 32.0;

/// bossHeightPx ist die Spielhöhe der Bosse in Spiel-Pixeln. data/paper.json nennt keine Bosse (der Browser-Client
/// zeichnet sie als Goblin); bis dort ein Eintrag steht, gilt diese Höhe (anderthalb Truppen).
pub const BOSS_HEIGHT_PX: f64 = 150.0;

/// bossAssets ordnet die Boss-IDs aus data/bosses.json ihren Papier-Assets zu (data/paper.json kennt keine Bosse).
pub const BOSS_ASSETS: [(&str, &str); 6] = [
    ("goblinLeader", "bosse/goblin-anfuehrer"),
    ("trollKing", "bosse/troll-koenig"),
    ("ratQueen", "bosse/ratten-koenigin"),
    ("lavaGolem", "bosse/lava-golem"),
    ("shardTitan", "bosse/splitter-titan"),
    ("crystalHeartWarden", "bosse/kristallherz-waechter"),
];

/// sheetAssets sind die Figuren mit Rig-Typ `requisite` (ohne Teile-Baum, native/k3c-rig/RIG.md): Sie bleiben Sheets.
/// catalog_tests.rs gleicht die Liste mit den Specs unter public/grafik ab.
pub const SHEET_ASSETS: [&str; 9] = [
    "gegner/fledermaus",
    "gegner/rattenschwarm",
    "gegner/grubengeist",
    "gegner/lavaschleim",
    "gegner/eisenkaefer",
    "gegner/feuergeist",
    "gegner/kristallspinne",
    "gegner/splitterling",
    "bosse/kristallherz-waechter",
];

/// Look sagt, wie eine Figurenart aussieht.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Look {
    /// Papier-Asset `gruppe/id` (Rig-Ordner bzw. Sheet)
    pub asset: String,
    /// Farbvariante, z. B. "spieler2" ("" = Standard)
    pub variant: String,
    /// Spielhöhe in Units (Rahmenhöhe des Assets)
    pub height: f64,
    /// Höhe des Fußpunkts über dem Boden in Units (Fledermaus)
    pub lift: f64,
    /// Deckkraft 0..1
    pub alpha: f64,
    /// aus Sheets statt als Rig
    pub sheet: bool,
}

/// Catalog sind die Looks aller Figurenarten.
#[derive(Clone, Debug, Default)]
pub struct Catalog {
    /// je Spieler-Index (zyklisch)
    pub players: Vec<Look>,
    /// Reittier des Monarchen `reittiere/<id>`
    pub mount: String,
    /// Welt-Reittiere nach Katalog-ID (`mounts` in data/paper.json): `reittiere/<id>` (RU62.9)
    pub world_mounts: BTreeMap<String, String>,
    /// Referenz-Tempo des Reittiers (Units/s) für die Gangart
    pub mount_ref: f64,
    /// nach Troop.Kind
    pub troops: BTreeMap<String, Look>,
    /// nach Enemy.Kind
    pub enemies: BTreeMap<String, Look>,
    /// nach Boss-ID (Enemy.Kind bei Enemy.Boss)
    pub bosses: BTreeMap<String, Look>,
    /// Gegner-Arten je Tiefe (enemies.json › depth, dazu Beschwörungen der Bosse)
    pub depth_kinds: BTreeMap<i64, Vec<String>>,
    /// Boss-IDs je Tiefe (bosses.json › depth)
    pub depth_bosses: BTreeMap<i64, Vec<String>>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct FigureSpec {
    #[serde(rename = "hoehe")]
    height: f64,
    y: f64,
    alpha: Option<f64>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct PaperFigures {
    #[serde(rename = "spieler")]
    players: Vec<FigureSpec>,
    #[serde(rename = "truppen")]
    troops: BTreeMap<String, FigureSpec>,
    #[serde(rename = "gegner")]
    enemies: BTreeMap<String, FigureSpec>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct PaperFile {
    #[serde(rename = "spieler")]
    players: Vec<String>,
    #[serde(rename = "reittiere")]
    mounts: BTreeMap<String, String>,
    #[serde(rename = "truppen")]
    troops: BTreeMap<String, String>,
    #[serde(rename = "gegner")]
    enemies: BTreeMap<String, String>,
    #[serde(rename = "figuren")]
    figures: PaperFigures,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MonarchBase {
    #[serde(alias = "Speed")]
    speed: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MonarchMount {
    sprite: String,
    #[serde(rename = "speedFactor")]
    speed_factor: f64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MonarchFile {
    base: MonarchBase,
    mount: MonarchMount,
}

/// bossAbility ist der Teil einer Boss-Fähigkeit, der eine Figur ins Level bringt (summon).
#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct BossAbility {
    pub enemy: String,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct BossPhase {
    pub ability: Option<BossAbility>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct BossEntry {
    pub id: String,
    pub base: String,
    pub depth: i64,
    pub ability: Option<BossAbility>,
    pub phases: Vec<BossPhase>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct BossesFile {
    bosses: Vec<BossEntry>,
    #[serde(rename = "endBosses")]
    end_bosses: Vec<BossEntry>,
}

/// LoadCatalog liest den Katalog aus den Daten; alle Lücken (Rolle ohne Höhe, Boss ohne Asset) kommen gesammelt.
pub fn load_catalog(src: &Source) -> Result<Catalog, Fehler> {
    load_catalog_from(&|name| src.read(name))
}

/// Wie `load_catalog`, aber über eine beliebige Leseart (Go: `data.FS`; die Tests reichen eigene Dateien herein).
pub fn load_catalog_from(read: &dyn Fn(&str) -> Result<String, Fehler>) -> Result<Catalog, Fehler> {
    let mut errs: Vec<String> = Vec::new();
    let p: PaperFile = read_json(read, "paper.json", &mut errs);
    let m: MonarchFile = read_json(read, "monarch.json", &mut errs);
    let b: BossesFile = read_json(read, "bosses.json", &mut errs);
    let e: BTreeMap<String, serde_json::Value> = read_json(read, "enemies.json", &mut errs);
    if !errs.is_empty() {
        return Err(Fehler::Pruefung(errs.join("\n")));
    }
    let mut c = Catalog {
        mount_ref: m.base.speed * m.mount.speed_factor,
        ..Catalog::default()
    };
    let mount = match p.mounts.get(&m.mount.sprite) {
        Some(v) => v.clone(),
        None => {
            errs.push(format!(
                "paper.json › reittiere: kein Eintrag für monarch.json › mount.sprite {:?}",
                m.mount.sprite
            ));
            String::new()
        }
    };
    c.mount = format!("reittiere/{mount}");
    c.world_mounts = p
        .mounts
        .iter()
        .map(|(k, v)| (k.clone(), format!("reittiere/{v}")))
        .collect();
    for (i, r) in p.players.iter().enumerate() {
        match p.figures.players.get(i) {
            Some(s) => c.players.push(look(r, s)),
            None => errs.push(format!(
                "paper.json › figuren.spieler: Höhe für Spieler {} fehlt",
                i + 1
            )),
        }
    }
    errs.extend(fill(&mut c.troops, &p.troops, &p.figures.troops, "truppen"));
    errs.extend(fill(&mut c.enemies, &p.enemies, &p.figures.enemies, "gegner"));
    let all: Vec<BossEntry> = b.bosses.iter().chain(b.end_bosses.iter()).cloned().collect();
    errs.extend(add_bosses(&mut c, &all));
    if c.players.is_empty() {
        errs.push("paper.json › spieler ist leer".to_string());
    }
    (c.depth_kinds, c.depth_bosses) = by_depth(&e, &all);
    if !errs.is_empty() {
        return Err(Fehler::Pruefung(errs.join("\n")));
    }
    if k3c_data::look::current().sizes
        && let Some(z) = read("sizes.json").ok().and_then(|raw| k3c_data::sizes::load(&raw).ok())
    {
        super::sizes::apply(&mut c, &z);
    }
    Ok(c)
}

/// Legt die Looks der Bosse an (Endboss ohne eigenes Asset: das Asset der Basisart, B-993).
fn add_bosses(c: &mut Catalog, all: &[BossEntry]) -> Vec<String> {
    let mut errs = Vec::new();
    for x in all {
        let asset = match BOSS_ASSETS.iter().find(|(id, _)| *id == x.id) {
            Some((_, a)) => (*a).to_string(),
            None => {
                let Some(base) = c.enemies.get(&x.base) else {
                    errs.push(format!(
                        "bosses.json: Boss {:?} hat kein Asset (actors › bossAssets) und keine Basisart {:?}",
                        x.id, x.base
                    ));
                    continue;
                };
                base.asset.clone()
            }
        };
        let spec = FigureSpec {
            height: BOSS_HEIGHT_PX,
            ..FigureSpec::default()
        };
        c.bosses.insert(x.id.clone(), look(&asset, &spec));
    }
    errs
}

/// fill legt je Rolle aus refs (Rolle → Asset) den Look mit seiner Höhe aus specs an.
fn fill(
    out: &mut BTreeMap<String, Look>,
    refs: &BTreeMap<String, String>,
    specs: &BTreeMap<String, FigureSpec>,
    group: &str,
) -> Vec<String> {
    let mut errs = Vec::new();
    for (kind, r) in refs {
        match specs.get(kind) {
            Some(s) if s.height > 0.0 => {
                out.insert(kind.clone(), look(r, s));
            }
            _ => errs.push(format!("paper.json › figuren.{group}.{kind}: Höhe fehlt")),
        }
    }
    errs
}

/// look baut einen Look aus `gruppe/id[@variante]` und den Maßen in Spiel-Pixeln.
fn look(r: &str, s: &FigureSpec) -> Look {
    let (asset, variant) = r.split_once('@').unwrap_or((r, ""));
    Look {
        asset: asset.to_string(),
        variant: variant.to_string(),
        height: s.height / UNIT_PX,
        lift: s.y / UNIT_PX,
        alpha: s.alpha.unwrap_or(1.0),
        sheet: SHEET_ASSETS.contains(&asset),
    }
}

fn read_json<T: Default + serde::de::DeserializeOwned>(
    read: &dyn Fn(&str) -> Result<String, Fehler>,
    name: &str,
    errs: &mut Vec<String>,
) -> T {
    let raw = match read(name) {
        Ok(r) => r,
        Err(e) => {
            errs.push(e.to_string());
            return T::default();
        }
    };
    match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            errs.push(format!("data/{name}: {e}"));
            T::default()
        }
    }
}

impl Catalog {
    /// Player liefert den Look des Monarchen mit Spieler-Index i.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn player(&self, i: i64) -> Look {
        let n = self.players.len() as i64;
        self.players[(((i % n) + n) % n) as usize].clone()
    }

    /// `mount_of`: Rig-Pfad des Reittiers eines Monarchen mit Welt-Reittier `id` (leer oder unbekannt: das Standard-Pferd).
    pub fn mount_of(&self, id: &str) -> &str {
        self.world_mounts
            .get(id)
            .filter(|_| !id.is_empty())
            .unwrap_or(&self.mount)
    }

    /// Enemy liefert den Look eines Gegners bzw. Bosses (`None`: unbekannte Art).
    pub fn enemy(&self, kind: &str, boss: bool) -> Option<Look> {
        if boss {
            return self.bosses.get(kind).cloned();
        }
        self.enemies.get(kind).cloned()
    }

    /// SheetAssets nennt die Assets, die als Sheet gezeichnet werden, sortiert und je einmal.
    pub fn sheet_assets(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for m in [&self.troops, &self.enemies, &self.bosses] {
            for l in m.values() {
                if l.sheet {
                    out.push(l.asset.clone());
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod catalog_tests;
