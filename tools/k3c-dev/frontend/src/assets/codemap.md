# tools/k3c-dev/frontend/src/assets/

## Responsibility
Reiter „Assets“ (B-298, Sprint M10): getrennte Ansichten Grafik und Sound aus dem Slot-Register `data/assets.json`; Zuordnungen nach Präferenz ordnen, Ausschnitt festlegen, Katalog durchsuchen, Mini-Szene (`assetvorschau.html`, B-299) als iframe.

## Design
- `assets.ts` (Vitest `assets.test.ts`): reine Logik – `filterSlots`, `countStatus`, `move`, `addChoice`, `removeChoice`, `withCut`, `previewSize` (×2, Units à 32 px), `sceneUrl`, `fileUrl`.
- `useAssets.ts`: lädt `backend.assetsList()`, speichert sofort mit `assetsSet` (Ablehnung aus Go in `error`), `startVite`.
- `AssetsPage.tsx`: Kopf (Art, Kategorie, Status, Suche, „Vite starten“), `SlotList`, `SlotDetail`.
- `SlotDetail.tsx`: `ChoiceList` (HTML5-Drag-and-Drop, ↑, ✕), `SoundProbe`, `ScenePreview`; `CutEditor.tsx` (Felder x/y/w/h, Canvas im Spielmaßstab, SA/NC-Hinweis nach Q70); `CatalogPicker.tsx` (entprellte Suche, ND gesperrt); `ResearchPanel.tsx` (Web-Recherche: Treffer mit Lizenz-Status, Vorschau, Link im Browser, Übernehmen/Importieren …, Lizenz bestätigen, Verwerfen).

## Flow
1. Reiter öffnen → `assetsList` (Slots mit Status, Vite-Adresse).
2. Umordnen, Hinzufügen, Entfernen, Ausschnitt → `assetsSet` → Go prüft und schreibt `data/assets.json` → Vite lädt die Mini-Szene neu.

## Integration
- Backend nur über `src/api/` (`AssetsList`, `AssetsSet`, `AssetsCatalog` in `app_assets.go`; Mock `mockAssets.ts`, Vite-Adresse im Browser über `?preview=`).
- Bilder und Mini-Szene vom Vite-Dienst des Spiels.
