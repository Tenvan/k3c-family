# tools/k3c-studio/frontend/src/assets/

## Responsibility
Gemeinsame Bausteine der Zuordnung von Slots: Logik der Zuordnungen, Laden und Speichern des Slot-Registers mit Stand, Ausschnitt-Editor, Mini-Szene und Detailinfo. Die Seite selbst liegt seit B-677 in `../assetpage/`.

## Design
- `assets.ts`: reine Logik ohne React: `filterSlots`, `countStatus`, `typOf`/`typen`, `CATEGORIES`, Zuordnungen ändern (`move`, `addChoice`, `removeChoice`, `withCut`), Maße (`previewSize`, `sceneLayout`, `UNIT_PX`, `GAME_SCALE`), `applySaved`, `selectionOf`, `fileUrl`.
- `useAssets.ts`: Hook; lädt `backend.assetsList()`, lädt bei `assets:changed` neu, speichert mit `assetsSet(id, wahl, stand)`; bei Ablehnung (`isStale`) neu laden und Meldung zeigen.
- `CutEditor.tsx`: Ausschnitt einer Grafik-Zuordnung.
- `MiniScene.tsx`: Slot im Spielmaßstab.
- `SlotInfo.tsx`, `slotRows.ts`: Reiter Details des Inspektors (`infoGroups` aus den Slot-Details).
- Tests: `assets.test.ts`, `slotRows.test.ts`, `SlotInfo.test.tsx`.

## Flow
1. `useAssets` lädt Register und `stand`; `../assetpage/useAssetData` nutzt den Hook.
2. Eine Änderung der Zuordnung (`../assetpage/Belegung.tsx`) ruft `assetsSet(id, wahl, stand)`.
3. Neuer Stand → `applySaved`; Ablehnung → Neuladen mit Hinweis; fremder Schreiber → `assets:changed` → Neuladen.

## Integration
- Konsumenten: `../assetpage/` (Seite, Belegung, Inspektor), `../graphics/ScenesView` und `../sound/` (`SoundScenes`, `Proben`) über `useAssets` und `fileUrl`.
- Abhängigkeiten: `../api` (`backend`, `isStale`, `STALE`), `@webkit/atoms`, `@webkit/molecules`, `@webkit/lib/errors`.
