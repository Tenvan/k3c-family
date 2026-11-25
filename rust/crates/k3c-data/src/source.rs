//! Woher die Daten kommen: ein Ordner auf der Platte oder die eingebetteten Dateien (Go: data/embed.go).
//!
//! Wie in Go: `K3C_DATA_DIR`, sonst der Ordner `data` neben der EXE (wenn er `worlds` enthält), sonst eingebettet.
//! Eingebettet sind genau die Dateien von `//go:embed *.json biomes/*.json worlds/standard.json`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::Fehler;

/// Go `DirEnv`: der Ordner, aus dem beim Start gelesen wird.
pub const DIR_ENV: &str = "K3C_DATA_DIR";

/// `Source` ist die Quelle der Daten.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// Ein Ordner wie data/.
    Dir(PathBuf),
    /// Die beim Bauen eingebetteten Dateien.
    Embedded,
    /// Dateien im Speicher (Pfad → Inhalt), z. B. Daten mit Wert-Varianten (Go: `fstest.MapFS`).
    Mem(BTreeMap<String, String>),
}

/// Eingebettete Dateien: Name unter data/ und Inhalt.
pub const EMBEDDED: [(&str, &str); 43] = [
    (
        "asset-prompts.json",
        include_str!("../../../../data/asset-prompts.json"),
    ),
    (
        "assets-recherche.json",
        include_str!("../../../../data/assets-recherche.json"),
    ),
    (
        "assets-vorschlaege.json",
        include_str!("../../../../data/assets-vorschlaege.json"),
    ),
    (
        "balance-hints.json",
        include_str!("../../../../data/balance-hints.json"),
    ),
    (
        "balance-targets.json",
        include_str!("../../../../data/balance-targets.json"),
    ),
    ("biomes/cave.json", include_str!("../../../../data/biomes/cave.json")),
    (
        "biomes/crystal.json",
        include_str!("../../../../data/biomes/crystal.json"),
    ),
    (
        "biomes/forest.json",
        include_str!("../../../../data/biomes/forest.json"),
    ),
    (
        "biomes/ironhold.json",
        include_str!("../../../../data/biomes/ironhold.json"),
    ),
    ("biomes/mine.json", include_str!("../../../../data/biomes/mine.json")),
    ("bosses.json", include_str!("../../../../data/bosses.json")),
    ("buildings.json", include_str!("../../../../data/buildings.json")),
    ("cargo.json", include_str!("../../../../data/cargo.json")),
    ("coop.json", include_str!("../../../../data/coop.json")),
    ("difficulty.json", include_str!("../../../../data/difficulty.json")),
    ("economy.json", include_str!("../../../../data/economy.json")),
    ("enemies.json", include_str!("../../../../data/enemies.json")),
    ("events.json", include_str!("../../../../data/events.json")),
    ("welt-events.json", include_str!("../../../../data/welt-events.json")),
    ("goals.json", include_str!("../../../../data/goals.json")),
    ("guards.json", include_str!("../../../../data/guards.json")),
    ("hub.json", include_str!("../../../../data/hub.json")),
    ("island-sizes.json", include_str!("../../../../data/island-sizes.json")),
    ("light.json", include_str!("../../../../data/light.json")),
    ("look.json", include_str!("../../../../data/look.json")),
    ("monarch.json", include_str!("../../../../data/monarch.json")),
    ("mounts.json", include_str!("../../../../data/mounts.json")),
    ("festival.json", include_str!("../../../../data/festival.json")),
    ("hermits.json", include_str!("../../../../data/hermits.json")),
    ("bottle.json", include_str!("../../../../data/bottle.json")),
    ("native.json", include_str!("../../../../data/native.json")),
    ("paper.json", include_str!("../../../../data/paper.json")),
    ("render.json", include_str!("../../../../data/render.json")),
    ("ruins.json", include_str!("../../../../data/ruins.json")),
    ("ship.json", include_str!("../../../../data/ship.json")),
    ("sizes.json", include_str!("../../../../data/sizes.json")),
    ("sounds.json", include_str!("../../../../data/sounds.json")),
    ("story.json", include_str!("../../../../data/story.json")),
    ("stresstests.json", include_str!("../../../../data/stresstests.json")),
    ("troops.json", include_str!("../../../../data/troops.json")),
    ("waves.json", include_str!("../../../../data/waves.json")),
    ("weather.json", include_str!("../../../../data/weather.json")),
    (
        "worlds/standard.json",
        include_str!("../../../../data/worlds/standard.json"),
    ),
];

