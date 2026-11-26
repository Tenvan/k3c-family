//! Gebäude-Planer der Oberfläche (Port von `app_hub.go`, B-923): `data/hub.json` als Matrix Hub-Stufe × Bauplatz
//! ansehen, einen Kandidaten prüfen, mit der Regeltabelle vergleichen und geprüft speichern. Die Logik liegt in
//! `hubplan`; dieselben Aufrufe machen die MCP-Tools `hub_*`. Die Methoden blockieren (Spiel-EXE).

use k3c_kit::uikit;
use serde_json::{Value, json};

use super::App;
use super::rpc::encode;
use crate::hubplan::{Editor, HubPlanView, HubRulesView, HubSaveResult, HubStand};

#[cfg(test)]
#[path = "hub_tests.rs"]
mod tests;

impl App {
    /// Gebäude-Planer der Repo-Wurzel.
    fn hubs(&self) -> Editor {
        self.wait();
        Editor::new(&self.root, (self.hub_engine)(&self.root))
    }

    /// Lädt `data/hub.json` und liefert Matrix und Befunde (Methode `HubPlan`).
    pub fn hub_plan(&self) -> HubPlanView {
        self.hubs().plan()
    }

    /// Prüft einen Kandidaten (Text der hub.json), ohne etwas zu schreiben (Methode `HubCheck`).
    pub fn hub_check(&self, candidate: &str) -> HubPlanView {
        self.hubs().check(candidate)
    }

    /// Vergleicht die Regeltabelle mit dem Planer-Stand; leer = `data/hub.json` (Methode `HubRules`).
    pub fn hub_rules(&self, hub_json: &str) -> HubRulesView {
        self.hubs().rules(hub_json)
    }

    /// Speichert den geprüften Kandidaten, wenn beide Dateien noch den Stand `stand` haben (Methode `HubSave`).
    pub fn hub_save(&self, candidate: &str, stand: HubStand) -> HubSaveResult {
        let result = self.hubs().save(candidate, stand);
        if result.saved {
            let fields = [
                ("ns", json!("dev")),
                ("file", json!("data/hub.json")),
                ("rulesError", json!(result.rules_error)),
            ];
            self.logger().info("💾 Gebäude-Planer gespeichert", &fields);
        } else if !result.error.is_empty() || !result.stale.is_empty() {
            let fields = [
                ("ns", json!("dev")),
                ("error", json!(result.error)),
                ("stale", json!(result.stale)),
            ];
            self.logger().warn("🚫 Gebäude-Planer nicht gespeichert", &fields);
        }
        result
    }
}

pub(super) fn plan(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.hub_plan())
}

pub(super) fn check(app: &App, args: &[Value]) -> Result<Value, String> {
    let (candidate,): (String,) = uikit::args(args.to_vec())?;
    encode(app.hub_check(&candidate))
}

pub(super) fn rules(app: &App, args: &[Value]) -> Result<Value, String> {
    let (hub_json,): (String,) = uikit::args(args.to_vec())?;
    encode(app.hub_rules(&hub_json))
}

pub(super) fn save(app: &App, args: &[Value]) -> Result<Value, String> {
    let (candidate, stand): (String, Option<HubStand>) = uikit::args(args.to_vec())?;
    encode(app.hub_save(&candidate, stand.unwrap_or_default()))
}
