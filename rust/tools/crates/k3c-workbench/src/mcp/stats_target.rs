//! Ziel eines `check_run`-Aufrufs aus den rohen Argumenten fürs Aufruf-Log (Port von `stats_target.go`, B-876).

use k3c_kit::mcp::clip;
use serde_json::Value;

pub(super) fn target_of(tool: &str, args: &str) -> String {
    if tool != "check_run" {
        return String::new();
    }
    let parsed: Value = serde_json::from_str(args).unwrap_or_default();
    clip(parsed["target"].as_str().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::target_of;

    #[test]
    fn ziel_nur_bei_check_run() {
        let long = format!(
            r#"{{"pattern":"{}","target":"task:check:diff","wait":true}}"#,
            "x".repeat(300)
        );
        assert_eq!(target_of("check_run", &long), "task:check:diff");
        assert_eq!(target_of("a", r#"{"target":"task:test"}"#), "");
        assert_eq!(target_of("check_run", "kaputt"), "");
    }
}
