# tools/k3c-studio/

## Responsibility
Asset-Studio (B-459, Sprint WZ6): eigene Wails-App mit je einer Seite für Grafik und Sound auf dem Slot-Register `data/assets.json`, getrennt von der Workbench k3c-dev, damit Umbauten am Asset-Studio die Workbench (MCP, Planung) nicht stören. Eigenes Go-Modul `k3c/tools/k3c-studio`; die Asset-Logik kommt aus `tools/assetkit` (`replace`). Kein MCP, kein eigener Port; die MCP-Tools `assets_*` bleiben in k3c-dev.

## Design
- `main.go`: Repo-Wurzel über `assets.FindRoot` (Arbeitsverzeichnis, ersatzweise Ordner der EXE), Fenster „Asset-Studio“, `go:embed` von `frontend/dist`, `AssetServer` mit `Handler: publicHandler(root)`, Single-Instance-Lock.
- `app.go`: `App` (Bindings), `startup` startet `assets.Watch` auf `data/assets.json` und sendet `assets:changed`; Test-Nähte `emit`, `poll`.
- `app_assets.go`: Bindings wie k3c-dev ohne Vite und Dienste: `AssetsList` (mit `stand` = Hash, `preview` = `/`), `AssetsSet(id, wahl, stand)` über `SetChoicesIf` (Ablehnung „Register geändert, bitte erneut speichern“), `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport` (Dateidialog), `AssetsDownload`, `AssetsLicenseOK`.
- `public.go`: `publicHandler` liefert `<Wurzel>/public` nur lesend (GET/HEAD, `os.Root`, keine Ordner, kein `..`, kein Backslash).
- `frontend/`: Vite/React/Radix (Versionen wie k3c-dev). Spiel-Code nur lesend über die Aliase `@game` (`src/`) und `@data` (`data/`), festgelegt in `paths.ts` für `vite.config.ts` (publicDir = `public/` nur im Dev-Modus, `server.fs.allow` auf die Repo-Wurzel) und `vitest.config.ts`; `tsconfig.json` mit denselben `paths`.
  - `src/App.tsx`: Rahmen mit Navigation Grafik/Sound (gemerkt unter `k3c-studio.app.page`), Mock-Hinweis ohne Wails.
  - `src/api/`: `types.ts` (Asset-Typen mit `stand`, `Backend`), `wails.ts` (Bindings, `assets:changed`), `mock.ts`/`mockAssets.ts` (Mock mit Stand, Schreibregel und Fremdschreiber `external`), `stale.ts` (`STALE`, `isStale`).
  - `src/assets/`: gemeinsame Seitenstruktur aus k3c-dev für Grafik und Sound (`AssetsPage` mit fester Art, `SlotDetail`, `CatalogPicker`, `ResearchPanel`, `CutEditor`, `MiniScene`), `useAssets` (Neuladen bei `assets:changed`, Speichern mit Stand, Ablehnung → neu laden und Meldung), `assets.ts` (reine Logik inkl. `applySaved`, `selectionOf`); Tests `assets.test.ts`, `reload.test.tsx` (happy-dom, AC-07).
  - `src/grafik/`: Seite Grafik mit Reitern Slots, Szenen, Packs (`GrafikPage`); `ScenesView` (Szenen Figuren, Gebäude, Umgebung, Objekte, HUD, Vergleich Präferenz n), `GebaeudeStreifen`, `Hintergruende`, `SlotCards` (Bildkarten, Figuren über `renderReference`), `PacksView` (`GRAFIK_PACKS`, `grafik/index.json`), `pixel.tsx` (Zeichnen ohne Glättung); `szenenLogic.ts` (umgezogen aus `src/tools/assetvorschauLogic.ts`) mit Test.
  - `src/sound/`: Seite Sound mit Reitern Slots, Szenen, Hörprobe (`SoundPage`, stoppt beim Verlassen alle Busse); `audio.ts` (ein `audioCore` des Spiels je Fenster, `playChoice`, `stopAll`), `playback.ts` (Abspiel-Zuordnung je Zuordnung: Atlas-Ton, „fern“, Musik mit Crossfade, Umgebung als Schleife; umgezogen aus `assetvorschau.ts`) mit Test, `SoundProbe` (Hörprobe am Slot), `SoundScenes` (sfx, musik, ambient), `Hoerprobe` (Liste aus `@game/tools/soundtestLogic`, Tastatur, Lautstärke); DOM-Test `SoundPage.test.tsx`.
  - `src/lib/`, `src/ui/`, `src/styles/`: Kopien aus k3c-dev (Prefs unter `k3c-studio.`, Fehlertext, Debounce, Bausteine, `Splitter`, Theme, Asset-Styles), `szenen.css` für Szenen, Packs und die Seite Sound.

## Flow
1. Start → `findRoot` → `wails.Run` → `startup` → `Watch`.
2. Oberfläche → `AssetsList` (Slots + Stand) → Bearbeiten → `AssetsSet(…, stand)` → neuer Stand oder Ablehnung.
3. Fremder Schreiber (MCP `assets_set`) → `Watch` → `assets:changed` → Oberfläche lädt neu.

## Integration
- Tasks: `task k3c-studio` (`wails dev`), `task k3c-studio:build` (→ `bin/k3c-studio.exe`), `task studio:frontend`, `task check:studio` (Studio-Vitest, Go-Tests, Lint; Teil von `task check:all`); CI-Job `k3c-studio`. Das Vitest der Wurzel nimmt `tools/k3c-studio/**` aus.
- Daten: `data/assets.json`, `public/**`, `data/assets-recherche.json` (über `tools/assetkit`).
