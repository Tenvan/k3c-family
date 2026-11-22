package mcpsrv

// takesCheckout: Tools mit dem Argument checkout im Schema (B-388; Gerüst: mcpkit.Options.TakesCheckout).
// console_tail wie check_run (B-855). Asset-, Hub- und Welten-Tools (WZ63) und sim_test (WZ63.5) liegen in k3c-studio.
func takesCheckout(tool string) bool {
	return writeTools[tool] || tool == "check_run" || tool == "console_tail"
}

// writeTools ändern Dateien oder Dienste. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die
// Repo-Wurzel gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
var writeTools = map[string]bool{"plan_create": true, "plan_set": true, "plan_section": true, "plan_sections": true, "plan_delete": true,
	"svc_start": true, "svc_stop": true, "svc_restart": true, "svc_start_all": true, "svc_stop_all": true, "task_start": true, "task_stop": true}
