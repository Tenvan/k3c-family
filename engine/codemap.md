# engine/

## Responsibility
Rest der Go-Engine (Entscheidung 001) nach RU41.1: nur noch Dev-Infrastruktur der Wails-Apps und die Repo-Regel-Tests.
Simulation, Level, Raum, Spielstände, Balancing und Zufall sind Rust (Entscheidung 011, `rust/crates/`, Übersicht
[`rust/codemap.md`](../rust/codemap.md) und [`rust/crates/codemap.md`](../rust/crates/codemap.md)); der letzte Go-Stand steht im
Tag `go-referenz-final`. Eigene Datei nur `Taskfile.yml`.

## Design

| Ordner | Rolle |
|--------|-------|
| [`conlog/`](conlog/codemap.md) | farbiger `slog.Handler` mit Emoji-Topics (`Topics`); ein neuer `ns` bekommt dort sein Emoji |
| [`devrun/`](devrun/codemap.md) | Dev-Supervisor der Tasks `<app>:dev`: baut die EXE als Kind (`-headless`), startet sie und baut sie bei Go-Änderungen neu |
| `internal/rules/` | Repo-Regeln für Go als Tests: `imports_test.go` (`engine/` importiert nichts aus `tools/`), `nesting_test.go` (Verschachtelungstiefe) |

`Taskfile.yml` ist der Namespace `go`: `test`, `lint`, `race` (nur `engine/`, braucht einen C-Compiler), `golden-setup`
(Git-Diff der gepackten Golden-Dateien). `golden` und `cover` entfallen; `testdata/golden/` ist eingefroren
(`docs/arbeitsweise.md` › Golden).

## Flow
1. Workbench und Studio rufen `conlog` für die Konsolenzeilen ihres Backends auf.
2. `task <app>:dev` startet eine EXE mit `-dev`; `devrun` überwacht die Quellen (`snapshot`), wartet auf Ruhe (`settle`) und
   baut das Kind neu (`rebuild`).
3. `internal/rules` läuft als normale Go-Tests (`task go:test`, `check:diff`).

## Integration
- Konsumenten: k3c-workbench und k3c-studio (`internal/applog`, `headless.go`: `conlog`, `devrun`).
- Simulation, Level und Läufe: nicht hier; Workbench und Studio starten die EXE `k3c-game sim <run|level|balance|world|hub|save>`
  (`docs/standards/k3c-game-laeufe.md`).
- `internal/` ([codemap](internal/codemap.md)) enthält laut Bestand außer `rules` noch ältere Einträge; maßgeblich ist der Ordner.
