# tools/k3c-studio/frontend/src/world/

## Responsibility
Seite Welten Editor (B-460/B-461/B-477/B-485): Welten, Inseln und Stufen wählen und anlegen, Chunk-Streifen und Live-Vorschau mit gemeinsamer Ansicht, Chunk-Infos im Bühnenband, Editor für geplante Chunks (inkl. Umwandeln und Inselgröße) und „Im Spiel öffnen“.

## Design
Seite und Ansicht
- `WorldPage.tsx`: Wurzel; Chunk-Streifen oben, Live-Vorschau (`../stage/LivePreview`) mit Chunk-Band, darunter zwei Spalten (`WorldStructure` links, `PlanEditor` + Gebäude-Baum rechts); Tasten Pfeile, +/−, M (kompakt/maximiert); neues Level zentriert auf den Hub, Entwurf-Reload behält die Mitte.
- `viewState.ts`: reine Funktionen der Ansicht: `Views` (gemeinsame Mitte, Zoom je Vorschau), `paneView`, `zoomPanes`, `panViews`, `centerViews`, `clampLeft`, `centerRange`, `wheelAction` (Strg zoomt beide, Alt eine, Umschalt scrollt), `jumpPoints` (Anfang, Mitte, Ende, Hub, Tor, Portale, Ausgang).
- `viewHooks.ts`: `useBox` misst Streifen und Bühne (ResizeObserver), `useViews` hält `Views` und liefert Pan, Zoom, `setCenter`, `onWheel`.
- `ViewBar.tsx`: Zoomknöpfe, Regler, Zoomstufen, Sprungpunkte und Positions-Schieber.
- `RangeSlider.tsx`: Bereichs-Schieber (Ziehen, Klick, Pfeile, Pos1/Ende), zweiter Griff für den Bereich der Live-Vorschau; `thumbBox`, `Span`.

Vorschauen
- `ChunkPreview.tsx`: Seed-Zeile, Level über `backend.worldLevel` (nur die letzte Anfrage zählt), `Strip` (Abschnitte, Objekte, Hub-Linie, Live-Rahmen, Hinweisränder), Legende, Warnungen; Mausrad an `onWheel`.
- `levelView.ts`: Zeichenmodell `levelModel`, Farben (`colorOf`, `BIG_KINDS`, `SPECIAL_CHUNKS`), Zufalls-Seed `seedFromBytes`, `openGameReason`.
- `LiveChunkBand.tsx`: Overlay (Portal) am oberen Rand der Bühne, je Chunk Farbe, Emoji, Name, Platz, Seed, feste Objekte; Klick (`useTap`) springt zum Chunk.
- `bandLayout.ts`: reine Logik des Bands: `bandBox`, `bandChunks` (Pixelbereich), `bandMode` (Farbe, kompakt, Zeile, voll), `slotOfIndex`, `chunkInfo`.
- `chunkMarks.ts`: Hinweise je Chunk-Platz (`slotHints`: geplant, Fehler), Abbildung auf Abschnitte (`chunkMarks`, `chunkAtSlot`, wie `engine/level/planned_gen.go`).
- `kindIcons.ts`: Emoji und deutscher Name je Chunk- und Objekt-Art, `resolveKind`, `rankOptions` (Suchreihenfolge).

