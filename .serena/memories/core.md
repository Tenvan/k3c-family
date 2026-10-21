# K3C – Family Three Crowns (Einstieg)

Couch-Koop-Side-Scroller (Kingdom Two Crowns-Stil). Go-Server (Engine, deterministische Simulation) + TS/Phaser-Client im Browser; Ziel Edge auf Xbox. Kommunikation mit Nutzer: Deutsch.

## Source Map
- `engine/` Go: `sim/` (Simulation), `level/`, `rng/`, `room/`, `net/` (HTTP, WS `/ws`, `docs/protocol.md`), `store/`, `watch/`, `conlog/`. Modul `k3c` (go.mod im Root).
- `cmd/` Binaries: `k3c-server`, `k3c-load`, `k3c-tui`.
- `src/` Client: `model/` (nur Typen), `online/` (Server-Client), `input/` (`PlayerInput`), `scenes/` (Phaser: GameScene, HudScene, `render/`, `hud/`), `core/` (`shell.ts`, `fullscreen.ts`), `landing/pages.ts`, `tools/devTiles.ts`.
- `data/` Balancing-JSON, einzige Quelle für Client und Server (`data/embed.go`). Werte nie in Code.
- `tools/` eigene Go-Module/Wails-Apps: `k3c-dev` (Workbench + MCP), `k3c-studio`, `k3c-starter`, `assetkit`, `webkit` (Atomic Design, Radix), `ruletable`, `atlas`, `buildings`, `cleanup`, `watch`.
- `docs/`: `arbeitsweise.md` + `glossar.md` vor jeder Session lesen; `roadmap.md`, `game-design.md`, `decisions/`, `backlog/` (Tickets), `sprints/{aktiv,geplant,erledigt}`, `projekte/`, `vorlagen/`.
- Root-`*.html` = Seiten (index = Landing/Shell, game, dev …), jede wird automatisch gebaut.
- `codemap.md` (Root und je Ordner) = Architektur-Atlas.

## Invarianten
- Neue Mechanik: Logik + Test in `engine/sim/` (Go), Client zeichnet nur. Keine TS-Simulation (Entscheidung 001).
- Determinismus: nur `engine/rng`, nie `math/rand`; im Client kein `Math.random()` für Logik.
- Branches: Arbeit auf `develop`, ein PR je Sprint (`sprint/<präfix>`), ein Commit je Session; `main` geschützt.
- Planung (Tickets/Sprints/Sessions) nur über k3c-dev MCP `plan_*`; Vorlagen aus `docs/vorlagen/` Pflicht. `Spec: freigegeben` und Abnahmen nur durch Nutzer.
- Code-Review (getrennt vom Schreiben) vor jedem Commit.
- Kein automatischer Worktree-Wechsel; nur auf ausdrückliche Anweisung.
- Keine Migration alter Spielstände.

Weiter: Stack `mem:tech_stack`, Befehle `mem:suggested_commands`, Stilregeln und Limits `mem:conventions`, Abschluss-Checks `mem:task_completion`.
