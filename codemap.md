# Repository Atlas: K3C – Family Three Crowns

## Project Responsibility
Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns für Edge auf der Xbox (Gamepad API), gehostet im Heimnetz. Ein Go-Server (`engine/`, `cmd/k3c-server`) rechnet die deterministische Simulation und verteilt Snapshots über WebSocket (Protokoll v4). Der Browser-Client (`src/`, TypeScript + Phaser 4 + Vite) sendet nur Eingaben und zeichnet. Balancing liegt als JSON in `data/` und ist die einzige Quelle für beide Seiten. `tools/k3c-dev` ist das Entwickler-Werkzeug (Wails-Desktop-App + MCP-Server).

## System Entry Points
- `game.html` → `src/main.ts`: Spiel-Client (Phaser-Szenen `load` → `lobby` → `game` + `hud`)
- `index.html` → `src/landing/landing.ts`: Landingpage/Shell, hält alle anderen Seiten im Vollflächen-iframe
- weitere `*.html` im Root: Test- und Info-Seiten (`src/tools/`)
- `cmd/k3c-server/`: Go-Spielserver (liefert `dist/` aus, `/api`, `/ws`)
- `Taskfile.yml`: einziger Einstieg für Befehle (`task dev`, `task start`, `task check` …)
- `package.json`, `tsconfig.json`, `vite.config.ts`: Client-Build (jede Root-`*.html` wird eine Seite; Dev-Proxy für `/api` und `/ws`; Watcher ignoriert Worktrees und Laufzeit-Ausgaben)
- `go.mod`: Go-Modul für Engine, Server und `tools/atlas`; `tools/k3c-dev/go.mod` ist ein eigenes Modul

