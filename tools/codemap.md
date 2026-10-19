# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels (Build-Time-CLIs, Desktop-Tools, Claude-Code-Plugin). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`atlas/`](atlas/codemap.md) | Build-Time-CLI: packt Sprite-Frames aus `data/sprites.json` zu Atlas-PNGs und Phaser-`atlas.json` |
| [`k3c-dev/`](k3c-dev/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing |
| [`k3c-studio/`](k3c-studio/codemap.md) | Frontend des Asset-Studios (Seite des Go-Servers unter `/studio/`, Backend `engine/studio`): Grafik, Sound, Monarch, Gegner, Gebäude-Planer, Welten Editor (B-459) |
| [`k3c-starter/`](k3c-starter/codemap.md) | Windows-Starter (Wails-Fenster): startet `bin/k3c-server`, zeigt Status und Adressen im Heimnetz (B-041) |
| [`assetkit/`](assetkit/codemap.md) | Gemeinsame Asset-Logik (Go-Modul) von k3c-dev und Asset-Studio |
| [`cleanup/`](cleanup/codemap.md) | CLI (`task aufraeumen`): löscht lokale Branches und Worktrees nur in sicheren Fällen |
| [`cockpit/`](cockpit/codemap.md) | Claude-Code-Plugin `tenvan-cc` mit Band, Panels und Chat-Skins |
| [`buildings/`](buildings/codemap.md) | CLI (`task gebaeude`): Gebäude-JSON → PNGs (Dichten 16/32/64/128) |
| [`ruletable/`](ruletable/codemap.md) | CLI (`rules:update`): Block Hub-Stufe → Gebäude in der Regeldoku |
| [`webkit/`](webkit/codemap.md) | Gemeinsame Oberflächen-Bausteine von k3c-dev und Asset-Studio (`@webkit/…`: Fehlertext, Prefs, Debounce, Radix-Bausteine, Theme) |

## Flow
1. `task atlas` (Dependency von `dev` und `build`) baut und startet `tools/atlas`.
2. `task k3c-dev` startet das Dev-Tool (`wails dev`), `task k3c-dev:build` baut die EXE; `task k3c-starter:build` baut `bin/k3c-starter`.
3. `task studio:frontend` (von `task build` aufgerufen) baut das Asset-Studio-Frontend nach `dist/studio/`; der Go-Server liefert es im Dev-Mode aus.

## Integration
- Eigene Go-Module für k3c-dev, k3c-starter und `assetkit` (k3c-dev und das Root-Modul binden `assetkit` per `replace` ein, kein `go.work`); `tools/atlas`, `tools/buildings`, `tools/ruletable` und `tools/cleanup` gehören zum Root-Modul.
