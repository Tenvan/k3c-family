package mcpsrv

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// register ist der einzige Katalog aller Tools (B-046): Name, Beschreibung, Annotations und Handler.
// Ein neues Tool kommt nur hier hinzu; Zähler, Parameter-Hinweis und Oberfläche lesen es von hier.
func register(s *Server) {
	add(s, &mcp.Tool{
		Name: "workbench_status",
		Description: "Zustand von k3c-workbench: Adresse, Laufzeit, Aufrufe, Fehler, Clients und parallele Aufrufe. " +
			"Nutze es bei: Werkzeug antwortet seltsam, welcher Checkout gilt, Versionen. Statt: Prozesse und Ports von Hand prüfen.",
		Annotations: readOnly(),
	}, s.workbenchStatus)
	add(s, &mcp.Tool{
		Name: "workbench_resources",
		Description: "Ressourcenverbrauch von k3c-workbench: Live-Heap, Prozess-RSS, Kindprozesse, Goroutinen, Handles, Trend je Stunde, " +
			"Größe der Puffer (Konsole, Prüfläufe), die Vorgänge mit der größten Allokation und abgelegte Heap-Profile (go tool pprof). " +
			"Nutze es bei: Workbench wird langsam oder frisst Speicher, Verdacht auf ein Leck. Statt: Task-Manager und Raten.",
		Annotations: readOnly(),
	}, s.workbenchResources)
	add(s, &mcp.Tool{
		Name: "notify_ui",
		Description: "Zeigt dem Nutzer einen Hinweis in der Oberfläche von k3c-workbench (level info, success, warn oder error). " +
			"Nutze es bei: lange Arbeit fertig, Entscheidung oder Freigabe nötig. Statt: auf den Chat hoffen.",
		Annotations: &mcp.ToolAnnotations{OpenWorldHint: new(bool)},
	}, s.notifyUI)
	closed, destructive := false, false
	add(s, &mcp.Tool{
		Name:        "check_run",
		Description: checkRunDescription,
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &destructive, OpenWorldHint: &closed},
	}, s.checkRun)
	registerLogs(s)
	registerServices(s)
	registerTasks(s)
	registerPlanning(s)
	// Weltdesign (world_*, assets_*, hub_*, level_generate, WZ63) und Balancing (sim_test, sim_run, replay_run,
	// reports_list, report_read, saves_list, WZ63.5) liegen im MCP-Server k3c-studio.
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

// registerServices sind die Dienste-Tools (B-067, Workbench-Spec § 2); Start ist nicht zerstörerisch, Stopp und Neustart schon.
func registerServices(s *Server) {
	no, yes, closed := false, true, false
	start := &mcp.ToolAnnotations{DestructiveHint: &no, OpenWorldHint: &closed}
	stop := &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed}
	add(s, &mcp.Tool{
		Name: "svc_status",
		Description: "Alle Dienste aus services.json mit Zustand, Port, PID, CPU, Speicher, Laufzeit, Neustarts, " +
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
		Description: "Stoppt einen Dienst samt Prozessbaum; übernommene (vor k3c-workbench gestartete) nur mit force. " +
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
