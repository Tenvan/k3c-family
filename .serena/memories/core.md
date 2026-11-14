# K3C – Family Three Crowns (Einstieg)

Couch-Koop-Side-Scroller (Kingdom Two Crowns-Stil). Natives Spiel in Go + Ebitengine (`native/k3c-game`, Entscheidung 008) am PC, deterministische Simulation in `engine/` (Go). Browser-Client, Spielserver und Online-Spiel sind entfallen (B-850, WR1/WR2). Kommunikation mit Nutzer: Deutsch.

## Source Map
- `engine/` Go: `sim/` (Simulation), `level/`, `rng/`, `room/` (Raum mit Stufen und Spielstand), `store/` (Spielstände, Berichte), `conlog/`, `devrun/`. Modul `k3c` (go.mod im Root).
- `cmd/` Binary: `k3c-sim` (Engine-CLI der Workbench: Simulation, Balancing, Level, Welten).
- `native/` eigenes Modul: `k3c-game` (Spiel: `game/`, `local/` = Raum über `engine/room`, `view/`, `hud/`, `menu/`, `input/` …) und Rig-Prototyp `k3c-rig`.
- `data/` Balancing-JSON, einzige Quelle für Engine und Spiel (`data/embed.go`). Werte nie in Code.
- `tools/` eigene Go-Module/Wails-Apps: `k3c-workbench` (Workbench + MCP), `assetkit`, `webkit` (Atomic Design, Radix), `paper` (Papierschnitt-Renderer), `ruletable`, `cleanup`, `checkdiff`.
- `docs/`: Session über Skill `k3c-session` + `plan_next`, `arbeitsweise.md` bei Regelfragen, `glossar.md` beim Schreiben von Specs; `roadmap.md`, `game-design.md`, `decisions/`, `backlog/` (Tickets), `sprints/{aktiv,geplant,erledigt}`, `projekte/`, `vorlagen/`.
- `site/` statische Präsentationsseite (GitHub Pages).
- `codemap.md` (Root und je Ordner) = Architektur-Atlas.

## Invarianten
- Neue Mechanik: Logik + Test in `engine/sim/` (Go), das Spiel zeichnet nur.
- Determinismus: nur `engine/rng`, nie `math/rand`.
- Branches: nur `main` (geschützt); Feature-Branches `sprint/<präfix>`, ein PR je Sprint, ein Commit je Session.
- Planung (Tickets/Sprints/Sessions) nur über k3c-workbench MCP `plan_*`; Vorlagen aus `docs/vorlagen/` Pflicht. `Spec: freigegeben` und Abnahmen nur durch Nutzer (Ausnahmen: `arbeitsweise.md` › SDD).
- Code-Review (getrennt vom Schreiben) vor dem Commit nur bei Risiko (`arbeitsweise.md` › Tempo im Lauf).
- Kein automatischer Worktree-Wechsel; nur auf ausdrückliche Anweisung.
- Keine Migration alter Spielstände.

Weiter: Stack `mem:tech_stack`, Befehle `mem:suggested_commands`, Stilregeln und Limits `mem:conventions`, Abschluss-Checks `mem:task_completion`.
