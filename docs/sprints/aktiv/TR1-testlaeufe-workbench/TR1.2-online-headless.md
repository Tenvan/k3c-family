# TR1.2 · `sim_test` online headless: Bot-Geräte gegen den Spielserver

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** tr1/2-online-headless
- **Abhängig von:** TR1.1
- **Tickets:** B-348
- **Kriterien:** AC-03, AC-05

## Ziel

`sim_test` mit `mode: online`, `clients: 0` legt `rooms` Räume mit je `players` Bot-Geräten auf dem Spielserver des Checkouts an. Die Bots steuern ihre Monarchen mit den Balancing-Profilen. Im Status stehen Tick-Dauer (p50/p99), CPU, Trennungen und Fehler, der Bericht bewertet Performance und Stabilität.

## Kontext

- Vorbild Gerät und Messung: `cmd/k3c-load/` (`bots.go` spricht `hello`, `create`, `join`, `input`, `leave` mit eigenen Nachrichtentypen; `api.go` liest `/api/status` mit Token; `poll.go` Proben; `report.go` Bewertung gegen `-target-p99` 10 ms). `cmd/` ist ein anderes Paket: Code nicht importieren, sondern in `tools/k3c-dev/internal/` neu und klein halten (oder das Protokoll über `engine/net` nutzen, falls exportiert). Serverzugriff der Workbench: `internal/serverapi` (Token, Adresse je Worktree).
- Bot-Entscheidung: `internal/balance/bots.go` (`type Bot func(w *sim.World, p *sim.Player) sim.PlayerCommand`). Der Bot braucht eine `sim.World` der eigenen Stufe: aus `snap` dekodieren und `delta` anwenden (Format `docs/protocol.md` › Zustand und Delta, Listen mit `id` laut `engine/net/delta.go › idLists`). Die Entscheidung läuft je empfangenem Zustand, die Eingabe geht als `input` zurück.
- Stabilität: Trennungen der Bot-Geräte, Fehler aus den Logs des Spielservers seit Laufbeginn (`internal/logs`, Muster `logs_errors`), Server-Absturz (Dienst `svc_status`).
- Focus `balance` online: Ereignisse (`castleFallen`, `built`, `wave`, `dawn`) aus den Zuständen zählen wie offline; Bewertung über dieselben Ziele, soweit die Ereignisse vorliegen.
- Test: Server im Testprozess (`engine/net` Handler mit `httptest`), 2 Räume × 2 Bots, 5 s.

## Erlaubte Dateien

- `tools/k3c-dev/internal/` (neu: Paket für Bot-Geräte, z. B. `internal/botdev/`; `mcpsrv/simtest_*.go`, Tests)
- `docs/sprints/aktiv/TR1-testlaeufe-workbench/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Clients (TR1.3), Änderungen am Server oder Protokoll (SRV-Ticket, falls nötig), Ablösung von `task load` (bleibt als CLI).

## Schritte

1. Branch, `Status: in Arbeit`. `cmd/k3c-load`, `docs/protocol.md`, `engine/net/delta.go` lesen.
2. Bot-Gerät: Verbindung, Raum anlegen/beitreten, Zustand halten (snap + delta → `sim.World`), Bot entscheiden, `input` senden.
3. Runner `online` headless: Räume und Geräte starten, Proben von `/api/status`, Fehler aus Logs, Bericht, Status.
4. Tests mit Server im Testprozess; ungültige Kombination (Server läuft nicht) abgelehnt.
5. `task check:dev`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-03: Test: 2 Räume × 2 Bot-Geräte bewegen ihre Monarchen (x ändert sich), Status zeigt Tick p99, Trennungen und Fehler, Bericht bewertet p99 gegen 10 ms.
- [ ] AC-05: Test: `online` ohne laufenden Server lehnt mit Hinweis auf `svc_start` ab.

## Prüfen

```bash
task check:dev
```

## Ergebnis

–
