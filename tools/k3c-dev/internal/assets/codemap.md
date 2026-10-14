# tools/k3c-dev/internal/assets/

## Responsibility
Ressourcen-Manager (B-298, Sprint M10): Slot-Register `data/assets.json` (Asset-Slots mit Zuordnungen nach Präferenz) und Asset-Katalog aller Bilder und Sounds unter `public/` mit Credit und Lizenz-Kennzeichen (Q70).

## Design
- `register.go`: Typen `Register`, `Slot`, `Choice`, `Rect`, `Size`; `Load` (ungültiges JSON = Fehler mit Pfad, nie überschreiben), `Save` (atomar, 2 Leerzeichen, Umlaute im Klartext), `SetChoices` (prüft Datei unter `public/`, Art per Endung, Ausschnitt in den Bildmaßen; Mutex `mu` reiht Oberfläche und MCP), `Status` (`zugeordnet`, `luecke`, `fehlt`), `Categories` je Art.
- `catalog.go`: `Catalog` läuft über `public/grafik`, `public/sprites`, `public/audio`, `public/audio-test`; Gruppe aus `public/grafik/index.json`, Credit aus den Tabellen `| Ordner | Pack | Urheber | Lizenz | Quelle |` (`readCredits`); `LicenseFlag` (ND vor NC vor SA), `Search` (alle Wörter, ohne Groß/Klein, Filter Art, Limit).
- Funktionen nehmen `root` explizit (Worktree-fähig); nur Standardbibliothek.

## Flow
1. MCP-Tool oder Binding → `Load(root)` → `Find`/`Status` für Listen.
2. `SetChoices(root, id, wahl)` → `checkChoice` je Zuordnung → `Save`.
3. `Catalog(root)` → `Search(list, query, art, limit)`.

## Integration
- Konsument: `internal/mcpsrv/tools_assets.go` (`assets_list`, `assets_get`, `assets_set`, `assets_catalog`).
- Daten: `data/assets.json` (Erstbefüllung aus `docs/assets/zuordnung*.md`), `public/**/CREDITS.md`, `public/audio/README.md`.
- Tests: `repo_test.go` prüft das echte Register (gültig, byte-gleiche Rundreise); `src/tools/assets.test.ts` prüft es gegen die Zuordnungstabellen.
