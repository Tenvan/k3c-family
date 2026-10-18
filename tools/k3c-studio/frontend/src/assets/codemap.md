# tools/k3c-studio/frontend/src/assets/

## Responsibility
Gemeinsame Seitenstruktur für Grafik und Sound: Slot-Liste mit Filter, Detailansicht mit Zuordnungen, Katalog-Auswahl, Recherche, Ausschnitt-Editor und Mini-Szene, dazu das Laden und Speichern des Slot-Registers mit Stand.

## Design
- `assets.ts`: reine Logik ohne React: `filterSlots`, `countStatus`, Zuordnungen ändern (`move`, `addChoice`, `removeChoice`, `withCut`), Maße (`previewSize`, `sceneLayout`, `UNIT_PX`, `GAME_SCALE`), `applySaved`, `selectionOf`, `fileUrl`.
- `useAssets.ts`: Hook; lädt `backend.assetsList()`, lädt bei `assets:changed` neu, speichert mit `assetsSet(id, wahl, stand)`; bei Ablehnung (`isStale`) neu laden und Meldung zeigen.
- `AssetsPage.tsx`: Seite mit fester Art (Grafik oder Sound), Liste, Filter und `Splitter`; `SlotDetail.tsx` Detail eines Slots; `CatalogPicker.tsx` Katalogsuche; `ResearchPanel.tsx` Recherche mit Treffern, Import, Download und Lizenzfreigabe; `CutEditor.tsx` Ausschnitt; `MiniScene.tsx` Slot im Spielmaßstab.
- Tests: `assets.test.ts`, `reload.test.tsx` (happy-dom, Neuladen und Schreibregel).

## Flow
1. `useAssets` lädt Register und `stand`; die Liste zeigt Slots nach Filter.
2. Auswahl öffnet `SlotDetail`; eine Änderung der Zuordnung ruft `assetsSet(id, wahl, stand)`.
3. Neuer Stand → `applySaved`; Ablehnung → Neuladen mit Hinweis; fremder Schreiber → `assets:changed` → Neuladen.

## Integration
- Konsumenten: `../grafik/GrafikPage` und `../sound/SoundPage` (je mit eigener Art und eigenen Vorschau-Bausteinen).
- Abhängigkeiten: `../api` (`backend`, `isStale`), `../ui/`, `../lib/`.
