package mcpsrv

import (
	"context"
	"fmt"
)

// takesCheckout: Tools mit dem Argument checkout im Schema (B-388; Gerüst: mcpkit.Options.TakesCheckout).
// console_tail wie check_run (B-855). Asset-, Hub- und Welten-Tools liegen seit WZ63 in k3c-studio.
func takesCheckout(tool string) bool {
	return writeTools[tool] || tool == "check_run" || tool == "console_tail" || tool == "sim_test"
}

// writeTools ändern Dateien oder Dienste. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die
// Repo-Wurzel gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
var writeTools = map[string]bool{"plan_create": true, "plan_set": true, "plan_section": true, "plan_sections": true, "plan_delete": true,
	"svc_start": true, "svc_stop": true, "svc_restart": true, "svc_start_all": true, "svc_stop_all": true, "task_start": true, "task_stop": true}

// ownCheckoutErr lehnt einen Aufruf ab, dessen Checkout nicht der der laufenden Workbench ist (B-924, B-925): sim_run,
// sim_test und replay_run rechnen Code und Daten dieses Prozesses, nicht die eines Worktrees.
func (s *Server) ownCheckoutErr(ctx context.Context, tool string) error {
	if s.kit.OwnCheckout(ctx) {
		return nil
	}
	return fmt.Errorf("%s abgelehnt: Checkout %s ist nicht der Checkout dieser Workbench (%s); die Engine rechnet im Prozess der Workbench. "+
		"Eigene Dev-Workbench im Worktree starten: task k3c-workbench:dev:worktree", tool, s.ws(ctx).Root, s.cfg.Root)
}
