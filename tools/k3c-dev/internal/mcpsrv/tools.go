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
		Name: "workbench_status",
		Description: "Zustand von k3c-dev: Adresse, Laufzeit, Aufrufe, Fehler, Clients und parallele Aufrufe. " +
			"Nutze es bei: Werkzeug antwortet seltsam, welcher Checkout gilt, Versionen. Statt: Prozesse und Ports von Hand prüfen.",
		Annotations: readOnly(),
	}, s.workbenchStatus)
	add(s, &mcp.Tool{
		Name: "notify_ui",
		Description: "Zeigt dem Nutzer einen Hinweis in der Oberfläche von k3c-dev (level info, success, warn oder error). " +
			"Nutze es bei: lange Arbeit fertig, Entscheidung oder Freigabe nötig. Statt: auf den Chat hoffen.",
		Annotations: &mcp.ToolAnnotations{OpenWorldHint: new(bool)},
	}, s.notifyUI)
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name: "check_run",
		Description: "Führt eine Prüfung aus einem festen Katalog aus (task:check, task:test, task:typecheck, task:lint, " +
			"task:build, task:check:go, go:test, go:lint, dev:test) und antwortet mit Exit-Code, Dauer und nur den Fehlerzeilen. " +
			"Ersetzt task check, task test und go test in der Shell. Volle Ausgabe: console_tail check:<ziel>. " +
			"Nutze es bei: jeder Prüfung vor einem Abschluss. Statt: task check, task test, go test oder golangci-lint in der Shell; volle Ausgabe über console_tail check:<ziel>.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, s.checkRun)
	registerLogs(s)
	add(s, &mcp.Tool{
		Name: "reports_list",
		Description: "Xbox-Berichte der Gamepad-Testseite (reports/*.json), neueste zuerst: Datum, Gerät, Controller, FPS. " +
			"Nutze es bei: neue Berichte von der Xbox finden. Statt: ls reports/.",
		Annotations: readOnly(),
	}, s.reportsList)
	add(s, &mcp.Tool{
		Name: "report_read",
		Description: "Ein Xbox-Bericht verdichtet: Controller mit gesehenen Tasten, FPS je Sprite-Stufe, Vollbild, Zurück-Navigationen. " +
			"Nutze es bei: Controller- oder FPS-Befund eines Berichts. Statt: JSON-Datei lesen.",
		Annotations: readOnly(),
	}, s.reportRead)
	add(s, &mcp.Tool{
		Name: "saves_list",
		Description: "Spielstände (saves/ oder K3C_SAVES_DIR) mit Stufe, Tag, Spielern und Datum, Sicherungen eingeschlossen. " +
			"Nutze es bei: welcher Spielstand existiert und wie weit er ist. Statt: saves/ durchsuchen.",
		Annotations: readOnly(),
	}, s.savesList)
	registerServer(s)
	registerEngine(s)
	registerSimTest(s)
	registerServices(s)
	registerTasks(s)
	registerPlanning(s)
}

// registerLogs sind die Tools der Seite Dienste & Logs (Workbench-Spec § 2): Konsole, Logdateien, Fehler.
func registerLogs(s *Server) {
	add(s, &mcp.Tool{
		Name: "console_tail",
		Description: "Flüchtiger Konsolenpuffer eines Dienstes oder Laufs (z. B. Vite, check:task:test), stderr mit \"! \"; " +
			"since liefert nur neuere Zeilen. Nutze es bei: Startfehler, Rohausgabe eines Prüflaufs. Statt: Ausgabe in der Shell mitlesen.",
		Annotations: readOnly(),
	}, s.consoleTail)
	add(s, &mcp.Tool{
		Name: "logs_services",
		Description: "Dienste mit Logdatei (logs/*.jsonl mit Größe und letztem Eintrag) und Konsolen-Quellen. " +
			"Nutze es bei: welcher Name gilt für service. Statt: ls logs/.",
		Annotations: readOnly(),
	}, s.logsSources)
	add(s, &mcp.Tool{
		Name: "logs_stats",
		Description: "Zahl der Einträge je Level oder Namensraum (groupBy) seit since, für einen oder alle Dienste. " +
			"Nutze es bei: Überblick, ob ein Dienst auffällig viel warnt. Statt: Zeilen zählen mit grep.",
		Annotations: readOnly(),
	}, s.logsStats)
	add(s, &mcp.Tool{
		Name: "logs_errors",
		Description: "Warnungen und Fehler, gleichartige Meldungen zu je einer Zeile verdichtet (Standard: ab WARN, 24h, alle Dienste). " +
			"Nutze es bei: erste Fehlersuche. Statt: Logdatei lesen.",
		Annotations: readOnly(),
	}, s.logsErrors)
	add(s, &mcp.Tool{
		Name: "logs_query",
		Description: "Logzeilen eines Dienstes, neueste zuerst, gefiltert nach level, ns, pattern (Regex auf die Meldung) und since. " +
			"Nutze es bei: gezielte Suche. Statt: Datei lesen oder grep.",
		Annotations: readOnly(),
	}, s.logsQuery)
	add(s, &mcp.Tool{
		Name: "logs_context",
		Description: "Zeilen um einen Zeitpunkt ts (before/after), älteste zuerst, \">\" markiert die erste Zeile ab ts. " +
			"Nutze es bei: was geschah vor und nach einem Fehler. Statt: Datei an der Stelle öffnen.",
		Annotations: readOnly(),
	}, s.logsContext)
	add(s, &mcp.Tool{
		Name: "logs_since",
		Description: "Neue Zeilen eines Dienstes ab einem Byte-Cursor, älteste zuerst, mit dem Cursor für den nächsten Aufruf. " +
			"Nutze es bei: Tailing während eines Tests. Statt: tail -f.",
		Annotations: readOnly(),
	}, s.logsSince)
}