Planung
- `plan.ts`: reine Entwurfs-Logik (`addChunk`, `updateChunk`, `removeChunk`, Objekte, `normalized`, `isDirty`, `savedPlan`, `savedCount`, `slotLabel`, `CHUNK_KINDS`, `OBJECT_KINDS`).
- `usePlan.ts`: Zustand des Editors: geladene Inseln, Auswahl, Entwurf mit Chunk-Anzahl, verzögerte Prüfung (`planCheck`), Speichern, Verwerfen; `source` für beide Vorschauen.
- `worldDraft.ts`: reine Logik des Welt-Entwurfs (WZ18.6): `draftOf`, `sameDraft`, `addIsland` (Kopie der letzten, freie id), `moveIsland`, `removeIsland`, `updateIsland`, `toggleDepth`.
- `useWorldDraft.ts`: Zustand des Welt-Entwurfs: verzögerte Prüfung (`planWorldCheck`, Fehler am Feld, Vorschau), Speichern auf dem Stand (`planWorldSave`, Stale lässt den Entwurf stehen), Verwerfen.
- `WorldDraftEditor.tsx`: Welt-Kopf (Name, Notizen), Inseln (Name, id, hoch/runter, entfernen, Stufen als Biome, Skalierung), Fehler am Feld, Vorschau der Welt-Datei, „Welt speichern“.
- `planSave.ts`: Speichern mit Stand (`planSave`), `Notice` (`hint`, `error`, `stale`), Sicht nach dem Speichern.
- `useWorlds.ts`: Liste und Wahl der Welten (`planWorlds`, `planWorldSelect`, `planWorldAdd`, `planWorldCopy`, `planWorldRemove`, `refreshWorlds`).
- `PlanEditor.tsx`: Liste geplanter Chunks mit Platz, Art, Chunk-Seed, festen Objekten; Speichern, Verwerfen, Neu laden; Prüffehler am Platz; meldet den Platz an `onSlot`.
- `KindPicker.tsx`: durchsuchbare Auswahlliste (Combobox) für Chunk- und Objekt-Art mit Emoji.
- `SizePicker.tsx`: Inselgröße der Stufe (Biom-Länge oder Eintrag aus `data/island-sizes.json`).
- `convert.ts`: reine Logik und ein Aufruf (`planChunkObjects`): Seed-Chunk in feste Objekte umwandeln (`convertLock`, `inSync`, `convertChunk`, `seedObjects`).
- `ConvertButton.tsx`: Knopf „In feste Objekte umwandeln“ je geplantem Chunk, gesperrt mit Grund, verwirft veraltete Ergebnisse.
- `WorldStructure.tsx`: linke Spalte: Welt wählen, anlegen (leer, Kopie), löschen (`WorldBar`, `AddWorld`), Welt-Entwurf (`WorldDraftEditor`), Insel und Stufe (`IslandBar`), Baum der Inseln und Stufen (`StructureTree`), `GameLauncher`; Wechsel nur ohne offenen Entwurf.
- `GameLauncher.tsx`: „Im Spiel öffnen“ und „Voll ausgebaut öffnen“ (`worldOpenGame`), Adresse des Spielservers gemerkt (`../lib/prefs`, gebunden an `k3c-studio.`).

Tests (je neben der Quelle): `levelView`, `viewState`, `plan`, `chunkMarks`, `bandLayout`, `kindIcons`, `convert`, `worldDraft` (`.test.ts`); `PlanEditor`, `GameLauncher`, `WorldPage`, `LiveChunkBand`, `RangeSlider`, `ConvertButton`, `ChunkCount`, `ChunkFocus`, `ChunkJump`, `WorldFiles`, `WorldDraft` (`.test.tsx`).

## Flow
1. `usePlan` lädt die Welt (`planLoad`) → `source` (Insel, Stufe, Entwurf, Chunk-Anzahl) → `ChunkPreview` holt das Level (`worldLevel`) und meldet das Modell an `WorldPage`; dieselbe `source` speist die Live-Vorschau (`worldPreview`).
2. Entwurf ändern (`PlanEditor`, `ConvertButton`, `SizePicker`) → verzögerte Prüfung (`planCheck`) → gültiger Stand geht an Streifen, Band und Vorschau; Hinweise (`slotHints`) markieren Chunks → Speichern (`planSave` mit Stand) schreibt die Weltdatei.
3. Ansicht: Rad, Tasten, `ViewBar`, Klick auf Chunk oder Platz → `useViews` (gemeinsame Mitte, Zoom je Vorschau) → `chunkView` und `liveView`.
4. Welt wählen/anlegen/löschen (`WorldStructure`, nur ohne Entwurf) → `useWorlds` → neu laden; Welt-Entwurf ändern → `useWorldDraft` prüft (`planWorldCheck`) → „Welt speichern“ (`planWorldSave`) → neu laden.
5. `GameLauncher` → `worldOpenGame` → Browser auf dem Spielserver.

## Integration
- Backend: HTTP über `../api/http.ts` (POST `/api/studio/rpc/<Methode>`) an `tools/k3c-studio/backend`: `app_world.go` (`WorldLevel`), `app_preview.go` (`WorldPreview`), `app_plan.go` (`PlanLoad`, `PlanCheck`, `PlanSave`), `app_plan_convert.go` (`PlanChunkObjects`), `app_world_draft.go` (`PlanWorldCheck`, `PlanWorldSave`), `app_world_files.go` (`PlanWorlds`, `PlanWorld*`), `app_game.go` (`WorldOpenGame`), `plan_file.go`; Mock `../api/mockWorld.ts`, `../api/mockPlan.ts`, `../api/mockWorlds.ts`, `../api/mockWorldDraft.ts`.
- Abhängigkeiten: `../stage/` (`LivePreview`, `hudBar`, `stageLogic`), `../api`, `../lib/prefs`, `../styles/world.css`, `@game/model/*`; Regeln `docs/rules/geplante-chunks.md`; eingehängt in `../App.tsx` (Seite `welt`, Tab „Welten Editor“).
