//! Gültige Parameter je Tool (Port von `mcpkit/param_hint.go`). Das Go-SDK lehnt unbekannte Felder per Schema ab,
//! `rmcp` ignoriert sie; darum prüft `reject_unknown` selbst und antwortet mit demselben Text.

use rmcp::model::{CallToolResult, ContentBlock, JsonObject};
use serde_json::Value;

use super::{result_text, text_result};

/// Teil der Meldung, an dem ein unbekanntes Feld erkennbar ist. Sie nennt das falsche Feld, aber nie die richtigen;
/// ohne Hinweis rät ein Agent weiter.
const ADDITIONAL_PROPS: &str = "additional properties";

/// Eingabe eines Tools laut seinem Schema.
pub(super) struct Params {
    /// Namen der Eigenschaften, sortiert.
    pub(super) names: Vec<String>,
    /// Das Schema erlaubt weitere Felder (Map als Eingabe).
    open: bool,
}

impl Params {
    pub(super) fn of(schema: &JsonObject) -> Params {
        let mut names: Vec<String> = schema
            .get("properties")
            .and_then(Value::as_object)
            .map(|p| p.keys().cloned().collect())
            .unwrap_or_default();
        names.sort();
        let open = matches!(
            schema.get("additionalProperties"),
            Some(Value::Bool(true) | Value::Object(_))
        );
        Params { names, open }
    }

    /// Fehler-Ergebnis mit dem Text des Go-SDK, wenn `args` ein Feld nennt, das das Schema nicht kennt.
    pub(super) fn reject_unknown(&self, args: Option<&JsonObject>) -> Option<CallToolResult> {
        if self.open {
            return None;
        }
        let unknown: Vec<String> = args?
            .keys()
            .filter(|key| !self.names.contains(key))
            .map(|key| format!("{key:?}"))
            .collect();
        if unknown.is_empty() {
            return None;
        }
        let text = format!(
            "validating \"arguments\": validating root: unexpected {ADDITIONAL_PROPS} [{}]",
            unknown.join(" ")
        );
        Some(text_result(text, true))
    }
}

/// Hängt an eine Ablehnung wegen unbekannter Felder die gültigen Parameter des Tools an.
pub(super) fn add_param_hint(names: &[String], result: &mut CallToolResult) {
    if result.is_error != Some(true) || !result_text(result).contains(ADDITIONAL_PROPS) {
        return;
    }
    let hint = if names.is_empty() {
        "gültige Parameter: keine".to_owned()
    } else {
        format!("gültige Parameter: {}", names.join(", "))
    };
    result.content.push(ContentBlock::text(hint));
}