// registerEngine sind die In-process-Tools (B-047): rechnen mit engine/level und engine/sim, ohne laufenden Server.
func registerEngine(s *Server) {
	add(s, &mcp.Tool{
		Name: "level_generate",
		Description: "Erzeugt ein Level in-process (engine/level, kein Server nötig): Kopfzeile, eine Zeile je Abschnitt, " +
			"Objekte nach Art, Warnungen der Prüfung. Gleicher Seed ergibt dasselbe Level wie im Spiel. " +
			"Nutze es bei: Level eines Seeds prüfen. Statt: Server starten und Level-Betrachter öffnen.",
		Annotations: readOnlyEngine(),
	}, s.levelGenerate)
	add(s, &mcp.Tool{
		Name: "sim_run",
		Description: "Deterministischer Simulationslauf in-process (engine/sim, kein Server nötig), höchstens 100000 Ticks (30/s): " +
			"Tag, Welle, Gold, Truppen, Verluste, Ende. Optional Eingaben je Monarch und Tick-Bereich (höchstens 100 Segmente). " +
			"Nutze es bei: kurzer deterministischer Regel-Check. Statt: go run oder ein Spiel von Hand.",
		Annotations: readOnlyEngine(),
	}, s.simRun)
	add(s, &mcp.Tool{
		Name: "replay_run",
		Description: "Spielt eine Replay-Datei des Balancing-Testers (task balance:run -- --replay-dir) in-process ohne Bot ab: " +
			"Endzustand-Hash, Burgfall-Tick, Vergleich mit der Aufnahme, Warnung bei anderem Datenstand. Nur Dateien im Repo. " +
			"Nutze es bei: Replay nachrechnen oder Abweichung finden. Statt: Balancing-Binary in der Shell.",
		Annotations: readOnlyEngine(),
	}, s.replayRun)
}

// readOnlyEngine: rechnet nur im Speicher, ändert nichts.
func readOnlyEngine() *mcp.ToolAnnotations { return readOnly() }

// registerServer sind die Tools für den laufenden Go-Server (B-047): nur lesend über /api/status.
func registerServer(s *Server) {
	open := false
	ro := &mcp.ToolAnnotations{ReadOnlyHint: true, IdempotentHint: true, OpenWorldHint: &open}
	add(s, &mcp.Tool{
		Name: "server_status",
		Description: "Laufender Go-Server über /api/status: Version, Laufzeit, Räume, Spielstände, Berichte, letzte Abstürze. " +
			"Token K3C_STATUS_TOKEN, Adresse K3C_SERVER_URL oder 127.0.0.1:K3C_HTTP_PORT. " +
			"Nutze es bei: läuft der Spielserver, welche Version, letzte Abstürze. Statt: curl /api/status.",
		Annotations: ro,
	}, s.serverStatus)
	add(s, &mcp.Tool{
		Name: "rooms_list",
		Description: "Laufende Räume des Go-Servers, eine Zeile je Raum: Geräte, Monarchen, Tick und Tick-Dauer (letzter, p99). " +
			"Nutze es bei: welche Räume laufen und wie schnell sie ticken. Statt: Monitor-Seite oder curl.",
		Annotations: ro,
	}, s.roomsList)
	add(s, &mcp.Tool{
		Name: "room_snapshot",
		Description: "Verdichteter Zustand eines Raums: Tiefe, Tag/Nacht, Welle, Gold je Monarch, Truppen, Gegner, Burg-HP. " +
			"Nutze es bei: Zustand eines Raums während eines Tests. Statt: Dungeon-Master-Seite oder curl.",
		Annotations: ro,
	}, s.roomSnapshot)
}

