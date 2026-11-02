# tools/k3c-studio/frontend/src/graphics/

## Responsibility
Bausteine der Grafik im Asset-Studio: Szenen-Karten, Gebäude-Streifen, Hintergründe, Wachstumsreihe und Pixel-Zeichnen im Spielmaßstab. Die Seite Grafik (Reiter Slots, Szenen, Packs) ist seit B-677 in `../assetpage/` aufgegangen; geblieben sind die Szenen-Bausteine, die die Sicht Szene nutzt.

## Design
- `ScenesView.tsx`: Szenen Figuren, Gebäude, Umgebung, Objekte, HUD und Vergleich (Präferenz n); eingebunden von `../assetpage/SceneView.tsx`.
- `SlotCards.tsx` (`ImageCard`, `FigurenCards` über `renderReference` aus dem Spielcode), `BuildingStrip.tsx` (`BuildingStrip`: Baum-Reihe, Gebäude 32 px neben 16 px ×2, kachelbar, Stufenreihe, Vorlage aus `grafik/gebaeude/index.json`), `Backgrounds.tsx` (`Backgrounds`: je Biom drei Schichten fern → nah, zwei Kacheln mit Naht), `HubGrowthStrip.tsx` (`HubGrowthStrip`: Wachstumsreihe der Burg vom Lagerfeuer bis zur Kristallburg, Gerüst als zweites Bild, GEB11).
- `pixel.tsx`: `ladeBild`, `bildZeichnen`, `Pixel`; Zeichnen ganzzahlig ohne Glättung.
- `scenesLogic.ts`: reine Logik: `SCENES`, `slotsOf`, `resolveSlot` (Slot → Zuordnung → Bild), `licenseFlag` (NC-Kennzeichen), `figureCard`, `gebaeudeKarten`, `stufenKarten`, `baumReihe`, `hintergrundStreifen`, `hubWachstum` (Reihe aus `HUB_GROWTH` des Spielcodes); Test `scenesLogic.test.ts`.
- Entfallen mit der Seite Grafik: `GraphicsPage.tsx`, `PacksView.tsx` (Pack-Übersicht; die Gruppierung nach Pack liegt jetzt in `../assetpage/filter.ts`).

## Flow
1. Sicht Szene → `SceneView` → `ScenesView` → `slotsOf(szene)` wählt die Slots → `resolveSlot` löst Bilder und Lücken auf → Karten zeichnen über `Pixel`.

## Integration
- Abhängigkeiten: `../assets/` (`useAssets`, `fileUrl`), `../api`, `@game` (`renderReference`), `../styles/szenen.css`.
- Eingebunden in `../assetpage/SceneView.tsx`; keine eigene Seite mehr in `../App.tsx`.