## Directory Map
| Directory | Responsibility Summary | Detailed Map |
|-----------|------------------------|--------------|
| `cmd/` | Go-Binaries (CLI entry points) über `engine/` | [View Map](cmd/codemap.md) |
| `cmd/k3c-server/` | Composition Root des Spielservers: Auslieferung, HTTP/WebSocket-API, Spielstände, Berichte, Räume, Logging | [View Map](cmd/k3c-server/codemap.md) |
| `cmd/k3c-load/` | Lasttest-Client: Bots in Testräumen, misst Tick-Dauer über `/api/status`, Bericht mit Verdict | [View Map](cmd/k3c-load/codemap.md) |
| `cmd/k3c-tui/` | Bubble-Tea-Terminal-UI zur Server-Diagnose über `/api/status` | [View Map](cmd/k3c-tui/codemap.md) |
| `data/` | Balancing-JSON, einzige Quelle für Client-Import und Server (`go:embed`) | [View Map](data/codemap.md) |
| `data/biomes/` | Ein JSON je Biom/Stufe (forest bis crystal, ironhold mit Lava) für Level-Generator und Simulation | [View Map](data/biomes/codemap.md) |
| `engine/` | Go-Spiel-Engine: Simulation, Level, Räume, Netzwerk, Persistenz | [View Map](engine/codemap.md) |
| `engine/sim/` | Domain Model und deterministischer Tick-Core (Stufe, Insel, Campaign): Wirtschaft, Bürger/Berufe, Monarch-Skills, Gegner-Traits, Saves, ohne I/O | [View Map](engine/sim/codemap.md) |
| `engine/level/` | Deterministischer Level-Generator mit Validator, Biome-Loader, Mauerlinien, Adern und Lava | [View Map](engine/level/codemap.md) |
| `engine/room/` | Room/Manager: Geräte, Slots, Takt, Fristen, Speichern (`saver`), Skills, Monitor-Messreihen, Crash-Supervision | [View Map](engine/room/codemap.md) |
| `engine/net/` | HTTP/WebSocket-Adapter (Protokoll v4): REST inkl. Saves/Metriken/Dev, Delta-Zustand, Aktionen, RTT-Ping | [View Map](engine/net/codemap.md) |
| `engine/store/` | File-Repository für Spielstände, Berichte und Spielmetrik (atomar, rotierende Sicherungen, `Saves.List`) | [View Map](engine/store/codemap.md) |
| `engine/rng/` | Deterministischer mulberry32-Generator, einzige Zufallsquelle | [View Map](engine/rng/codemap.md) |
| `engine/conlog/` | Farbiger `slog.Handler` mit Emoji-Topics | [View Map](engine/conlog/codemap.md) |
| `src/` | Browser-Client (Thin Client): Composition Root `main.ts` | [View Map](src/codemap.md) |
| `src/scenes/` | Presentation Layer: Phaser-Szenen, Split-Screen, Rendering, Glyph-Hinweise, Skill-Menü, Reittier, Debug-Overlay | [View Map](src/scenes/codemap.md) |
| `src/online/` | WebSocket-Client `/ws` (Protokoll v4): `RoomClient`, Stufen-Streams, Delta, Timeline, Interpolation, Vorhersage, Latenz | [View Map](src/online/codemap.md) |
| `src/model/` | Client-Typen (`World`, `GameEvent` …) und typisierter `data/*.json`-Zugriff | [View Map](src/model/codemap.md) |
| `src/input/` | `PlayerInput`-Adapter für Tastatur, Gamepad und Touch; Tastenbelegung der Slots in `slotBindings.ts` | [View Map](src/input/codemap.md) |
| `src/audio/` | Web-Audio: `AudioCore`, `Mixer`-Busse, Ton aus `GameEvent`s | [View Map](src/audio/codemap.md) |
| `src/core/` | Infrastructure: Shell-Protokoll, Vollbild, Client-Log, Persistenz, i18n, Hinweis-Merker, Version | [View Map](src/core/codemap.md) |
| `src/landing/` | Landingpage/Shell: Kacheln aus `pages.ts`, iframe-Host | [View Map](src/landing/codemap.md) |
| `src/tools/` | Test- und Info-Seiten (Gamepad-Test, Level-Betrachter, Grafiken, Monitor, Hörprobe, Dungeon Master, Lizenzen) | [View Map](src/tools/codemap.md) |
| `tools/` | Entwickler-Werkzeuge außerhalb des Spiels | [View Map](tools/codemap.md) |
| `tools/atlas/` | Build-Time-CLI: packt Sprites zu Atlas-PNGs und Phaser-`atlas.json` | [View Map](tools/atlas/codemap.md) |
| `tools/k3c-dev/` | Wails-Desktop-Host: Composition Root, App-Bindings, MCP-Server, Dienste | [View Map](tools/k3c-dev/codemap.md) |
| `tools/k3c-dev/internal/` | Interne Packages von k3c-dev | [View Map](tools/k3c-dev/internal/codemap.md) |
| `tools/k3c-dev/internal/mcpsrv/` | MCP-Server (Streamable HTTP): Tool-Katalog, Worktree-Scoping, Check-Pipeline | [View Map](tools/k3c-dev/internal/mcpsrv/codemap.md) |
| `tools/k3c-dev/internal/planning/` | Tickets, Sprints, Sessions, Roadmap unter `docs/`, transaktional mit Vorlagenpflicht, Prio-Vererbung | [View Map](tools/k3c-dev/internal/planning/codemap.md) |
| `tools/k3c-dev/internal/services/` | Supervisor für Dev-Dienste aus `services.json` mit Health-Check, Crash-Restart und Reload | [View Map](tools/k3c-dev/internal/services/codemap.md) |
| `tools/k3c-dev/internal/proc/` | Kindprozesse mit Prozessbaum-Kill (Job Object) | [View Map](tools/k3c-dev/internal/proc/codemap.md) |
| `tools/k3c-dev/internal/serverapi/` | Read-only-Client für `/api/status` und Raum-Diagnose | [View Map](tools/k3c-dev/internal/serverapi/codemap.md) |
| `tools/k3c-dev/internal/applog/` | Eigenes Log `logs/k3c-dev.jsonl` per `slog`-Fanout | [View Map](tools/k3c-dev/internal/applog/codemap.md) |
| `tools/k3c-dev/internal/console/` | In-Memory-Konsolenpuffer je Quelle mit Live-Callback | [View Map](tools/k3c-dev/internal/console/codemap.md) |
| `tools/k3c-dev/internal/logs/` | Rückwärts-Reader für `logs/*.jsonl` mit Cursor, Filter, Digest | [View Map](tools/k3c-dev/internal/logs/codemap.md) |
| `tools/k3c-dev/internal/taskcat/` | Task-Katalog aus `task --list-all --json` | [View Map](tools/k3c-dev/internal/taskcat/codemap.md) |
| `tools/k3c-dev/internal/taskrun/` | Task-Runner (ein Lauf je Task) und Argument-Validierung | [View Map](tools/k3c-dev/internal/taskrun/codemap.md) |
| `tools/k3c-dev/internal/enginetools/` | In-process-Facade auf `engine/level` und `engine/sim` (`level_generate`, `sim_run`) | [View Map](tools/k3c-dev/internal/enginetools/codemap.md) |
| `tools/k3c-dev/internal/balance/` | Balancing-Tester: Bot-Szenario-Matrix, Zielkorridor, Baseline, Sensitivität, Kurven, Replays | [View Map](tools/k3c-dev/internal/balance/codemap.md) |
| `tools/k3c-dev/internal/balance/cmd/` | CLI `k3c-balance` für Matrix-Läufe und Replays | [View Map](tools/k3c-dev/internal/balance/cmd/codemap.md) |
| `tools/k3c-dev/internal/gamedata/` | Read-only-Zusammenfassungen von `reports/` und `saves/` | [View Map](tools/k3c-dev/internal/gamedata/codemap.md) |
| `tools/k3c-dev/internal/github/` | `gh`-CLI-Client mit TTL-Cache für PR- und CI-Stand | [View Map](tools/k3c-dev/internal/github/codemap.md) |
| `tools/k3c-dev/internal/usage/` | Statistik der MCP-Aufrufe mit JSON-Persistenz | [View Map](tools/k3c-dev/internal/usage/codemap.md) |
| `tools/k3c-dev/frontend/` | Vite/TS-Toolchain der Wails-UI, ohne Wails mit Mock im Browser lauffähig | [View Map](tools/k3c-dev/frontend/codemap.md) |
| `tools/k3c-dev/frontend/src/` | React-SPA-Shell: App/Header mit vier Reitern, Theme | [View Map](tools/k3c-dev/frontend/src/codemap.md) |
| `tools/k3c-dev/frontend/src/api/` | Backend-Vertrag: Wails-Adapter oder Mock je nach Laufzeit | [View Map](tools/k3c-dev/frontend/src/api/codemap.md) |
| `tools/k3c-dev/frontend/src/services/` | Reiter Dienste: Statuskarten, Steuerung, Metriken, Logs | [View Map](tools/k3c-dev/frontend/src/services/codemap.md) |
| `tools/k3c-dev/frontend/src/logs/` | Logs-Bereich: Quellen, Live-Konsole, Abfrage, verdichtete Fehler | [View Map](tools/k3c-dev/frontend/src/logs/codemap.md) |
| `tools/k3c-dev/frontend/src/tasks/` | Reiter Tasks: Katalog, Start/Stopp, Konsolenausgabe | [View Map](tools/k3c-dev/frontend/src/tasks/codemap.md) |
| `tools/k3c-dev/frontend/src/planning/` | Reiter Planung: Sprints, Sessions, Backlog mit PR-Status, Kopf-Felder editierbar, Chat-Prompts | [View Map](tools/k3c-dev/frontend/src/planning/codemap.md) |
| `tools/k3c-dev/frontend/src/mcp/` | Reiter MCP: Server-Zustand, Live-Verlauf, Aufruf-Log, Statistik, Instructions | [View Map](tools/k3c-dev/frontend/src/mcp/codemap.md) |
| `tools/k3c-dev/frontend/src/ui/` | Gemeinsame UI-Bausteine über Radix Themes | [View Map](tools/k3c-dev/frontend/src/ui/codemap.md) |
| `tools/k3c-dev/frontend/src/lib/` | Utilities und Hooks (Formatierung, Prefs, `useDebounced`, `useNow`) | [View Map](tools/k3c-dev/frontend/src/lib/codemap.md) |
