package mcpsrv

import (
	"context"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/planning"
)

type planIndexIn struct {
	Dry *bool `json:"dry,omitempty" jsonschema:"nur die Zahlen zeigen, nichts schreiben (Standard true)"`
}

// plan_index schreibt Dateien: Argument checkout und Zeile „Checkout:“ wie die anderen schreibenden Tools (B-388).
func init() { writeTools["plan_index"] = true }

// registerPlanIndex ist plan_index: Haupt- und Archiv-Index aus den Ticket-Dateien neu aufbauen (B-864).
func registerPlanIndex(s *Server) {
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name: "plan_index",
		Description: "Baut docs/backlog/README.md (offene und eingeplante Tickets) und docs/backlog/archiv/README.md (erledigte " +
			"und verworfene) aus den Ticket-Dateien neu auf: Zeilen gelöschter oder verschobener Dateien verschwinden, fehlende " +
			"kommen nach Nummer dazu, die Reihenfolge bisheriger Zeilen bleibt. dry (Standard true) zeigt nur die Zahlen. " +
			"Nutze es bei: Index passt nicht mehr zu den Dateien (z. B. nach git rm oder Umnummerieren). Statt: Index von Hand bearbeiten.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in planIndexIn) (string, error) {
		dry := in.Dry == nil || *in.Dry
		return planning.RebuildIndex(s.ws(ctx).root, dry)
	})
}
