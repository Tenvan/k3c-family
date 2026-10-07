# TR1.1 · `sim_test` mit Lauf-Register und Modus offline

- **Status:** fertig
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

- [x] AC-01: Test: `start` kehrt sofort mit ID zurück, `status` ≤ 10 Zeilen, `stop` beendet und schreibt einen Bericht, `list` zeigt Läufe.
- [x] AC-02: Test: offline mit Seeds bewertet die Ziele aus `data/balance-targets.json`, Bericht unter `reports/`.
- [x] AC-05: Test: ungültige Werte (z. B. `players: 9`, unbekannter Bot) und `offline` mit Clients werden mit einer Zeile Grund abgelehnt.
- [x] AC-06: Regel steht in `CLAUDE.md` und `docs/arbeitsweise.md`.

## Prüfen

```bash
task check:dev
task check
```

## Ergebnis

- **AC-01 umgesetzt, geprüft** (`go test ./internal/mcpsrv -run SimTest`): `sim_test` mit `start`, `status`, `stop` und `list` (`simtest.go`).
  - `TestSimTestStartSofortUndStop`: `start` kehrt sofort mit ID zurück, der Lauf überlebt den Aufruf; `stop` bricht ab und hinterlässt `reports/simtest-<id>/simtest.md`.
  - `TestSimTestStatusHoechstensZehnZeilen`.
  - `TestSimTestHoechstensZweiLaeufe`: höchstens 2 Läufe je Checkout.
  - Beim Ende der Workbench (`Stop`) werden laufende Läufe abgebrochen.
- **AC-02 umgesetzt, geprüft:** `mode: offline` baut den Balancing-Tester im Checkout (`bin/k3c-balance`, fester Pfad, rechnet mit der Engine des Worktrees) und ruft ihn mit `--targets` auf. Der Bericht des Testers und `simtest.md` liegen unter `reports/simtest-<id>/`.
  - `TestSimTestOfflineBewertetZiele` mit Fake-Prozessen.
  - Echter Lauf mit 3 Seeds am 2026-10-07 nach 6 s fertig: Burg hält Nacht 1–5 0 % (Fail), Erste Mauer 100 % (Pass), Zerstörte Gebäude 0 (Pass).
- **AC-05 umgesetzt, geprüft:** `TestSimTestLehntAb` prüft 14 Fälle, darunter unbekannte Aktion oder Modus, Werte außerhalb der Grenzen, unbekannter Bot oder Focus, `offline` mit Clients, `offline` mit perf und `offline` mit Spielern. Jeder Fall wird mit einer Zeile Grund abgelehnt, ohne Prozessstart. `online` lehnt bis TR1.2/TR1.3 mit Hinweis ab.
- **AC-06 umgesetzt:** Die Regel steht in `CLAUDE.md` („k3c-dev zuerst“) und `docs/arbeitsweise.md` (Autonomer Ablauf), die Instruktionen der Workbench haben den Abschnitt „Testläufe“.
- **Abweichung:** Der Status zeigt die Phase (bauen, rechnen) und die Dauer, aber keinen Fortschritt n/N je Seed, weil der Tester erst am Ende berichtet. Ein Fortschrittsausgang im Tester folgt, falls er gebraucht wird; YAGNI.
- `task check:dev` ist grün (0 Lint-Befunde), der Planungstest ebenfalls. Die laufende Workbench kennt `sim_test` erst nach einem Neubau aus `develop`.
