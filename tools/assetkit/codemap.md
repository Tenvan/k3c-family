# tools/assetkit/

## Responsibility
Eigenes Go-Modul `k3c/tools/assetkit` (Paket `assets`), von k3c-workbench per `replace` eingebunden (B-459). Ressourcen-Manager (B-298): Slot-Register `data/sounds.json` (Sound-Slots mit Zuordnungen nach Präferenz, seit CU1.2; der Block `mischung` mit den Mischpegeln des Spiels bleibt beim Speichern unverändert), Asset-Katalog aller Bilder und Sounds unter `public/` mit Credit und Lizenz-Kennzeichen (Q70), Recherche-Treffer, Download-Vorschlagsliste und Vorrat. Unterpakete: `browser/`, `ticket/`, `windowstate/`.

## Design
- `register.go`: Typen `Register` (mit `Mix` als rohem JSON für `mischung`), `Slot`, `Choice`, `Rect`, `Size`; `Load` (ungültiges JSON = Fehler mit Pfad, nie überschreiben), `Save` (atomar, 2 Leerzeichen, Umlaute im Klartext), `SetChoices` (prüft Datei unter `public/`, Art per Endung, Ausschnitt in den Bildmaßen; Mutex reiht Schreiber), `Status` (`zugeordnet`, `luecke`, `fehlt`), `WriteAtomic`.
- `catalog.go`: `Catalog` läuft über `catalogDirs` (`public/grafik`, `public/sprites`, `public/audio`, `public/audio-test`; fehlende Ordner sind kein Fehler) mit Credit aus den Tabellen der jeweiligen Credit-Datei (`readCredits`), Gruppe aus `public/grafik/index.json` (`grafikGroups`); `LicenseFlag` (ND vor NC vor SA), `Search` (alle Wörter, ohne Groß/Klein, Filter Art, Limit).
- `research.go` / `research_add.go`: Recherche-Treffer in `data/assets-recherche.json` (`ResearchPath`; `LoadResearch`, `AddHits` ohne Doppel je Link, `DropHit`, `HitsOf`, `RunResearch`), `ClassifyLicense` nach Q70 (`erkannt`, `pruefen`, `ausgeschlossen`); `ResearchCandidates`, `PlanResearchImport`, `ImportResearch` übernehmen Treffer in die Vorschlagsliste.
- `sources.go`: `NewSearcher` mit freesound (API, Token aus `K3C_FREESOUND_TOKEN` oder `.work/freesound-token`), Wikimedia Commons (MediaWiki-API) und OpenGameArt (Suchseite als HTML); Quellen parallel, Fehler einer Quelle als Hinweis. Tests spielen `testdata/` ab.
- `import.go`: `ImportFile` (Endung je Art, Größenlimit, Datei-Signatur, Name aus dem Titel ohne Pfadanteile, nie überschreiben → `kandidaten/`; Downloads laufen über eine Temp-Datei in `.work/tmp` der Repo-Wurzel (`workTmp`), Zuordnung als letzte Präferenz), `DownloadHit` (nur `https`, erlaubte Hosts und Weiterleitungen, erkannte Lizenz). `credits.go`: Credit-Zeile je Datei im Abschnitt „Importe (k3c-workbench)“, `ConfirmLicense`.
- `stamp.go`: `FindRoot` (aufwärts `go.mod` mit `module k3c`), `Stamp` (SHA-256 von `data/sounds.json`), `SetChoicesIf` (nur auf gleichem Stand, sonst `ErrStale` ohne Schreibzugriff), `Watch` (Poll: Größe/Änderungszeit als Vorfilter, Hash entscheidet).
- `suggestions*.go` (B-601, B-673, B-600): Download-Vorschlagsliste `data/assets-vorschlaege.json` (`SuggestionsPath`); `LoadSuggestions`, `SuggestionTargets` (Lücken und Skill-Linien mit je `MinSuggestions` Kandidaten), `CheckSuggestions` (Pflichtfelder, Lizenz, `MaxSuggestionBytes`, eindeutige IDs/URLs); `PlanSuggestions`/`AddSuggestions` (Größe per `HeadSize`, Änderung währenddessen → `ErrSuggestionsChanged`), `DropSuggestion` (Status „verworfen“).
- `vorrat.go` / `vorrat_adopt.go` (WZ29.6): Download-Cache `.work/vorrat/` mit `index.json` (`LoadVorrat`, `VorratStates`, `CheckVorratDownload`, `VorratHosts`/`VorratRedirect` mit höchstens 5 Sprüngen, `VorratDownload` über Temp-Datei mit SHA-256 und `Progress`); `VorratAdopt`/`VorratAdoptIf` kopieren nach `public/` mit Credit und hängen die Zuordnung an, `VorratCleanupPreview`/`VorratCleanup` entfernen unverwendete Dateien.
- `candidates.go` / `candidates_assign.go` (B-677): `Candidates` führt Zuordnungen, Vorrat, Vorschläge, Recherche-Treffer und Katalog je Slot zu einer Liste mit Lizenz-Marke zusammen; `AssignCandidate` löst die ID neu auf und ordnet über `VorratAdoptIf`, `DownloadHit` oder `SetChoicesIf` zu.
- `target.go` (B-685): `Units` (Zielmaß in Units), `checkTarget`, `SetTargetIf`.
- Funktionen nehmen `root` explizit (Worktree-fähig); nur Standardbibliothek.

## Flow
1. MCP-Tool → `Load(root)` → `Status` je Slot; `Catalog(root)` → `Search`.
2. `assets_research` → `NewSearcher` → `RunResearch` → `AddHits` in `data/assets-recherche.json`; `assets_import` → `DownloadHit`/`ImportFile` → Credit-Zeile → Zuordnung.
3. Vorschlag: `AddSuggestions` → `VorratDownload` → `VorratAdoptIf` → `appendChoice` im Register.

## Integration
- Konsumenten in k3c-workbench: `internal/mcpsrv/tools_assets_research.go` (`assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import`), `internal/studioview/studioview.go` (`Load`, `Status`, `Catalog`, `LoadResearch`). Unterpakete: siehe [`ticket/`](ticket/codemap.md), [`windowstate/`](windowstate/codemap.md), [`browser/`](browser/codemap.md).
- Daten: `data/sounds.json`, `data/assets-recherche.json`, `data/assets-vorschlaege.json`, `.work/vorrat/`, Credit-Dateien unter `public/`.
- Tests: u. a. `repo_test.go` (echtes Register, byte-gleiche Rundreise), `vorrat_adopt_parallel_test.go`, `writepaths_test.go`.
