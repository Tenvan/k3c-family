# src/

## Responsibility
Browser-Client (TypeScript + Phaser 4) von K3C. Reiner Thin Client: sendet Eingaben an den Go-Server und zeichnet dessen Snapshots; es gibt keine Simulation im Browser.

## Design
- `main.ts` ist der Composition Root für `game.html`: `installPageChrome()` und `installClientLog()`, dann Server-Build per `fetchServerBuild()` prüfen, `createRoomClient()` anlegen und das `Phaser.Game` mit den Szenen `load`, `lobby`, `game`, `hud` (plus Options) aufbauen.
- Schichten in Unterordnern:
  - `model/` – Domain Model (Typen, `data/*.json`-Zugriff, alle Biome per Glob), ohne Logik
  - `online/` – Protocol Adapter zum Server (`/ws`, Protokoll v6)
  - `input/` – Adapter für Tastatur (zwei Layouts), Gamepad, Touch und Bot-Feed hinter `PlayerInput`
  - `scenes/` – Presentation Layer (Phaser-Szenen, Rendering)
  - `audio/` – Web-Audio aus `GameEvent`s (Effekte, Musik-Zustandsautomat mit Crossfade)
  - `core/` – Infrastructure (Shell, Vollbild, Logging, Persistenz inkl. Spielstand-Liste, i18n-Texte, Version)
  - `landing/` – Shell/Landingpage (`index.html`)
  - `tools/` – Test- und Info-Seiten

## Flow
1. `game.html` lädt `main.ts` → `installPageChrome()` (Home-Button, Zurück-Falle).
2. `fetchServerBuild()`; ohne Server zeigt `showNoServer()` (`landing/serverCheck.ts`) einen Hinweis.
3. `start(server)` → `createRoomClient()` → `new Phaser.Game(...)` → `LoadScene` lädt den Atlas und startet `LobbyScene`.
4. Die Lobby wählt den Raum; `GameScene` pollt `PlayerInput`, schickt Eingaben über `online/`, zeichnet Snapshots, `HudScene` legt das HUD darüber.

## Integration
- Einstiegsseiten im Repo-Root: `game.html` (`main.ts`), `index.html` (`landing/`), weitere `*.html` (`tools/`). Vite baut jede Root-`*.html` (`vite.config.ts`).
- Server: Go-Spielserver (`engine/net`) über `/api` und `/ws`; im Dev leitet der Vite-Proxy beide an Port 8080 weiter.
- Daten: `data/*.json` (Balancing) per Import.
