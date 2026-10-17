# tools/

## Responsibility
Entwickler-Werkzeuge außerhalb des Spiels (Build-Time-Tools und Desktop-Dev-Tool). Keine eigenen Quelldateien.

## Design
| Ordner | Rolle |
|--------|-------|
| [`atlas/`](atlas/codemap.md) | Build-Time-CLI: packt Sprite-Frames aus `data/sprites.json` zu Atlas-PNGs und Phaser-`atlas.json` |
| [`k3c-dev/`](k3c-dev/codemap.md) | Wails-Desktop-Tool mit MCP-Server: Prüfläufe, Dienste, Logs, Planung, Balancing |
| [`k3c-studio/`](k3c-studio/codemap.md) | Asset-Studio: Wails-Fenster mit Seiten Grafik und Sound (B-459) |
| [`assetkit/`](assetkit/codemap.md) | Gemeinsame Asset-Logik (Go-Modul) von k3c-dev und Asset-Studio |

## Flow
1. `task atlas` (Dependency von `dev` und `build`) baut und startet `tools/atlas`.
2. `task k3c-dev` startet das Dev-Tool (`wails dev`), `task k3c-dev:build` baut die EXE; `task k3c-studio` und `task k3c-studio:build` (→ `bin/k3c-studio.exe`) ebenso für das Asset-Studio.

## Integration
- Eigene Go-Module für k3c-dev, Asset-Studio und `assetkit` (beide Werkzeuge binden `assetkit` per `replace` ein, kein `go.work`); `tools/atlas` gehört zum Root-Modul.
