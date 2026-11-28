# rust/tools/crates/k3c-studio/

## Responsibility

Backend des Studios (Tauri 2): MCP-Server `k3c-studio` und Oberfläche für Welten, Gebäude-Planer, Assets, Balancing und Testläufe.

## Design

- `app` (Zustand, Fenster, `headless`, Aufruf-Tabelle `app/rpc.rs`), `mcpsrv` (Katalog nur in `tools.rs`, Anleitung `instructions.md`).
- Assets: `assetsui`, `paperassets`, `review`, `studioview`.
- Welten und Läufe: `world`, `hubplan`, `gameexe` (startet `k3c-game sim …`), `simrun`, `balance` (Schreibschutz in `writeguard.rs`), `datadoc`, `gamedata`.
- `ports.rs`: MCP 5181, UI-API 8193, Vite 5193.

## Flow

`main` → `run`: Repo-Wurzel suchen (`rust/Cargo.toml` plus `data/`), dann Fenster oder `--headless`. Level, Welt-Prüfung und Läufe rechnet die Spiel-EXE.

## Integration

- Frontend `tools/k3c-studio/frontend`; Render-CLI `tools/paper`.
- Tasks `k3c-studio:dev|window|build|headless|test|lint`; EXE `bin/k3c-studio.exe`, Dienst ohne Fenster `bin/k3c-studio-headless.exe`.
