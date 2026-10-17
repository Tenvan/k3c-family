# tools/assetkit/

## Responsibility
Eigenes Go-Modul `k3c/tools/assetkit` (Paket `assets`), gemeinsam von k3c-dev und Asset-Studio per `replace` eingebunden (B-459; vorher `tools/k3c-dev/internal/assets`). Ressourcen-Manager (B-298, Sprint M10): Slot-Register `data/assets.json` (Asset-Slots mit Zuordnungen nach Präferenz) und Asset-Katalog aller Bilder und Sounds unter `public/` mit Credit und Lizenz-Kennzeichen (Q70).

## Design
- `register.go`: Typen `Register`, `Slot`, `Choice`, `Rect`, `Size`; `Load` (ungültiges JSON = Fehler mit Pfad, nie überschreiben), `Save` (atomar, 2 Leerzeichen, Umlaute im Klartext), `SetChoices` (prüft Datei unter `public/`, Art per Endung, Ausschnitt in den Bildmaßen; Mutex `mu` reiht Oberfläche und MCP), `Status` (`zugeordnet`, `luecke`, `fehlt`), `Categories` je Art.
- `catalog.go`: `Catalog` läuft über `public/grafik`, `public/sprites`, `public/audio`, `public/audio-test`; Gruppe aus `public/grafik/index.json`, Credit aus den Tabellen `| Ordner | Pack | Urheber | Lizenz | Quelle |` (`readCredits`); `LicenseFlag` (ND vor NC vor SA), `Search` (alle Wörter, ohne Groß/Klein, Filter Art, Limit).
- `research.go`: Recherche-Treffer in `data/assets-recherche.json` (`LoadResearch`, `AddHits` ohne Doppel je Link, `DropHit`, `HitsOf`, `RunResearch`), `ClassifyLicense` nach Q70 (`erkannt`, `pruefen`, `ausgeschlossen`).
- `sources.go`: `Searcher` mit freesound (API, Token aus `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`), Wikimedia Commons (MediaWiki-API, direkter Download) und OpenGameArt (Suchseite als HTML); Quellen parallel, Fehler einer Quelle als Hinweis. Tests spielen `testdata/` ab.
- `import.go`: `ImportFile` (Endung je Art, höchstens 20 MB, Datei-Signatur, Name aus dem Titel ohne Pfadanteile, nie überschreiben → `public/grafik/kandidaten/` bzw. `public/audio/kandidaten/`, Zuordnung als letzte Präferenz), `DownloadHit` (nur `https` und erkannte Lizenz). `credits.go`: Credit-Zeile je Datei im Abschnitt „Importe (k3c-dev)“ der Credit-Datei, `ConfirmLicense` (nur Oberfläche, 🧑).
- `stamp.go`: `FindRoot` (aufwärts `go.mod` mit `module k3c`), `Stamp` (SHA-256 von `data/assets.json`), `SetChoicesIf` (Schreibregel: nur auf gleichem Stand, sonst `ErrStale` „Register geändert, bitte erneut speichern“ ohne Schreibzugriff), `Watch` (Poll-Schleife: Größe/Änderungszeit als Vorfilter, Hash entscheidet; meldet nur echte Änderungen).
- Funktionen nehmen `root` explizit (Worktree-fähig); nur Standardbibliothek.

## Flow
1. MCP-Tool oder Binding → `Load(root)` → `Find`/`Status` für Listen.
2. `SetChoices(root, id, wahl)` → `checkChoice` je Zuordnung → `Save`.
3. `Catalog(root)` → `Search(list, query, art, limit)`.

## Integration
- Konsumenten k3c-dev: `internal/mcpsrv/tools_assets.go` (`assets_list`, `assets_get`, `assets_set`, `assets_catalog`), `tools_assets_research.go` (`assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import`), `app_assets.go` (Bindings). Asset-Studio: `tools/k3c-studio/app_assets.go` (Bindings mit `Stamp`/`SetChoicesIf`), `app.go` (`Watch` → Ereignis `assets:changed`), `main.go` (`FindRoot`).
- Daten: `data/assets.json` (Erstbefüllung aus `docs/assets/zuordnung*.md`), `public/**/CREDITS.md`, `public/audio/README.md`.
- Tests: `repo_test.go` prüft das echte Register (gültig, byte-gleiche Rundreise); `src/tools/assets.test.ts` prüft es gegen die Zuordnungstabellen.
