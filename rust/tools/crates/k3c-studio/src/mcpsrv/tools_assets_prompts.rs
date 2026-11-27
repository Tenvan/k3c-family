//! Prompt-Vorlagen als MCP-Prompts (Port von `tools_assets_prompts.go`; paper-mcp.md › MCP-Prompts): `asset-<id>`,
//! Argumente = Platzhalter im Text. Neue oder gelöschte Vorlagen gelten nach einem Neustart, geänderte Texte sofort
//! (jeder Abruf liest die Datei).

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::{Captures, Regex};
use rmcp::ErrorData;
use rmcp::handler::server::prompt::PromptContext;
use rmcp::handler::server::router::prompt::{PromptRoute, PromptRouter};
use rmcp::model::{GetPromptResult, Prompt, PromptArgument, PromptMessage, Role};
use serde_json::{Map, Value};

use super::tools::Studio;
use crate::assetsui::Vorlage;

/// Die Datei der Prompt-Vorlagen relativ zum Checkout.
const VORLAGEN_PFAD: &str = "data/asset-prompts.json";

// Das Muster ist fest; ein Fehler darin fiele in jedem Test auf.
#[allow(clippy::expect_used)]
static PLATZHALTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{(asset|rig|teil|pfad|zustand|basis|eingabe|material|ton|slot|datei|pack|lizenz|treffer)\}")
        .expect("Muster der Platzhalter")
});

/// Dürfen leer bleiben; alle anderen Platzhalter sind Pflicht.
fn optional(name: &str) -> bool {
    matches!(name, "zustand" | "basis")
}

fn lese_vorlagen(root: &Path) -> Result<Vec<Vorlage>, String> {
    let text = fs::read_to_string(root.join(VORLAGEN_PFAD)).map_err(|e| format!("{VORLAGEN_PFAD}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{VORLAGEN_PFAD}: {e}"))
}

/// Die Platzhalter eines Textes in Reihenfolge des ersten Auftretens.
fn platzhalter_in(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for m in PLATZHALTER.captures_iter(text) {
        let name = m.get(1).map_or("", |g| g.as_str());
        if !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

fn prompt(v: &Vorlage) -> Prompt {
    let argumente: Vec<PromptArgument> = platzhalter_in(&v.text)
        .into_iter()
        .map(|name| {
            let a = PromptArgument::new(name);
            // Wie Go: `required` fehlt bei optionalen Argumenten, statt false zu sein.
            if optional(name) { a } else { a.with_required(true) }
        })
        .collect();
    let beschreibung = format!("Asset-Prompt ({}): {}", v.ort, v.titel);
    let argumente = (!argumente.is_empty()).then_some(argumente);
    Prompt::new(format!("asset-{}", v.id), Some(beschreibung), argumente).with_title(v.titel.clone())
}

/// Der ausgefüllte Text der Vorlage `id`.
pub(super) fn prompt_text(root: &Path, id: &str, args: &Map<String, Value>) -> Result<GetPromptResult, String> {
    let vorlagen = lese_vorlagen(root)?;
    let Some(v) = vorlagen.iter().find(|v| v.id == id) else {
        return Err(format!("Vorlage „{id}“ fehlt in {VORLAGEN_PFAD}"));
    };
    if let Some((name, _)) = args.iter().find(|(_, wert)| !wert.is_string()) {
        return Err(format!("Argument „{name}“ ist kein Text"));
    }
    let wert = |name: &str| args.get(name).and_then(Value::as_str).unwrap_or_default();
    if let Some(name) = platzhalter_in(&v.text)
        .into_iter()
        .find(|name| wert(name).is_empty() && !optional(name))
    {
        return Err(format!("Argument „{name}“ fehlt"));
    }
    let text = PLATZHALTER.replace_all(&v.text, |m: &Captures<'_>| {
        wert(m.get(1).map_or("", |g| g.as_str())).to_owned()
    });
    let nachricht = PromptMessage::new_text(Role::User, text.into_owned());
    Ok(GetPromptResult::new(vec![nachricht]).with_description(v.titel.clone()))
}

/// Je Vorlage der Repo-Wurzel ein Prompt; ohne Datei (z. B. in Tests ohne Repo) keine Prompts.
pub(super) fn prompt_router(root: &Path) -> Option<PromptRouter<Studio>> {
    let vorlagen = lese_vorlagen(root).ok()?;
    if vorlagen.is_empty() {
        return None;
    }
    let mut router = PromptRouter::new();
    for v in &vorlagen {
        let (root, id) = (root.to_path_buf(), v.id.clone());
        router.add_route(PromptRoute::new_dyn(
            prompt(v),
            move |context: PromptContext<'_, Studio>| {
                let args = context.arguments.unwrap_or_default();
                let result = prompt_text(&root, &id, &args)
                    .map(Into::into)
                    .map_err(|e| ErrorData::invalid_params(e, None));
                Box::pin(std::future::ready(result))
            },
        ));
    }
    Some(router)
}
