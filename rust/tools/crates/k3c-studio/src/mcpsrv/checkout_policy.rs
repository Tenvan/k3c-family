//! Welche Tools das Argument `checkout` haben und welche schreiben (Port von `checkout_policy.go`, 1:1).

/// Tools mit dem Argument `checkout` im Schema (B-388). Asset-, Gebäude-Planer- und Welten-Tools nehmen es immer, auch
/// die lesenden; `sim_test` wie in der Workbench. `level_generate`, `sim_run` und `replay_run` rechnen mit der EXE dieses
/// Checkouts.
pub(super) fn takes_checkout(tool: &str) -> bool {
    write_tool(tool)
        || tool.starts_with("assets_")
        || tool.starts_with("hub_")
        || tool.starts_with("world_")
        || matches!(tool, "sim_test" | "level_generate" | "sim_run" | "replay_run")
}

/// Tools, die Dateien ändern. Ihre Antwort nennt immer den Checkout (B-275).
pub(super) fn write_tool(tool: &str) -> bool {
    matches!(
        tool,
        "assets_set"
            | "assets_create"
            | "assets_style_set"
            | "assets_render"
            | "assets_delete"
            | "assets_research"
            | "assets_hit_add"
            | "assets_hit_drop"
            | "assets_import"
            | "hub_save"
            | "world_create"
            | "world_set"
            | "world_override_set"
            | "world_chunks_set"
            | "world_story_set"
            | "world_delete"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkout_nur_wo_go_ihn_hat() {
        for tool in [
            "assets_get",
            "hub_get",
            "world_list",
            "sim_test",
            "level_generate",
            "world_set",
        ] {
            assert!(takes_checkout(tool), "{tool}");
        }
        for tool in ["reports_list", "report_read", "saves_list"] {
            assert!(!takes_checkout(tool) && !write_tool(tool), "{tool}");
        }
        assert!(write_tool("hub_save") && !write_tool("hub_get"));
    }
}
