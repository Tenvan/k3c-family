# tools/k3c-workbench/internal/studioview/

## Responsibility

Was vom Asset-Studio lesend blieb (Entscheidung 005 › 5, Sprint WB1): Sound-Slots und Quellen für den Bereich Assets und „Im Spiel ansehen“, das einen voll ausgebauten Testspielstand anlegt und das native Spiel damit startet (B-847). Schreibt nichts nach `data/` oder `public/`.

## Design

- `studioview.go`: `Sounds(root)` (Sound-Slots aus `data/sounds.json` mit `assets.Status`) → `[]SoundEintrag`; `SoundData(root, pfad)` (Audiodatei unter `public/` als Data-URL, über `os.Root`, nur `.ogg/.mp3/.wav/.m4a`, ohne `..`/Backslash/absolute Pfade, höchstens 10 MB); `Sources(root)` → `Quellen` (Katalog `assets.Catalog` und Recherche-Treffer `assets.LoadResearch`). Fehler für die Oberfläche als `hint` in ganzen Sätzen.
- `game.go`: `Game` (`NewGame(ctx, root, savesDir, log)`); `Open(seed)` prüft `fullSeed`, legt über `fullSave` den Spielstand `voll-<seed>` an (`room.NewManager` im Dev-Modus mit `store.Saves`, Welten aus `data/worlds`, `FullSave`) und startet das Rust-Spiel losgelöst (`startDetached`) mit `--saves`, `--load`, `--players 1`.
- EXE-Suche `nativeExe`: zuerst `rust/target/release/k3c-game.exe` (Arbeitsordner Checkout), dann `dist/k3c-game/k3c-game.exe`; fehlt beides, Hinweis auf `task build` (`nativeBuildTask`).
- Spielstände unter `DefaultSavesDir` (`<Checkout>/.work/studioview/saves`, ignoriert; Regel „Schreiben nur im Repo“). `Close`: danach startet `Open` nichts mehr, ein laufendes Spiel bleibt offen.

## Flow

1. Oberfläche (Assets › Sound/Quellen) → `App.StudioSounds`/`StudioSoundData`/`StudioSources` → Funktionen in `studioview.go`.
2. „Im Spiel ansehen“ → `App.StudioGameOpen(seed)` → `studioGame()` legt `Game` beim ersten Aufruf an → `Open` → `fullSave` → `exe` → `startDetached`.
3. `App.shutdown` → `closeStudio` → `Game.Close`.

## Integration

- Konsumenten: `tools/k3c-workbench/app_studio.go` (Bindings), `app.go` (Feld `game`), `uiMethods` in `uiapi.go`; Oberfläche `frontend/src/assets/sound/`, `sources/`, `game/`.
- Abhängigkeiten: `engine/room`, `engine/store`, `tools/assetkit` (Richtung tools → engine, nie umgekehrt); Rust-EXE des Spiels (`rust/target/release/` oder `dist/k3c-game/`).
