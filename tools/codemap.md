# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels: die Oberflächen der Apps k3c-workbench und k3c-studio (Tauri 2, je ein MCP-Server; Backends in `rust/tools/`), TypeScript-CLIs und Generatoren (auch fürs Rust-Spielpaket), der Papierschnitt-Renderer und gemeinsame Oberflächen-Bausteine. Direkt im Ordner liegt nur `Taskfile.yml`, der Namensraum `gen` der Generatoren.

## Design
| Ordner | Rolle |
|--------|-------|
| [`k3c-workbench/`](k3c-workbench/codemap.md) | Frontend, Taskfile, Dienstliste und Icon der Workbench; Backend `rust/tools/crates/k3c-workbench` |
| [`k3c-studio/`](k3c-studio/codemap.md) | Frontend, Taskfile und Icon des Studios; Backend `rust/tools/crates/k3c-studio` |
| [`paper/`](paper/codemap.md) | Papierschnitt-Renderer und Verträge (Entscheidung 005), TypeScript unter Bun: Render-CLI `bin/paper.ts`, Generator `bin/gen.ts` |
| [`webkit/`](webkit/codemap.md) | Gemeinsame Oberflächen-Bausteine der Tauri-Frontends (`@webkit/…`, Atomic Design auf Radix Themes) |
| [`checkdiff/`](checkdiff/codemap.md) | CLI `task check:diff`: prüft nur, was die geänderten Dateien eines Commits brauchen (Bahnen `js`, `rust`) |
| [`cleanup/`](cleanup/codemap.md) | CLI `task ops:aufraeumen`: löscht lokale Branches und Worktrees nur in sicheren Fällen sowie alte EXE entfernter Programme in `bin/` |
| [`ruletable/`](ruletable/codemap.md) | CLI `task gen:rules`: Block Hub-Stufe → Gebäude in der Regeldoku |
| [`rustpaket/`](rustpaket/codemap.md) | CLI für den Paketbau des Rust-Spiels: `lizenzen` (LIZENZEN.txt aus `cargo metadata`) und `pruefen` (Paketordner gegen feste Liste) |
| [`soundatlas/`](soundatlas/codemap.md) | CLI `task gen:soundatlas`: Sound-Atlas aus den Effekt-Slots von `data/sounds.json` (ffmpeg) |
| [`sync/`](sync/codemap.md) | CLI `sync quelle=ziel`: kopiert nur Geändertes ins Spielpaket (Ordner zusammengeführt) |
| `measure/`, `rigexport/` | Messskript des Rust-Spiels (`measure.ps1`) und Rig-Export (`task gen:rig`); ohne eigene codemap |
| `hooks/` | `planGuard.ts`: Claude-Code-Hook, der Änderungen an Planungsdateien außerhalb der MCP-Tools ablehnt; `tempGuard.ts` prüft nur noch Go-Befehle, die es nicht mehr gibt (Entfernen liegt bei Ralf); ohne codemap |

- `Taskfile.yml` (eingebunden als `gen` in der Wurzel): `paper` (= `paper-render` + `paper-sync`, rendert per `bun tools/paper/bin/gen.ts` nach `.work/grafik` und kopiert nach `public/grafik`), `rules` (`bun tools/ruletable/main.ts`), `soundatlas` (`bun tools/soundatlas/main.ts`, braucht ffmpeg), `rig` und `rig:check` (Rig-Teile nach `public/rig/` schreiben bzw. byte-genau vergleichen). `sources`/`generates`/`status` verhindern unnötige Läufe.

## Flow
1. `task k3c-workbench` startet das Dev-Tool als Client/Server (Backend `--headless`, Oberfläche unter Vite; Fenster: `k3c-workbench:window`), `task k3c-workbench:build` baut die EXE. `task k3c-studio` (dann `k3c-studio:window`, `k3c-studio:build`) startet analog das Studio (Welten, Hub, Assets, Balancing).
2. `task gen:paper` (auch Abhängigkeit von `task test`, `task build`, `rust:game:build`) → `tools/paper/bin/gen.ts` → `public/grafik/<gruppe>/`.
3. `task rust:game:build` → `tools/sync` (Dateien ins Paket) → `rustpaket lizenzen` und `rustpaket pruefen`.
4. `task check:diff` → `bun tools/checkdiff/main.ts`.

## Integration
- CLIs laufen unter bun und nur mit Node-APIs; ihre Tests laufen im Vitest der Wurzel.
- TypeScript: `paper/` läuft im Vitest der Wurzel und unter Bun; `webkit/` wird in den Frontends typgeprüft und getestet.
