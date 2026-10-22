# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels (Build-Time-CLIs, Desktop-Tools, Claude-Code-Plugin). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`atlas/`](atlas/codemap.md) | Build-Time-CLI: packt Sprite-Frames aus `data/sprites.json` zu Atlas-PNGs und Phaser-`atlas.json` |
| [`k3c-workbench/`](k3c-workbench/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing |
| [`k3c-studio/`](k3c-studio/codemap.md) | Asset-Studio als eigenes Wails-Fenster (Backend `k3c-studio/backend`, Frontend, eingebundene Engine; keine Seite des Spielservers, Entscheidung 004): Grafik, Sound, Monarch, Gegner, Gebäude-Planer, Welten Editor (B-459, B-507) |
| [`k3c-starter/`](k3c-starter/codemap.md) | Windows-Starter (Wails-Fenster): startet `bin/k3c-server`, zeigt Status und Adressen im Heimnetz (B-041) |
| [`assetkit/`](assetkit/codemap.md) | Gemeinsame Asset-Logik (Go-Modul) von k3c-workbench und Asset-Studio |
| [`cleanup/`](cleanup/codemap.md) | CLI (`task ops:aufraeumen`): löscht lokale Branches und Worktrees nur in sicheren Fällen |
| [`cockpit/`](cockpit/codemap.md) | Claude-Code-Plugin `tenvan-cc` mit Band, Panels und Chat-Skins |
| [`buildings/`](buildings/codemap.md) | CLI (`task gen:gebaeude`): Gebäude-JSON → PNGs (Dichten 16/32/64/128) |
| [`ruletable/`](ruletable/codemap.md) | CLI (`task gen:rules`): Block Hub-Stufe → Gebäude in der Regeldoku |
| [`webkit/`](webkit/codemap.md) | Gemeinsame Oberflächen-Bausteine von k3c-workbench und Asset-Studio (`@webkit/…`: Fehlertext, Prefs, Debounce, Radix-Bausteine, Theme) |

## Flow
1. `task gen:atlas` (Dependency von `dev` und `build`) baut und startet `tools/atlas`.
2. `task k3c-workbench` startet das Dev-Tool als Client/Server (Backend `-dev` über `engine/devrun`, Oberfläche unter Vite; Fenster: `k3c-workbench:window`), `task k3c-workbench:build` baut die EXE; `task k3c-starter:build` baut `bin/k3c-starter`, `task k3c-starter:dev` startet ihn als Client/Server (Backend `-dev`, Vite 5194).
3. `task k3c-studio:frontend` baut das Asset-Studio-Frontend nach `tools/k3c-studio/frontend/dist/`; nur das Fenster `k3c-studio` liefert es aus (`task build` baut kein Studio, B-507).

## Integration
- Eigene Go-Module für k3c-workbench, k3c-starter und `assetkit` (k3c-workbench und das Root-Modul binden `assetkit` per `replace` ein, kein `go.work`); `tools/atlas`, `tools/buildings`, `tools/ruletable` und `tools/cleanup` gehören zum Root-Modul.
