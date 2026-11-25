//! Automatisches Balancing von `sim balance` (Port von `engine/simcli/tune.go`, RU40.1): `--tune sweep|search` mit
//! `--targets` stimmt ein bis zwei Werte aus `data/` gegen die Ziele ab und schreibt `tuning-<Zeit>.json`. `data/` bleibt
//! unverändert; verfehlte Ziele sind kein Fehler.
//!
//! `k3c-balance` meldet den Fortschritt je Punkt aus einem Rückruf (`Send`, `'static`); die Abstimmung läuft darum in einem
//! eigenen Thread, die Zeilen `progress` schreibt der Hauptthread. Der Abbruch wird nach jedem Punkt geprüft (Go: nach jedem
//! Lauf, `TuneOptions` hat in Rust keinen Rückruf je Lauf; Ticket in der Folge).

use std::panic::resume_unwind;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Instant;

use k3c_balance::tune::{Tune, TuneOptions, Tuning};
use k3c_balance::vary::{current_data, set_current_data};

use super::RunError;
use super::balance::{ABORT_TEXT, Env, fail, io};
use super::flags::Parsed;
use super::targets::{TargetOpts, stamp};

/// Die Optionen der Abstimmung (Go: `tuneOpts`).
pub struct TuneOpts {
    pub mode: String,
    pub params: Vec<String>,
    pub steps: i64,
    pub budget: i64,
}

impl TuneOpts {
    /// `None` ohne `--tune`.
    pub fn from(p: &Parsed) -> Option<TuneOpts> {
        let mode = p.text("tune");
        (!mode.is_empty()).then(|| TuneOpts {
            mode,
            params: p.list("param"),
            steps: p.int("steps"),
            budget: p.int("budget"),
        })
    }
}

/// Stimmt die Parameter gegen die Ziele ab und schreibt `tuning-<Zeit>.json` nach `--report-dir`.
pub fn run(o: &TargetOpts, t: &TuneOpts, v: &mut Env) -> Result<(), RunError> {
    let ts = o.targets(v)?;
    let start = Instant::now();
    let (tx, rx) = channel::<(usize, usize)>();
    let stop = v.stop.clone();
    let opts = TuneOptions {
        mode: t.mode.clone(),
        params: t.params.clone(),
        steps: t.steps,
        budget: t.budget,
        seeds: o.seeds,
        variants: o.variants.clone(),
        progress: Some(Box::new(move |done, total| {
            let _ = tx.send((done, total));
            if stop.is_set() {
                resume_unwind(Box::new(ABORT_TEXT));
            }
        })),
    };
    let data = current_data(); // die Daten von `--data` gelten auch im Thread der Abstimmung
    let joined = std::thread::scope(|s| {
        let ts = &ts;
        let worker = s.spawn(move || {
            set_current_data(data);
            ts.tune(opts)
        });
        for (done, total) in rx {
            if o.progress {
                v.out.progress(done as i64, total as i64);
            }
        }
        worker.join()
    });
    let res = match joined {
        Ok(r) => r.map_err(|e| fail(e, &v.stop))?,
        Err(p) if p.downcast_ref::<&str>().is_some_and(|s| *s == ABORT_TEXT) => return Err(RunError::Aborted),
        Err(p) => resume_unwind(p),
    };
    write_result(&o.dir, &res, start, v)
}

fn write_result(dir: &str, res: &Tuning, start: Instant, v: &mut Env) -> Result<(), RunError> {
    std::fs::create_dir_all(dir).map_err(|e| io(&e))?;
    let json = res.json().map_err(|e| RunError::Failed(e.to_string()))?;
    let path = Path::new(dir)
        .join(format!("tuning-{}.json", stamp()))
        .to_string_lossy()
        .into_owned();
    std::fs::write(&path, json).map_err(|e| io(&e))?;
    v.out.report(std::slice::from_ref(&path));
    v.logf(&format!(
        "💾 k3c-game sim: {} Punkte in {}s, Ergebnis {path}\n{}",
        res.points.len(),
        start.elapsed().as_secs(),
        res.reason
    ));
    Ok(())
}
