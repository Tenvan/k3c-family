# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels: das Dev-Werkzeug k3c-workbench (Wails + MCP-Server), Build-Time-CLIs und Generatoren, der Papierschnitt-Renderer und gemeinsame Oberflächen-Bausteine. Direkt im Ordner liegt nur `Taskfile.yml`, der Namensraum `gen` der Generatoren.

## Design
| Ordner | Rolle |
|--------|-------|
| [`k3c-workbench/`](k3c-workbench/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing, Bereich Assets |
| [`assetkit/`](assetkit/codemap.md) | Eigenes Go-Modul: Asset-Logik (Slot-Register, Katalog, Recherche, Vorschläge, Vorrat) sowie die Unterpakete `ticket`, `windowstate`, `browser` |
| [`paper/`](paper/codemap.md) | Papierschnitt-Renderer und Verträge (Entscheidung 005), TypeScript unter Bun: Render-CLI `bin/paper.ts`, Generator `bin/gen.ts` |
| [`webkit/`](webkit/codemap.md) | Gemeinsame Oberflächen-Bausteine der Wails-Frontends (`@webkit/…`, Atomic Design auf Radix Themes) |
| [`checkdiff/`](checkdiff/codemap.md) | CLI `task check:diff`: prüft nur, was die geänderten Dateien eines Commits brauchen; Paketlisten ohne Sim-Pakete |
| [`cleanup/`](cleanup/codemap.md) | CLI `task ops:aufraeumen`: löscht lokale Branches und Worktrees nur in sicheren Fällen |
| [`ruletable/`](ruletable/codemap.md) | CLI `task gen:rules`: Block Hub-Stufe → Gebäude in der Regeldoku |
| [`soundatlas/`](soundatlas/codemap.md) | CLI `task gen:soundatlas`: Sound-Atlas aus den Effekt-Slots von `data/sounds.json` (ffmpeg) |
| `hooks/` | `planGuard.ts`: Claude-Code-Hook, der Änderungen an Planungsdateien außerhalb der MCP-Tools ablehnt (ohne codemap) |

- `Taskfile.yml` (eingebunden als `gen` in der Wurzel): `paper` (= `paper-render` + `paper-sync`, rendert per `bun tools/paper/bin/gen.ts` nach `.work/grafik` und kopiert nach `public/grafik`), `rules` (`bin/regeltabelle`), `soundatlas` (`bin/soundatlas`, braucht ffmpeg). Feste EXEs unter `bin/`, `sources`/`generates`/`status` verhindern unnötige Läufe.

## Flow
1. `task k3c-workbench` startet das Dev-Tool als Client/Server (Backend `-dev` über `engine/devrun`, Oberfläche unter Vite; Fenster: `k3c-workbench:window`), `task k3c-workbench:build` baut die EXE.
2. `task gen:paper` (auch Abhängigkeit von `task test`, `task build`, `native:game:build`) → `tools/paper/bin/gen.ts` → `public/grafik/<gruppe>/`.
3. `task check:diff` → `go run ./tools/checkdiff`; `task check:go` und `k3c-workbench:test` holen ihre Paketlisten über `checkdiff -gopkgs` / `-wbpkgs`.

## Integration
- Go-Module: `k3c-workbench` und `assetkit` haben eigene `go.mod` (die Workbench bindet `assetkit` und das Wurzel-Modul per `replace` ein, kein `go.work`); `checkdiff`, `cleanup`, `ruletable`, `soundatlas` gehören zum Wurzel-Modul.
- TypeScript: `paper/` läuft im Vitest der Wurzel und unter Bun; `webkit/` wird im Frontend der Workbench typgeprüft und getestet.