impl Source {
    /// `from_env` wählt wie Gos `data.Files`: `K3C_DATA_DIR`, sonst data neben der EXE, sonst eingebettet.
    pub fn from_env() -> Source {
        let env = std::env::var(DIR_ENV).unwrap_or_default();
        let dir = if env.is_empty() {
            beside_exe()
        } else {
            Some(PathBuf::from(env))
        };
        Source::choose(dir.as_deref())
    }

    /// `choose` liefert den Ordner dir, wenn es ihn gibt, sonst die eingebetteten Dateien (Go `choose`).
    pub fn choose(dir: Option<&Path>) -> Source {
        match dir {
            Some(d) if d.is_dir() => Source::Dir(d.to_path_buf()),
            _ => Source::Embedded,
        }
    }

    /// `read` liest die Datei name (z. B. `biomes/cave.json`). Der Fehler lautet wie bei Gos `loadFrom`:
    /// `data/<name>: open <name>: <Grund>`.
    pub fn read(&self, name: &str) -> Result<String, Fehler> {
        let fail = |why: &str| Fehler::Lesen(format!("data/{name}: open {name}: {why}"));
        match self {
            Source::Embedded => EMBEDDED
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, text)| text.to_string())
                .ok_or_else(|| fail("file does not exist")),
            Source::Mem(files) => files.get(name).cloned().ok_or_else(|| fail("file does not exist")),
            Source::Dir(dir) => std::fs::read_to_string(dir.join(name)).map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => fail(NOT_FOUND),
                _ => fail(&e.to_string()),
            }),
        }
    }

    /// `files` liefert alle Dateien der Quelle (Pfad mit `/` → Inhalt), sortiert; ein Ordner wird rekursiv gelesen
    /// (Go: `fs.WalkDir` über `data.Files`).
    pub fn files(&self) -> Result<BTreeMap<String, String>, Fehler> {
        match self {
            Source::Embedded => Ok(EMBEDDED.iter().map(|(n, t)| (n.to_string(), t.to_string())).collect()),
            Source::Mem(files) => Ok(files.clone()),
            Source::Dir(dir) => {
                let mut out = BTreeMap::new();
                walk(dir, "", &mut out).map_err(|e| Fehler::Lesen(format!("data/: {e}")))?;
                Ok(out)
            }
        }
    }
}

/// Liest den Ordner dir rekursiv; `prefix` ist der Pfad unter data/ (leer oder mit `/` am Ende).
fn walk(dir: &Path, prefix: &str, out: &mut BTreeMap<String, String>) -> std::io::Result<()> {
    for e in std::fs::read_dir(dir)? {
        let e = e?;
        let name = format!("{prefix}{}", e.file_name().to_string_lossy());
        if e.file_type()?.is_dir() {
            walk(&e.path(), &format!("{name}/"), out)?;
        } else {
            out.insert(name, std::fs::read_to_string(e.path())?);
        }
    }
    Ok(())
}

/// Text des Betriebssystems für eine fehlende Datei, wie Go ihn über `os.DirFS` meldet.
#[cfg(windows)]
const NOT_FOUND: &str = "The system cannot find the file specified.";
#[cfg(not(windows))]
const NOT_FOUND: &str = "no such file or directory";

/// `beside_exe` ist der Ordner data neben der laufenden EXE, wenn er `worlds` enthält (Go `besideExe`).
fn beside_exe() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?.join("data");
    dir.join("worlds").is_dir().then_some(dir)
}
