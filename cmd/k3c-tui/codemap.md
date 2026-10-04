# cmd/k3c-tui/

## Responsibility

CLI entry point (Terminal-UI): Diagnose-Oberfläche für den laufenden `k3c-server` mit Räumen, Geräten, Tick-Dauer, Speicher, Abstürzen und Log (B-002). Spricht nur die HTTP-Diagnose-API mit Token; `-once` druckt den Zustand einmal als Text.

## Design

- Elm-Architektur mit Bubble Tea v2: `model` (`Init`/`Update`/`View`) in `model.go`, Ansichten `screen` (Übersicht, Raum, Log), Nachrichten `statusMsg`, `roomMsg`, `logMsg`, `actionMsg`; `call` führt Aufrufe mit Zeitlimit als `tea.Cmd` aus, `chain` ordnet Takte einer Ansicht zu.
- `client.go`: `client` (`status`, `room`, `disconnect`, `save`, `log`) mit eigenen JSON-Typen; importiert nichts aus `engine/`. Token nie in Fehlertexten, Fehler als `meldung`.
- Rendering: `view.go` (`render`, `plain` ohne Farbe), `roomview.go` (`renderRoom`), `logview.go` (`logState`, `renderLog`, `formatLogLine` für slog-JSON) mit lipgloss-Stilen.
- `keys.go` (Tastenverteilung je Ansicht), `actions.go` (`disconnectCmd`, `saveCmd`).

## Flow

1. `main` -> `run`: Flag `-once`, `clientFromEnv` (`K3C_STATUS_TOKEN`, `K3C_SERVER_URL` oder `K3C_HTTP_PORT`).
2. `-once`: `printOnce` holt `status` (5 s Limit), druckt `render(plain)`, Exit 0/1.
3. Sonst `tea.NewProgram(newModel(c)).Run()`: `Init` -> `poll` (jede Sekunde, nach einem Fehler alle 2 s).
4. Tasten -> `key` -> Ansicht wechseln (`enter`), Gerät trennen (mit Rückfrage `askDisconnect`), Raum speichern, Log öffnen (`openLog`, holt Seiten per Cursor).
5. Antworten kommen als Nachrichten in `Update`, `View` rendert.

## Integration

- Server-Endpunkte: `GET /api/status[?room=]`, `GET /api/status/log`, `POST /api/status/disconnect` und `/save` (`engine/net`, JSON-Formen aus `engine/room`).
- Abhängigkeiten: `charm.land/bubbletea/v2`, lipgloss.
- Gebaut im `Dockerfile` nach `/usr/local/bin/k3c-tui`; `.github/workflows/ci.yml` ruft `k3c-tui -once` im Container auf.
