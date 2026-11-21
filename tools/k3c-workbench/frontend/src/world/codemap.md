# tools/k3c-workbench/frontend/src/world/

## Responsibility

Seite **Welten-Editor** (B-879, Reiter `welten` im Bereich Studio; Port aus dem früheren Asset-Studio): Welten unter
`data/worlds/` ansehen, Level einer Insel-Stufe zu einem Seed als Chunk-Streifen zeigen, Abschnitte inspizieren und per
Prompt an Claude ändern lassen, Natur-Werte der Stufe festsetzen und Overrides der Basis-Dateien bearbeiten; dazu
Welten anlegen und löschen, Etappen der Inseln setzen (Querschnitt, B-963) und den Stand der Geschichte zeigen.

## Design

- `WorldPage.tsx`: `ListDetail` mit Welten-Liste links (feste Welten ohne Datei als „noch nicht umgesetzt“, ein Klick legt
  sie aus `standard` an; darunter `NewWorld`); rechts Kopf mit `CatalogLinks`, `DeleteWorld`, `StoryState`, danach
  `Overview` (Querschnitt und Seekarte), Auswahl „Insel · Stufe“ (`<insel>:<index>`), `ChunkPreview`, `ChunkInspector`,
  `NatureValues`, `WorldValues` (je Welt neu aufgebaut, `key`). Ein Klick im Querschnitt wählt die Stufe und scrollt zum Editor.
- `Overview.tsx`: Seekarte links (`SeaMap.tsx`, bleibt beim Scrollen stehen, Etappen untereinander, Inseln als Knoten mit
  Mini-Querschnitt) und Querschnitt rechts (`CrossSection.tsx`, B-963): je Insel eine Karte, jede Stufe ein Streifen
  (Breite proportional zur Länge, Farben der Biom-Palette, Chunk-Raster), Etappen-Bänder, Eingabefeld der Etappe je Insel
  (leer = Listenplatz + 1, gespeichert beim Verlassen). Ein Klick auf einen Seekarten-Knoten scrollt zur Insel und hebt sie hervor.
- `NewWorld.tsx` (B-954: ID, Name, optional Eltern-Welt, Muster `ID_PATTERN`), `DeleteWorld.tsx` (B-1172: Bestätigung mit dem
  Namen; das Backend lehnt feste Welten und Welten mit Erben ab, Spielstände bleiben), `CatalogLinks.tsx` (`openCatalog`:
  setzt den Welt-Filter in der Adresse und wechselt zum Katalog, exklusiv oder gesamt; WZ62.4, B-1171), `StoryState.tsx`
  (B-975: `storyGaps`, `storyPrompt`, Stand „vorhanden/unvollständig/fehlt“ mit Prompt-Button; geschrieben wird über das
  MCP-Tool `world_story_set`).
- `ChunkPreview.tsx`: Seed, Zoom (`SCALES` 1/2/4 Pixel je Unit), `Strip` mit Abschnitten, Objekten und Hub-Mitte,
  Legende und Warnungen der Spielbarkeits-Prüfung; Klick wählt einen Abschnitt.
- `ChunkInspector.tsx`: gewählter Abschnitt mit Art, Platz, Objekten und Planung; `chunkPrompt` baut den Prompt (Ort,
  Ist-Zustand, Datei, Regeln), kopieren oder `backend.openInClaude`. Die Planung liegt in der Welt-Datei.
- `NatureValues.tsx` (B-891, `docs/rules/welten.md` § 2.1): je Eigenschaft und Variante der geltende Wert, Herkunft
  (Seed oder fest) und Auswahl zum Festsetzen; „aus Seed“ setzt zurück. `natureRows` ordnet wie im Biom.
- `WorldValues.tsx` (B-680, B-682, B-1173): Tabelle je Feld mit Standardwert, Wert der Welt (`ValueField`), Status und
  Abweichung (`deviation`); Gruppen nach Thema einklappbar (ab `COLLAPSE_FROM = 60` Feldern eingeklappt, außer mit
  Überschreibungen), feste Kopfzeile, Dateiwahl, Suche in Pfad und Basiswert und Modus alle/überschrieben/Sitzung; eine
  gültige Eingabe schreibt den Override sofort, ↺ setzt zurück; nennt Eltern- und Kind-Welten.
- Reine Logik: `levelView.ts` (`levelModel` als Zeichenmodell, unvollständige Antwort -> leeres Modell mit Warnung
  `INCOMPLETE`; `slotsOf` wie `gen.slotIndex` in Go umgekehrt; `objectsIn`), `worldOverrides.ts` (`valueRows`,
  `filterRows` mit `RowMode`, `groupRows`/`RowGroup`, `filesOf`, `GENERAL`; `valueText` aus `@webkit/lib/valueEdit`),
  `crossModel.ts` (Etappen-Gruppen `groupEtappen`, `etappeOf`, Palette `paletteOf`/`gradientOf`, `bandText`, `islandAnchor`,
  `stageId`), `kindIcons.ts` (`iconOf`, `nameOf`, `FALLBACK_ICON`).
- `useWorld.ts`: Welten-Liste, gewählte Welt (gemerkt `world.id`), `StageChoice` (Welt, Insel, Index), Seed
  (`DEFAULT_SEED = 'k3c'`), Level und Werte; eine neuere Anfrage macht eine ältere ungültig. Dazu `create`, `remove`,
  `setEtappe` (lädt die Welt danach neu) und `crossRefresh` (verwirft eine Stufe im Querschnitt nach einer Natur-Änderung);
  ein vorgemerkter Hinweis erscheint nach dem Laden der nächsten Welt.
- `useCross.ts`: lädt alle Stufen der Welt für den Querschnitt über `worldLevel` (Cache je Welt, Seed, Inseln und Tiefen,
  höchstens `LIMIT = 3` Aufrufe gleichzeitig; ein Fehler lässt nur die Stufe aus).

## Flow

1. Mount -> `useWorld` -> `backend.worldList()` -> gewählte Welt -> `worldDetail(id)` und `worldValues(id)`.
2. Insel-Stufe und Seed -> `backend.worldLevel(...)` -> `levelModel` -> `ChunkPreview`/`ChunkInspector`.
3. Natur festsetzen -> `worldNatureSet` (schreibt in `natureByDepth` der Insel, geprüft wie im Server) -> Level neu.
4. Override eingeben -> `ValueField` -> `worldOverrideSet` -> Werte neu laden.
5. Anlegen/Löschen -> `worldCreate`/`worldDelete` (Antwort `WorldCreateView`, `error` nennt die Ablehnung); Etappe ->
   `worldIslandEtappe` -> `worldDetail` neu.

## Integration

- Konsument: `src/App.tsx` (`Tabs.Content value="welten"`).
- Backend: Bindings `World*` in `tools/k3c-workbench/app_world.go` (Paket `internal/world`).
- Weiterer Konsument: `assets/` (Welt-Filter) über die Adresse, gesetzt von `CatalogLinks`.
- Abhängigkeiten: `src/api` (Typen aus `api/typesWorld.ts`), `lib/prefs`, `@webkit/lib/valueEdit`, `@webkit/lib/weltFilter`, `@webkit/atoms|molecules|templates`.
  Stile: `../styles/world.css`. Mock: `api/mockWorld.ts`.
