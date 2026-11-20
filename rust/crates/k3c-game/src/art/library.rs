//! Grafik-Bibliothek: Schrift, Rigs und die Sheets der gerade gebrauchten Level; Vorlage
//! `native/k3c-game/art/library.go`.
//!
//! Abweichungen von Go (RU7-README › Vorentscheidungen): Hochladen und Freigeben brauchen die Zeichenfläche
//! (`&mut dyn Canvas`); `LevelLoad` hält keinen Zeiger auf die Library, `step` und `abort` bekommen sie übergeben.
//! Eine nil-Library gibt es nicht (Aufrufer halten `Option<Library>`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use k3c_platform::Canvas;
use k3c_rig::paint::Assets;

use super::png::read_png;
use super::scales::{apply_scales, apply_sizes, without_sized};
use super::{ArtError, Budget, Font, Plan, Sheet, SheetSpec, load_font, plan_level, upload};

/// Pfade relativ zur Repo-Wurzel (Aufruf über `task rust:game:run`).
pub const DEFAULT_GRAFIK_DIR: &str = "public/grafik";
pub const DEFAULT_RIG_DIR: &str = "public/rig";
pub const DEFAULT_FONT_PATH: &str = "public/schrift/k3c.ttf";

/// Config sagt, woher die Grafik kommt. `rig_dir` leer = ohne Rig-Bibliothek (Tests). `strict_font`: fehlende Schrift
/// bricht ab (`default_config`: an, die Schrift liegt eingecheckt unter public/schrift).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Config {
    pub grafik_dir: PathBuf,
    pub rig_dir: PathBuf,
    pub font_path: PathBuf,
    pub strict_font: bool,
    /// Anzeige-Maßstab je Asset `gruppe/id` (`data_scales`); leer = überall 1.
    pub scales: BTreeMap<String, f64>,
    /// Größenstufen (`data/sizes.json`, nur im Look-Profil k2c); setzen die Größe der Gebäude und Objekte mit Eintrag.
    pub sizes: Option<k3c_data::sizes::Sizes>,
}

/// Liefert die Pfade im Repo.
pub fn default_config() -> Config {
    Config {
        grafik_dir: DEFAULT_GRAFIK_DIR.into(),
        rig_dir: DEFAULT_RIG_DIR.into(),
        font_path: DEFAULT_FONT_PATH.into(),
        strict_font: true,
        scales: super::scales::data_scales(),
        sizes: k3c_data::look::current()
            .sizes
            .then(|| k3c_data::sizes::from_source(&k3c_data::Source::from_env()))
            .flatten(),
    }
}

/// Schlüssel eines Sheets: (Asset `gruppe/id`, Animation); sortiert wie Gos `cmp.Or(asset, anim)`.
pub type SheetKey = (String, String);

/// Level → Assets `gruppe/id`.
pub type Levels = BTreeMap<String, Vec<String>>;

/// Library hält die Grafik: die Schrift und die Rigs dauerhaft, die Sheets der Level, die gerade gebraucht werden
/// (mehrere zugleich: zwei Spieler können auf Stufen verschiedener Level stehen; ein Sheet, das mehrere Level nennen,
/// liegt einmal auf der GPU). Nicht nebenläufig: Laden und Zeichnen laufen im Hauptfaden.
pub struct Library {
    pub font: Font,

    pub(super) cfg: Config,
    /// Rig-Bilder (`load_rigs`, `art/rigs.rs`).
    pub(super) rigs: Option<Assets>,
    levels: Levels,
    /// Asset → Animation → Sheet (zweistufig, damit `sheet` ohne Schlüssel-Objekt nachschlagen kann).
    sheets: BTreeMap<String, BTreeMap<String, Sheet>>,
    budget: Budget,
}

impl Library {
    /// Öffnet die Bibliothek: prüft den Rig-Ordner (die Bilder lädt `load_rigs` mit der Zeichenfläche) und lädt die Schrift. Sheets kommen erst mit `set_levels`.
    pub fn new(cfg: Config) -> Result<Self, ArtError> {
        if !cfg.rig_dir.as_os_str().is_empty() {
            super::rigs::check_rigs(&cfg.rig_dir)?;
        }
        let font = load_font(&cfg.font_path, cfg.strict_font)?;
        Ok(Self {
            font,
            cfg,
            rigs: None,
            levels: Levels::new(),
            sheets: BTreeMap::new(),
            budget: Budget::default(),
        })
    }

