//! Balancing-Seite (Port von `app_balance.go`, WZ28.4, B-596): Offline-Läufe im Register von `sim_test`
//! (`crate::simrun`), gleiche Grenze von `MAX_PER_ROOT` Läufen je Checkout; weitere warten. Die Seite bedient die
//! Repo-Wurzel. Dazu die Handler der Aufruf-Tabelle [`super::rpc`]. Alle Methoden blockieren.

use k3c_kit::uikit;
use serde::Serialize;
use serde_json::{Value, json};

use super::App;
use super::rpc::encode;
use crate::balance::baselines::{BaselineInfo, has_baseline, list_baselines};
use crate::balance::insights::Insights;
use crate::balance::summary::Summary;
use crate::balance::tune::Tuning;
use crate::datadoc;
use crate::kitx::{go_marshal, go_quoted};
use crate::simrun::manage::{delete_report, delete_reports, rate_report, rate_reports};
use crate::simrun::reportlist::{ReportInfo, reports};
use crate::simrun::reports::{read_insights, read_report, read_tuning, write_html};
use crate::simrun::{Info, Spec};

#[cfg(test)]
#[path = "balance_tests.rs"]
mod tests;

impl App {
    /// Reiht einen Offline-Lauf ein; er rechnet sofort, wenn ein Platz frei ist. Mit `perVariant` reiht er den
    /// Basis-Lauf und je feste Variante einen Lauf ein und liefert den Basis-Lauf (B-662).
    pub fn balance_start(&self, spec: &Spec) -> Result<Info, String> {
        if !spec.baseline.is_empty() && !has_baseline(&self.root, &spec.baseline) {
            return Err(format!(
                "lauf abgelehnt: baseline {} gibt es nicht",
                go_quoted(&spec.baseline)
            ));
        }
        self.sims()?.start_offline(&self.root, "Repo-Wurzel", spec)
    }

    /// Alle Läufe der Repo-Wurzel (auch die von `sim_test`), neueste zuerst.
    pub fn balance_runs(&self) -> Result<Vec<Info>, String> {
        Ok(self.sims()?.list(&self.root))
    }

    /// Bricht einen Lauf ab; ein wartender verschwindet aus der Liste.
    pub fn balance_stop(&self, id: &str) -> Result<(), String> {
        self.sims()?.stop(&self.root, id).map(|_| ()).map_err(|e| e.to_string())
    }

    /// Die gespeicherten Berichte unter `reports/`, neueste zuerst.
    pub fn balance_reports(&self) -> Result<Vec<ReportInfo>, String> {
        reports(&self.root)
    }

    /// Ein Bericht; nur Pfade unter `reports/`.
    pub fn balance_report(&self, path: &str) -> Result<Summary, String> {
        read_report(&self.root, path)
    }

    /// Ein Abstimm-Ergebnis `tuning-*.json` (WZ28.6); nur Pfade unter `reports/`.
    pub fn balance_tuning(&self, path: &str) -> Result<Tuning, String> {
        read_tuning(&self.root, path)
    }

    /// Die gesicherten Basis-Läufe (`reports/baseline/`, B-640).
    pub fn balance_baselines(&self) -> Result<Vec<BaselineInfo>, String> {
        list_baselines(&self.root)
    }

    /// Die Dateien in `data/`, die das Balancing beeinflussen, mit Zweck aus `docs/data/` (WZ28.10, B-610).
    pub fn balance_data_files(&self) -> Vec<datadoc::File> {
        datadoc::list(&self.root)
    }

    /// Die Beschreibung `docs/data/<name>.md`; nur Namen der Liste.
    pub fn balance_data_doc(&self, name: &str) -> Result<String, String> {
        datadoc::doc(&self.root, name)
    }

    /// Die Zahlenwerte einer variierbaren Datei (Pfade wie in Varianten).
    pub fn balance_data_values(&self, name: &str) -> Result<Vec<datadoc::Value>, String> {
        datadoc::values(&self.root, name)
    }

    /// Die Zahlenwerte aller variierbaren Dateien mit Datei und Pfad (Suche, B-662).
    pub fn balance_data_values_all(&self) -> Result<Vec<datadoc::FileValue>, String> {
        datadoc::all_values(&self.root)
    }

    /// Setzt die Bewertung eines Berichts (gut, schlecht, unklar, referenz; leer = keine, WZ28.13).
    pub fn balance_report_rate(&self, path: &str, rating: &str) -> Result<(), String> {
        self.reports_done(rate_report(&self.root, path, rating))
    }

    /// Löscht einen Bericht unter `reports/` samt `.md` und Bewertung (WZ28.13).
    pub fn balance_report_delete(&self, path: &str) -> Result<(), String> {
        self.reports_done(delete_report(&self.root, path))
    }

    /// Bewertet mehrere Berichte in einem Aufruf (B-662); ein Pfad außerhalb `reports/` lehnt alle ab.
    pub fn balance_reports_rate(&self, paths: &[String], rating: &str) -> Result<(), String> {
        self.reports_done(rate_reports(&self.root, paths, rating))
    }

