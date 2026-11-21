# tools/k3c-workbench/frontend/src/world/

## Responsibility

Seite **Welten-Editor** (B-879, Reiter `welten` im Bereich Studio; Port aus dem früheren Asset-Studio): Welten unter
`data/worlds/` ansehen, Level einer Insel-Stufe zu einem Seed als Chunk-Streifen zeigen, Abschnitte inspizieren und per
Prompt an Claude ändern lassen, Natur-Werte der Stufe festsetzen und Overrides der Basis-Dateien bearbeiten.

## Design

- `WorldPage.tsx`: `ListDetail` mit Welten-Liste links; rechts Kopf mit Auswahl „Insel · Stufe“ (`<insel>:<index>`),
  `ChunkPreview`, `ChunkInspector`, `NatureValues`, `WorldValues`.
- `ChunkPreview.tsx`: Seed, Zoom (`SCALES` 1/2/4 Pixel je Unit), `Strip` mit Abschnitten, Objekten und Hub-Mitte,
  Legende und Warnungen der Spielbarkeits-Prüfung; Klick wählt einen Abschnitt.
- `ChunkInspector.tsx`: gewählter Abschnitt mit Art, Platz, Objekten und Planung; `chunkPrompt` baut den Prompt (Ort,
  Ist-Zustand, Datei, Regeln), kopieren oder `backend.openInClaude`. Die Planung liegt in der Welt-Datei.
- `NatureValues.tsx` (B-891, `docs/rules/welten.md` § 2.1): je Eigenschaft und Variante der geltende Wert, Herkunft
  (Seed oder fest) und Auswahl zum Festsetzen; „aus Seed“ setzt zurück. `natureRows` ordnet wie im Biom.
- `WorldValues.tsx` (B-680, B-682): je Feld einer Basis-Datei Basiswert, Wert der Welt und Status; Enter/Verlassen
  schreibt den Override sofort, ↺ setzt zurück; nennt Eltern- und Kind-Welten.
- Reine Logik: `levelView.ts` (`levelModel` als Zeichenmodell, unvollständige Antwort -> leeres Modell mit Warnung
  `INCOMPLETE`; `slotsOf` wie `gen.slotIndex` in Go umgekehrt; `objectsIn`), `worldOverrides.ts` (`valueRows`,
  `valueText`, `parseValue`, Filter), `kindIcons.ts` (`iconOf`, `nameOf`, `FALLBACK_ICON`).
- Reiter der Seite (B-1216, gemerkt `world.tab`): **Inseln** (`WorldPromptBar` = „Welt ändern lassen“ mit `worldPrompts.ts`, Übersicht, Chunk-Vorschau, Inspektor, Natur), **Geschichte** (`StoryTab`: links kompakter Baum Start → Etappen mit Inseln (Ankunft, Endboss, Sieg) → Ende → Flaschenpost → Wortschatz, rechts Text und Prompt des gewählten Knotens über `storyPrompts.ts`, `PromptRow`), **Katalog** (`CatalogTab`, `catalogView.ts`: nur die eigenen, exklusiven Assets der Welt (`useWeltSicht`) als `AssetTree` aus `assets/catalog`, Insel-Auswahl filtert per Namensregel (neutral oder Biom der Insel), rechts derselbe Asset-Editor wie die Seite Katalog (`AssetDetail`), Prompt `assets_*`), **Werte** (`WorldValues` mit Baum `ValueTree`: aufklappbarer Baum aus Dateien und Themen mit Symbol und Namen aus `valueIcons.ts`, Wahl filtert die Tabelle).
- Auto-Reload (B-1216): Das Backend (`internal/world/watch.go`) meldet geänderte Dateien unter data/worlds und data/*.json als Ereignis `world:changed`; `useWorld` lädt Liste, gewählte Welt (still, Stufe bleibt) und Level neu.
- `useWorld.ts`: Welten-Liste, gewählte Welt (gemerkt `world.id`), `StageChoice` (Welt, Insel, Index), Seed
  (`DEFAULT_SEED = 'k3c'`), Level und Werte; eine neuere Anfrage macht eine ältere ungültig.

## Flow

1. Mount -> `useWorld` -> `backend.worldList()` -> gewählte Welt -> `worldDetail(id)` und `worldValues(id)`.
2. Insel-Stufe und Seed -> `backend.worldLevel(...)` -> `levelModel` -> `ChunkPreview`/`ChunkInspector`.
3. Natur festsetzen -> `worldNatureSet` (schreibt in `natureByDepth` der Insel, geprüft wie im Server) -> Level neu.
4. Override eingeben -> `parseValue` -> `worldOverrideSet` -> Werte neu laden.

## Integration

- Konsument: `src/App.tsx` (`Tabs.Content value="welten"`).
- Backend: Bindings `World*` in `tools/k3c-workbench/app_world.go` (Paket `internal/world`).
- Abhängigkeiten: `src/api` (Typen aus `api/typesWorld.ts`), `lib/prefs`, `@webkit/atoms|molecules|templates`.
  Stile: `../styles/world.css`. Mock: `api/mockWorld.ts`.
