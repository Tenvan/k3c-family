# tools/assetkit/

## Responsibility
Eigenes Go-Modul `k3c/tools/assetkit` (Paket `assets`), gemeinsam von k3c-workbench und Asset-Studio per `replace` eingebunden (B-459; vorher `tools/k3c-workbench/internal/assets`). Ressourcen-Manager (B-298, Sprint M10): Slot-Register `data/sounds.json` (Sound-Slots mit Zuordnungen nach Präferenz; seit CU1.2, davor mit Grafik-Slots) und Asset-Katalog aller Bilder und Sounds unter `public/` mit Credit und Lizenz-Kennzeichen (Q70).

## Design
- `register.go`: Typen `Register`, `Slot`, `Choice`, `Rect`, `Size`; `Load` (ungültiges JSON = Fehler mit Pfad, nie überschreiben), `Save` (atomar, 2 Leerzeichen, Umlaute im Klartext), `SetChoices` (prüft Datei unter `public/`, Art per Endung, Ausschnitt in den Bildmaßen; Mutex `mu` reiht Oberfläche und MCP), `Status` (`zugeordnet`, `luecke`, `fehlt`), `Categories` je Art.
- `catalog.go`: `Catalog` läuft über `public/grafik`, `public/sprites` (beide fallen mit CU1.5, fehlende Ordner sind kein Fehler), `public/audio`, `public/audio-test`; Gruppe aus `public/grafik/index.json`, Credit aus den Tabellen `| Ordner | Pack | Urheber | Lizenz | Quelle |` (`readCredits`); `LicenseFlag` (ND vor NC vor SA), `Search` (alle Wörter, ohne Groß/Klein, Filter Art, Limit).
- `research.go`: Recherche-Treffer in `data/assets-recherche.json` (`LoadResearch`, `AddHits` ohne Doppel je Link, `DropHit`, `HitsOf`, `RunResearch`), `ClassifyLicense` nach Q70 (`erkannt`, `pruefen`, `ausgeschlossen`).
- `sources.go`: `Searcher` mit freesound (API, Token aus `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`), Wikimedia Commons (MediaWiki-API, direkter Download) und OpenGameArt (Suchseite als HTML); Quellen parallel, Fehler einer Quelle als Hinweis. Tests spielen `testdata/` ab.
- `import.go`: `ImportFile` (Endung je Art, höchstens 20 MB, Datei-Signatur, Name aus dem Titel ohne Pfadanteile, nie überschreiben → `public/grafik/kandidaten/` bzw. `public/audio/kandidaten/`, Zuordnung als letzte Präferenz), `DownloadHit` (nur `https` und erkannte Lizenz). `credits.go`: Credit-Zeile je Datei im Abschnitt „Importe (k3c-workbench)“ der Credit-Datei, `ConfirmLicense` (nur Oberfläche, 🧑).
- `stamp.go`: `FindRoot` (aufwärts `go.mod` mit `module k3c`), `Stamp` (SHA-256 von `data/sounds.json`), `SetChoicesIf` (Schreibregel: nur auf gleichem Stand, sonst `ErrStale` „Register geändert, bitte erneut speichern“ ohne Schreibzugriff), `Watch` (Poll-Schleife: Größe/Änderungszeit als Vorfilter, Hash entscheidet; meldet nur echte Änderungen).
- `candidates.go` / `candidates_assign.go`: Kandidaten eines Slots in einer Liste (B-677, WZ33): `Candidates` führt Zuordnungen (`wahl:`), Vorrat (`sug:` geladen), Netz (`sug:` offen, Recherche-Treffer `hit:`) und Katalog-Treffer (`cat:`) zusammen, gleiche Datei-URL wird eine Zeile mit allen Herkünften, Lizenz-Marke `frei/sa/pruefen/gesperrt`. `AssignCandidate` löst die ID neu auf (nie Lizenz oder Größe aus der Anfrage), prüft den Stand vor jedem Schreiben und Download und ordnet als letzte Präferenz zu (Vorschlag: `VorratDownload` + `VorratAdoptIf`, Treffer: `DownloadHit`, Katalog: `SetChoicesIf`).
- `target.go` (B-685): `Units` (Zielmaß `ziel` in Units), `checkTarget` (beim Laden: je Seite 0 bis `MaxTargetUnits`), `SetTargetIf` (Schreibregel wie `SetChoicesIf`, nil oder Seite 0 entfernt).
- Funktionen nehmen `root` explizit (Worktree-fähig); nur Standardbibliothek.

## Flow
1. MCP-Tool oder Binding → `Load(root)` → `Find`/`Status` für Listen.
2. `SetChoices(root, id, wahl)` → `checkChoice` je Zuordnung → `Save`.
3. `Catalog(root)` → `Search(list, query, art, limit)`.

## Integration
- Konsumenten k3c-workbench: `internal/mcpsrv/tools_assets.go` (`assets_list`, `assets_get`, `assets_set`, `assets_catalog`), `tools_assets_research.go` (`assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import`). Asset-Studio: `tools/k3c-studio/app_assets.go` (Bindings mit `Stamp`/`SetChoicesIf`), `app.go` (`Watch` → Ereignis `assets:changed`), `main.go` (`FindRoot`).
- Daten: `data/sounds.json` (nur Sound-Slots, CU1.2), `public/**/CREDITS.md`, `public/audio/README.md`.
- Tests: `repo_test.go` prüft das echte Register (gültig, byte-gleiche Rundreise); `src/tools/assets.test.ts` prüft es gegen die Zuordnungstabellen.
