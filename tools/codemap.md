# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels (Build-Time-Tools und Desktop-Dev-Tool). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`atlas/`](atlas/codemap.md) | Build-Time-CLI: packt Sprite-Frames aus `data/sprites.json` zu Atlas-PNGs und Phaser-`atlas.json` |
| [`k3c-dev/`](k3c-dev/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing |

## Flow
1. `task atlas` (Dependency von `dev` und `build`) baut und startet `tools/atlas`.
2. `task k3c-dev` startet das Dev-Tool (`wails dev`), `task k3c-dev:build` baut die EXE.

## Integration
- Eigenes Go-Modul für k3c-dev (`tools/k3c-dev/go.mod`); `tools/atlas` gehört zum Root-Modul.
