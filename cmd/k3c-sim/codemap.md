# cmd/k3c-sim/

## Responsibility

CLI Entry Point (`package main`): das Engine-CLI der Workbench (B-726). Rechnet Simulation, Level, Balancing und
Welten-Prüfung als Kindprozess, damit die Workbench `engine/sim` nicht importieren muss. Feste EXE `bin/k3c-sim`.

## Design

- `main.go`: Unterbefehl-Dispatch (`run`, `level`, `balance`, `world`), Schnittstellen-Vertrag im Kopfkommentar.
  `emitter` schreibt nur JSON-Zeilen mit `"event"` auf stdout (`progress`, `result`, `report`, `error`, `aborted`),
  Meldungen für Menschen gehen nach stderr. `usageError` → Exit 2, Lauffehler → 1, Abbruch (Strg+C, CTRL_BREAK,
  SIGTERM) → 130.
- `input.go`: `runInput`/`levelInput` lesen JSON aus `--input DATEI` oder stdin und rufen `enginetools.Run` bzw.
  `enginetools.Level`.
- `balance.go`: `matrixOpts`, Flags wie das frühere `k3c-balance`; Matrix-Bericht (`balance.Run`), Bot-Spiel
  (`balance.Play`, `balance.BotNames`), Replay (`balance.ReadReplay`). Das JSON-Objekt der Eingabe liefert Flag-Werte,
  die Befehlszeile ergänzt bzw. überschreibt sie.
- `targets.go` (`targetOpts`, Seed-Modus B-634): Zielläufe gegen `balance.DefaultTargets`, Varianten
  (`balance.WithVariants`), Neubewertung (`balance.ReadSummary`, `balance.Reevaluate`), Berichte
  (`balance.WriteReports`, `balance.FiguresOf`).
- `tune.go` (`tuneOpts`, `paramList` für wiederholbares `--param`), `sensitivity.go` (`sensOpts`): Tuning- und
  Sensitivitätsläufe über `engine/balance`.
- `world.go` (B-879, Natur B-891): `worldInput` mit `op` `level` | `values` | `check` für den Welten-Editor; Antworten
  `worldLevelReport` (Layout, Natur-Werte, Eigenschaften, Gewichts-Gruppen) und `worldValuesReport` (aufgelöste
  Welt-Werte samt Nachfolgern).
- `Taskfile.yml`: Namespace `k3c-sim` mit `build` (nur bei geänderten Quellen, nach `bin/k3c-sim`), `run`, `default`.

## Flow

1. Workbench startet `bin/k3c-sim <befehl> --input …` als Kindprozess.
2. `run`/`level`: JSON → `enginetools` → `{"event":"result","text":…}`.
3. `balance`: Flags + JSON → `engine/balance` → `progress`-Zeilen, dann `result` (Bericht) oder `report`
   (Dateien in `--report-dir`, Workbench: `reports/simtest-<id>/`).
4. `world`: Entwurf lesen (`sim.ParseWorld`, `sim.LoadWorld`, `sim.ResolveWorldValues`), Level erzeugen und prüfen
   (`level.LoadBiome`, `level.GenerateWith`, `level.ValidateWith`, `level.NatureValues`) → `result` mit `report`.

## Integration

- Abhängigkeiten: `k3c/engine/enginetools`, `k3c/engine/balance`, `k3c/engine/level`, `k3c/engine/sim`, `k3c/data`.
- Konsumenten: `tools/k3c-workbench/internal/simexe` (Start der EXE), `internal/simrun`, `internal/mcpsrv`
  (`sim_run`, `level_generate`, `sim_test`, Replay), `internal/world` (Welten-Editor), `internal/balance`.
- Gebaut über `task k3c-sim:build` bzw. `task buildall`.
