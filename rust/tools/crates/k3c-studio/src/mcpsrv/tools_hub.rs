//! Gebäude-Planer als MCP-Tools (Port von `tools_hub.go`, B-923): `data/hub.json` als Matrix Hub-Stufe × Bauplatz,
//! Prüfung, Abgleich mit der Regeltabelle und geprüftes Speichern. Dieselbe Logik wie die Seite Gebäude-Planer
//! (`hubplan`); die Tools heißen `hub_*`, damit sie nicht mit den Planungs-Tools `plan_*` kollidieren. Eingebunden aus
//! `tools.rs`.

use std::fmt::Write as _;
use std::path::Path;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Map, Value};

use super::super::kit::Workspace;
use super::super::tools_world::blocking;
use super::Studio;
use crate::hubplan::{Editor, HubFindingView, HubRow, HubRulesView};
use crate::kitx::{go_marshal, go_quoted};

#[path = "tools_hub_merge.rs"]
mod merge;
#[cfg(test)]
#[path = "tools_hub_tests.rs"]
mod tests;

/// Zeilen der Matrix in einer Antwort.
const ROW_LIMIT: usize = 60;
/// Abweichende Zeilen der Regeltabelle in einer Antwort.
const DIFF_LIMIT: usize = 8;
/// Stellen des Stands (SHA-256) in der Antwort.
const STAND_LEN: usize = 12;

/// Argumente von `hub_check`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct CheckIn {
    #[serde(default)]
    #[schemars(
        description = "JSON-Merge-Patch (RFC 7396) auf data/hub.json: null löscht, Objekte werden gemischt, Listen (sites, islandSites, levels) ganz angeben"
    )]
    patch: Option<Map<String, Value>>,
    #[serde(default)]
    #[schemars(description = "stattdessen der ganze Text einer hub.json")]
    candidate: String,
}

/// Argumente von `hub_save`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct SaveIn {
    #[serde(default)]
    #[schemars(
        description = "JSON-Merge-Patch (RFC 7396) auf data/hub.json: null löscht, Objekte werden gemischt, Listen (sites, islandSites, levels) ganz angeben"
    )]
    patch: Option<Map<String, Value>>,
    #[serde(default)]
    #[schemars(description = "stattdessen der ganze Text einer hub.json")]
    candidate: String,
    #[serde(default)]
    #[schemars(
        description = "Pflicht: Stand aus hub_get (Zeile Stand, hub=…); ist die Datei seitdem geändert, wird nichts geschrieben"
    )]
    stand: String,
}

/// Die Tools des Gebäude-Planers (B-923).
#[tool_router(router = hub_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Gebäude-Planer: data/hub.json als Matrix Hub-Stufe × Bauplatz (Platz, ab Stufe, Tiefe), Befunde der Prüfung (sim.ValidateHubData), Abweichung der Regeltabelle und Stand für hub_save. Nutze es bei: wann wird welches Gebäude frei, vor jeder Änderung an data/hub.json. Statt: data/hub.json und docs/rules/materialien-gebaeude.md lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn hub_get(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        blocking(move || text(&editor(&ws.root), "")).await
    }

    #[tool(
        description = "Gebäude-Planer: prüft eine Änderung an data/hub.json (patch oder ganzer candidate), ohne zu schreiben: Matrix, Befunde, Abweichung der Regeltabelle. Nutze es bei: Änderung an Plätzen oder Hub-Stufen vorab prüfen. Statt: hub.json von Hand ändern und go test fahren.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn hub_check(
        &self,
        Parameters(input): Parameters<CheckIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || {
            let editor = editor(&ws.root);
            let cand = candidate(&editor, input.patch, input.candidate)?;
            text(&editor, &cand)
        })
        .await
    }

    #[tool(
        description = "Gebäude-Planer: schreibt eine geprüfte Änderung in data/hub.json (format-stabil, ein Wert eine Zeile) und den erzeugten Block in docs/rules/materialien-gebaeude.md; mit Befunden, bei geänderter Datei (stand) oder ohne Änderung wird nichts geschrieben. Eine REG-Änderung: Commit mit Begründung. Nutze es bei: Plätze, Hub-Stufen, Händler oder Baugrund ändern. Statt: data/hub.json von Hand ändern.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn hub_save(
        &self,
        Parameters(input): Parameters<SaveIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || save(&editor(&ws.root), input)).await
    }
}

