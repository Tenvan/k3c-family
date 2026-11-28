//! Tool `notify_ui` (Port von `tools_notify.go`): ein Hinweis an den Nutzer, die Oberfläche zeigt ihn als Meldung.

use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Gültige Stufen eines Hinweises.
const LEVELS: [&str; 4] = ["info", "success", "warn", "error"];

/// Hinweis an den Nutzer; zugleich die Argumente des Tools und die Daten des Ereignisses `ui:notify`.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Notice {
    #[schemars(description = "info, success, warn oder error")]
    pub level: String,
    #[schemars(description = "kurze Überschrift")]
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[schemars(description = "optionaler Text darunter")]
    pub text: String,
}

/// Nimmt Hinweise für die Oberfläche an.
pub type NotifyHook = Arc<dyn Fn(&Notice) + Send + Sync>;

pub(super) fn notify_ui(hook: Option<&NotifyHook>, notice: &Notice) -> Result<String, String> {
    if !LEVELS.contains(&notice.level.as_str()) {
        return Err(format!(
            "level {:?} ungültig: erlaubt {}",
            notice.level,
            LEVELS.join(", ")
        ));
    }
    if notice.title.trim().is_empty() {
        return Err("title fehlt".to_owned());
    }
    let hook = hook.ok_or("keine Oberfläche: k3c-workbench läuft ohne Fenster")?;
    hook(notice);
    Ok(format!("angezeigt: {}", notice.title))
}
