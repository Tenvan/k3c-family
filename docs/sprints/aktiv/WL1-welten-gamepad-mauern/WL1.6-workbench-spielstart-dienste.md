# WL1.6 · Workbench: Spielstart Dev/Release als Dienste mit Log

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wl1/6-workbench-spielstart-dienste
- **Abhängig von:** –
- **Tickets:** B-1214
- **Kriterien:** AC-05

## Ziel

Dev- und Release-Start des Rust-Spiels sind Dienste der Workbench (`game-dev`, `game-release`: Start, Stopp, Status in Oberfläche und `svc_*`), ihre Ausgabe steht im Log je Dienst, und ein Neubau-Task beendet vorher das laufende Paket-Spiel (B-1214 AC-01 bis AC-03, Sprint-AC-05).

## Kontext

- Dienste: `tools/k3c-workbench/services.json`, Paket `internal/services` (Konfiguration `config.go`, Ablauf `lifecycle.go`, Übernahme `adopt.go`, Start über `workkit/proc` im Job-Objekt `runtime.go`, Ausgabe-Log `outlog.go` nach `logs/<dienst>.jsonl`, von `logs_query` gefunden).
- Bisher braucht jeder Dienst Port und Health `http`/`tcp`; das Spiel hat keinen Port. Neu: Health `process` (gesund, solange der Prozess läuft, Port 0), `manual` (nicht über StartAll), `needs`/`needsHint` (fehlende EXE: „erst task build“), `stopBefore` (Tasks, vor denen der Dienst endet).
- Tasks der Workbench starten in `app_tasks.go` (`TaskStart`, auch MCP `task_start`). Die Oberfläche kennt leere Health schon als „process“ (`frontend/src/services/ServiceCard.tsx`).

## Erlaubte Dateien

- `tools/k3c-workbench/**`
- `engine/devrun/**`

## Nicht-Ziele

Spielstand-Steuerung aus der Workbench (RU34); Start des Spiels aus der Shell (`task build` außerhalb der Workbench) verändern; neue Oberflächen-Bausteine.

## Schritte

1. Tests in `internal/services/game_test.go`: Dienst-Definitionen, Start/Stopp mit Platzhalter-Prozess, Ende ohne Fehler, fehlende EXE, Ausgabe im Log, Stopp vor Neubau-Task.
2. Health `process` samt Feldern in `config.go`, Logik in `process.go`, Anbindung in `lifecycle.go`, `adopt.go`, `query.go`, `controller.go`, `runtime.go`.
3. `services.json` um `game-dev` und `game-release`; `app_tasks.go` ruft `StopForTask`; MCP-Antworten und Karte zeigen „ohne Port“.
4. go vet, go test, Frontend tsc und vitest, `tests/projectRules`, `atomicDesign`, `mcpToolsDisjoint`, `task check:diff`.

## Fertig, wenn

- [ ] AC-05 / B-1214 AC-01: `TestServicesJSONKenntDieSpielDienste`, `TestProzessDienstStartStopUndEnde` grün.
- [ ] B-1214 AC-02: `TestProzessDienstAusgabeImLog` grün (Quelle `logs/game-dev.jsonl`).
- [ ] B-1214 AC-03: `TestStopForTask` und `TestProzessDienstOhneExe` grün; Abnahme am PC durch 🧑.

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

- AC-05 / B-1214 AC-01 umgesetzt: Dienste `game-dev` (`task rust:game:dev`) und `game-release` (`dist/k3c-game/k3c-game.exe`, Arbeitsordner `dist/k3c-game`) mit Health `process` (ohne Port, nur von Hand, nicht über StartAll/Adopt). Geprüft: `TestServicesJSONKenntDieSpielDienste`, `TestProzessDienstStartStopUndEnde`, `TestLoadProzessDienste`.
- B-1214 AC-02 umgesetzt: Ausgabe über den bestehenden Ausgabe-Log nach `logs/game-dev.jsonl` bzw. `logs/game-release.jsonl` (Quelle für `logs_query`). Geprüft: `TestProzessDienstAusgabeImLog`.
- B-1214 AC-03 umgesetzt: Stopp über `workkit/proc` (Job-Objekt); `stopBefore` beendet `game-release` vor `build`, `buildall`, `rust:game:build`, `rust:game:run` aus der Workbench (Hinweis im Log); fehlende EXE meldet „erst task build“. Geprüft: `TestStopForTask`, `TestProzessDienstOhneExe`; Abnahme am PC durch 🧑 offen.
- Prüfung: go vet, go test (`.`, `internal/services`, `internal/mcpsrv`), Frontend tsc und vitest (330), `tests/projectRules`, `atomicDesign`, `mcpToolsDisjoint` grün; check:diff ohne golangci-lint (fehlt auf dem Rechner). Kein Reviewer-Agent gelaufen.
- Grenze: `task build` aus der Shell stoppt das Spiel nicht (nur Tasks aus der Workbench); `engine/devrun` blieb unberührt.