    /// Stellt die geladenen Sheets auf genau die Level in want (Level → Assets `gruppe/id`): lädt, was fehlt, und
    /// gibt frei, was kein Level aus want mehr braucht. Fehlt Grafik, bricht es vor jedem Hochladen ab (Fehler mit
    /// allen Pfaden, `Missing`) und der alte Stand bleibt. Das Texturbudget des neuen Stands steht im Log.
    pub fn set_levels(&mut self, want: &Levels, canvas: &mut dyn Canvas) -> Result<(), ArtError> {
        let mut ld = self.begin_levels(want)?;
        let n = ld.todo.len();
        ld.step(self, n, canvas).map(|_| ())
    }

    /// Plant das Laden der Level in want wie `set_levels`, lädt aber noch nichts. Fehlt Grafik, kommt der Fehler
    /// (`Missing`) schon hier.
    pub fn begin_levels(&self, want: &Levels) -> Result<LevelLoad, ArtError> {
        let plans = plan_levels(&self.cfg.grafik_dir, want)?;
        let (load, free) = diff_sheets(&self.sheet_keys(), &plans);
        Ok(LevelLoad {
            want: want.clone(),
            plans,
            todo: load,
            free,
            got: BTreeMap::new(),
            done: false,
        })
    }

    /// Liefert die Animation `anim` des Assets `gruppe/id` aus den geladenen Leveln, sonst `None`. Ohne Allokation.
    pub fn sheet(&self, asset: &str, anim: &str) -> Option<&Sheet> {
        self.sheets.get(asset)?.get(anim)
    }

    /// Meldet, ob das Asset (`gruppe/id`) vollständig ladbar ist (Manifest, PNG, Maße). Weltbilder fragen das vor
    /// `set_levels`, damit ein fehlendes Bild nie ein Level verhindert.
    pub fn usable(&self, asset: &str) -> bool {
        plan_level(&self.cfg.grafik_dir, "", &[asset.to_string()]).is_ok()
    }

    /// Die geladenen Sheets als Schlüssel (sortiert).
    fn sheet_keys(&self) -> BTreeSet<SheetKey> {
        self.sheets
            .iter()
            .flat_map(|(a, m)| m.keys().map(move |n| (a.clone(), n.clone())))
            .collect()
    }

    /// Nennt die geladenen Level sortiert (leer = keins).
    pub fn levels(&self) -> Vec<String> {
        self.levels.keys().cloned().collect()
    }

    /// Liefert das Texturbudget der geladenen Sheets.
    pub fn budget(&self) -> Budget {
        self.budget
    }

    /// Liefert den Speicher der bisher geladenen Rig-Bilder in Mebibyte.
    pub fn rig_mib(&self) -> f64 {
        self.rigs
            .as_ref()
            .map_or(0.0, |a| a.bytes as f64 / f64::from(1u32 << 20))
    }

    /// Die Konfiguration, mit der die Bibliothek geöffnet wurde.
    pub fn config(&self) -> &Config {
        &self.cfg
    }

    /// Zahl der geladenen Sheets (Tests und Log).
    pub fn sheet_count(&self) -> usize {
        self.sheets.values().map(BTreeMap::len).sum()
    }
}

/// LevelLoad ist ein `set_levels` in Schritten (B-949): `step` lädt einige Sheets, zwischen den Schritten kann die
/// Schleife einen Fortschritt zeichnen. Erst der letzte Schritt gibt frei und stellt den neuen Stand ein; bis dahin
/// bleibt der alte unverändert.
#[derive(Debug, Default)]
pub struct LevelLoad {
    want: Levels,
    plans: Vec<Plan>,
    todo: Vec<SheetSpec>,
    free: Vec<SheetKey>,
    got: BTreeMap<SheetKey, Sheet>,
    done: bool,
}

impl LevelLoad {
    /// Lädt bis zu n weitere Sheets und meldet, ob alles geladen und eingestellt ist. Bei einem Fehler sind die schon
    /// geladenen Sheets wieder frei und der alte Stand bleibt.
    pub fn step(&mut self, l: &mut Library, mut n: usize, canvas: &mut dyn Canvas) -> Result<bool, ArtError> {
        if self.done {
            return Ok(true);
        }
        while n > 0 && self.got.len() < self.todo.len() {
            let spec = &self.todo[self.got.len()];
            match upload_one(spec, canvas) {
                Ok(sh) => {
                    self.got.insert((spec.asset.clone(), spec.anim.clone()), sh);
                }
                Err(e) => {
                    free_sheets(&self.got, canvas);
                    self.got.clear(); // sonst gibt ein folgendes abort dieselben Texturen ein zweites Mal frei
                    return Err(e);
                }
            }
            n -= 1;
        }
        if self.got.len() < self.todo.len() {
            return Ok(false);
        }
        for (asset, anim) in &self.free {
            if let Some(s) = l.sheets.get_mut(asset).and_then(|m| m.remove(anim)) {
                free_sheet(&s, canvas);
            }
        }
        l.sheets.retain(|_, m| !m.is_empty());
        match &l.cfg.sizes {
            Some(z) => {
                apply_scales(&mut self.got, &without_sized(&l.cfg.scales, z));
                apply_sizes(&mut self.got, z);
            }
            None => apply_scales(&mut self.got, &l.cfg.scales),
        }
        for ((asset, anim), s) in std::mem::take(&mut self.got) {
            l.sheets.entry(asset).or_default().insert(anim, s);
        }
        l.levels = self.want.clone();
        l.budget = budget_of(&self.plans);
        self.log_loaded(l);
        self.done = true;
        Ok(true)
    }

