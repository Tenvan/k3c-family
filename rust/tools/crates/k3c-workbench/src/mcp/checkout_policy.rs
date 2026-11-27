//! Welche Tools `checkout` nehmen und welche schreiben (Port von `checkout_policy.go`). Die Liste ist vollständig wie in
//! Go; die Tools selbst kommen mit den folgenden Sessions.

/// Ändern Dateien oder Dienste. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die Repo-Wurzel
/// gilt (B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
const WRITE_TOOLS: &[&str] = &[
    "plan_create",
    "plan_set",
    "plan_section",
    "plan_sections",
    "plan_delete",
    // Go trägt beide im `init` ihrer Datei ein.
    "plan_digest",
    "plan_index",
    "playtest_frame",
    "playtest_findings_set",
    "playtest_tickets_create",
    "svc_start",
    "svc_stop",
    "svc_restart",
    "svc_start_all",
    "svc_stop_all",
    "task_start",
    "task_stop",
    "git_commit",
];

pub(super) fn write_tool(tool: &str) -> bool {
    WRITE_TOOLS.contains(&tool)
}

/// Tools mit dem Argument `checkout` im Schema (B-388); `console_tail` wie `check_run` (B-855).
pub(super) fn takes_checkout(tool: &str) -> bool {
    write_tool(tool) || tool == "check_run" || tool == "console_tail"
}
