//! `Data` sind alle Spiel-Daten aus einer Quelle, geladen und geprüft wie beim Start des Go-Spiels.

use std::collections::BTreeMap;

use crate::source::{EMBEDDED, Source};
use crate::{
    Fehler, balance_hints, balance_targets, biome, bosses, buildings, cargo, coop, difficulty, economy, enemies,
    events, festival, goals, guards, hermits, hub, island_sizes, light, monarch, mounts, native, paper, render, ruins,
    ship, sounds, story, stresstests, troops, waves, weather, world,
};

/// `Data` hält jede Spiel-Daten-Datei in ihrer Struktur.
#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    pub balance_hints: balance_hints::Hints,
    pub balance_targets: balance_targets::Targets,
    /// Biome nach Tiefe sortiert (stabil, wie `loadBiomes` in engine/sim).
    pub biomes: Vec<biome::Biome>,
    pub bosses: bosses::BossesFile,
    pub buildings: BTreeMap<String, buildings::BuildingData>,
    pub difficulty: difficulty::DifficultyFile,
    pub economy: economy::EconomyData,
    pub enemies: BTreeMap<String, enemies::EnemyData>,
    pub events: events::EventsFile,
    pub festival: festival::FestivalFile,
    pub goals: goals::Goals,
    pub guards: guards::GuardRules,
    pub hermits: hermits::HermitsFile,
    pub hub: hub::HubData,
    pub island_sizes: island_sizes::IslandSizesFile,
    pub light: light::Light,
    pub monarch: monarch::MonarchData,
    pub mounts: mounts::MountsFile,
    pub native: native::Native,
    pub paper: paper::Paper,
    pub render: render::Render,
    pub ruins: ruins::RuinRules,
    pub cargo: cargo::CargoFile,
    pub coop: coop::CoopFile,
    pub ship: ship::ShipFile,
    pub sounds: sounds::Sounds,
    pub story: story::StoryLimits,
    pub stresstests: stresstests::Stresstests,
    pub troops: BTreeMap<String, troops::TroopData>,
    pub waves: waves::WavesData,
    /// Wetter je Welt (RU62.5).
    pub weather: weather::WeatherFile,
    /// Die Standard-Welt (Kopf, Inseln roh).
    pub standard_world: world::WorldFile,
}

/// `biome_ids` sind die Biome der Quelle (Dateien `biomes/*.json`), sortiert nach Namen wie `fs.Glob`.
pub fn biome_ids(src: &Source) -> Result<Vec<String>, Fehler> {
    let mut ids: Vec<String> = match src {
        Source::Embedded => EMBEDDED
            .iter()
            .filter_map(|(n, _)| n.strip_prefix("biomes/")?.strip_suffix(".json").map(String::from))
            .collect(),
        Source::Dir(dir) => std::fs::read_dir(dir.join("biomes"))
            .map_err(|e| Fehler::Lesen(format!("data/biomes: {e}")))?
            .filter_map(|e| e.ok()?.file_name().to_str()?.strip_suffix(".json").map(String::from))
            .collect(),
    };
    ids.sort();
    Ok(ids)
}

/// `load_biomes` lädt die Biome, prüft ihre Gegner-Pools und sortiert stabil nach Tiefe (Go `loadBiomes`).
fn load_biomes(src: &Source, enemies: &BTreeMap<String, enemies::EnemyData>) -> Result<Vec<biome::Biome>, Fehler> {
    let mut out = Vec::new();
    for id in biome_ids(src)? {
        let b = biome::load(&id, &src.read(&format!("biomes/{id}.json"))?)?;
        biome::check_pool(&b, enemies)?;
        out.push(b);
    }
    out.sort_by_key(|b| b.depth);
    Ok(out)
}

/// `load_game` wie `load_all`, dazu mit dem Abstand der Hub-Plätze nach Gebäudebreite (LA3.1, `hub_layout`). Für das
/// Spiel; die Go-Vergleiche laden mit `load_all` die rohe `hub.json`.
pub fn load_game(src: &Source) -> Result<Data, Fehler> {
    let mut d = load_all(src)?;
    let sizes = crate::sizes::load(&src.read(crate::sizes::FILE)?)?;
    let half = d
        .biomes
        .iter()
        .fold(f64::INFINITY, |m, b| m.min(b.hub_width_units / 2.0));
    crate::hub_layout::spread(&mut d.hub, &sizes.sites, half)
        .map_err(|e| Fehler::Pruefung(format!("data/sizes.json: {e}")))?;
    Ok(d)
}

/// `load_all` lädt alle Spiel-Daten aus src und macht die Prüfungen, die Go beim Laden macht, auch die über
/// mehrere Dateien (Bosse, Events, Ruinen, Gegner-Pools der Biome).
pub fn load_all(src: &Source) -> Result<Data, Fehler> {
    let read = |name: &str| src.read(name);
    let enemies = enemies::load(&read(enemies::FILE)?)?;
    let biomes = load_biomes(src, &enemies)?;
    let depths: Vec<i64> = biomes.iter().map(|b| b.depth).collect();
    let buildings = buildings::load(&read(buildings::FILE)?)?;
    let mut events = events::load(&read(events::FILE)?)?;
    // Welt-Events (B-991, B-992) stehen in eigener Datei, damit das Golden von events.json bytegleich bleibt
    events.events.extend(events::load(&read(events::WELT_FILE)?)?.events);
    events::check_night_events(&events.events, &enemies)?;
    Ok(Data {
        balance_hints: balance_hints::load_hints(&read(balance_hints::FILE)?)?,
        balance_targets: balance_targets::load_targets(&read(balance_targets::FILE)?)?,
        bosses: bosses::load(&read(bosses::FILE)?, &enemies, &depths)?,
        ruins: ruins::load(&read(ruins::FILE)?, &buildings)?,
        difficulty: difficulty::load(&read(difficulty::FILE)?)?,
        economy: economy::load(&read(economy::FILE)?)?,
        goals: goals::load(&read(goals::FILE)?)?,
        guards: guards::load(&read(guards::FILE)?)?,
        hub: hub::load(&read(hub::FILE)?)?,
        island_sizes: island_sizes::load(&read(island_sizes::FILE)?)?,
        light: light::load(&read(light::FILE)?)?,
        monarch: monarch::load(&read(monarch::FILE)?)?,
        mounts: mounts::load(&read(mounts::FILE)?)?,
        festival: festival::load(&read(festival::FILE)?)?,
        hermits: hermits::load(&read(hermits::FILE)?)?,
        native: native::load(&read(native::FILE)?)?,
        paper: paper::load(&read(paper::FILE)?)?,
        render: render::load(&read(render::FILE)?)?,
        cargo: cargo::load(&read(cargo::FILE)?)?,
        coop: coop::load(&read(coop::FILE)?)?,
        ship: ship::load(&read(ship::FILE)?)?,
        sounds: sounds::load(&read(sounds::FILE)?)?,
        story: story::load(&read(story::FILE)?)?,
        stresstests: stresstests::load(&read(stresstests::FILE)?)?,
        troops: troops::load(&read(troops::FILE)?)?,
        waves: waves::load(&read(waves::FILE)?)?,
        weather: weather::load(&read(weather::FILE)?)?,
        standard_world: world::parse(&read("worlds/standard.json")?, "standard")?,
        biomes,
        buildings,
        enemies,
        events,
    })
}
