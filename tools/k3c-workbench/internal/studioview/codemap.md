# tools/k3c-workbench/internal/studioview/

## Responsibility

Was vom Asset-Studio lesend blieb (Entscheidung 005 › 5, Sprint WB1, D-03/D-04): Sound-Slots und Quellen für den Bereich
Assets und „Im Spiel ansehen“ mit der Engine als Bibliothek. Schreibt nichts nach `data/` oder `public/`.

## Design

- `studioview.go`: `Sounds(root)` (Sound-Slots aus `data/sounds.json` mit `assets.Status`), `SoundData(root, pfad)`
  (Audiodatei unter `public/` als Data-URL, über `os.Root`, nur `.ogg/.mp3/.wav/.m4a`, ohne `..`/Backslash/absolute Pfade,
  höchstens 10 MB), `Sources(root)` (Katalog `assets.Catalog` und Recherche-Treffer `assets.LoadResearch`).
- `game.go`: `Game` startet beim ersten `Open(seed)` die Engine (`room.Manager` im Dev-Mode, Welten aus `data/worlds`,
  `engine/net` als Spiel-API) auf `127.0.0.1:0`, legt `voll-<seed>` an und liefert `…/index.html#game.html?autostart=1&save=…`.
  Spielstände unter `DefaultSavesDir` (`<UserCacheDir>/k3c/studioview/<Hash des Checkouts>/saves`), nie im Repo. Ohne
  `dist/index.html` Hinweis `task build`. `Close` beendet Listener und wartet auf das Speichern der Räume.
- `handler.go`: Listener-Handler aus dem früheren `tools/k3c-studio/server.go`: `hostGuard` (Loopback-Gegenstelle,
  Host localhost oder Loopback-IP), `gameGuard` (schreibend nur gleiche Herkunft), Dateien erst aus `public/`, dann `dist/`.

## Integration

- Bindings in `app_studio.go` (`StudioSounds`, `StudioSoundData`, `StudioSources`, `StudioGameOpen`), `uiMethods` in `uiapi.go`;
  Oberfläche `frontend/src/assets/sound/`, `sources/`, `game/`. `App.shutdown` ruft `closeStudio`.
- Importiert `engine/net`, `engine/room`, `engine/store` und `tools/assetkit` (Richtung tools → engine, nie umgekehrt).
