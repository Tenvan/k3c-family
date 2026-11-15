# cmd/k3c-sim/

## Responsibility

CLI Entry Point (`package main`): dünne Hülle um `engine/simcli` für Taskfile und CI (B-924). Die Logik der
Unterbefehle (`run`, `level`, `balance`, `world`, `hub`) steht in `engine/simcli`; die Workbench ruft sie im Prozess auf,
ohne `bin/k3c-sim`. Feste EXE `bin/k3c-sim`.

## Design

- `main.go`: Schnittstellen-Vertrag im Kopfkommentar; `main` ruft `simcli.Exec`, `onAbort` meldet Strg+C, CTRL_BREAK und
  SIGTERM als `{"event":"aborted"}` mit Exit 130.
- `main_test.go`: Test mit echtem Prozess (Abbruch per Prozess-Ende, Signal-Abbruch); die Rechen-Tests liegen in
  `engine/simcli`.
- `Taskfile.yml`: Namespace `k3c-sim` mit `build` (nur bei geänderten Quellen, nach `bin/k3c-sim`), `run`, `default`.

## Integration

- Abhängigkeiten: `k3c/engine/simcli`.
- Konsumenten: `task k3c-sim:*`, CI.
