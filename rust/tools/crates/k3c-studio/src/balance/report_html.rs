//! HTML-Bericht (B-632): eine Datei ohne externe Ressourcen. Vorlage, Stil und Skripte liegen in `report/`
//! (Byte-Kopien der Go-Dateien, eingebettet), die Auswertung kommt als JSON hinein, Spielgrafiken aus `public/` als
//! data-URIs (erster Frame, ohne transparenten Rand).

use std::collections::BTreeMap;
use std::path::Path;

use super::insights::Insights;
use super::report_png::cropped_data_uri;
use crate::kitx::{compact, go_marshal_indent};

const PAGE: &str = include_str!("report/report.html");
const CSS: &str = include_str!("report/report.css");

/// Die Skripte in Lade-Reihenfolge: core definiert die Hilfen, next startet.
const SCRIPTS: [&str; 4] = [
    include_str!("report/report-core.js"),
    include_str!("report/report-charts.js"),
    include_str!("report/report-view.js"),
    include_str!("report/report-next.js"),
];

/// Name, Datei unter `public/` und Größe eines Frames (`None` = ganzes Bild, sonst das Feld oben links).
type Asset = (&'static str, &'static str, Option<(u32, u32)>);

/// Die Grafiken des Berichts.
const ASSETS: [Asset; 10] = [
    ("castle", "grafik/gebaeude-paper/burg/stand.png", None),
    ("wall", "grafik/gebaeude-paper/mauer/stand.png", None),
    ("tower", "grafik/gebaeude-paper/turm/stand.png", None),
    ("fire", "grafik/gebaeude-paper/lagerfeuer/stand.png", None),
    ("axe", "grafik/objekte/schild-axt/idle.png", Some((36, 51))),
    ("pick", "grafik/objekte/schild-spitzhacke/idle.png", Some((36, 51))),
    ("ore", "grafik/objekte/erzbrocken/idle.png", Some((43, 28))),
    ("coin", "grafik/icons/muenze/idle.png", Some((26, 26))),
    ("enemy", "grafik/gegner/skelett/idle.png", Some((159, 192))),
    ("enemy2", "grafik/gegner/goblin/idle.png", Some((143, 192))),
];

/// Ersetzt alle Platzhalter in einem Durchgang (wie Go `strings.Replacer`: Eingesetztes wird nicht erneut ersetzt).
fn fill(page: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = String::new();
    let mut rest = page;
    while let Some((at, key, value)) = pairs
        .iter()
        .filter_map(|(key, value)| rest.find(key).map(|at| (at, key, value)))
        .min_by_key(|found| found.0)
    {
        out.push_str(&rest[..at]);
        out.push_str(value);
        rest = &rest[at + key.len()..];
    }
    out.push_str(rest);
    out
}

/// Go `json.Marshal`: kompakt, `<`, `>` und `&` maskiert (kein `</script>` im Skript-Block).
fn marshal<T: serde::Serialize>(value: &T) -> Result<String, String> {
    Ok(compact(&go_marshal_indent(value)?))
}

/// Liest die Grafiken; unlesbare fehlen in der Liste.
fn load_assets(root: &Path) -> BTreeMap<&'static str, String> {
    ASSETS
        .iter()
        .filter_map(|&(key, path, frame)| {
            let raw = std::fs::read(root.join("public").join(path)).ok()?;
            Some((key, cropped_data_uri(&raw, frame)?))
        })
        .collect()
}

impl Insights {
    /// Der Bericht als HTML; `root` ist der Checkout (fehlende Grafiken zeigt der Bericht als Platzhalter).
    pub fn html(&self, root: &Path) -> Result<String, String> {
        let js: String = SCRIPTS.iter().map(|s| format!("{s}\n")).collect();
        let title = self.title.replace(['<', '>', '&'], "");
        Ok(fill(
            PAGE,
            &[
                ("__CSS__", CSS),
                ("__JS__", &js),
                ("__DATA__", &marshal(self)?),
                ("__ASSETS__", &marshal(&load_assets(root))?),
                ("__TITLE__", &title),
            ],
        ))
    }
}

#[cfg(test)]
#[path = "report_html_tests.rs"]
mod tests;
