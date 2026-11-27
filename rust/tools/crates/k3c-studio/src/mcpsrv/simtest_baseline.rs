//! `sim_test action baseline` (Port von `simtest_baseline.go`, B-640): ohne `saveBaseline` die gesicherten Baselines
//! auflisten, mit `id` und `saveBaseline` einen durchgelaufenen Offline-Lauf unter einem Namen sichern. Ablage
//! `reports/baseline/<name>/`.

use std::path::Path;

use super::simtest::{MAX_STATUS_LINES, unknown_run};
use super::simtest_seam::Sim;
use super::simtest_spec::SimTestIn;
use crate::balance::baselines::list_baselines;
use crate::kitx::go_quoted;
use crate::simrun::DONE;
use crate::simrun::baseline::run_report;

/// Blockiert (Dateien lesen und schreiben).
pub(super) fn baseline(sim: &Sim, root: &Path, input: &SimTestIn) -> Result<String, String> {
    if input.save_baseline.is_empty() {
        return list(root);
    }
    let id = &input.id;
    if id.is_empty() {
        return Err("sim_test baseline: id des durchgelaufenen Laufs fehlt".to_owned());
    }
    let info = sim.registry.get(root, id).map_err(|_| unknown_run(id))?;
    if info.state != DONE {
        return Err(format!(
            "lauf {id} ist {}; gesichert werden nur durchgelaufene Läufe",
            info.state
        ));
    }
    let report = run_report(root, id)?;
    // derselbe Weg wie „Als Baseline sichern“ der Balancing-Seite (B-662)
    let saved = sim
        .registry
        .save_baseline(root, &report, &input.save_baseline, input.ohne_pass)?;
    Ok(format!(
        "💾 Baseline {} gesichert aus {}: {} Seeds ({}), Bots {}, Welt {}\nVergleichen: sim_test {{action: start, baseline: {}}}",
        saved.name,
        saved.run,
        saved.seeds,
        saved.seed_mode,
        saved.bots.join(","),
        saved.worlds.join(","),
        go_quoted(&saved.name)
    ))
}

fn list(root: &Path) -> Result<String, String> {
    let all = list_baselines(root)?;
    if all.is_empty() {
        return Ok(
            "keine gesicherten Baselines; sichern mit sim_test {action: baseline, id, saveBaseline: M1}".to_owned(),
        );
    }
    let lines: Vec<String> = all
        .iter()
        .take(MAX_STATUS_LINES)
        .map(|b| {
            format!(
                "{} · {} · {} · {} Seeds · Bots {}",
                b.name,
                b.saved,
                b.run,
                b.seeds,
                b.bots.join(",")
            )
        })
        .collect();
    Ok(lines.join("\n"))
}