    /// Löscht mehrere Berichte in einem Aufruf (B-662); erst werden alle Pfade geprüft, ein Pfad außerhalb `reports/`
    /// lehnt den ganzen Aufruf ab, ohne etwas zu löschen.
    pub fn balance_reports_delete(&self, paths: &[String]) -> Result<(), String> {
        self.reports_done(delete_reports(&self.root, paths))
    }

    /// Sichert einen durchgelaufenen Bericht `reports/<lauf>/balance-*.json` als Baseline `name`, auf demselben Weg wie
    /// `sim_test {action: baseline}` (B-662).
    pub fn balance_save_baseline(&self, path: &str, name: &str) -> Result<BaselineInfo, String> {
        let saved = self.sims()?.save_baseline(&self.root, path, name, false)?;
        self.logger().info(
            "💾 Baseline gesichert",
            &[
                ("ns", json!("main")),
                ("name", json!(saved.name)),
                ("lauf", json!(saved.run)),
            ],
        );
        Ok(saved)
    }

    /// Wertet einen Bericht aus (B-632): Ampeln, Kurzauswertung, Hinweise und nächster Lauf.
    pub fn balance_insights(&self, path: &str) -> Result<Insights, String> {
        read_insights(&self.root, path)
    }

    /// Schreibt `.md` und `.html` eines Berichts neu (auch alter, B-632) und öffnet den HTML-Bericht im Browser;
    /// Ergebnis ist sein Pfad relativ zum Checkout.
    pub fn balance_report_html(&self, path: &str) -> Result<String, String> {
        let page = write_html(&self.root, path)?;
        (self.open_file)(&page.to_string_lossy())?;
        let name = page.file_name().map(|n| n.to_string_lossy().into_owned());
        self.logger().info(
            "📄 HTML-Bericht geöffnet",
            &[("ns", json!("main")), ("datei", json!(name))],
        );
        let rel = page.strip_prefix(&self.root).unwrap_or(&page);
        Ok(rel.to_string_lossy().replace('\\', "/"))
    }
}

/// Zahlen wie Go (`100` statt `100.0`).
fn encode_go(value: impl Serialize) -> Result<Value, String> {
    serde_json::from_str(&go_marshal(&encode(value)?)).map_err(|e| e.to_string())
}

fn no_args(args: &[Value]) -> Result<(), String> {
    uikit::args::<()>(args.to_vec())
}

/// Genau ein Text.
fn one(args: &[Value]) -> Result<String, String> {
    let (value,): (String,) = uikit::args(args.to_vec())?;
    Ok(value)
}

pub(super) fn start(app: &App, args: &[Value]) -> Result<Value, String> {
    let (spec,): (Spec,) = uikit::args(args.to_vec())?;
    encode(app.balance_start(&spec)?)
}

pub(super) fn runs(app: &App, args: &[Value]) -> Result<Value, String> {
    no_args(args)?;
    encode(app.balance_runs()?)
}

pub(super) fn stop(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_stop(&one(args)?)?)
}

pub(super) fn report_list(app: &App, args: &[Value]) -> Result<Value, String> {
    no_args(args)?;
    encode(app.balance_reports()?)
}

pub(super) fn report(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_report(&one(args)?)?)
}

pub(super) fn tuning(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_tuning(&one(args)?)?)
}

pub(super) fn baselines(app: &App, args: &[Value]) -> Result<Value, String> {
    no_args(args)?;
    encode(app.balance_baselines()?)
}

pub(super) fn data_files(app: &App, args: &[Value]) -> Result<Value, String> {
    no_args(args)?;
    encode(app.balance_data_files())
}

pub(super) fn data_doc(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_data_doc(&one(args)?)?)
}

pub(super) fn data_values(app: &App, args: &[Value]) -> Result<Value, String> {
    encode_go(app.balance_data_values(&one(args)?)?)
}

pub(super) fn data_values_all(app: &App, args: &[Value]) -> Result<Value, String> {
    no_args(args)?;
    encode_go(app.balance_data_values_all()?)
}

pub(super) fn report_rate(app: &App, args: &[Value]) -> Result<Value, String> {
    let (path, rating): (String, String) = uikit::args(args.to_vec())?;
    encode(app.balance_report_rate(&path, &rating)?)
}

pub(super) fn report_delete(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_report_delete(&one(args)?)?)
}

pub(super) fn reports_rate(app: &App, args: &[Value]) -> Result<Value, String> {
    let (paths, rating): (Option<Vec<String>>, String) = uikit::args(args.to_vec())?;
    encode(app.balance_reports_rate(&paths.unwrap_or_default(), &rating)?)
}

pub(super) fn reports_delete(app: &App, args: &[Value]) -> Result<Value, String> {
    let (paths,): (Option<Vec<String>>,) = uikit::args(args.to_vec())?;
    encode(app.balance_reports_delete(&paths.unwrap_or_default())?)
}

pub(super) fn save_baseline(app: &App, args: &[Value]) -> Result<Value, String> {
    let (path, name): (String, String) = uikit::args(args.to_vec())?;
    encode(app.balance_save_baseline(&path, &name)?)
}

pub(super) fn insights(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_insights(&one(args)?)?)
}

pub(super) fn report_html(app: &App, args: &[Value]) -> Result<Value, String> {
    encode(app.balance_report_html(&one(args)?)?)
}