    fn log_loaded(&self, l: &Library) {
        let (levels, rigs) = (format!("{:?}", l.levels()), format!("{:.1} MiB", l.rig_mib()));
        k3c_platform::log::info(
            "📂 Grafik geladen",
            &[
                ("level", &levels),
                ("neu", &self.todo.len()),
                ("frei", &self.free.len()),
                ("sheets", &l.budget),
                ("rigs", &rigs),
            ],
        );
    }

    /// Bricht ein laufendes Laden ab und gibt die schon geladenen Sheets frei; der alte Stand bleibt. Nach Ende oder
    /// Abbruch ohne Wirkung.
    pub fn abort(&mut self, canvas: &mut dyn Canvas) {
        if self.done {
            return;
        }
        free_sheets(&self.got, canvas);
        self.got.clear();
        self.done = true;
    }

    /// Anteil der geladenen Sheets, 0 … 1 (ohne Sheets: 1).
    pub fn progress(&self) -> f64 {
        if self.done || self.todo.is_empty() {
            return 1.0;
        }
        self.got.len() as f64 / self.todo.len() as f64
    }
}

/// Plant alle Level in fester Reihenfolge (nach Namen); alle Fehler gesammelt.
pub fn plan_levels(dir: &std::path::Path, want: &Levels) -> Result<Vec<Plan>, ArtError> {
    let mut out = Vec::with_capacity(want.len());
    for (lv, assets) in want {
        out.push(plan_level(dir, lv, assets)?);
    }
    Ok(out)
}

/// Vergleicht den geladenen Stand have mit den Plänen: load sind die geplanten Sheets, die fehlen (jedes einmal, in
/// Plan-Reihenfolge), free die geladenen, die kein Plan mehr nennt (sortiert).
pub fn diff_sheets(have: &BTreeSet<SheetKey>, plans: &[Plan]) -> (Vec<SheetSpec>, Vec<SheetKey>) {
    let mut need = BTreeSet::new();
    let mut load = Vec::new();
    for p in plans {
        for s in &p.sheets {
            let k = (s.asset.clone(), s.anim.clone());
            if !have.contains(&k) && !need.contains(&k) {
                load.push(s.clone());
            }
            need.insert(k);
        }
    }
    // BTreeSet liefert die Schlüssel schon sortiert wie `slices.SortFunc(asset, anim)` (Schlüssel eindeutig).
    let free = have.iter().filter(|k| !need.contains(*k)).cloned().collect();
    (load, free)
}

/// Zählt jedes Sheet der Pläne einmal (gleiche Sheets mehrerer Level liegen nur einmal auf der GPU).
pub fn budget_of(plans: &[Plan]) -> Budget {
    let mut b = Budget::default();
    let mut seen = BTreeSet::new();
    for p in plans {
        for s in &p.sheets {
            if seen.insert((s.asset.as_str(), s.anim.as_str())) {
                b.add(s.w, s.h);
            }
        }
    }
    b
}

/// Liest das PNG des Sheets und lädt es hoch.
fn upload_one(spec: &SheetSpec, canvas: &mut dyn Canvas) -> Result<Sheet, ArtError> {
    let rgba = read_png(&spec.path)
        .map_err(|e| ArtError::Missing(format!("Sheet {} nicht lesbar ({e})", spec.path.display())))?;
    Ok(upload(spec, &rgba.pix, canvas))
}

/// Gibt die GPU-Bilder frei.
fn free_sheets(m: &BTreeMap<SheetKey, Sheet>, canvas: &mut dyn Canvas) {
    for s in m.values() {
        free_sheet(s, canvas);
    }
}

fn free_sheet(s: &Sheet, canvas: &mut dyn Canvas) {
    if let Some(t) = s.texture() {
        canvas.free_texture(t);
    }
}
