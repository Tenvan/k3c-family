# tools/k3c-studio/frontend/src/assets/catalog/

## Responsibility

Seite **Katalog** im Bereich Studio (WB1.2, WB1.3): Liste aller Papierschnitt-Specs, Vorschau des gewählten Assets mit
Animation, Frame-Player, Teil-Auswahl per Klick, Teil-Ausblenden und Prompt-Buttons. Nur Ansehen; Änderungen laufen über
die MCP-Tools `assets_*`.

## Design

- `CatalogPage.tsx`: Wurzel der Seite; links `AssetList`, rechts `Preview` mit `TeilLayer` (Teil-Auswahl) oder
  `SizeCompare` (Größenvergleich mit dem Monarchen vor einem Biom, B-733), darunter `PromptButtons`, `TeilSicht` und
  `GameOpen`. `{zustand}` für die Prompts kommt aus `zustandVon` (`../prompts/fill.ts`).
- `AssetList.tsx`: Specs nach Gruppe; ungültige rot mit `feld · regel` und Text, Abzeichen „im Diff“; Knopf „Neu laden“.
  Filter (B-1169, WZ62.2): Text, Welt mit Modus gesamt/exklusiv (`WeltFilter`, Sicht aus `useWeltSicht`, Zuordnung über
  `zuordnung` aus `@webkit/lib/weltFilter`), Gruppe, Art (`rig`) und „nur geänderte“; der Zustand steht über
  `useWeltFilter` in der Adresse. Einträge zeigen „exklusiv“ bzw. „geerbt von <Welt>“, Zähler je Modus; `ListStatus`
  meldet Assets-Laden, Welt-Laden oder keine Treffer; eine noch nicht umgesetzte Welt trägt einen Hinweis.
- `Preview.tsx`: Animation per `ChipSwitch`, Frame-Schieber, Abspielen mit `fps`; lädt beim Abspielen die Frames
  nacheinander vor (höchstens ein Render offen, Abbruch bei Anhalten oder Wechsel). Zoom über `useZoom` (`../zoom.tsx`,
  Start 2×); das Bild bleibt verborgen, bis seine Größe bekannt ist.
- `TeilSicht.tsx`: gewähltes Teil aus- und einblenden (B-734); gilt bis zum Asset-Wechsel.
- `catalogStore.ts`: Modul-Cache (B-732) für Liste, Prompt-Vorlagen, IDs im Diff, `specCache` und `frameCache`; überlebt
  den Tab-Wechsel. Die Liste kommt ungeprüft (`assetsIndex`), die Prüfung folgt in Häppchen zu 8 (`assetsCheck`);
  `currentGeneration` verwirft Antworten eines älteren Ladens. „Neu laden“ und ein Neuverbinden leeren den Cache (samt
  `clearSizeCache` aus `../size/sizeCache.ts`).
- `useCatalog.ts`: `useCatalog` (= Store-Hook), `useSpec` (Fehler je Asset statt Abbruch), `cachedFrame` (Promise je
  Asset/Animation/Frame/ausgeblendete Teile; Fehler bleiben bis zum Asset-Wechsel) und der Bild-Hook, der das alte Bild
  bis zum neuen stehen lässt.
- `catalogTesting.tsx`: Test-Helfer (`mountCatalog`, `select`, `playFrames`), die auf Bedingungen statt feste Zeiten warten.

## Flow

1. `CatalogPage` -> `useCatalog` -> `catalogStore` lädt `assetsIndex`, `assetsPrompts`, `assetsReview('HEAD')` (IDs im
   Diff) -> `assetsCheck` in Häppchen setzt Gültigkeit und Fehler je Eintrag.
2. Auswahl in `AssetList` -> `useSpec(id)` (`assetsGet`) -> `Preview` -> `cachedFrame` -> `assetsFrame` bzw.
   `assetsFrameOhne` (mit ausgeblendeten Teilen) -> PNG als Data-URL.
3. Klick ins Bild -> `TeilLayer` -> `teilAn` (`../prompts/hit.ts`) -> gewähltes Teil -> `PromptButtons` füllen die
   Vorlagen mit `fillPrompt`.
4. Backend-Neuverbindung (`backend.on('ui:reconnect')`) -> `loadCatalog(true)` leert den Cache und lädt neu.

## Integration

- Eingehängt von `../AssetsArea.tsx` (Reiter `katalog`).
- Abhängigkeiten: `../api.ts` (Asset-Methoden des `backend`), `../prompts/`, `../size/`, `../game/GameOpen.tsx`,
  `../zoom.tsx`, `@webkit/atoms|molecules`.
- Tests: eigenes Vitest des Frontends gegen den Mock (`src/api/mock*`), Fixtures aus `tools/paper/fixtures/ui`.