/// Der Planer des Checkouts `root`. Die Unit-Tests rechnen mit der Spiel-EXE der Repo-Wurzel auf den Daten ihres
/// Fixture-Checkouts (Gos Ersatz `hubEngine`).
fn editor(root: &Path) -> Editor {
    #[cfg(not(test))]
    let engine = crate::hubplan::exe(root);
    #[cfg(test)]
    let engine = crate::hubplan::editor_tests::fixture_engine(root);
    Editor::new(root, engine)
}

/// Der Text einer hub.json aus `patch` (auf der Datei des Checkouts) oder `candidate`; leer = die Datei selbst.
fn candidate(editor: &Editor, patch: Option<Map<String, Value>>, candidate: String) -> Result<String, String> {
    let Some(patch) = patch else {
        return Ok(candidate);
    };
    if !candidate.is_empty() {
        return Err("patch oder candidate, nicht beides".to_owned());
    }
    let current = editor.plan();
    if !current.error.is_empty() {
        return Err(current.error);
    }
    merge::merge_patch(&current.raw, &go_marshal(&Value::Object(patch)))
        .map_err(|e| format!("patch nicht anwendbar: {e}"))
}

/// Die Antwort zu `data/hub.json` (`cand` leer) bzw. zu einem Kandidaten: Kopf, Stand, Matrix, Befunde, Regeltabelle.
fn text(editor: &Editor, cand: &str) -> Result<String, String> {
    let view = if cand.is_empty() {
        editor.plan()
    } else {
        editor.check(cand)
    };
    if !view.error.is_empty() {
        return Err(view.error);
    }
    let mut head = view.path.clone();
    if !cand.is_empty() {
        head += " (Kandidat, nichts geschrieben)";
    }
    let rows = &view.matrix.rows;
    let mut out = format!(
        "{head} · {} Hub-Stufen · {} Bauplätze · Prüfung: {}\n",
        view.matrix.levels,
        rows.len(),
        findings_label(view.findings.len())
    );
    if cand.is_empty() {
        let (hub, rules) = (short(&view.stand.hub), short(&view.stand.rules));
        let _ = writeln!(out, "Stand: hub={hub} rules={rules}");
    }
    out.push_str("Bauplätze (Schlüssel · Liste#Index · ab Hub-Stufe · Platz[ · Tiefe]):\n");
    for row in rows.iter().take(ROW_LIMIT) {
        let _ = writeln!(out, "{}", row_line(row));
    }
    if rows.len() > ROW_LIMIT {
        let _ = writeln!(out, "… {} weitere", rows.len() - ROW_LIMIT);
    }
    for finding in &view.findings {
        let _ = writeln!(out, "{}", finding_line(finding));
    }
    for note in &view.notes {
        let _ = writeln!(out, "Hinweis: {note}");
    }
    out.push_str(&rules_line(&editor.rules(&view.raw)));
    Ok(out)
}

fn finding_line(f: &HubFindingView) -> String {
    format!("Befund: {} · {} · {}", f.rule, f.site, f.detail)
}

fn findings_label(count: usize) -> String {
    if count == 0 {
        return "ok".to_owned();
    }
    format!("{count} Befunde")
}

fn short(stand: &str) -> &str {
    if stand.is_empty() {
        return "-";
    }
    stand.get(..STAND_LEN).unwrap_or(stand)
}

/// Eine Zahl wie Gos `%g`: kürzeste Ziffern, mit Exponent unter 1e-4 und ab 1e6.
fn go_g(value: f64) -> String {
    let scientific = format!("{value:e}");
    let Some((mantissa, exponent)) = scientific.split_once('e') else {
        return scientific;
    };
    let Ok(exponent) = exponent.parse::<i32>() else {
        return scientific;
    };
    if (-4..6).contains(&exponent) {
        return format!("{value}");
    }
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.unsigned_abs())
}

