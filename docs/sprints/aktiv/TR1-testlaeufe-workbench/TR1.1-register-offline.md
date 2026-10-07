# TR1.1 · `sim_test` mit Lauf-Register und Modus offline

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** tr1/1-register-offline
- **Abhängig von:** –
- **Tickets:** B-348
- **Kriterien:** AC-01, AC-02, AC-05, AC-06

## Ziel

Die Workbench hat das MCP-Tool `sim_test` mit `start`, `status`, `stop` und `list`. Ein Lauf arbeitet im Hintergrund. Der Modus `offline` (headless, Mocks) rechnet Balancing-Läufe über viele Seeds und bewertet die Ziele. Die Regel „Testläufe nur über `sim_test`“ steht in `CLAUDE.md` und `docs/arbeitsweise.md`.

## Kontext

- Workbench: `tools/k3c-dev/internal/mcpsrv/` (`tools*.go` registrieren Tools über `add`, Muster `check_run.go`: fester Katalog, `runProcess`, Zustand `checkRuns`; `tools_engine.go` für `sim_run`). Kompakte Ausgabe wie `check_compact.go`. Checkout je Aufruf: `workspace.go` (`X-K3C-Root`).
- Balancing-Tester: `tools/k3c-dev/internal/balance/` (`run.go` Matrix, `targets.go`/`evaluate.go` Ziele aus `data/balance-targets.json`, `summary.go` Berichte, `bots.go` Profile über `BotNames()`); CLI `internal/balance/cmd` (`task balance`). Der Lauf ruft das Paket im Prozess auf (kein Unterprozess nötig) und meldet den Fortschritt je Seed.
- Parameter (B-348 › Anforderungen): `action`, `id`, `mode` (`offline`|`online`), `clients` (0–4), `players` (1–4), `rooms` (1–8), `bots` (aus `BotNames()`), `seeds` (1–200), `days` (1–30), `duration` (online), `focus` (`balance`,`perf`,`stability`). In dieser Session funktioniert nur `offline` mit `clients: 0`. `online` lehnt mit „folgt mit TR1.2/TR1.3“ ab, `offline` mit Clients dauerhaft mit Grund.
- Lauf-Register: ID `run-<n>` je Workbench, höchstens 2 gleichzeitige Läufe je Checkout, Zeitlimit je Lauf, `stop` über `context.Cancel`. Beim Ende schreibt der Lauf `reports/simtest-<Zeit>-<id>.json` und `.md` in den Checkout (Muster `summary.go`).
- Status: höchstens 10 Zeilen (Kopf mit ID, Modus, Phase, Fortschritt n/N, Dauer; dann Ziele mit Pass/Fail; Fehlerzahl; Bericht).
- Instruktionen der Workbench für Agenten: `tools/k3c-dev/internal/mcpsrv/instructions.md`.

## Erlaubte Dateien

- `tools/k3c-dev/internal/mcpsrv/` (neu: `tools_simtest.go`, `simtest_*.go` und Tests; `instructions.md`; Registrierung in `tools.go`)
- `tools/k3c-dev/internal/balance/` (nur ein Einstieg für Fortschritt und Abbruch, falls nötig)
- `CLAUDE.md` (Abschnitt „k3c-dev zuerst“), `docs/arbeitsweise.md` (Autonomer Ablauf, ein Satz)
- `docs/sprints/aktiv/TR1-testlaeufe-workbench/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Modus `online` (TR1.2, TR1.3), Änderung der Bot-Profile, Oberfläche der Workbench (eigenes Ticket, falls gewünscht).

## Schritte

1. Branch, `Status: in Arbeit`. `check_run.go`, `tools_engine.go`, `internal/balance/run.go` lesen.
2. Register und Tool `sim_test` mit Parameterprüfung; Läufe in Goroutine mit Context.
3. Runner `offline`: Balancing-Matrix im Prozess mit Fortschritt, Bewertung der Ziele, Bericht.
4. Tests: Fake-Runner (start sofort mit ID, status ≤ 10 Zeilen, stop schreibt Bericht mit Abbruch, list); offline mit 3 Seeds bewertet Ziele; ungültige Werte und Kombinationen abgelehnt.
5. Regel in `CLAUDE.md` und `docs/arbeitsweise.md`, `instructions.md` ergänzen.
6. `check_run dev:test`, `task check`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-01: Test: `start` kehrt sofort mit ID zurück, `status` ≤ 10 Zeilen, `stop` beendet und schreibt einen Bericht, `list` zeigt Läufe.
- [ ] AC-02: Test: offline mit Seeds bewertet die Ziele aus `data/balance-targets.json`, Bericht unter `reports/`.
- [ ] AC-05: Test: ungültige Werte (z. B. `players: 9`, unbekannter Bot) und `offline` mit Clients werden mit einer Zeile Grund abgelehnt.
- [ ] AC-06: Regel steht in `CLAUDE.md` und `docs/arbeitsweise.md`.

## Prüfen

```bash
task check:dev
task check
```

## Ergebnis

–
