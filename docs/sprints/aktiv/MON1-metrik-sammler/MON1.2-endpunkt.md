# MON1.2 · Diagnose-Ereignisse und GET /api/metrics

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** mon1/2-endpunkt
- **Abhängig von:** MON1.1
- **Tickets:** B-281
- **Kriterien:** AC-02, AC-05, AC-06, AC-07

## Ziel

Ein Ring der letzten 200 Diagnose-Ereignisse (Absturz, 🐢-Tick, Trennung, Client-Fehler) und `GET /api/metrics?since=<Unix-ms>` hinter dem Diagnose-Token liefern Messreihen und Ereignisse als Delta; `docs/protocol.md` beschreibt den Endpunkt mit Beispielantwort.

## Kontext

- Sammler und Messreihen aus MON1.1: `engine/room/monitor.go` (`Monitor`, `Since`), am `Manager`.
- Quellen der Ereignisse: Absturz `Manager.crash` (`engine/room/run.go`), 🐢-Tick dort, wo `noteTick` (`engine/room/logging.go`) meldet (höchstens einmal je 10 s je Raum, nicht je Tick), Trennung `Room.Drop` (`engine/room/actions.go`), Client-Fehler `clientLog` (`engine/net/accesslog.go`, Stufe `error`).
- Token-Schutz wie `/api/status`: `server.authorized` (`engine/net/status.go`): ohne `K3C_STATUS_TOKEN` 404, falsches Token 401, falsche Methode 405. Testvorlage `engine/net/status_test.go`.
- **Parallel:** DBG3 fügt in `engine/net/handler.go` Routen `/api/dev*` hinzu. Hier nur **eine** Zeile in der Routenliste, damit der Merge leicht bleibt.
- Doku: `docs/protocol.md` hat Abschnitte „Diagnose: CPU im Status (B-175)“ und „Spielmetrik-Report (B-150)“; der neue Abschnitt kommt dazwischen.

## Erlaubte Dateien

- `engine/room/monitor.go`, `engine/room/monitor_test.go`; kleine Eingriffe in `engine/room/run.go`, `logging.go`, `actions.go`
- `engine/net/metrics.go`, `engine/net/metrics_test.go` (neu); eine Zeile in `engine/net/handler.go`, kleiner Eingriff in `engine/net/accesslog.go`
- `docs/protocol.md` (nur neuer Abschnitt zu `/api/metrics`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status, Notizen und neue Tickets)

## Nicht-Ziele

Seite (MON2), WebSocket-Push, Speicherung über einen Neustart, pprof, Protokolländerung am WebSocket.

## Schritte

1. `Status: in Arbeit`.
2. Ereignis-Ring (200) im `Monitor`: Zeit, Raum, Art (`crash`, `slow`, `drop`, `client`), Text; Einträge aus den vier Quellen.
3. `engine/net/metrics.go`: `GET /api/metrics?since=` mit `authorized`; Antwort `startedAt`, `now`, Reihen und Ereignisse nur mit Zeit > `since` (fehlt oder ungültig = alles).
4. Tests: Delta und Token (404/401/405) wie `status_test.go` (AC-02); Absturz, 🐢-Tick und Trennung je ein Ereignis mit Zeit, Raum und Art (AC-05).
5. `docs/protocol.md`: Abschnitt mit Feldern und Beispielantwort (AC-06).
6. `task check:go`, `task check`. Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-02: Test: `/api/metrics?since=` liefert `startedAt` und nur neuere Punkte; ohne Token 404, falsches Token 401.
- [ ] AC-05: Test: nach Absturz, 🐢-Tick und Trennung je ein Eintrag mit Zeit, Raum und Art.
- [ ] AC-06: `docs/protocol.md` beschreibt `/api/metrics` mit Beispielantwort.
- [ ] AC-07: `task check:go` grün; dazu `task check` grün, keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

–
