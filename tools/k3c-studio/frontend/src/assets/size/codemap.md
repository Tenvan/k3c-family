# tools/k3c-studio/frontend/src/assets/size/

## Responsibility

Größenvergleich (B-733, B-1170) für Katalog und Review: das Asset in Spielgröße am Boden neben dem Monarchen, wahlweise
vor einem Biom-Hintergrund, mit Lineal in M (1 M = berittener Monarch) und Größenstufen aus `data/sizes.json`; mit Basis
(B-853) Vorher und Nachher nebeneinander.

## Design

- `SizeCompare.tsx`: Einstieg mit `id`, `basis`, `nachher`; Biom-Wahl (zuletzt gewähltes Biom gilt für alle Assets und
  beide Seiten, ohne Biom neutraler Himmel und Boden). `useSzenenDaten` lädt Figuren, Monarch, Biome und Stufen. Ohne
  Basis nur der Arbeitsstand; mit Basis zwei Szenen, beide mit dem Monarchen des Arbeitsstands als Maßstab und einem
  gemeinsamen Szenenmaß (`gemeinsamesMass`, `groesseresMass`: gleiche M sehen gleich groß aus). Fehlt das Asset in der
  Basis, steht dort der Grund. `Stufenwahl` unter der Szene: Klick auf eine Größenstufe zeichnet ihre Höhe als Linie ein
  (zweiter Klick nimmt sie zurück), die Stufe des Assets ist hervorgehoben; fehlt `sizes.json`, steht „Kein Lineal in M“.
  Keine Zoomstufen mehr.
- `SizeScene.tsx`: SVG-Szene in Spiel-Pixeln (`BODEN = 900`, `UNIT = 32`), die das Fenster füllt (`viewBox` mit
  `preserveAspectRatio`, Mindestmaße, Streifen `UNTEN` für den Maßstab in Units); `szenenMass` rechnet Breite und Höhe aus
  Asset, Monarch, Stufen und gewählter Stufe. Asset links mit Blick nach rechts, Monarch rechts gespiegelt; Lineal in M
  links, Marken und Linien (`1 M = Monarch`, Stufe des Assets, gewählte Stufe), Höhenlinie des Assets in M, Units und
  Pixeln.
- `sizeData.ts`: lädt je Figur Spec, Frame und Maßstab (`assetsGet`, `assetsFrame`, `assetsGroesse` bzw.
  `assetsGroesseBasis`), die Hintergründe (`assetsHintergruende`, `assetsHintergrund`) und die Größenstufen (`stufen()` über
  `assetsStufen`; `stufeVon(stufen, id)` sucht die Stufe über den Schlüssel `<gruppe>/<id>`); `MONARCH = 'monarch'`,
  `vergleichsAnimation` (`idle`, sonst erste ohne Reittier, sonst erste); `figur(id, basis)`.
- `sizeCache.ts`: Modul-Cache ohne Backend-Import (`cached`, `clearSizeCache`); Fehler werden nicht gecacht. Ohne
  API-Import, damit reine Node-Tests den Review-Store laden können.

## Flow

1. `SizeCompare` -> `figur(id, basis)` und `figur(MONARCH)` -> `sizeData` -> `cached` -> Asset-Methoden des Backends.
2. Biom-Wahl -> `hintergruende()` / `hintergrund(teil)` -> Kacheln in `SizeScene`; `stufen()` liefert das Lineal, die
   Stufenwahl setzt `markiert`.
3. „Neu laden“ in Katalog oder Review -> `clearSizeCache`.

## Integration

- Konsumenten: `../catalog/CatalogPage.tsx`, `../review/index.tsx`; `clearSizeCache` aus `../catalog/catalogStore.ts`
  und `../review/reviewStore.ts`.
- Abhängigkeiten: `../api.ts`, `@webkit/atoms|molecules`.
- Tests: `SizeCompare.test.tsx`.
