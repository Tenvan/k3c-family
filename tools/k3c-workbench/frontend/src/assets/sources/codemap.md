# tools/k3c-workbench/frontend/src/assets/sources/

## Responsibility

Teil **Quellen** der Seite „Sound & Quellen“ im Bereich Studio (WB1.4, D-03): Katalog der Dateien unter `public/` mit
Lizenz-Kennzeichen und die Recherche-Treffer je Slot, gemeinsam filterbar, mit Prompt-Buttons (B-729). Recherche,
Download, Import und Lizenz-Bestätigung bleiben MCP (`assets_research`, `assets_hits`, `assets_import`).

## Design

- `sourcesFilter.ts` (rein): `QuellenEintrag` (Datei oder Treffer mit Werten für Filter und Prompt), `eintraege(q)`
  flacht die Antwort ab, `lizenzFamilie` (Familie ohne Version, ohne Angabe `UNBEKANNT`), `QUELLEN_FILTER`
  (`ListFilterConfig` aus webkit: Herkunft, Lizenz, Pack, Art; Treffer ohne Art als „–“).
- `SourcesPage.tsx`: lädt die Quellen einmal, zeigt die gefilterte Liste; „Öffnen“ eines Treffers über
  `backend.openUrl(link)`. Ein gewählter Eintrag zeigt `PromptButtons` mit Datei, Pack und Lizenz.

## Flow

1. Mount -> `backend.studioSources()` -> `eintraege` -> Filter (`QUELLEN_FILTER`) -> Liste.
2. Auswahl -> `PromptButtons` (`../prompts/`, Vorlagen der Orte `datei` bzw. `treffer`) mit Platzhaltern `datei`,
   `pack`, `lizenz`, `slot`, `treffer`.
3. „Öffnen“ -> `backend.openUrl` (externer Browser).

## Integration

- Eingehängt von `../SoundSourcesPage.tsx`.
- Backend: Binding `StudioSources` (`tools/k3c-workbench/app_studio.go`, `internal/studioview`).
- Abhängigkeiten: `backend` aus `src/api`, Typen aus `../api.ts`, `../prompts/`, `@webkit/lib/listFilter`.
- Tests: `Sources.test.tsx`, `sourcesFilter.test.ts`.
