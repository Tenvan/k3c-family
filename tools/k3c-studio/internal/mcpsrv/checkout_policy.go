package mcpsrv

import "strings"

// takesCheckout: Tools mit dem Argument checkout im Schema (B-388; Gerüst: mcpkit.Options.TakesCheckout).
// Asset-Tools (paper-mcp.md › 1), Gebäude-Planer-Tools (hub_*, B-923) und Welten-Tools (world_*, B-954) nehmen checkout
// immer, auch die lesenden; sim_test wie in der Workbench (B-388, seit WZ63.5 im Studio). level_generate, sim_run und replay_run nehmen checkout und
// rechnen mit der EXE dieses Checkouts (RU34.2, RU40.3); sim_test startet sie mit `sim balance`.
func takesCheckout(tool string) bool {
	return writeTools[tool] || strings.HasPrefix(tool, "assets_") || strings.HasPrefix(tool, "hub_") ||
		strings.HasPrefix(tool, "world_") || tool == "sim_test" ||
		tool == "level_generate" || tool == "sim_run" || tool == "replay_run"
}

// writeTools ändern Dateien. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die Repo-Wurzel
// gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
var writeTools = map[string]bool{
	"assets_set": true, "assets_create": true, "assets_style_set": true, "assets_render": true, "assets_delete": true,
	"assets_research": true, "assets_hit_add": true, "assets_hit_drop": true, "assets_import": true, "hub_save": true,
	"world_create": true, "world_set": true, "world_override_set": true, "world_chunks_set": true, "world_story_set": true, "world_delete": true}
