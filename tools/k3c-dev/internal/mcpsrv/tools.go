package mcpsrv

import (
	"context"
	"reflect"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// register ist der einzige Katalog aller Tools (B-046): Name, Beschreibung, Annotations und Handler.
// Ein neues Tool kommt nur hier hinzu; Zähler, Parameter-Hinweis und Oberfläche lesen es von hier.
func register(s *Server) {
	add(s, &mcp.Tool{
		Name:        "workbench_status",
		Description: "Zustand von k3c-dev: Adresse, Laufzeit, Aufrufe, Fehler, Clients und parallele Aufrufe.",
		Annotations: readOnly(),
	}, s.workbenchStatus)
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name: "check_run",
		Description: "Führt eine Prüfung aus einem festen Katalog aus (npm:check, npm:test, npm:typecheck, npm:lint, " +
			"npm:build, go:test, go:lint, dev:test) und antwortet mit Exit-Code, Dauer und nur den Fehlerzeilen. " +
			"Ersetzt npm run check, npm test und go test in der Shell. Volle Ausgabe: console_tail check:<ziel>.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, s.checkRun)
	add(s, &mcp.Tool{
		Name:        "console_tail",
		Description: "Letzte Zeilen einer Konsolen-Quelle, z. B. check:npm:test (volle Ausgabe eines Prüflaufs).",
		Annotations: readOnly(),
	}, s.consoleTail)
}

// readOnly sind die Annotations eines Tools, das nur liest.
func readOnly() *mcp.ToolAnnotations {
	closed := false
	return &mcp.ToolAnnotations{ReadOnlyHint: true, IdempotentHint: true, OpenWorldHint: &closed}
}

// add registriert ein Tool, dessen Handler Text liefert; ein Fehler wird zum Fehler-Ergebnis.
func add[In any](s *Server, t *mcp.Tool, h func(context.Context, In) (string, error)) {
	s.params[t.Name] = jsonNames(reflect.TypeFor[In]())
	s.stats.register(t.Name, t.Description)
	mcp.AddTool(s.mcp, t, func(ctx context.Context, _ *mcp.CallToolRequest, in In) (*mcp.CallToolResult, any, error) {
		text, err := h(ctx, in)
		if err != nil {
			return nil, nil, err
		}
		return textResult(text, false), nil, nil
	})
}
