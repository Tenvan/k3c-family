# tools/k3c-workbench/frontend/src/assets/size/

## Responsibility

Größenvergleich (B-733) für Katalog und Review: das Asset in Spielgröße am Boden neben dem Monarchen, wahlweise vor
einem Biom-Hintergrund; mit Basis (B-853) Vorher und Nachher nebeneinander.

## Design

- `SizeCompare.tsx`: Einstieg mit `id`, `basis`, `nachher`; Biom-Wahl (zuletzt gewähltes Biom gilt für alle Assets und
  beide Seiten, ohne Biom neutraler Himmel und Boden), Zoom über `useZoom` (`../zoom.tsx`). Ohne Basis nur der
  Arbeitsstand; mit Basis zwei Szenen, beide mit dem Monarchen des Arbeitsstands als Maßstab. Fehlt das Asset in der
  Basis, steht dort der Grund.
- `SizeScene.tsx`: SVG-Szene in Spiel-Pixeln (`SZENE_H = 1080`, `BODEN = 900`, `UNIT = 32`); Asset links mit Blick
  nach rechts, Monarch rechts gespiegelt, Höhenlinie in Units und Pixeln, Maßstab hoch und breit; scrollt beim Zoomen auf
  Figuren und Boden.
- `sizeData.ts`: lädt je Figur Spec, Frame und Maßstab (`assetsGet`, `assetsFrame`, `assetsGroesse` bzw.
  `assetsGroesseBasis`) und die Hintergründe (`assetsHintergruende`, `assetsHintergrund`); `MONARCH = 'monarch'`,
  `vergleichsAnimation` (`idle`, sonst erste ohne Reittier, sonst erste); `figur(id, basis)`.
- `sizeCache.ts`: Modul-Cache ohne Backend-Import (`cached`, `clearSizeCache`); Fehler werden nicht gecacht. Ohne
  API-Import, damit reine Node-Tests den Review-Store laden können.

## Flow

1. `SizeCompare` -> `figur(id, basis)` und `figur(MONARCH)` -> `sizeData` -> `cached` -> Asset-Methoden des Backends.
2. Biom-Wahl -> `hintergruende()` / `hintergrund(teil)` -> Kacheln in `SizeScene`.
3. „Neu laden“ in Katalog oder Review -> `clearSizeCache`.

## Integration

- Konsumenten: `../catalog/CatalogPage.tsx`, `../review/index.tsx`; `clearSizeCache` aus `../catalog/catalogStore.ts`
  und `../review/reviewStore.ts`.
- Abhängigkeiten: `../api.ts`, `../zoom.tsx`, `@webkit/atoms|molecules`.
- Tests: `SizeCompare.test.tsx`.