/// Eine Zeile der Matrix: Platz und Tiefe der ersten Stufe, in der sie verfügbar ist.
fn row_line(row: &HubRow) -> String {
    let mut line = format!("{} · {}#{} · ab {}", row.key, row.source, row.index, row.hub_level);
    let Some(cell) = row.cells.iter().find(|c| c.available) else {
        return line;
    };
    let (sign, offset) = match cell.offset_units {
        v if row.both_sides => ("±", v),
        v if v < 0.0 => ("−", -v),
        v => ("+", v),
    };
    let _ = write!(line, " · {sign}{}", go_g(offset));
    if cell.from_depth > 0 {
        let _ = write!(line, " · ab Tiefe {}", cell.from_depth);
    }
    if cell.needs_deeper {
        line.push_str(" · nur mit tieferer Stufe");
    }
    if cell.only_depth >= 0 {
        let _ = write!(line, " · nur Tiefe {}", cell.only_depth);
    }
    line
}

/// Die Zeile zur Regeltabelle: sie stimmt, weicht ab (mit den ersten Zeilen) oder ist nicht vergleichbar.
fn rules_line(view: &HubRulesView) -> String {
    if !view.error.is_empty() {
        return format!("Regeltabelle: {}\n", view.error);
    }
    if view.same {
        return "Regeltabelle: stimmt\n".to_owned();
    }
    let count = view.lines.len();
    let mut out = format!(
        "Regeltabelle: weicht ab ({count} Zeilen in {} § 3), {} schreibt sie neu\n",
        view.path, view.task
    );
    for line in view.lines.iter().take(DIFF_LIMIT) {
        let (ist, soll) = (go_quoted(&line.ist), go_quoted(&line.soll));
        let _ = writeln!(out, "  Zeile {}: ist {ist}, soll {soll}", line.line);
    }
    if count > DIFF_LIMIT {
        let _ = writeln!(out, "… {} weitere", count - DIFF_LIMIT);
    }
    out
}

/// Speichert den Kandidaten wie die Seite: Prüfung ohne Befund, Stand der Dateien, dann hub.json und Regeltabelle.
fn save(editor: &Editor, input: SaveIn) -> Result<String, String> {
    if input.patch.is_none() && input.candidate.is_empty() {
        return Err("patch oder candidate fehlt".to_owned());
    }
    let cand = candidate(editor, input.patch, input.candidate)?;
    let plan = editor.plan();
    if !plan.error.is_empty() {
        return Err(plan.error);
    }
    if input.stand.is_empty() {
        return Err("stand fehlt: erst hub_get lesen und dessen Stand (hub=…) mitgeben".to_owned());
    }
    if !plan.stand.hub.starts_with(&input.stand) {
        return Err("nicht gespeichert: data/hub.json seit hub_get geändert, bitte neu lesen".to_owned());
    }
    let result = editor.save(&cand, plan.stand);
    if !result.stale.is_empty() {
        return Err(format!("nicht gespeichert: {}", result.stale));
    }
    if !result.error.is_empty() {
        let mut lines = vec![result.error];
        lines.extend(result.findings.iter().map(finding_line));
        return Err(lines.join("\n"));
    }
    if result.unchanged {
        return Ok("Unverändert: der Kandidat bedeutet dasselbe wie data/hub.json, nichts geschrieben.".to_owned());
    }
    let mut out = format!("Gespeichert: data/hub.json und Regeltabelle. {}", result.notice);
    if !result.rules_error.is_empty() {
        let _ = write!(out, "\nRegeltabelle nicht geschrieben: {}", result.rules_error);
    }
    let (hub, rules) = (short(&result.stand.hub), short(&result.stand.rules));
    let _ = write!(out, "\nStand: hub={hub} rules={rules}");
    Ok(out)
}
