package mcpsrv

import (
	"context"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/planning"
)

type planDigestIn struct {
	OlderThan string `json:"älter_als,omitempty" jsonschema:"nur Sprints vor dieser Sprint-ID (gleiches Präfix, z. B. SP10) oder mit letzter Abnahme vor dem Datum JJJJ-MM-TT; leer = alle erledigten"`
	Dry       *bool  `json:"dry,omitempty" jsonschema:"nur anzeigen, nichts schreiben (Standard true)"`
	Renew     bool   `json:"neu,omitempty" jsonschema:"vorhandene ZUSAMMENFASSUNG.md neu schreiben (Standard: unverändert lassen)"`
}

// registerPlanDigest ist plan_digest: Zusammenfassungen erledigter Sprints (B-696 Teil C).
func registerPlanDigest(s *Server) {
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name: "plan_digest",
		Description: "Schreibt je erledigtem Sprint docs/sprints/erledigt/<Sprint>/ZUSAMMENFASSUNG.md (höchstens 60 Zeilen): Ziel und " +
			"Abnahme gekürzt, Kriterien mit Status aus den Session-Ergebnissen, Tickets mit Status; regelbasiert, ohne KI. " +
			"Originale bleiben, vorhandene Zusammenfassungen nur mit neu. dry (Standard true) zeigt nur die Sprints. " +
			"Nutze es bei: erledigte Sprints verdichten, bevor eine Session sie liest. Statt: Sprint-Ordner ganz lesen oder Zusammenfassungen von Hand.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, func(ctx context.Context, in planDigestIn) (string, error) {
		dry := in.Dry == nil || *in.Dry
		return planning.Digest(s.ws(ctx).root, planning.DigestIn{OlderThan: in.OlderThan, Dry: dry, Renew: in.Renew})
	})
}
