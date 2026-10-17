# tools/k3c-studio/

## Responsibility
Asset-Studio (B-459, Sprint WZ6): eigene Wails-App mit je einer Seite für Grafik und Sound auf dem Slot-Register `data/assets.json`, getrennt von der Workbench k3c-dev, damit Umbauten am Asset-Studio die Workbench (MCP, Planung) nicht stören. Eigenes Go-Modul `k3c/tools/k3c-studio`; die Asset-Logik kommt aus `tools/assetkit` (`replace`). Kein MCP, kein eigener Port; die MCP-Tools `assets_*` bleiben in k3c-dev.

## Design
- `main.go`: Repo-Wurzel über `assets.FindRoot` (Arbeitsverzeichnis, ersatzweise Ordner der EXE), Fenster „Asset-Studio“, `go:embed` von `frontend/dist`, `AssetServer` mit `Handler: publicHandler(root)`, Single-Instance-Lock.
- `app.go`: `App` (Bindings), `startup` startet `assets.Watch` auf `data/assets.json` und sendet `assets:changed`; Test-Nähte `emit`, `poll`.
- `app_assets.go`: Bindings wie k3c-dev ohne Vite und Dienste: `AssetsList` (mit `stand` = Hash, `preview` = `/`), `AssetsSet(id, wahl, stand)` über `SetChoicesIf` (Ablehnung „Register geändert, bitte erneut speichern“), `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport` (Dateidialog), `AssetsDownload`, `AssetsLicenseOK`.
- `public.go`: `publicHandler` liefert `<Wurzel>/public` nur lesend (GET/HEAD, `os.Root`, keine Ordner, kein `..`, kein Backslash).
- `frontend/`: Vite/React/Radix (Versionen wie k3c-dev), `src/App.tsx` Rahmen mit Navigation Grafik/Sound (gemerkt in `localStorage` `studio.page`); Seiteninhalte folgen mit WZ6.3/WZ6.4. Komponententest `src/App.test.tsx` läuft im Vitest des Hauptprojekts.

## Flow
1. Start → `findRoot` → `wails.Run` → `startup` → `Watch`.
2. Oberfläche → `AssetsList` (Slots + Stand) → Bearbeiten → `AssetsSet(…, stand)` → neuer Stand oder Ablehnung.
3. Fremder Schreiber (MCP `assets_set`) → `Watch` → `assets:changed` → Oberfläche lädt neu.

## Integration
- Tasks: `task k3c-studio` (`wails dev`), `task k3c-studio:build` (→ `bin/k3c-studio.exe`), `task studio:frontend`, `task check:studio` (Teil von `task check:all`); CI-Job `k3c-studio`.
- Daten: `data/assets.json`, `public/**`, `data/assets-recherche.json` (über `tools/assetkit`).
