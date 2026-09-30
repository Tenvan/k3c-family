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
	add(s, &mcp.Tool{
		Name:        "logs_sources",
		Description: "Alle Log-Quellen (logs/*.jsonl mit Größe und letztem Eintrag) und Konsolen-Quellen.",
		Annotations: readOnly(),
	}, s.logsSources)
	add(s, &mcp.Tool{
		Name: "logs_query",
		Description: "Einträge einer Log-Quelle, neueste zuerst, gefiltert nach minLevel, ns, pattern (Regex auf die " +
			"Meldung) und since; statt die Datei zu lesen.",
		Annotations: readOnly(),
	}, s.logsQuery)
	add(s, &mcp.Tool{
		Name:        "logs_errors",
		Description: "Warnungen und Fehler einer Log-Quelle, gleichartige Meldungen zu je einer Zeile verdichtet (Standard: ab WARN, 24h).",
		Annotations: readOnly(),
	}, s.logsErrors)
	add(s, &mcp.Tool{
		Name:        "logs_since",
		Description: "Neue Einträge einer Log-Quelle ab einem Byte-Cursor, älteste zuerst, mit dem Cursor für den nächsten Aufruf.",
		Annotations: readOnly(),
	}, s.logsSince)
	add(s, &mcp.Tool{
		Name:        "reports_list",
		Description: "Xbox-Berichte der Gamepad-Testseite (reports/*.json), neueste zuerst: Datum, Gerät, Controller, FPS.",
		Annotations: readOnly(),
	}, s.reportsList)
	add(s, &mcp.Tool{
		Name:        "report_read",
		Description: "Ein Xbox-Bericht verdichtet: Controller mit gesehenen Tasten, FPS je Sprite-Stufe, Vollbild, Zurück-Navigationen.",
		Annotations: readOnly(),
	}, s.reportRead)
	add(s, &mcp.Tool{
		Name:        "saves_list",
		Description: "Spielstände (saves/ oder K3C_SAVES_DIR) mit Stufe, Tag, Spielern und Datum, Sicherungen eingeschlossen.",
		Annotations: readOnly(),
	}, s.savesList)
	registerServices(s)
}

// registerServices sind die Dienste-Tools (B-067); Start ist nicht zerstörerisch, Stopp und Neustart schon.
func registerServices(s *Server) {
	no, yes, closed := false, true, false
	add(s, &mcp.Tool{
		Name: "svc_status",
		Description: "Alle Dienste (Vite, Heimnetz …) mit Zustand, Port, PID, CPU, Speicher, Laufzeit, Neustarts, " +
			"Log-Level der letzten 60 min und letztem Fehler.",
		Annotations: readOnly(),
	}, s.svcStatus)
	add(s, &mcp.Tool{
		Name: "svc_start",
		Description: "Startet einen Dienst und wartet, bis er gesund ist (höchstens 60 s). Statt npm run dev oder " +
			"npm start in der Shell; Ausgabe über console_tail <Dienst>.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &no, OpenWorldHint: &closed},
	}, s.svcStart)
	add(s, &mcp.Tool{
		Name:        "svc_stop",
		Description: "Stoppt einen Dienst samt Prozessbaum; übernommene (vor k3c-dev gestartete) nur mit force.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed},
	}, s.svcStop)
	add(s, &mcp.Tool{
		Name:        "svc_restart",
		Description: "Startet einen eigenen Dienst neu und wartet, bis er gesund ist.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed},
	}, s.svcRestart)
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
