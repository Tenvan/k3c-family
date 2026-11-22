# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels: die Dev-Werkzeuge k3c-workbench und k3c-studio (Wails + je ein MCP-Server; gemeinsamer Go-Code in `workkit`), Build-Time-CLIs und Generatoren (auch fürs Rust-Spielpaket und den Go-Rust-Testabgleich), der Papierschnitt-Renderer und gemeinsame Oberflächen-Bausteine. Direkt im Ordner liegt nur `Taskfile.yml`, der Namensraum `gen` der Generatoren.

## Design
| Ordner | Rolle |
|--------|-------|
| [`k3c-workbench/`](k3c-workbench/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing, Bereich Assets |
| [`assetkit/`](assetkit/codemap.md) | Eigenes Go-Modul: Asset-Logik (Slot-Register, Katalog, Recherche, Vorschläge, Vorrat) sowie die Unterpakete `ticket`, `windowstate`, `browser` |
| [`paper/`](paper/codemap.md) | Papierschnitt-Renderer und Verträge (Entscheidung 005), TypeScript unter Bun: Render-CLI `bin/paper.ts`, Generator `bin/gen.ts` |
| [`webkit/`](webkit/codemap.md) | Gemeinsame Oberflächen-Bausteine der Wails-Frontends (`@webkit/…`, Atomic Design auf Radix Themes) |
| [`checkdiff/`](checkdiff/codemap.md) | CLI `task check:diff`: prüft nur, was die geänderten Dateien eines Commits brauchen (Bahnen `js`, `go`, `rust`); Paketlisten ohne Sim-Pakete |
| [`cleanup/`](cleanup/codemap.md) | CLI `task ops:aufraeumen`: löscht lokale Branches und Worktrees nur in sicheren Fällen sowie alte EXE entfernter Programme in `bin/` |
| [`portcheck/`](portcheck/codemap.md) | CLI `task gen:portcheck`: gleicht Go-Tests des Tags `go-referenz-final` mit Rust-Funktionsnamen ab, Bericht `docs/messung/port-testabgleich.md` |
| [`ruletable/`](ruletable/codemap.md) | CLI `task gen:rules`: Block Hub-Stufe → Gebäude in der Regeldoku |
| [`rustpaket/`](rustpaket/codemap.md) | CLI für den Paketbau des Rust-Spiels: `lizenzen` (LIZENZEN.txt aus `cargo metadata`) und `pruefen` (Paketordner gegen feste Liste) |
| [`soundatlas/`](soundatlas/codemap.md) | CLI `task gen:soundatlas`: Sound-Atlas aus den Effekt-Slots von `data/sounds.json` (ffmpeg) |
| [`sync/`](sync/codemap.md) | CLI `sync quelle=ziel`: kopiert nur Geändertes ins Spielpaket (Ordner zusammengeführt) |
| `measure/`, `rigexport/` | Messskript des Rust-Spiels (`measure.ps1`) und Rig-Export (`task gen:rig`); ohne eigene codemap |
| `hooks/` | `planGuard.ts`: Claude-Code-Hook, der Änderungen an Planungsdateien außerhalb der MCP-Tools ablehnt (ohne codemap) |

- `Taskfile.yml` (eingebunden als `gen` in der Wurzel): `paper` (= `paper-render` + `paper-sync`, rendert per `bun tools/paper/bin/gen.ts` nach `.work/grafik` und kopiert nach `public/grafik`), `rules` (`bin/regeltabelle`), `soundatlas` (`bin/soundatlas`, braucht ffmpeg), `rig` und `rig:check` (Rig-Teile nach `public/rig/` schreiben bzw. byte-genau vergleichen), `portcheck` (`go run ./tools/portcheck`). Feste EXEs unter `bin/`, `sources`/`generates`/`status` verhindern unnötige Läufe.

## Flow
1. `task k3c-workbench` startet das Dev-Tool als Client/Server (Backend `-dev` über `engine/devrun`, Oberfläche unter Vite; Fenster: `k3c-workbench:window`), `task k3c-workbench:build` baut die EXE. `task k3c-studio` (dann `k3c-studio:window`, `k3c-studio:build`) startet analog das Studio (Welten, Hub, Assets).
2. `task gen:paper` (auch Abhängigkeit von `task test`, `task build`, `rust:game:build`) → `tools/paper/bin/gen.ts` → `public/grafik/<gruppe>/`.
3. `task rust:game:build` → `tools/sync` (Dateien ins Paket) → `rustpaket lizenzen` und `rustpaket pruefen`.
4. `task check:diff` → `go run ./tools/checkdiff`; `task check:go` und `k3c-workbench:test` holen ihre Paketlisten über `checkdiff -gopkgs` / `-wbpkgs`.

## Integration
- Go-Module: `k3c-workbench` und `assetkit` haben eigene `go.mod` (die Workbench bindet `assetkit` und das Wurzel-Modul per `replace` ein, kein `go.work`); `checkdiff`, `cleanup`, `portcheck`, `ruletable`, `rustpaket`, `soundatlas`, `sync` gehören zum Wurzel-Modul.
- TypeScript: `paper/` läuft im Vitest der Wurzel und unter Bun; `webkit/` wird im Frontend der Workbench typgeprüft und getestet.
