# tools/k3c-studio/frontend/src/graphics/

## Responsibility
Seite Grafik des Asset-Studios: Reiter Slots, Szenen und Packs; zeigt Bild-Slots im Spielmaßstab, Gebäude-Streifen, Hintergründe und Grafik-Packs.

## Design
- `GraphicsPage.tsx`: Reiter Slots (Grundseite aus `../assets/AssetsPage` mit Art Grafik), Szenen (`ScenesView`), Packs (`PacksView`).
- `ScenesView.tsx`: Szenen Figuren, Gebäude, Umgebung, Objekte, HUD und Vergleich (Präferenz n); `SlotCards.tsx` (`ImageCard`, `FigurenCards` über `renderReference` aus dem Spielcode), `BuildingStrip.tsx` (`BuildingStrip`: Baum-Reihe, Gebäude 32 px neben 16 px ×2, kachelbar, Stufenreihe, Vorlage aus `grafik/gebaeude/index.json`), `Backgrounds.tsx` (`Backgrounds`: je Biom drei Schichten fern → nah, zwei Kacheln mit Naht), `HubGrowthStrip.tsx` (`HubGrowthStrip`: Wachstumsreihe der Burg vom Lagerfeuer bis zur Kristallburg, Gerüst als zweites Bild, GEB11).
- `PacksView.tsx`: Pack-Übersicht aus `GRAFIK_PACKS` und `grafik/index.json`.
- `pixel.tsx`: `ladeBild`, `bildZeichnen`, `Pixel`; Zeichnen ganzzahlig ohne Glättung.
- `scenesLogic.ts`: reine Logik: `SCENES`, `slotsOf`, `resolveSlot` (Slot → Zuordnung → Bild), `licenseFlag` (NC-Kennzeichen), `figureCard`, `gebaeudeKarten`, `stufenKarten`, `baumReihe`, `hintergrundStreifen`, `hubWachstum` (Reihe aus `HUB_GROWTH` des Spielcodes); Test `scenesLogic.test.ts`.

## Flow
1. Reiter Szenen → `slotsOf(szene)` wählt die Slots → `resolveSlot` löst Bilder und Lücken auf → Karten zeichnen über `Pixel`.
2. Reiter Slots und Packs lesen Register bzw. `public/grafik/index.json`.

## Integration
- Abhängigkeiten: `../assets/`, `../api`, `@game` (`renderReference`, `GRAFIK_PACKS`), `../styles/szenen.css`.
- Eingehängt in `../App.tsx` (Seite `grafik`, Komponente `GraphicsPage`).
