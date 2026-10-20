# tools/watch/

## Responsibility

CLI `k3c-watch`: Watcher für EXE-Projekte ohne eigenen Hot-Reload (`task k3c-server:dev`, `task k3c-starter:dev`). Beobachtet Quellen, baut bei Änderung neu und tauscht die laufende EXE am festen Pfad (Firewall-Freigabe bleibt). Die Wails-Fenster `k3c-dev` und `k3c-studio` nutzen `wails dev`.

## Design

- `main.go`: Flags (`--name`, `--paths`, `--ext`, `--build`, `--poll`, `--quiet`, EXE und Argumente hinter `--`), `parseArgs`, Signalbehandlung.
- `watch.go`: `snapshot`/`changed` (Polling über Änderungszeit und Größe, ohne Testdateien, versteckte Ordner und `node_modules`), `supervisor` (`run`, `settle`, `rebuild`), Austausch der EXE durch Umbenennen (`moveAside`, `restoreAside`, `discard`), Kindprozess (`childProc`).
- Test-Nähte: `supervisor.build` und `supervisor.start` (Interface `proc`), damit die Tests ohne echte Prozesse laufen.

## Flow

1. Erster Build, dann Start der EXE.
2. Änderung → Ruhezeit (`--quiet`) → die laufende EXE wird in `<exe>.old` umbenannt (Windows erlaubt das, überschreiben nicht) → Build schreibt die neue EXE am festen Pfad.
3. Erfolg: altes Programm um ein sauberes Ende bitten (Strg+Pause/SIGINT, der Server speichert die Räume; `proc_windows.go`, `proc_other.go`), nach `--grace` (Standard 10 s) hart beenden, `.old` löschen, neues starten. Fehler: alte EXE zurückbenennen, sie läuft weiter.
4. Beendet sich das Programm von selbst, wartet der Watcher auf die nächste Änderung (keine Neustart-Schleife).
5. Der Stand der Dateien wird vor dem Build festgehalten: Ein Build, der beobachtete Dateien selbst umschreibt (`wails build` → `go.mod`, `wailsjs`), würde sich endlos auslösen. Solche Endungen gehören nicht in `--ext` (`k3c-starter:dev` beobachtet nur `.go` und `.html`).

## Integration

- Taskfile `tools/watch/Taskfile.yml` (`watch:build` baut `bin/k3c-watch`); die Tasks `<projekt>:dev` rufen die EXE auf.
- Log über `engine/conlog` (`ns` `watch`, Emoji 👀).
