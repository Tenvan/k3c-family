# RS2.2 · Workbench startet Sim, Balancing und Level als Kindprozess

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/rs2
- **Abhängig von:** RS2.1
- **Tickets:** B-726
- **Kriterien:** AC-01

## Ziel

Die Workbench ruft Sim, Balancing und Level-Erzeugung nur noch als Kindprozess `bin/k3c-sim` auf und importiert `k3c/engine/sim`, `room`, `net`, `level` nicht mehr (Ausnahme `studioview`). Nach dem Sprint ist es für die Workbench gleich, ob die Engine Go oder Rust ist.

## Kontext

B-726 (freigegeben). Die Schnittstelle liefert RS2.1 (`cmd/k3c-sim`). Betroffen: `tools/k3c-workbench/internal/{balance,botdev,enginetools,botfeed,mcpsrv}`, `studioview` darf laut Nicht-Ziel den laufenden Spielserver nutzen. MCP-Tools `sim_run`, `sim_test` (start, status, stop, list; wait), `level_generate`, Balancing-Seite. Lange Läufe laufen nur über `sim_test`. Fehlt oder veraltet die EXE: klare Meldung mit `task k3c-sim:build`; Absturz: Lauf `fehlgeschlagen`, letzte stderr-Zeilen im Status. Wails-/Workbench-Frontends folgen Atomic Design (`@webkit`), hier nur Go-Seite.

### Hinweise aus RS2.1

- **Kopien entfernen (Pflicht):** RS2.1 hat `internal/balance` (samt Tests, `report/`, `testdata/`) nach `engine/balance` und `internal/enginetools` nach `engine/enginetools` kopiert (nur Importpfade angepasst), `internal/balance/cmd` lebt als `cmd/k3c-sim balance` weiter. Bis RS2.2 laufen die Balance-Tests (~5 min ohne `-short`) und die langen CLI-Tests doppelt. RS2.2 löscht `internal/balance/cmd`, `internal/enginetools` und die Engine-Teile von `internal/balance` samt deren Tests, dazu den Namensraum `balance` (`tools/k3c-workbench/internal/balance/Taskfile.yml`: `balance:build` → `k3c-sim:build`; `fullgame`/`milestone` laufen dann über `go test ./engine/balance`, Taskfile-Änderung dort mit vermerken).
- **Berichtsteil bleibt in der Workbench:** Die Workbench liest Berichte (`ReadSummary`, `Insights`, `GoalView` und deren Typen, `BaselinePath`/`BaselineFile` in `simrun/spec.go`). Diese NICHT aus `engine/balance` importieren (zieht `engine/sim` nach); sie bleiben als reiner Leseteil in `internal/balance` (ohne `engine/*`-Importe, mit ihren schnellen Tests) oder lesen das Berichts-JSON selbst.
- **Offline-Weg:** `simrun/offline.go` baut heute `bin/k3c-balance[-n]` und liest `PROGRESS n/m` von stdout. Neu: `bin/k3c-sim balance --targets --report-dir … --progress …` (gleiche Flags), stdout nur JSON-Zeilen `{"event":"progress","done":n,"total":m}`, `{"event":"report","files":[…]}`, `{"event":"error","message":…}`, `{"event":"aborted"}`; Exit 0/1/2/130. Schnittstelle im Kopfkommentar von `cmd/k3c-sim/main.go`. Parallel-Plätze (`k3c-balance-2` …) entsprechend als `bin/k3c-sim-<n>` oder eine EXE mit mehreren Prozessen.
- `sim_run`/`level_generate`: `k3c-sim run|level` mit JSON auf stdin, Ergebnis `{"event":"result","text":…}` byte-gleich zum heutigen Text.

## Erlaubte Dateien

- `tools/k3c-workbench/**` (Go-Seite)
- `tools/k3c-workbench/Taskfile.yml`
- `docs/messung/rst-baseline.md` (Notiz zum neuen Aufruf)

## Nicht-Ziele

- Rust. Änderungen an `engine/`. Neue Workbench-Oberflächen. `studioview` umbauen.

## Schritte

1. Alle Importstellen aus `go list -deps` ableiten; je Aufruf auf `exec` gegen `bin/k3c-sim` umstellen (Prozess starten, stdout-Zeilen lesen, Bericht lesen).
2. `sim_test`-Fortschritt, `status`, `wait`, `stop` auf die JSON-Zeilen abbilden; Job-Objekt/Prozess-Ende für Abbruch.
3. Fehlerfälle: EXE fehlt/veraltet, Absturz (stderr-Zeilen im Status), Zeitüberschreitung.
4. Importe bereinigen; `go list -deps ./tools/k3c-workbench/...` prüfen; `depguard`-Regel für die Workbench ergänzen, damit sie nicht zurückkehrt.
5. Vergleich: BAS-Standardlauf (1 Seed) und `sim_run`/`level_generate` mit festen Seeds, Bericht-Diff leer bis auf Zeitstempel; Meilenstein-Lauf (coop2, standard, 40 Tage) Dauer ≤ +5 % über `sim_test` messen (lange Läufe nur über `sim_test`).

## Fertig, wenn

- [ ] AC-01: `go list -deps ./tools/k3c-workbench/...` enthält `k3c/engine/sim`, `room`, `net`, `level` nicht (Ausnahme `studioview`, im Ergebnis benannt).
- [ ] AC-01: `sim_run`, `sim_test` offline und `level_generate` liefern dieselben Ergebnisse wie vorher (Bericht-Diff leer bis auf Zeitstempel).
- [ ] AC-01: Meilenstein-Lauf höchstens 5 % langsamer (Zahl im Ergebnis).
- [ ] CI-Voraussetzung: `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Umfang kann 150 geänderte Zeilen überschreiten: dann Reviewer-Agent vor dem Commit.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
