# WR2.2 · Workbench: Dienste-Texte, Mocks und Fixtures ohne Spielserver und Vite

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/wr2
- **Abhängig von:** WR2.1
- **Tickets:** B-897
- **Kriterien:** AC-01, AC-04, AC-08

## Ziel

Dienste-Infrastruktur der Workbench bleibt generisch; Texte zum Port-Versatz, Frontend-Mocks und Fixtures nennen weder Spielserver noch Vite.

## Kontext

B-897 › Anforderungen; Default der offenen Frage: Dienste und `svc_*` bleiben.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/internal/services/**`, `tools/k3c-workbench/internal/studioview/**`
- `tools/k3c-workbench/frontend/src/api/**` und die Tests, die deren Mock-Daten prüfen
- `tools/checkdiff/**`

## Nicht-Ziele

- Dienste-Infrastruktur entfernen; Test-Fixtures mit „Spielserver“ als erfundenem Dienstnamen.

## Schritte

1. Kommentare zum Port-Versatz und zu Diensten in Go nachziehen.
2. Frontend-Mocks (Dienste, Tasks, Logs, Planung) auf erfundene generische Dienste umstellen.
3. Fixtures in `tools/checkdiff` ohne `engine/net`.

## Fertig, wenn

- AC-01, AC-04 erfüllt, `task check:diff` grün.

## Prüfen

- `check_run task:check:diff` (Worktree).

## Ergebnis

- Dienste-Infrastruktur und `svc_*` bleiben (Default der offenen Frage, generisch für künftige Dienste); `services.json` bleibt `[]`.
- Kommentare zu Port-Versatz, Proxy-Ports und Supervisor in `mcpsrv/worktree_services.go`, `mcpsrv/tools_services.go`, `services/config.go`, `controller.go`, `system.go`, `resmon/proc_windows.go`, `studioview/game.go` ohne Spielserver und Vite bzw. als Historie mit Ticket-Verweis.
- Frontend-Mocks (`mockServices`, `mockLogs`, `mockLogFiles`, `mockMcp`, `mockTasks`) mit erfundenen Diensten Docs, Backend, Indexer, Suche (Ports 7000 ff.) statt Vite und Spielserver.
- Behalten: Unit-Test-Fixtures mit „Vite“/„Spielserver“ als erfundenen Dienstnamen (Go und Vitest, Nicht-Ziel) und `mockPlanning.ts` (erfundene Planungsdaten, von Planungs-Tests genutzt).
- `tools/checkdiff/select_test.go` nutzt `engine/room` statt `engine/net`.
- `task check:diff` grün; gezielt Vitest `src/api src/logs src/services src/mcp src/tasks` grün (96 Tests).
