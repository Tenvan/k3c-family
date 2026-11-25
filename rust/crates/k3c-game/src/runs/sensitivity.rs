//! Sensitivität und Grad-Kurven von `sim balance` (Port von `engine/simcli/sensitivity.go`, RU40.1): `--sensitivity`,
//! `--vary` (Pfade) und `--curves` rechnen Variationen der Werte aus `data/` und schreiben `sensitivity-<Zeit>.json` und
//! `.md` nach `--report-dir`. `--vary datei.json:pfad=wert` ist dagegen eine feste Variante für `--targets`.

use std::path::Path;
use std::time::Instant;

use k3c_balance::curves::CURVE_NIGHTS;
use k3c_balance::sensitivity::{Sensitivity, SensitivityOptions};
use k3c_balance::targets::default_targets;
use k3c_balance::vary::DEFAULT_PATHS;

use super::RunError;
use super::balance::{Env, fail, io, split};
use super::flags::Parsed;
use super::targets::stamp;

/// Die Optionen aus `--sensitivity` und `--vary` (Go: `sensOpts`).
#[derive(Debug, Default)]
pub struct VaryOpts {
    /// Variationen rechnen (`--sensitivity` oder ein Pfad in `--vary`).
    pub vary: bool,
    /// Pfade `datei.json:a.b.c`; leer = `DEFAULT_PATHS`.
    pub paths: Vec<String>,
    /// Wert-Varianten `datei.json:a.b.c=wert` (nur `--targets`, WZ28.4).
    pub variants: Vec<String>,
}

impl VaryOpts {
    /// `--vary` darf mehrfach und mit Komma stehen; ein Teil mit `=` ist eine Wert-Variante, sonst ein Pfad.
    pub fn from(p: &Parsed) -> VaryOpts {
        let mut o = VaryOpts {
            vary: p.flag("sensitivity"),
            ..VaryOpts::default()
        };
        for part in p.list("vary").iter().flat_map(|s| split(s)) {
            if part.contains('=') {
                o.variants.push(part);
            } else {
                o.vary = true;
                o.paths.push(part);
            }
        }
        o
    }
}

/// Rechnet Variationen und/oder Grad-Kurven und schreibt `sensitivity-<Zeit>.json` und `.md` nach `--report-dir`.
pub fn run(p: &Parsed, vary: &VaryOpts, v: &mut Env) -> Result<(), RunError> {
    let ts = default_targets().map_err(|e| fail(e, &v.stop))?;
    let mut opt = SensitivityOptions {
        seeds: if p.is_set("seeds") { p.int("seeds") } else { 0 },
        tick: Some(v.tick()),
        ..SensitivityOptions::default()
    };
    if vary.vary {
        opt.paths = if vary.paths.is_empty() {
            DEFAULT_PATHS.iter().map(ToString::to_string).collect()
        } else {
            vary.paths.clone()
        };
    }
    if p.flag("curves") {
        opt.nights = CURVE_NIGHTS;
    }
    let start = Instant::now();
    let rep = ts.sensitivity(&opt).map_err(|e| fail(e, &v.stop))?;
    let json = rep.json().map_err(|e| RunError::Failed(e.to_string()))?;
    let dir = p.text("report-dir");
    std::fs::create_dir_all(&dir).map_err(|e| io(&e))?;
    let stem = Path::new(&dir)
        .join(format!("sensitivity-{}", stamp()))
        .to_string_lossy()
        .into_owned();
    std::fs::write(format!("{stem}.json"), json).map_err(|e| io(&e))?;
    std::fs::write(format!("{stem}.md"), rep.markdown()).map_err(|e| io(&e))?;
    v.out.report(&[format!("{stem}.json"), format!("{stem}.md")]);
    v.logf(&format!(
        "💾 k3c-game sim: Sensitivität mit {} Seeds in {}s, Bericht {stem}.json und {stem}.md",
        rep.seeds,
        start.elapsed().as_secs()
    ));
    Ok(())
}