// registerServices sind die Dienste-Tools (B-067, Workbench-Spec § 2); Start ist nicht zerstörerisch, Stopp und Neustart schon.
func registerServices(s *Server) {
	no, yes, closed := false, true, false
	start := &mcp.ToolAnnotations{DestructiveHint: &no, OpenWorldHint: &closed}
	stop := &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed}
	add(s, &mcp.Tool{
		Name: "svc_status",
		Description: "Alle Dienste (Vite, Spielserver …) mit Zustand, Port, PID, CPU, Speicher, Laufzeit, Neustarts, " +
			"Log-Level der letzten 60 min und letztem Fehler. Nutze es bei: jeder Arbeit an der laufenden Umgebung, als ersten Aufruf. " +
			"Statt: tasklist, netstat oder Raten.",
		Annotations: readOnly(),
	}, s.svcStatus)
	add(s, &mcp.Tool{
		Name: "svc_health",
		Description: "Prüft einen Dienst sofort mit seiner Health-Prüfung, ohne ihn zu ändern; ungesund mit Grund und Belegung des Ports. " +
			"Nutze es bei: Dienst läuft laut svc_status, antwortet aber nicht. Statt: curl auf den Port.",
		Annotations: readOnly(),
	}, s.svcHealth)
	add(s, &mcp.Tool{
		Name: "svc_start",
		Description: "Startet einen Dienst und wartet, bis er gesund ist (waitSeconds, höchstens 60 s). " +
			"Nutze es bei: Dienst gestoppt oder fehlgeschlagen. Statt: task dev oder task start in der Shell; Ausgabe über console_tail <Dienst>.",
		Annotations: start,
	}, s.svcStart)
	add(s, &mcp.Tool{
		Name: "svc_stop",
		Description: "Stoppt einen Dienst samt Prozessbaum; übernommene (vor k3c-dev gestartete) nur mit force. " +
			"Nutze es bei: Port freigeben, Dienst beenden. Statt: taskkill oder Stop-Process.",
		Annotations: stop,
	}, s.svcStop)
	add(s, &mcp.Tool{
		Name: "svc_restart",
		Description: "Startet einen eigenen Dienst neu und wartet, bis er gesund ist; verlangt confirm=true. " +
			"Nutze es bei: Dienst hängt oder braucht neue Konfiguration. Statt: Stopp und Start von Hand in der Shell.",
		Annotations: stop,
	}, s.svcRestart)
	add(s, &mcp.Tool{
		Name: "svc_start_all",
		Description: "Startet alle Dienste, die nicht laufen, und wartet, bis sie gesund sind; antwortet mit svc_status. " +
			"Nutze es bei: Umgebung frisch hochfahren. Statt: mehrere task-Befehle in der Shell.",
		Annotations: start,
	}, s.svcStartAll)
	add(s, &mcp.Tool{
		Name: "svc_stop_all",
		Description: "Stoppt alle eigenen Dienste in umgekehrter Startreihenfolge; übernommene bleiben. " +
			"Nutze es bei: Umgebung herunterfahren. Statt: Prozesse einzeln beenden.",
		Annotations: stop,
	}, s.svcStopAll)
	add(s, &mcp.Tool{
		Name: "ports_status",
		Description: "Belegung der Dev-Ports (Dienste und MCP), auch durch fremde Prozesse, mit PID. " +
			"Nutze es bei: Start scheitert an belegtem Port. Statt: netstat -ano oder Get-NetTCPConnection.",
		Annotations: readOnly(),
	}, s.portsStatus)
	add(s, &mcp.Tool{
		Name: "get_urls",
		Description: "Erreichbare Adressen je Dienst und des MCP-Servers (target: Dienst oder mcp). " +
			"Nutze es bei: vor jedem HTTP-Aufruf und jeder Browserprüfung, gerade im Worktree mit versetzten Ports. Statt: Ports raten.",
		Annotations: readOnly(),
	}, s.getURLs)
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
