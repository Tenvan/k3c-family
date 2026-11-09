# src/

## Responsibility
Browser-Client (TypeScript + Phaser 4) von K3C. Reiner Thin Client: sendet Eingaben an den Go-Server und zeichnet dessen Snapshots; es gibt keine Simulation im Browser.

## Design
- `main.ts` ist der Composition Root für `game.html`: `installPageChrome()` und `installClientLog()`, dann Server-Build per `fetchServerBuild()` prüfen, `createRoomClient()` anlegen und das `Phaser.Game` mit den Szenen `load`, `lobby`, `game`, `hud` (plus Options) aufbauen.
- Schichten in Unterordnern:
  - `model/` – Domain Model (Typen, `data/*.json`-Zugriff, alle Biome per Glob), ohne Logik
  - `online/` – Protocol Adapter zum Server (`/ws`, Protokoll v7)
  - `input/` – Adapter für Tastatur (zwei Layouts), Gamepad, Touch und Bot-Feed hinter `PlayerInput`
  - `scenes/` – Presentation Layer (Phaser-Szenen, Rendering)
  - `audio/` – Web-Audio aus `GameEvent`s (Effekte, Musik-Zustandsautomat mit Crossfade)
  - `core/` – Infrastructure (Shell der Werkzeug-Seiten, Panel-Host, Vollbild-Halter, `runConfig`, Frisch-Laden, Logging, Persistenz inkl. Spielstand-Liste, i18n-Texte, Version)
  - `tools/` – Test- und Info-Seiten

## Flow
1. `index.html` (und sein Alias `game.html`) lädt `main.ts` → `installPageChrome({ home: false })` (Zurück-Falle) und den Vollbild-Halter.
2. `fetchServerBuild()`; ohne Server zeigt `showNoServer()` (`core/serverCheck.ts`) einen Hinweis, eine neuere Server-Version lädt den Client frisch (`core/fresh.ts`).
3. `start(server)` → `createRoomClient()` → `new Phaser.Game(...)` → `LoadScene` lädt den Atlas und startet `LobbyScene`.
4. Die Lobby wählt den Raum; `GameScene` pollt `PlayerInput`, schickt Eingaben über `online/`, zeichnet Snapshots, `HudScene` legt das HUD darüber.

## Integration
- Einstiegsseiten im Repo-Root: `index.html` und `game.html` (beide `main.ts`, das Spiel mit seinem Menü), weitere `*.html` (`tools/`, öffnen als Panel im Menü). Vite baut jede Root-`*.html` (`vite.config.ts`).
- Server: Go-Spielserver (`engine/net`) über `/api` und `/ws`; im Dev leitet der Vite-Proxy beide an Port 8080 weiter.
- Daten: `data/*.json` (Balancing) per Import.
