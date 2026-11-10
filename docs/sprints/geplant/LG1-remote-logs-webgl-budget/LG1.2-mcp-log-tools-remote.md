# LG1.2 · MCP-Log-Tools lesen entfernte Server

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** ?
- **Branch:** lg1/2-mcp-log-tools-remote
- **Abhängig von:** LG1.1
- **Tickets:** B-771
- **Kriterien:** AC-02

## Ziel

Die `logs_*`-Tools, `server_status` und die Berichts-Tools der Workbench lesen mit `server` einen entfernten Server (Standard `pi` = `pi-gaming:8080`) über die Dev-Endpunkte (B-771).

## Kontext

Die Log-Tools der Workbench (`tools/k3c-workbench`, MCP-Tools `logs_errors`, `logs_query`, `logs_since`, `logs_context`, `logs_stats`, `logs_services`) lesen lokale Logs. LG1.1 liefert `/api/dev/logs`, `/api/dev/log?file=&since=` und `/api/dev/status` mit allen Logs. MCP-Regeln: Namen `<bereich>_<verb>`, Beschreibung endet mit „Nutze es bei: … Statt: …“. Spec: B-771.

## Erlaubte Dateien

- `tools/k3c-workbench/` (MCP-Tool-Definitionen, Log-Zugriff, Konfiguration bekannter Server) samt Tests
- `docs/` nur die MCP-Beschreibung der Tools

## Nicht-Ziele

Kein Schreiben und keine Dienststeuerung auf dem Remote-Server, kein Streaming, keine Vermischung lokaler und entfernter Zeilen.

## Schritte

1. Konfiguration bekannter Server (Kurzname → Adresse), `pi` vorbelegt; nur diese oder Heimnetz-Adressen, keine Weiterleitungen, Zeitlimit.
2. Remote-Quelle hinter den `logs_*`-Tools: Zeilen über `/api/dev/log`, gleiche Filter und Verdichtung wie lokal, Antwort nennt die Quelle.
3. `server_status`/`workbench_status` und `reports_list`/`report_read` mit `server`.
4. Tests mit `httptest`-Server: Erfolg, nicht erreichbar, 403, „Log aus“, unbekannter Name, fremde Adresse. Reviewer-Agent vor dem Commit (Netzwerkzugriff).

## Fertig, wenn

- [ ] AC-02: `B-771/AC-01` bis `AC-03` grün (`k3c-workbench:gotest`), lokale Aufrufe unverändert.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
