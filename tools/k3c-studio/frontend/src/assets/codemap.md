# tools/k3c-studio/frontend/src/assets/

## Responsibility

Asset-Seiten im Studio (k3c-studio) (Entscheidung 005 › 5, Sprint WB1): Review (RV1), Katalog, Styleguide
und „Sound & Quellen“. Nur Ansehen – kein schreibendes Control; Assets ändern nur die MCP-Tools `assets_*`. Daten über
die RPC-Methoden laut `docs/standards/paper-mcp.md` › 4.

## Design

- `AssetsArea.tsx`: die vier Seiten als `Tabs.Content` im `Tabs.Root` von AppFrame (`App.tsx`, Reiter aus `Header.tsx`:
  `review`, `katalog`, `styleguide`, `sound`); rechnet beim Start das Review gegen `HEAD` im Hintergrund vor
  (`loadReview('HEAD')`, B-732).
- `SoundSourcesPage.tsx`: Seite „Sound & Quellen“ (B-892) mit Umschalter (`ChipSwitch`, gemerkt `studio.soundTeil`)
  zwischen `sound/SoundPage` und `sources/SourcesPage`.
- `api.ts`: dünne Zugriffe auf die Asset-Methoden des `backend` (`assetsList`, `assetsIndex`, `assetsCheck`, `assetsGet`,
  `assetsFrame`, `assetsFrameOhne`, `assetsGroesse`, `assetsGroesseBasis`, `assetsHintergruende`, `assetsHintergrund`,
  `assetsStufen`, `assetsStyle`, `assetsPrompts`, `assetsReview`, `assetsPixelDiff`) und Re-Export der Typen aus `api/typesAssets.ts`.
- `useWeltSicht.ts`: Hook `useWeltSicht(welt)` für die Welt-Filter von Katalog und Sound: lädt die Weltliste
  (`backend.worldList`, Typ `WeltAuswahl`, feste Welten auch ohne Datei) und die Sicht der gewählten Welt
  (`backend.studioWeltSicht`, B-1169: welche Assets sie sieht, mit Herkunft). Ohne Welt gibt es keine Sicht (alles
  sichtbar); Antworten für eine inzwischen andere Welt werden verworfen, bis dahin ist `laedt` wahr.
- `zoom.tsx`: Zoom-Stufen 0,5–4× der Grafik (B-733); `useZoom(ansicht, start)` merkt die Stufe je Ansicht über Asset-
  und Tab-Wechsel. Genutzt von `catalog/Preview` und `review/Compare` (der Größenvergleich skaliert seit B-1170 ohne Zoomstufen).
- `testing.tsx`: Test-Helfer (`mount`, `until`, `click`, `chip` …) für die Vitest-Tests der Unterordner.
- Unterordner (je mit eigener codemap): `catalog/` (Katalog), `review/` (Review), `prompts/` (Platzhalter, Teil-Auswahl,
  Prompt-Buttons), `size/` (Größenvergleich mit dem Monarchen), `style/` (Styleguide), `sound/`, `sources/`, `game/`
  („Im Spiel ansehen“).
- Stile: `../styles/assets.css` (Präfix `as-`).

## Flow

1. `App` -> `AssetsArea` -> `loadReview('HEAD')` im Hintergrund.
2. Reiterwechsel im Bereich Studio -> jeweilige Seite lädt über `api.ts` bzw. `backend.studio*` ihre Daten selbst; Katalog
   und Review halten Modul-Caches, die den Tab-Wechsel überleben.
3. Prompt-Buttons auf allen Seiten -> `prompts/fill.ts` -> Zwischenablage oder `backend.openInClaude`.

## Integration

- Konsument: `src/App.tsx`.
- Backend: Bindings in `tools/k3c-studio/app_assets.go` (Paket `internal/assetsui`) und `app_studio.go`
  (`internal/studioview`); im Browser über `src/api/http.ts`, im Mock über `src/api/mockAssets.ts`.
- Abhängigkeiten: `src/api`, `src/lib/prefs`, `@webkit/atoms|molecules|lib` (Welt-Filter: `@webkit/lib/useWeltFilter`, `weltFilter`, Baustein `WeltFilter`).
- Tests: `readonly.test.tsx` klickt jede Seite durch und erlaubt nur Lesemethoden (AC-02).
