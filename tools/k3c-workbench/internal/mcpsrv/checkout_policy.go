package mcpsrv

import (
	"context"
	"fmt"
	"strings"
)

// takesCheckout: Tools mit dem Argument checkout im Schema (B-388; Gerüst: mcpkit.Options.TakesCheckout).
// Asset-Tools (paper-mcp.md › 1) und Gebäude-Planer-Tools (hub_*, B-923) und Welten-Tools (world_*, B-954) nehmen checkout immer, auch die lesenden; console_tail wie check_run (B-855).
func takesCheckout(tool string) bool {
	return writeTools[tool] || tool == "check_run" || tool == "console_tail" || tool == "sim_test" ||
		strings.HasPrefix(tool, "assets_") || strings.HasPrefix(tool, "hub_") || strings.HasPrefix(tool, "world_")
}

// writeTools ändern Dateien oder Dienste. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die
// Repo-Wurzel gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
var writeTools = map[string]bool{"plan_create": true, "plan_set": true, "plan_section": true, "plan_sections": true, "plan_delete": true,
	"svc_start": true, "svc_stop": true, "svc_restart": true, "svc_start_all": true, "svc_stop_all": true, "task_start": true, "task_stop": true,
	"assets_set": true, "assets_create": true, "assets_style_set": true, "assets_render": true, "assets_delete": true,
	"assets_research": true, "assets_hit_add": true, "assets_hit_drop": true, "assets_import": true, "hub_save": true,
	"world_create": true, "world_set": true, "world_override_set": true, "world_chunks_set": true, "world_story_set": true, "world_delete": true}

// ownCheckoutErr lehnt einen Aufruf ab, dessen Checkout nicht der der laufenden Workbench ist (B-924, B-925): sim_run,
// sim_test, replay_run und level_generate rechnen Code und Daten dieses Prozesses, nicht die eines Worktrees.
func (s *Server) ownCheckoutErr(ctx context.Context, tool string) error {
	if s.kit.OwnCheckout(ctx) {
		return nil
	}
	return fmt.Errorf("%s abgelehnt: Checkout %s ist nicht der Checkout dieser Workbench (%s); die Engine rechnet im Prozess der Workbench. "+
		"Eigene Dev-Workbench im Worktree starten: task k3c-workbench:dev:worktree", tool, s.ws(ctx).Root, s.cfg.Root)
}
